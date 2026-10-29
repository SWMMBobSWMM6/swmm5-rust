//! controls.c of EPA SWMM 5.2.4, in safe Rust.
//!
//-----------------------------------------------------------------------------
//   controls.c
//
//   Project:  EPA SWMM5
//   Version:  5.2
//   Date:     06/01/22  (Build 5.2.1)
//   Author:   L. Rossman
//
//   Rule-based controls functions.
//
//   Control rules have the format:
//     RULE name
//     IF <premise>
//     AND / OR <premise>
//     etc.
//     THEN <action>
//     AND  <action>
//     etc.
//     ELSE <action>
//     AND  <action>
//     etc.
//     PRIORITY <p>
//
//   <premise> consists of:
//      <variable> <relational operator> value / <variable>
//   where <variable> is <object type> <id name> <attribute>
//   E.g.: Node 123 Depth > 4.5
//         Node 456 Depth < Node 123 Depth
//
//   <action> consists of:
//      <variable> = setting
//   E.g.: Pump abc status = OFF
//         Weir xyz setting = 0.5
//
//  Update History
//  ==============
//  Build 5.1.008:
//  - Support added for r.h.s. variables in rule premises.
//  - Node volume added as a premise variable.
//  Build 5.1.009:
//  - Fixed problem with parsing a RHS premise variable.
//  Build 5.1.010:
//  - Support added for link TIMEOPEN & TIMECLOSED premises.
//  Build 5.1.011:
//  - Support added for DAYOFYEAR attribute.
//  - Modulated controls no longer included in reported control actions.
//  Build 5.2.0:
//  - Additional attributes added to condition clauses.
//  - Support added for named variables in condition clauses.
//  - Support added for math expressions in condition clauses.
//  Build 5.2.1:
//  - A refactoring bug from 5.2.0 causing duplicate actions to be added
//    to the list of control actions to take was fixed.
//-----------------------------------------------------------------------------
//
// Representation notes:
// * A rule's premise list is built by appending (lastPremise->next = p), so
//   `premises` is a Vec in input order.
// * A rule's THEN and ELSE action lists are built by prepending
//   (a->next = Rules[r].thenActions), so `thenActions`/`elseActions` hold
//   the actions in reverse input order, which is the order C walks them.
// * The global ActionList is a linked list of items, each pointing to an
//   action (or NULL after clearActionList); new items are prepended and
//   items are never freed until controls_delete. Here it is a Vec of
//   Option<ActionRef> in C list order, where an ActionRef names an action
//   by rule, list (THEN/ELSE) and position in that list.

use crate::prelude::*;

//-----------------------------------------------------------------------------
//  Constants
//-----------------------------------------------------------------------------
// enum RuleState
const r_RULE: i32 = 0;
const r_IF: i32 = 1;
const r_AND: i32 = 2;
const r_OR: i32 = 3;
const r_THEN: i32 = 4;
const r_ELSE: i32 = 5;
const r_PRIORITY: i32 = 6;
const r_VARIABLE: i32 = 7;
const r_EXPRESSION: i32 = 8;
const r_ERROR: i32 = 9;

// enum RuleObject
const r_GAGE: i32 = 0;
const r_NODE: i32 = 1;
const r_LINK: i32 = 2;
const r_CONDUIT: i32 = 3;
const r_PUMP: i32 = 4;
const r_ORIFICE: i32 = 5;
const r_WEIR: i32 = 6;
const r_OUTLET: i32 = 7;
const r_SIMULATION: i32 = 8;

// enum RuleAttrib
const r_DEPTH: i32 = 0;
const r_MAXDEPTH: i32 = 1;
const r_HEAD: i32 = 2;
const r_VOLUME: i32 = 3;
const r_INFLOW: i32 = 4;
const r_FLOW: i32 = 5;
const r_FULLFLOW: i32 = 6;
const r_FULLDEPTH: i32 = 7;
const r_STATUS: i32 = 8;
const r_SETTING: i32 = 9;
const r_LENGTH: i32 = 10;
const r_SLOPE: i32 = 11;
const r_VELOCITY: i32 = 12;
const r_TIMEOPEN: i32 = 13;
const r_TIMECLOSED: i32 = 14;
const r_TIME: i32 = 15;
const r_DATE: i32 = 16;
const r_CLOCKTIME: i32 = 17;
const r_DAYOFYEAR: i32 = 18;
const r_DAY: i32 = 19;
const r_MONTH: i32 = 20;

// enum RuleRelation
const EQ: i32 = 0;
const NE: i32 = 1;
const LT: i32 = 2;
const LE: i32 = 3;
const GT: i32 = 4;
const GE: i32 = 5;

// enum RuleSetting
const r_CURVE: i32 = 0;
const r_TIMESERIES: i32 = 1;
const r_PID: i32 = 2;
const r_NUMERIC: i32 = 3;

const MAXVARNAME: i32 = 32;

static ObjectWords: &[&[u8]] =
    &[b"GAGE", b"NODE", b"LINK", b"CONDUIT", b"PUMP", b"ORIFICE", b"WEIR", b"OUTLET",
      b"SIMULATION"];
static AttribWords: &[&[u8]] =
    &[b"DEPTH", b"MAXDEPTH", b"HEAD", b"VOLUME", b"INFLOW",
      b"FLOW", b"FULLFLOW", b"FULLDEPTH", b"STATUS", b"SETTING",
      b"LENGTH", b"SLOPE", b"VELOCITY", b"TIMEOPEN", b"TIMECLOSED",
      b"TIME", b"DATE", b"CLOCKTIME", b"DAYOFYEAR", b"DAY", b"MONTH"];
static RelOpWords: &[&[u8]] = &[b"=", b"<>", b"<", b"<=", b">", b">="];
static StatusWords: &[&[u8]] = &[b"OFF", b"ON"];
static ConduitWords: &[&[u8]] = &[b"CLOSED", b"OPEN"];
static SettingTypeWords: &[&[u8]] = &[b"CURVE", b"TIMESERIES", b"PID"];
static IntensityWord: &[u8] = b"INTENSITY";

//-----------------------------------------------------------------------------
// Data Structures
//-----------------------------------------------------------------------------
// Rule Premise Variable
#[derive(Clone, Copy, Default)]
struct TVariable {
    object: i32,          // type of object
    index: i32,           // index in object's array
    attribute: i32,       // object's attribute
}

// Named Variable
#[derive(Clone, Copy)]
struct TNamedVariable {
    variable: TVariable,                        // a rule premise variable
    name: [u8; (MAXVARNAME + 1) as usize],      // name used in math expression
}
impl Default for TNamedVariable {
    fn default() -> Self {
        Self { variable: TVariable::default(), name: [0; (MAXVARNAME + 1) as usize] }
    }
}

// Rule Premise Function
#[derive(Clone)]
struct TExpression {
    expression: Option<MathExpr>,               // tokenized math expression
    name: [u8; (MAXVARNAME + 1) as usize],      // expression name
}
impl Default for TExpression {
    fn default() -> Self {
        Self { expression: None, name: [0; (MAXVARNAME + 1) as usize] }
    }
}

// Rule Premise Clause
#[derive(Clone, Copy, Default)]
struct TPremise {
    type_: i32,                 // clause type (IF/AND/OR)
    exprIndex: i32,             // expression index (-1 if N/A)
    lhsVar: TVariable,          // left hand side variable
    rhsVar: TVariable,          // right hand side variable
    relation: i32,              // relational operator (>, <, =, etc)
    value: f64,                 // right hand side value
    // next: the next element of TRule.premises
}

// Rule Action Clause
#[derive(Clone, Copy, Default)]
struct TAction {
    rule: i32,             // index of rule that action belongs to
    link: i32,             // index of link being controlled
    attribute: i32,        // attribute of link being controlled
    curve: i32,            // index of curve for modulated control
    tseries: i32,          // index of time series for modulated control
    value: f64,            // control setting for link attribute
    kp: f64,               // coeffs. for PID modulated control
    ki: f64,
    kd: f64,
    e1: f64,               // PID set point error from previous time steps
    e2: f64,
    // next: the next element of the rule's action list
}

// List of Control Actions: an item of ActionList points to an action
// (C: struct TActionList { TAction* action; TActionList* next; })
#[derive(Clone, Copy, PartialEq)]
struct ActionRef {
    rule: usize,           // rule holding the action
    elseList: bool,        // false: in thenActions, true: in elseActions
    k: usize,              // position in that list
}

// Control Rule
#[derive(Clone, Default)]
struct TRule {
    ID: Option<Vec<u8>>,              // rule ID
    priority: f64,                    // priority level
    premises: Vec<TPremise>,          // C: firstPremise ... lastPremise
    thenActions: Vec<TAction>,        // linked list of actions if true
    elseActions: Vec<TAction>,        // linked list of actions if false
}

/// The file-scope variables of controls.c.
#[derive(Default)]
pub struct ControlsState {
    Rules: Vec<TRule>,                    // array of control rules
    ActionList: Vec<Option<ActionRef>>,   // linked list of control actions
    InputState: i32,                      // state of rule interpreter
    RuleCount: i32,                       // total number of rules
    ControlValue: f64,                    // value of controller variable
    SetPoint: f64,                        // value of controller setpoint
    CurrentDate: DateTime,                // current date in whole days
    CurrentTime: DateTime,                // current time of day (decimal)

    VariableCount: i32,
    ExpressionCount: i32,
    CurrentVariable: i32,
    CurrentExpression: i32,
    NamedVariable: Vec<TNamedVariable>,   // array of named variables
    Expression: Vec<TExpression>,         // array of math expressions
}

//-----------------------------------------------------------------------------
//  External functions (declared in funcs.h)
//-----------------------------------------------------------------------------
//     controls_create
//     controls_delete
//     controls_init
//     controls_addToCount
//     controls_addVariable
//     controls_addExpression
//     controls_addRuleClause
//     controls_evaluate

//=============================================================================

/// input.c's `match` (case-insensitive test that `substr` begins `str` after
/// its leading blanks). The skeleton has no stub for it (`match` is a Rust
/// keyword); this private copy stands in until input.rs provides one.
fn r#match(str: &[u8], substr: &[u8]) -> i32
//
//  Input:   str = character string being searched
//           substr = sub-string being searched for
//  Output:  returns 1 if sub-string found, 0 if not
//  Purpose: sees if a sub-string of characters appears in a string
//           (not case sensitive).
//
{
    // a byte of a C string: 0 at and past its end
    let at = |s: &[u8], i: usize| -> u8 { if i < s.len() { s[i] } else { 0 } };
    let mut i: usize;
    let mut j: usize;
    let mut k: usize;

    // --- fail if substring is empty
    if at(substr, 0) == 0 {
        return 0;
    }

    // --- skip leading blanks of str
    k = 0;
    while at(str, k) != 0 {
        if at(str, k) != b' ' {
            break;
        }
        k += 1;
    }

    // --- check if substr matches remainder of str
    i = k;
    j = 0;
    while at(substr, j) != 0 {
        if at(str, i) == 0 || UCHAR(at(str, i)) != UCHAR(at(substr, j)) {
            return 0;
        }
        i += 1;
        j += 1;
    }
    1
}

//=============================================================================

/// The action an ActionRef names.
fn action(ctl: &ControlsState, a: ActionRef) -> &TAction {
    let rule = &ctl.Rules[a.rule];
    if a.elseList { &rule.elseActions[a.k] } else { &rule.thenActions[a.k] }
}

fn action_mut(ctl: &mut ControlsState, a: ActionRef) -> &mut TAction {
    let rule = &mut ctl.Rules[a.rule];
    if a.elseList { &mut rule.elseActions[a.k] } else { &mut rule.thenActions[a.k] }
}

/// A rule's ID as C's "%s" prints it (glibc prints a NULL string as "(null)").
fn ruleID(ctl: &ControlsState, r: usize) -> Vec<u8> {
    match &ctl.Rules[r].ID {
        Some(id) => id.clone(),
        None => b"(null)".to_vec(),
    }
}

//=============================================================================

/// Called by: input.
pub fn controls_init(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: initializes the control rule system.
//
{
    let c = &mut prj.controls;
    c.Rules = Vec::new();
    c.NamedVariable = Vec::new();
    c.Expression = Vec::new();
    c.RuleCount = 0;
    c.VariableCount = 0;
    c.ExpressionCount = 0;
}

//=============================================================================

/// Called by: input.
pub fn controls_addToCount(prj: &mut Project, s: &[u8])
//
//  Input:   s = either VARIABLE or EXPRESSION
//  Output:  none
//  Purpose: updates the number of named variables or math expressions used
//           by control rules.
//
{
    if r#match(s, w_VARIABLE) != 0 {
        prj.controls.VariableCount += 1;
    } else if r#match(s, w_EXPRESSION) != 0 {
        prj.controls.ExpressionCount += 1;
    }
}

//=============================================================================

/// Called by: project.
pub fn controls_create(prj: &mut Project, n: i32) -> i32
//
//  Input:   n = total number of control rules
//  Output:  returns error code
//  Purpose: creates an array of control rules.
//
{
    let c = &mut prj.controls;
    c.ActionList = Vec::new();
    c.InputState = r_PRIORITY;
    c.RuleCount = n;
    if c.RuleCount > 0 {
        // calloc'd, then ID, premises and action lists set to NULL and
        // priority to 0.0: all of which is TRule::default()
        c.Rules = vec![TRule::default(); c.RuleCount as usize];
    }

    c.CurrentVariable = -1;
    c.CurrentExpression = -1;
    if c.VariableCount > 0 {
        c.NamedVariable = vec![TNamedVariable::default(); c.VariableCount as usize];
    }
    if c.ExpressionCount > 0 {
        c.Expression = vec![TExpression::default(); c.ExpressionCount as usize];
    }
    0
}

//=============================================================================

/// Called by: project.
pub fn controls_delete(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: deletes all control rules.
//
{
    let mut i: i32;

    i = 0;
    while i < prj.controls.ExpressionCount {
        // C dereferences Expression[i] here; Expression is still NULL when
        // controls_create never ran (input error found while counting
        // objects), and C crashes. Here such entries are skipped.
        if let Some(e) = prj.controls.Expression.get_mut(i as usize) {
            mathexpr_delete(e.expression.take());
            e.expression = None;
        }
        i += 1;
    }
    prj.controls.Expression = Vec::new();
    prj.controls.NamedVariable = Vec::new();

    if prj.controls.RuleCount == 0 {
        return;
    }
    deleteActionList(prj);
    deleteRules(prj);
}

//=============================================================================

/// Called by: input.
pub fn controls_addVariable(prj: &mut Project, tok: &[Vec<u8>], nToks: i32) -> i32
//
//  Input:   tok = an array of string tokens
//           n = the size of tok[]
//  Output:  returns error code
//  Purpose: adds a named variable to the control rule system from a
//           tokenized line of input with formats:
//    VARIABLE  name = Object  id  attribute
//    VARIABLE  name = SIMULATION attribute
//
{
    let mut v1 = TVariable::default();
    let mut k: i32;
    let err: i32;

    prj.controls.CurrentVariable += 1;
    if nToks < 5 {
        return ERR_ITEMS;
    }
    if findExactMatch(&tok[1], AttribWords) >= 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
    }
    if r#match(&tok[2], b"=") == 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[2]);
    }
    if r#match(&tok[3], b"SIMULATION") == 0 && nToks < 6 {
        return ERR_ITEMS;
    }
    k = 3;
    err = getPremiseVariable(prj, tok, nToks, &mut k, &mut v1);
    if err > 0 {
        return err;
    }
    k = prj.controls.CurrentVariable;
    let nv = &mut prj.controls.NamedVariable[k as usize];
    nv.variable = v1;
    sstrncpy(&mut nv.name, &tok[1], MAXVARNAME as usize);
    0
}

//=============================================================================

/// Called by: input.
pub fn controls_addExpression(prj: &mut Project, tok: &[Vec<u8>], nToks: i32) -> i32
//
//  Input:   tok = an array of string tokens
//           n = number of tokens
//  Output:  returns error code
//  Purpose: adds a math expression to the control rule system from a
//           a tokenized line of input with format:
//    EXPRESSION  name = <math expression containing VARIABLE names>
//
{
    let mut i: i32;
    let k: i32;
    let mut s = [0u8; MAXLINE as usize + 1];
    let expr: Option<MathExpr>;

    prj.controls.CurrentExpression += 1;
    if nToks < 4 {
        return ERR_ITEMS;
    }
    k = prj.controls.CurrentExpression;
    prj.controls.Expression[k as usize].expression = None;
    sstrncpy(&mut prj.controls.Expression[k as usize].name, &tok[1], MAXVARNAME as usize);
    sstrncpy(&mut s, &tok[3], MAXLINE as usize);
    i = 4;
    while i < nToks {
        sstrcat(&mut s, b" ", MAXLINE as usize);
        sstrcat(&mut s, &tok[i as usize], MAXLINE as usize);
        i += 1;
    }

    {
        let pr: &Project = prj;
        expr = mathexpr_create(cio::cstr(&s), &mut |varName: &[u8]| getVariableIndex(pr, varName));
    }
    if expr.is_none() {
        return error_setInpError(prj, ERR_MATH_EXPR, b"");
    }

    prj.controls.Expression[k as usize].expression = expr;
    0
}

//=============================================================================

fn getVariableIndex(prj: &Project, varName: &[u8]) -> i32
//
//  Input:   varName = string containing a variable name
//  Output:  returns the index of the named variable or -1 if not found
//  Purpose: finds the array index of a named variable.
//
{
    let mut i: i32 = 0;
    while i < prj.controls.VariableCount {
        if r#match(varName, &prj.controls.NamedVariable[i as usize].name) != 0 {
            return i;
        }
        i += 1;
    }
    -1
}

//=============================================================================

fn getNamedVariableValue(prj: &Project, varIndex: i32) -> f64
//
//  Input:   varIndex = index of a named variable
//  Output:  returns the current value of the variable
//  Purpose: finds the value of a named variable.
//
{
    getVariableValue(prj, prj.controls.NamedVariable[varIndex as usize].variable)
}

//=============================================================================

fn getExpressionIndex(prj: &Project, exprName: &[u8]) -> i32
//
//  Input:   exprName = string containing an expression name
//  Output:  returns the index of the expression or -1 if not found
//  Purpose: finds the array index of a math expression
//
{
    let mut i: i32 = 0;
    while i < prj.controls.ExpressionCount {
        if r#match(exprName, &prj.controls.Expression[i as usize].name) != 0 {
            return i;
        }
        i += 1;
    }
    -1
}

//=============================================================================

/// Called by: input.
pub fn controls_addRuleClause(prj: &mut Project, r: i32, keyword: i32, tok: &[Vec<u8>], nToks: i32) -> i32
//
//  Input:   r = rule index
//           keyword = the clause's keyword code (IF, THEN, etc.)
//           tok = an array of string tokens that comprises the clause
//           nToks = number of tokens
//  Output:  returns an error  code
//  Purpose: addd a new clause to a control rule.
//
{
    match keyword {
        r_RULE => {
            if prj.controls.Rules[r as usize].ID.is_none() {
                let id = project_findID(prj, CONTROL, &tok[1]);
                prj.controls.Rules[r as usize].ID = id;
            }
            prj.controls.InputState = r_RULE;
            if nToks > 2 {
                return ERR_RULE;
            }
            0
        }

        r_IF => {
            if prj.controls.InputState != r_RULE {
                return ERR_RULE;
            }
            prj.controls.InputState = r_IF;
            addPremise(prj, r, r_AND, tok, nToks)
        }

        r_AND => {
            if prj.controls.InputState == r_IF {
                addPremise(prj, r, r_AND, tok, nToks)
            } else if prj.controls.InputState == r_THEN || prj.controls.InputState == r_ELSE {
                addAction(prj, r, tok, nToks)
            } else {
                ERR_RULE
            }
        }

        r_OR => {
            if prj.controls.InputState != r_IF {
                return ERR_RULE;
            }
            addPremise(prj, r, r_OR, tok, nToks)
        }

        r_THEN => {
            if prj.controls.InputState != r_IF {
                return ERR_RULE;
            }
            prj.controls.InputState = r_THEN;
            addAction(prj, r, tok, nToks)
        }

        r_ELSE => {
            if prj.controls.InputState != r_THEN {
                return ERR_RULE;
            }
            prj.controls.InputState = r_ELSE;
            addAction(prj, r, tok, nToks)
        }

        r_PRIORITY => {
            if prj.controls.InputState != r_THEN && prj.controls.InputState != r_ELSE {
                return ERR_RULE;
            }
            prj.controls.InputState = r_PRIORITY;
            // getDouble stores strtod's result even when it reports failure
            let mut priority = prj.controls.Rules[r as usize].priority;
            let ok = getDouble(&tok[1], &mut priority);
            prj.controls.Rules[r as usize].priority = priority;
            if ok == 0 {
                return ERR_NUMBER;
            }
            if nToks > 2 {
                return ERR_RULE;
            }
            0
        }

        _ => 0,
    }
}

//=============================================================================

/// Called by: routing.
pub fn controls_evaluate(prj: &mut Project, currentTime: DateTime, elapsedTime: DateTime, tStep: f64) -> i32
//
//  Input:   currentTime = current simulation date/time
//           elapsedTime = decimal days since start of simulation
//           tStep = simulation time step (days)
//  Output:  returns number of new actions taken
//  Purpose: evaluates all control rules at current time of the simulation.
//
{
    let mut r: i32;                    // control rule index
    let mut result: i32;               // TRUE if rule premises satisfied

    // --- save date and time to shared variables
    prj.controls.CurrentDate = currentTime.floor();
    prj.controls.CurrentTime = currentTime - currentTime.floor();
    prj.ElapsedTime = elapsedTime;

    // --- evaluate each rule
    if prj.controls.RuleCount == 0 {
        return 0;
    }
    clearActionList(prj);
    r = 0;
    while r < prj.controls.RuleCount {
        // --- evaluate rule's premises
        result = TRUE;
        let mut k: usize = 0;
        while k < prj.controls.Rules[r as usize].premises.len() {
            let p = prj.controls.Rules[r as usize].premises[k];
            if p.type_ == r_OR {
                if result == FALSE {
                    result = evaluatePremise(prj, &p, tStep);
                }
            } else {
                if result == FALSE {
                    break;
                }
                result = evaluatePremise(prj, &p, tStep);
            }
            k += 1;
        }

        // --- if premises true, add THEN clauses to action list
        //     else add ELSE clauses to action list
        let elseList = result != TRUE;
        let nActions = if elseList {
            prj.controls.Rules[r as usize].elseActions.len()
        } else {
            prj.controls.Rules[r as usize].thenActions.len()
        };
        let mut k: usize = 0;
        while k < nActions {
            let a = ActionRef { rule: r as usize, elseList, k };
            updateActionValue(prj, a, currentTime, tStep);
            updateActionList(prj, a);
            k += 1;
        }
        r += 1;
    }

    // --- execute actions on action list
    if !prj.controls.ActionList.is_empty() {
        executeActionList(prj, currentTime)
    } else {
        0
    }
}

//=============================================================================

fn addPremise(prj: &mut Project, r: i32, type_: i32, tok: &[Vec<u8>], nToks: i32) -> i32
//
//  Input:   r = control rule index
//           type = type of premise (IF, AND, OR)
//           tok = array of string tokens containing premise statement
//           nToks = number of string tokens
//  Output:  returns an error code
//  Purpose: adds a new premise to a control rule.
//
{
    let relation: i32;
    let mut n: i32;
    let mut err: i32 = 0;
    let mut value: f64 = MISSING;
    let mut v1 = TVariable::default();
    let mut v2 = TVariable::default();
    let obj: i32;
    let exprIndex: i32;
    let mut varIndex: i32 = -1;

    // --- initialize LHS variable v1
    if nToks < 4 {
        return ERR_ITEMS;
    }
    v1.attribute = -1;
    v1.object = -1;
    v1.index = -1;
    n = 1;

    // --- check if 2nd token is a math expression
    exprIndex = getExpressionIndex(prj, &tok[1]);

    // --- if not then check if it's a named variable
    if exprIndex < 0 {
        varIndex = getVariableIndex(prj, &tok[n as usize]);
        if varIndex >= 0 {
            v1 = prj.controls.NamedVariable[varIndex as usize].variable;
        }
        // otherwise parse object|index|attribute tokens
        else {
            err = getPremiseVariable(prj, tok, nToks, &mut n, &mut v1);
            if err > 0 {
                return err;
            }
        }
    }

    // --- get relational operator
    n += 1;
    if n >= nToks {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    relation = findExactMatch(&tok[n as usize], RelOpWords);
    if relation < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[n as usize]);
    }

    // --- initialize RHS variable v2
    v2.attribute = -1;
    v2.object = -1;
    v2.index = -1;
    n += 1;
    if n >= nToks {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check for named RHS variable
    varIndex = getVariableIndex(prj, &tok[n as usize]);
    if varIndex >= 0 {
        v2 = prj.controls.NamedVariable[varIndex as usize].variable;
    }
    // --- check for object|index|attribute variable
    else {
        obj = findmatch(&tok[n as usize], ObjectWords);
        if obj >= 0 {
            err = getPremiseVariable(prj, tok, nToks, &mut n, &mut v2);
            if err > 0 {
                return ERR_RULE;
            }
            if exprIndex < 0 && v1.attribute != v2.attribute {
                let id = ruleID(&prj.controls, r as usize);
                report_writeWarningMsg(prj, WARN11, &id);
            }
        }
        // --- check for a single RHS value
        else {
            err = getPremiseValue(prj, &tok[n as usize], v1.attribute, &mut value);
            if err > 0 {
                return err;
            }
        }
    }

    // --- make sure another clause is not on same line
    n += 1;
    if n < nToks && findmatch(&tok[n as usize], RuleKeyWords) >= 0 {
        return ERR_RULE;
    }

    // --- create the premise object
    let p = TPremise {
        type_,
        exprIndex,
        lhsVar: v1,
        rhsVar: v2,
        relation,
        value,
    };
    // appended after the rule's last premise
    prj.controls.Rules[r as usize].premises.push(p);
    0
}

//=============================================================================

fn getPremiseVariable(prj: &mut Project, tok: &[Vec<u8>], nToks: i32, k: &mut i32, v: &mut TVariable) -> i32
//
//  Input:   tok = array of string tokens
//           nToks = number of tokens
//           k = index of current token
//  Output:  returns an error code; updates k to new current token and
//           places identity of specified variable in v
//  Purpose: parses a variable (e.g., Node 123 Depth) used in a control rule.
//
{
    let mut n = *k;
    let mut object: i32 = -1;
    let mut index: i32 = -1;
    let obj: i32;
    let attrib: i32;

    // --- get object type
    obj = findmatch(&tok[n as usize], ObjectWords);
    if obj < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[n as usize]);
    }

    // --- get object index from its name
    n += 1;
    if n >= nToks {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    match obj {
        r_GAGE => {
            index = project_findObject(prj, GAGE, &tok[n as usize]);
            if index < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[n as usize]);
            }
            object = r_GAGE;
        }

        r_NODE => {
            index = project_findObject(prj, NODE, &tok[n as usize]);
            if index < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[n as usize]);
            }
            object = r_NODE;
        }

        r_LINK | r_CONDUIT | r_PUMP | r_ORIFICE | r_WEIR | r_OUTLET => {
            index = project_findObject(prj, LINK, &tok[n as usize]);
            if index < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[n as usize]);
            }
            object = r_LINK;
        }
        _ => {
            n -= 1;
        }
    }
    n += 1;
    if n >= nToks {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- get attribute index from its name
    if object == r_GAGE {
        attrib = getGageAttrib(&tok[n as usize]);
    } else {
        attrib = findmatch(&tok[n as usize], AttribWords);
    }
    if attrib < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[n as usize]);
    }

    // --- check that attribute belongs to object type
    if obj == r_GAGE {
    } else if obj == r_NODE {
        match attrib {
            r_DEPTH | r_MAXDEPTH | r_HEAD | r_VOLUME | r_INFLOW => {}
            _ => return error_setInpError(prj, ERR_KEYWORD, &tok[n as usize]),
        }
    }
    // --- check for link TIMEOPEN & TIMECLOSED attributes
    else if object == r_LINK && index >= 0
        && (attrib == r_TIMEOPEN || attrib == r_TIMECLOSED)
    {
        // nothing to do here
    } else if obj == r_LINK || obj == r_CONDUIT {
        match attrib {
            r_STATUS | r_DEPTH | r_FULLFLOW | r_FULLDEPTH | r_FLOW | r_LENGTH | r_SLOPE
            | r_VELOCITY => {}
            _ => return error_setInpError(prj, ERR_KEYWORD, &tok[n as usize]),
        }
    } else if obj == r_PUMP {
        match attrib {
            r_FLOW | r_SETTING | r_STATUS => {}
            _ => return error_setInpError(prj, ERR_KEYWORD, &tok[n as usize]),
        }
    } else if obj == r_ORIFICE || obj == r_WEIR || obj == r_OUTLET {
        match attrib {
            r_FLOW | r_SETTING => {}
            _ => return error_setInpError(prj, ERR_KEYWORD, &tok[n as usize]),
        }
    } else {
        match attrib {
            r_TIME | r_DATE | r_CLOCKTIME | r_DAY | r_MONTH | r_DAYOFYEAR => {}
            _ => return error_setInpError(prj, ERR_KEYWORD, &tok[n as usize]),
        }
    }

    // --- populate variable structure
    v.object = object;
    v.index = index;
    v.attribute = attrib;
    *k = n;
    0
}

//=============================================================================

fn getGageAttrib(token: &[u8]) -> i32
//
//  Input:   token = a string token
//  Output:  returns an attribute code or -1 if an error occurred
//  Purpose: determines the atrribute code for a rain gage variable.
//  Note:    a valid token is INTENSITY for current rainfall intensity
//           (attribute code = 0) or nHR_PRECIP for total rain depth
//           over past n hours (attribute code = n).
//
{
    let attrib: i32;

    // --- check if token is currrent rainfall intensity
    if r#match(token, IntensityWord) != 0 {
        return 0;
    }

    // --- token is past rain depth - read number of past hours
    attrib = cio::atoi(token);

    // --- check that number of hours is in allowable range
    if attrib < 1 || attrib > MAXPASTRAIN {
        return -1;
    }
    attrib
}

//=============================================================================

fn getPremiseValue(prj: &mut Project, token: &[u8], attrib: i32, value: &mut f64) -> i32
//
//  Input:   token = a string token
//           attrib = index of a node/link attribute
//  Output:  value = attribute value;
//           returns an error code;
//  Purpose: parses the numerical value of a particular node/link attribute
//           in the premise clause of a control rule.
//
{
    let mut strDate = [0u8; 25];
    match attrib {
        r_STATUS => {
            *value = findmatch(token, StatusWords) as f64;
            if *value < 0.0 {
                *value = findmatch(token, ConduitWords) as f64;
            }
            if *value < 0.0 {
                return error_setInpError(prj, ERR_KEYWORD, token);
            }
        }

        r_TIME | r_CLOCKTIME | r_TIMEOPEN | r_TIMECLOSED => {
            if datetime_strToTime(token, value) == 0 {
                return error_setInpError(prj, ERR_DATETIME, token);
            }
        }

        r_DATE => {
            if datetime_strToDate(prj, token, value) == 0 {
                return error_setInpError(prj, ERR_DATETIME, token);
            }
        }

        r_DAY => {
            if getDouble(token, value) == 0 {
                return error_setInpError(prj, ERR_NUMBER, token);
            }
            if *value < 1.0 || *value > 7.0 {
                return error_setInpError(prj, ERR_DATETIME, token);
            }
        }

        r_MONTH => {
            if getDouble(token, value) == 0 {
                return error_setInpError(prj, ERR_NUMBER, token);
            }
            if *value < 1.0 || *value > 12.0 {
                return error_setInpError(prj, ERR_DATETIME, token);
            }
        }

        r_DAYOFYEAR => {
            sstrncpy(&mut strDate, token, 6);
            sstrcat(&mut strDate, b"/1947", 25);
            if datetime_strToDate(prj, cio::cstr(&strDate), value) != 0 {
                *value = datetime_dayOfYear(*value) as f64;
            } else if getDouble(token, value) == 0 || *value < 1.0 || *value > 365.0 {
                return error_setInpError(prj, ERR_DATETIME, token);
            }
        }

        _ => {
            if getDouble(token, value) == 0 {
                return error_setInpError(prj, ERR_NUMBER, token);
            }
        }
    }
    0
}

//=============================================================================

fn addAction(prj: &mut Project, r: i32, tok: &[Vec<u8>], nToks: i32) -> i32
//
//  Input:   r = control rule index
//           tok = array of string tokens containing action statement
//           nToks = number of string tokens
//  Output:  returns an error code
//  Purpose: adds a new action to a control rule.
//
{
    let obj: i32;
    let link: i32;
    let mut attrib: i32;
    let mut curve: i32 = -1;
    let mut tseries: i32 = -1;
    let mut n: i32;
    let err: i32;
    let mut values: [f64; 3] = [1.0, 0.0, 0.0];

    // --- check for proper number of tokens
    if nToks < 6 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }

    // --- check for valid object type
    obj = findmatch(&tok[1], ObjectWords);
    if obj != r_LINK && obj != r_CONDUIT && obj != r_PUMP &&
       obj != r_ORIFICE && obj != r_WEIR && obj != r_OUTLET
    {
        return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
    }

    // --- check that object name exists and is of correct type
    link = project_findObject(prj, LINK, &tok[2]);
    if link < 0 {
        return error_setInpError(prj, ERR_NAME, &tok[2]);
    }
    let linkType = prj.Link[link as usize].type_;
    match obj {
        r_CONDUIT => {
            if linkType != CONDUIT {
                return error_setInpError(prj, ERR_NAME, &tok[2]);
            }
        }
        r_PUMP => {
            if linkType != PUMP {
                return error_setInpError(prj, ERR_NAME, &tok[2]);
            }
        }
        r_ORIFICE => {
            if linkType != ORIFICE {
                return error_setInpError(prj, ERR_NAME, &tok[2]);
            }
        }
        r_WEIR => {
            if linkType != WEIR {
                return error_setInpError(prj, ERR_NAME, &tok[2]);
            }
        }
        r_OUTLET => {
            if linkType != OUTLET {
                return error_setInpError(prj, ERR_NAME, &tok[2]);
            }
        }
        _ => {}
    }

    // --- check for valid attribute name
    attrib = findmatch(&tok[3], AttribWords);
    if attrib < 0 {
        return error_setInpError(prj, ERR_KEYWORD, &tok[3]);
    }

    // --- get control action setting
    if obj == r_CONDUIT {
        if attrib == r_STATUS {
            values[0] = findmatch(&tok[5], ConduitWords) as f64;
            if values[0] < 0.0 {
                return error_setInpError(prj, ERR_KEYWORD, &tok[5]);
            }
        } else {
            return error_setInpError(prj, ERR_KEYWORD, &tok[3]);
        }
    } else if obj == r_PUMP {
        if attrib == r_STATUS {
            values[0] = findmatch(&tok[5], StatusWords) as f64;
            if values[0] < 0.0 {
                return error_setInpError(prj, ERR_KEYWORD, &tok[5]);
            }
        } else if attrib == r_SETTING {
            err = setActionSetting(prj, tok, nToks, &mut curve, &mut tseries,
                                   &mut attrib, &mut values);
            if err > 0 {
                return err;
            }
        } else {
            return error_setInpError(prj, ERR_KEYWORD, &tok[3]);
        }
    } else if obj == r_ORIFICE || obj == r_WEIR || obj == r_OUTLET {
        if attrib == r_SETTING {
            err = setActionSetting(prj, tok, nToks, &mut curve, &mut tseries,
                                   &mut attrib, &mut values);
            if err > 0 {
                return err;
            }
            if attrib == r_SETTING
                && (values[0] < 0.0 || values[0] > 1.0)
            {
                return error_setInpError(prj, ERR_NUMBER, &tok[5]);
            }
        } else {
            return error_setInpError(prj, ERR_KEYWORD, &tok[3]);
        }
    } else {
        return error_setInpError(prj, ERR_KEYWORD, &tok[1]);
    }

    // --- check if another clause is on same line
    n = 6;
    if curve >= 0 || tseries >= 0 {
        n = 7;
    }
    if attrib == r_PID {
        n = 9;
    }
    if n < nToks && findmatch(&tok[n as usize], RuleKeyWords) >= 0 {
        return ERR_RULE;
    }

    // --- create the action object
    //     (C mallocs it: kp, ki, kd, e1, e2 stay uninitialized unless the
    //     action is a PID one, and are only read for PID actions)
    let mut a = TAction::default();
    a.rule = r;
    a.link = link;
    a.attribute = attrib;
    a.curve = curve;
    a.tseries = tseries;
    a.value = values[0];
    if attrib == r_PID {
        a.kp = values[0];
        a.ki = values[1];
        a.kd = values[2];
        a.e1 = 0.0;
        a.e2 = 0.0;
    }
    // prepended to the rule's THEN or ELSE list
    if prj.controls.InputState == r_THEN {
        prj.controls.Rules[r as usize].thenActions.insert(0, a);
    } else {
        prj.controls.Rules[r as usize].elseActions.insert(0, a);
    }
    0
}

//=============================================================================

fn setActionSetting(prj: &mut Project, tok: &[Vec<u8>], nToks: i32, curve: &mut i32, tseries: &mut i32,
                    attrib: &mut i32, values: &mut [f64; 3]) -> i32
//
//  Input:   tok = array of string tokens containing action statement
//           nToks = number of string tokens
//  Output:  curve = index of controller curve
//           tseries = index of controller time series
//           attrib = r_PID if PID controller used
//           values = values of control settings
//           returns an error code
//  Purpose: identifies how control actions settings are determined.
//
{
    let k: i32;
    let mut m: i32;

    // --- see if control action is determined by a Curve or Time Series
    if nToks < 6 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    k = findmatch(&tok[5], SettingTypeWords);
    if k >= 0 && nToks < 7 {
        return error_setInpError(prj, ERR_ITEMS, b"");
    }
    match k {
        // --- control determined by a curve - find curve index
        r_CURVE => {
            m = project_findObject(prj, CURVE, &tok[6]);
            if m < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[6]);
            }
            *curve = m;
        }

        // --- control determined by a time series - find time series index
        r_TIMESERIES => {
            m = project_findObject(prj, TSERIES, &tok[6]);
            if m < 0 {
                return error_setInpError(prj, ERR_NAME, &tok[6]);
            }
            *tseries = m;
            prj.Tseries[m as usize].refersTo = CONTROL;
        }

        // --- control determined by PID controller
        r_PID => {
            if nToks < 9 {
                return error_setInpError(prj, ERR_ITEMS, b"");
            }
            m = 6;
            while m <= 8 {
                if getDouble(&tok[m as usize], &mut values[(m - 6) as usize]) == 0 {
                    return error_setInpError(prj, ERR_NUMBER, &tok[m as usize]);
                }
                m += 1;
            }
            *attrib = r_PID;
        }

        // --- direct numerical control is used
        _ => {
            if getDouble(&tok[5], &mut values[0]) == 0 {
                return error_setInpError(prj, ERR_NUMBER, &tok[5]);
            }
        }
    }
    0
}

//=============================================================================

fn updateActionValue(prj: &mut Project, a: ActionRef, currentTime: DateTime, dt: f64)
//
//  Input:   a = an action object
//           currentTime = current simulation date/time (days)
//           dt = time step (days)
//  Output:  none
//  Purpose: updates value of actions found from Curves or Time Series.
//
{
    let act = *action(&prj.controls, a);
    if act.curve >= 0 {
        let cv = prj.controls.ControlValue;
        let v = table_lookup(&mut prj.Curve[act.curve as usize], cv);
        action_mut(&mut prj.controls, a).value = v;
    } else if act.tseries >= 0 {
        let v = table_tseriesLookup(&mut prj.Tseries[act.tseries as usize], currentTime, TRUE as i8);
        action_mut(&mut prj.controls, a).value = v;
    } else if act.attribute == r_PID {
        let v = getPIDSetting(prj, a, dt);
        action_mut(&mut prj.controls, a).value = v;
    }
}

//=============================================================================

fn getPIDSetting(prj: &mut Project, a: ActionRef, mut dt: f64) -> f64
//
//  Input:   a = an action object
//           dt = current time step (days)
//  Output:  returns a new link setting
//  Purpose: computes a new setting for a link subject to a PID controller.
//
//  Note:    a->kp = gain coefficient,
//           a->ki = integral time (minutes)
//           a->k2 = derivative time (minutes)
//           a->e1 = error from previous time step
//           a->e2 = error from two time steps ago
{
    let mut e0: f64;
    let mut setting: f64;
    let p: f64;
    let i: f64;
    let d: f64;
    let mut update: f64;
    let tolerance: f64 = 0.0001;

    // --- convert time step from days to minutes
    dt *= 1440.0;

    // --- determine relative error in achieving controller set point
    let SetPoint = prj.controls.SetPoint;
    let ControlValue = prj.controls.ControlValue;
    e0 = SetPoint - ControlValue;
    if e0.abs() > TINY {
        if SetPoint != 0.0 {
            e0 = e0 / SetPoint;
        } else {
            e0 = e0 / ControlValue;
        }
    }

    let act = action_mut(&mut prj.controls, a);

    // --- reset previous errors to 0 if controller gets stuck
    if (e0 - act.e1).abs() < tolerance {
        act.e2 = 0.0;
        act.e1 = 0.0;
    }

    // --- use the recursive form of the PID controller equation to
    //     determine the new setting for the controlled link
    p = e0 - act.e1;
    if act.ki == 0.0 {
        i = 0.0;
    } else {
        i = e0 * dt / act.ki;
    }
    d = act.kd * (e0 - 2.0 * act.e1 + act.e2) / dt;
    update = act.kp * (p + i + d);
    if update.abs() < tolerance {
        update = 0.0;
    }

    // --- update previous errors
    act.e2 = act.e1;
    act.e1 = e0;
    let link = act.link;

    setting = prj.Link[link as usize].targetSetting + update;

    // --- check that new setting lies within feasible limits
    if setting < 0.0 {
        setting = 0.0;
    }
    if prj.Link[link as usize].type_ != PUMP && setting > 1.0 {
        setting = 1.0;
    }
    setting
}

//=============================================================================

fn updateActionList(prj: &mut Project, a: ActionRef)
//
//  Input:   a = an action object
//  Output:  none
//  Purpose: adds a new action to the list of actions to be taken.
//
{
    let ctl = &mut prj.controls;
    let aRule = action(ctl, a).rule;
    let aLink = action(ctl, a).link;
    let priority = ctl.Rules[aRule as usize].priority;

    // --- check if link referred to in action is already listed
    let mut listItem: usize = 0;
    while listItem < ctl.ActionList.len() {
        let a1 = match ctl.ActionList[listItem] {
            Some(a1) => a1,
            None => break,
        };
        let a1Link = action(ctl, a1).link;
        if a1Link == aLink {
            // --- replace old action if new action has higher priority
            let a1Rule = action(ctl, a1).rule;
            if priority > ctl.Rules[a1Rule as usize].priority {
                ctl.ActionList[listItem] = Some(a);
            }
            return;
        }
        listItem += 1;
    }

    // --- action not listed so add it to ActionList                           //5.2.1
    if listItem == ctl.ActionList.len() {
        // new item prepended to the list
        ctl.ActionList.insert(0, None);
        listItem = 0;
    }
    ctl.ActionList[listItem] = Some(a);
}

//=============================================================================

fn executeActionList(prj: &mut Project, currentTime: DateTime) -> i32
//
//  Input:   currentTime = current date/time of the simulation
//  Output:  returns number of new actions taken
//  Purpose: executes all actions required by fired control rules.
//
{
    let mut count: i32 = 0;

    let mut listItem: usize = 0;
    while listItem < prj.controls.ActionList.len() {
        let a1r = match prj.controls.ActionList[listItem] {
            Some(a1r) => a1r,
            None => break,
        };
        let a1 = *action(&prj.controls, a1r);
        if a1.link >= 0 {
            if prj.Link[a1.link as usize].targetSetting != a1.value {
                prj.Link[a1.link as usize].targetSetting = a1.value;
                if prj.RptFlags.controls != 0 && a1.curve < 0
                    && a1.tseries < 0 && a1.attribute != r_PID
                {
                    let linkID = prj.Link[a1.link as usize].ID.clone();
                    let rID = ruleID(&prj.controls, a1.rule as usize);
                    report_writeControlAction(prj, currentTime, &linkID,
                                              a1.value, &rID);
                }
                count += 1;
            }
        }
        listItem += 1;
    }
    count
}

//=============================================================================

fn evaluatePremise(prj: &mut Project, p: &TPremise, tStep: f64) -> i32
//
//  Input:   p = a control rule premise condition
//           tStep = current time step (days)
//  Output:  returns TRUE if the condition is true or FALSE otherwise
//  Purpose: evaluates the truth of a control rule premise condition.
//
{
    let lhsValue: f64;
    let rhsValue: f64;
    let mut result: i32 = FALSE;

    // --- check if left hand side (lhs) of premise is an expression
    if p.exprIndex >= 0 {
        let pr: &Project = prj;
        lhsValue = match &pr.controls.Expression[p.exprIndex as usize].expression {
            Some(expr) => mathexpr_eval(expr,
                &mut |varIndex: i32| getNamedVariableValue(pr, varIndex)),
            // C: mathexpr_eval(NULL, ...) evaluates nothing and returns 0.0
            None => 0.0,
        };
    }
    // --- otherwise get value of the lhs variable
    else {
        lhsValue = getVariableValue(prj, p.lhsVar);
    }

    // --- if right hand side (rhs) of premise is a variable then get its value
    if p.value == MISSING {
        rhsValue = getVariableValue(prj, p.rhsVar);
    } else {
        rhsValue = p.value;
    }
    if lhsValue == MISSING || rhsValue == MISSING {
        return FALSE;
    }

    // --- compare the lhs of the premise to the rhs
    match p.lhsVar.attribute {
        r_TIME | r_CLOCKTIME => compareTimes(prj, lhsValue, p.relation, rhsValue, tStep / 2.0),
        r_TIMEOPEN | r_TIMECLOSED => {
            result = compareTimes(prj, lhsValue, p.relation, rhsValue, tStep / 2.0);
            prj.controls.ControlValue = lhsValue * 24.0;  // convert time from days to hours
            result
        }
        _ => compareValues(prj, lhsValue, p.relation, rhsValue),
    }
}

//=============================================================================

fn getVariableValue(prj: &Project, v: TVariable) -> f64
{
    let mut i: i32 = -1;    // a node index
    let mut j: i32 = -1;    // a link index

    if v.object == r_GAGE {
        return getRainValue(prj, v);
    }
    if v.object == r_NODE {
        i = v.index;
    }
    if v.object == r_LINK {
        j = v.index;
    }

    match v.attribute {
        r_TIME => prj.ElapsedTime,

        r_DATE => prj.controls.CurrentDate,

        r_CLOCKTIME => prj.controls.CurrentTime,

        r_DAY => datetime_dayOfWeek(prj.controls.CurrentDate) as f64,

        r_MONTH => datetime_monthOfYear(prj.controls.CurrentDate) as f64,

        r_DAYOFYEAR => datetime_dayOfYear(prj.controls.CurrentDate) as f64,

        r_STATUS => {
            if j < 0 ||
                (prj.Link[j as usize].type_ != CONDUIT && prj.Link[j as usize].type_ != PUMP)
            {
                MISSING
            } else {
                prj.Link[j as usize].setting
            }
        }

        r_SETTING => {
            if j < 0 || (prj.Link[j as usize].type_ != PUMP &&
                         prj.Link[j as usize].type_ != ORIFICE &&
                         prj.Link[j as usize].type_ != WEIR)
            {
                MISSING
            } else {
                prj.Link[j as usize].setting
            }
        }

        r_FLOW => {
            if j < 0 {
                MISSING
            } else {
                prj.Link[j as usize].direction as i32 as f64 * prj.Link[j as usize].newFlow
                    * UCF(prj, FLOW)
            }
        }

        r_FULLFLOW | r_FULLDEPTH | r_VELOCITY | r_LENGTH | r_SLOPE => {
            if j < 0 {
                return MISSING;
            } else if prj.Link[j as usize].type_ != CONDUIT {
                return MISSING;
            }
            let lk = &prj.Link[j as usize];
            match v.attribute {
                r_FULLFLOW => lk.qFull * UCF(prj, FLOW),
                r_FULLDEPTH => lk.xsect.yFull * UCF(prj, LENGTH),
                r_VELOCITY => {
                    link_getVelocityShared(prj, j, lk.newFlow, lk.newDepth)
                        * UCF(prj, LENGTH)
                }
                r_LENGTH => prj.Conduit[lk.subIndex as usize].length * UCF(prj, LENGTH),
                r_SLOPE => prj.Conduit[lk.subIndex as usize].slope,
                _ => MISSING,
            }
        }

        r_DEPTH => {
            if j >= 0 {
                prj.Link[j as usize].newDepth * UCF(prj, LENGTH)
            } else if i >= 0 {
                prj.Node[i as usize].newDepth * UCF(prj, LENGTH)
            } else {
                MISSING
            }
        }

        r_MAXDEPTH => {
            if i >= 0 {
                prj.Node[i as usize].fullDepth * UCF(prj, LENGTH)
            } else {
                MISSING
            }
        }

        r_HEAD => {
            if i < 0 {
                return MISSING;
            }
            (prj.Node[i as usize].newDepth + prj.Node[i as usize].invertElev) * UCF(prj, LENGTH)
        }

        r_VOLUME => {
            if i < 0 {
                return MISSING;
            }
            prj.Node[i as usize].newVolume * UCF(prj, VOLUME)
        }

        r_INFLOW => {
            if i < 0 {
                MISSING
            } else {
                prj.Node[i as usize].newLatFlow * UCF(prj, FLOW)
            }
        }

        r_TIMEOPEN => {
            if j < 0 {
                return MISSING;
            }
            if prj.Link[j as usize].setting <= 0.0 {
                return MISSING;
            }
            prj.controls.CurrentDate + prj.controls.CurrentTime - prj.Link[j as usize].timeLastSet
        }

        r_TIMECLOSED => {
            if j < 0 {
                return MISSING;
            }
            if prj.Link[j as usize].setting > 0.0 {
                return MISSING;
            }
            prj.controls.CurrentDate + prj.controls.CurrentTime - prj.Link[j as usize].timeLastSet
        }

        _ => MISSING,
    }
}

//=============================================================================

fn getRainValue(prj: &Project, v: TVariable) -> f64
//
//  Input:   v = a rule premise variable for a rain gage
//  Output:  returns current or past rainfall amount
//  Purpose: retrieves either the current rainfall intensity or the past
//           rainfall total for a rain gage.
//
{
    if v.index < 0 {
        MISSING
    } else if prj.Gage[v.index as usize].isUsed == FALSE {
        0.0
    } else if v.attribute == 0 {
        prj.Gage[v.index as usize].rainfall
    } else {
        gage_getPastRain(prj, v.index, v.attribute)
    }
}

//=============================================================================

fn compareTimes(prj: &mut Project, lhsValue: f64, relation: i32, rhsValue: f64, halfStep: f64) -> i32
//
//  Input:   lhsValue = date/time value on left hand side of relation
//           relation = relational operator code (see RuleRelation enumeration)
//           rhsValue = date/time value on right hand side of relation
//           halfStep = 1/2 the current time step (days)
//  Output:  returns TRUE if time relation is satisfied
//  Purpose: evaluates the truth of a relation between two date/times.
//
{
    if relation == EQ {
        if lhsValue >= rhsValue - halfStep
            && lhsValue < rhsValue + halfStep
        {
            return TRUE;
        }
        FALSE
    } else if relation == NE {
        if lhsValue < rhsValue - halfStep
            || lhsValue >= rhsValue + halfStep
        {
            return TRUE;
        }
        FALSE
    } else {
        compareValues(prj, lhsValue, relation, rhsValue)
    }
}

//=============================================================================

fn compareValues(prj: &mut Project, lhsValue: f64, relation: i32, rhsValue: f64) -> i32
//  Input:   lhsValue = value on left hand side of relation
//           relation = relational operator code (see RuleRelation enumeration)
//           rhsValue = value on right hand side of relation
//  Output:  returns TRUE if relation is satisfied
//  Purpose: evaluates the truth of a relation between two values.
{
    prj.controls.SetPoint = rhsValue;
    prj.controls.ControlValue = lhsValue;
    match relation {
        EQ => {
            if lhsValue == rhsValue {
                return TRUE;
            }
        }
        NE => {
            if lhsValue != rhsValue {
                return TRUE;
            }
        }
        LT => {
            if lhsValue < rhsValue {
                return TRUE;
            }
        }
        LE => {
            if lhsValue <= rhsValue {
                return TRUE;
            }
        }
        GT => {
            if lhsValue > rhsValue {
                return TRUE;
            }
        }
        GE => {
            if lhsValue >= rhsValue {
                return TRUE;
            }
        }
        _ => {}
    }
    FALSE
}

//=============================================================================

fn clearActionList(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: clears the list of actions to be executed.
//
{
    for listItem in prj.controls.ActionList.iter_mut() {
        *listItem = None;
    }
}

//=============================================================================

fn deleteActionList(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: frees the memory used to hold the list of actions to be executed.
//
{
    prj.controls.ActionList = Vec::new();
}

//=============================================================================

fn deleteRules(prj: &mut Project)
//
//  Input:   none
//  Output:  none
//  Purpose: frees the memory used for all of the control rules.
//
{
    // each rule's premises and actions are owned by the rule
    prj.controls.Rules = Vec::new();
    prj.controls.RuleCount = 0;
}

//=============================================================================

fn findExactMatch(s: &[u8], keyword: &[&[u8]]) -> i32
//
//  Input:   s = character string
//           keyword = array of keyword strings
//  Output:  returns index of keyword which matches s or -1 if no match found
//  Purpose: finds exact match between string and array of keyword strings.
//
{
    let mut i: usize = 0;
    while i < keyword.len() {
        if strcomp(s, keyword[i]) != 0 {
            return i as i32;
        }
        i += 1;
    }
    -1
}
