#!/usr/bin/env python3
"""The swmm5-rust essay: one source, rendered into the app's Doc tab and into Word.

Blocks: ('h2', title, anchor) ('p', html) ('fig', key, caption) ('code', lang, text)
('table', [header...], [[row...]...]) ('ul', [html...]) ('ol', [html...])
Numbers in {braces} are filled from measured results by build.py.
Written in the manner of Robert A. Heinlein explaining a piece of engineering.
"""

TITLE = "Every Byte Accounted For"
SUBTITLE = "EPA SWMM 5.2.4 translated into safe Rust and held to EPA's output byte for byte, and what the translation turned up on the way"

BLOCKS = [
('h2', 'The job', 'job'),
('p', "The job was easy to state. Take the stormwater engine that most of the world's drainage engineers run, EPA SWMM 5.2.4: "
      "{C_FILES} C files of {C_LINES_C} lines, {C_LINES} with their headers. Write it again in Rust, in the part of Rust that will not let a program touch "
      "memory it does not own. Change nothing. Not the physics, not the order of the arithmetic, not the wording of a warning, "
      "not the last bit of a floating-point number in the binary output file."),
('p', "Landing on the Moon is easy to state, too. The trouble is in the doing, and the doing is mostly bookkeeping. So before "
      "anything else, settle how you will know you are done. Here is the test, and it is the only one that counts: run a model "
      "through EPA's engine and through the new one, then compare the two binary <code>.out</code> files byte for byte and the "
      "two <code>.rpt</code> reports line for line, after striking the three lines that print the wall clock. Either the bytes "
      "match or they do not. There is no partial credit, no \"close enough\", no tolerance to argue over at the next meeting."),
('p', "By that test, swmm5-rust matches EPA on {S221} of the {N221} models of the smoke and coverage set. The other {UB221} are "
      "models where EPA's C reads or writes memory it does not own, so there is no EPA answer to match; we will come to them. "
      "This essay is about how the translation was done, why it was worth doing, and what it turned up in the C: four "
      "defects that had to be copied faithfully, two of which we found written down nowhere else, and six inputs on which the C "
      "has no defined answer at all."),

('h2', 'Why bother', 'why'),
('p', "A fair question. SWMM 5 works. Bob Dickinson, who asked for this translation, has worked on SWMM since the Fortran "
      "days of 1973 at the University of Florida and co-wrote <code>rdii.c</code> for EPA's C version (the file's header "
      "carries his name beside Lew Rossman's), so he knows better than most "
      "how well it works. Why rewrite a working engine?"),
('p', "Because C makes a promise it cannot keep. It promises to do exactly what you wrote, and it does, including when what "
      "you wrote is to read the eleventh element of a ten-element array. In C that is not an error. It is \"undefined behaviour\", "
      "which means the compiler is excused from caring what happens next. Usually what happens next is that the program reads "
      "whatever bytes sit in that spot and carries on. Sometimes it crashes. Sometimes the answer depends on what the program "
      "did a minute earlier. The engineer at the keyboard sees none of this. He sees a report."),
('p', "Rust draws a line through the language. On one side, called safe Rust, the compiler checks every array index, every "
      "reference and every lifetime, and refuses to build a program it cannot prove keeps its hands to itself. On the other side, "
      "marked with the keyword <code>unsafe</code>, you may do as you like, as in C. A translation that lives on the safe side "
      "cannot read past an array. If it tries, it stops and names the line, and no number computed from the wrong memory reaches the report. That is worth having "
      "in an engine whose reports go into permits, bond issues and flood maps."),
('fig', 'unsafe', "Lines of source in each tree, and how many the compiler cannot check. The safe engine carries "
                  "<code>#![forbid(unsafe_code)]</code>, so an unsafe block anywhere in it is a build error, not a code-review finding."),
('p', "Notice the bar for c2rust. There is a tool of that name that turns C into Rust mechanically, and it does a fine job of it. "
      "But it has to keep every pointer, so every one of the {C2R_FNS} functions it produces is marked <code>unsafe</code>, and every "
      "global and static variable becomes a <code>static mut</code>, {C2R_STATICS} of them, that any function may change at any time. That is "
      "C written with Rust spelling. Bob had already been down that road with an earlier port, and it taught him something we will "
      "use in a moment. The point of this one was to get all the way across."),

('h2', 'Two translations, one judge', 'two'),
('p', "Here is the plan, and it is the plan a careful engineer uses on any big job: never take two steps at once. "
      "First, run c2rust over EPA's source and get a Rust crate that is the C, statement for statement. That crate is ugly, unsafe "
      "and never ships. What it is good for is this: it is a second copy of EPA's engine that can be compiled into the same program "
      "as the safe translation. Put the two side by side, step them one routing step at a time, and compare every depth, flow and "
      "volume after every step. The first variable whose bits differ points at the function that computed it. That tool is called "
      "<code>lockstep</code>, and it turns \"the report is different somewhere\" into \"this line, this time step\"."),
('fig', 'pipeline', "The route from C to safe Rust. Both translations face the same judge: the output bytes of EPA 5.2.4 built from "
                    "the official v5.2.4 tag."),
('p', "Second, write a skeleton for the safe engine: the data types, one structure called <code>Project</code> that owns the "
      "whole model, and an empty stub for every function. Third, translate the 54 modules into that skeleton, in ten batches that "
      "ran side by side, every batch working from the same rule book. Fourth, put it together and run the gate. The baseline went "
      "in at 9:12 on the evening of 28 October 2026. The last module was merged and the first model came out byte-identical at "
      "twenty past midnight. The gates have been running since."),
('p', "The c2rust crate passed its own gate, {C2R_SAME} of 221 models identical, the six memory-error decks included: a "
      "statement-for-statement copy goes wrong where the C goes wrong, and here it went wrong the same way. But not on the first try. On the first try "
      "{C2R_W09} reports differed by one line. That line is a story in itself."),

('h2', 'The number that would not fit', 'cast'),
('p', "Line 251 of <code>gage.c</code> works out how often a rain gage's time series records, in seconds, and puts the answer "
      "in an <code>int</code>:"),
('code', 'c', "gageInterval = (int)(floor(Tseries[k].dxMin*SECperDAY + 0.5));"),
('p', "<code>dxMin</code> is the smallest step between two points of the series, in days. A series with only one point has no "
      "steps, so <code>dxMin</code> keeps its starting value, which SWMM calls <code>BIG</code>: ten billion days. Multiply by "
      "86,400 seconds a day and you have about 8.6 × 10¹⁴. The largest number an <code>int</code> holds is 2,147,483,647. "
      "The number does not fit."),
('p', "C does not say what happens then. The C standard calls it undefined, and each machine answers in its own way. An Intel "
      "or AMD processor's conversion instruction answers with the most negative <code>int</code> there is, -2,147,483,648, which the "
      "manual calls the \"integer indefinite\" value. An ARM processor answers with the largest positive one. Rust does not leave it "
      "to the machine: its <code>as</code> conversion clamps to the nearest value that fits, so it agrees with ARM. Four lines later, "
      "the engine writes WARNING 09 if the gage's recording interval is less than <code>gageInterval</code>. Negative, no warning; "
      "huge, warning. Same C, same model, different report, depending on the processor."),
('fig', 'cast', "The same conversion on three machines. EPA's reference build runs on x86-64, so its answer is the one that "
                "has to be reproduced."),
('p', "Bob had met this exact line before. His earlier Rust port, the c2rust one, differed from EPA in 25 reports by one warning "
      "line, and the hunt for the cause ended at line 251. The OpenSWMM code review of 2026 lists it too, as CRASH-21. So this time the fix went in at the bottom, once, for everybody. Every "
      "float-to-integer conversion in swmm5-rust, {CASTS} of them, goes through a little module called <code>ccast</code> whose "
      "functions give the x86 answer on purpose. In range they are exactly Rust's <code>as</code>. Out of range they return what "
      "EPA's build returns. The WebAssembly build of EPA's C, by the way, gives the x86 answer too, because Clang guards the "
      "conversion and substitutes the same value."),
('p', "Is that a fix? No. It is a faithful copy of a defect. The defect is in the C, and it belongs in EPA's bug list, not "
      "in a translation. A translation that quietly fixed it would fail the gate, and a translation that fails the gate for good "
      "reasons cannot be told apart from one that fails it for bad ones. Keep the contract simple and you can check it."),

('h2', 'The rule book', 'rules'),
('p', "Ten batches translated side by side will drift apart unless they share rules, so the rules were written down first, in a "
      "file called <code>TRANSLATING.md</code>, and every batch got the same copy. The heart of it fits on a card:"),
('ul', ["<b>Translate literally.</b> Same functions, same names, same parameters, same local variables, same statements in the "
        "same order, same comments. The C field <code>Node[j].newDepth</code> is <code>prj.Node[j].newDepth</code>.",
        "<b>Arithmetic is bit for bit.</b> <code>a*b*c</code> is <code>(a*b)*c</code>, and that is not <code>a*(b*c)</code> in "
        "floating point. Do not tidy it. Do not replace <code>x/2.0</code> with <code>x*0.5</code>. Call the same <code>pow</code> "
        "and <code>exp</code> from the same library.",
        "<b>Strings are bytes.</b> Element names and report lines are byte arrays, never Unicode text, because that is what "
        "C's <code>char</code> is.",
        "<b>Every global is a field.</b> There are no globals. EPA's global variables become fields of "
        "<code>Project</code>, {PROJECT_GLOBALS} of them, and each module's file-scope statics sit in a state structure of its own inside it, {PROJECT_MODS} "
        "of them. Every function that touches the model takes it as its first argument, called <code>prj</code>.",
        "<b>No unsafe, anywhere.</b> The crate forbids it, so the rule enforces itself."]),
('p', "Most of the work after that is a matter of saying in Rust what C said with a pointer. A pointer that may be NULL becomes "
      "an <code>Option</code>, so the NULL check becomes part of the type and cannot be forgotten. A <code>malloc</code>'d array "
      "becomes a <code>Vec</code>. A callback with a <code>void*</code> becomes a closure. And a linked list becomes a vector, "
      "with one catch that is worth a figure."),
('fig', 'lists', "A linked list becomes a vector in the order C walks it. When C prepends, Rust inserts at the front; there are "
                 "{PREPENDS} such places."),
('p', "The catch is that floating-point addition is not associative. Add three flows in one order and in another and the last "
      "bit can differ. So it is not enough for the vector to hold the same items as the list; it must hold them in the same order, "
      "because something downstream will add them up."),
('p', "Then there is the text. SWMM writes its reports with <code>fprintf</code> and reads its input with <code>sscanf</code>, "
      "<code>strtod</code> and <code>atoi</code>. Rust has its own ways of formatting numbers, and they do not round, pad or print "
      "exponents the way glibc does. A report is compared byte for byte, so \"nearly the same\" is a failure. The answer was to write "
      "the C routines over again in safe Rust, in a module called <code>cio</code>, and then not trust it. It was checked against "
      "glibc itself, the library EPA's build calls, on millions of inputs."),
('fig', 'cio', "The cio checks: the same format and value handed to glibc and to cio; the bytes must be equal. Every one was."),

('h2', 'The compiler\'s one objection', 'borrow'),
('p', "Now we come to the part of Rust that people complain about, the borrow checker. Its rule is short: at any moment, a "
      "piece of data may have many readers or one writer, not both. C has no such rule. In C you may hand a function the whole "
      "model and, in the same breath, a pointer to one link's cross-section inside that model, and let the function change both. "
      "Rust refuses to compile that, and every module that hands a function one piece of the model meets the refusal."),
('p', "There are four ways around it, and they were written into the rule book in order of preference."),
('fig', 'borrow', "The four ways to call a function that needs the model and a piece of it. The rule beneath them: never hold a "
                  "reference into the model across a call that takes the model."),
('p', "The second way, copy and pass the copy, is the easy one, and that is where the danger is. A copy is safe and it is clean, "
      "and it is wrong whenever the C function was quietly changing the original through its pointer. The borrow checker will "
      "not tell you that. It only tells you that you cannot have it both ways. Which brings us to one bit."),

('h2', 'One bit', 'onebit'),
('p', "SWMM has a cross-section called FILLED_CIRCULAR: a round pipe with sediment in the bottom. The engine stores the pipe's "
      "\"full\" depth as the open part above the sediment. To compute an area, a depth or a hydraulic radius it does something sensible: it adds "
      "the sediment back to get the whole circle, uses the circle formulas, then subtracts the sediment again. It does this "
      "through a pointer to the link's own cross-section, so for a moment the link really does have the larger depth. "
      "Here is the area function as EPA released it:"),
('code', 'c', '@@FUNC:xsect.c|double filled_circ_getAofY(TXsect* xsect, double y)'),
('p', "Add a number and subtract it, and you are back where you started. On paper. In binary floating point, not always. A "
      "0.91-foot pipe with 0.41 feet of sediment has an open depth of exactly 0.5. Add 0.41 and you get 0.9099999999999999: neither "
      "0.41 nor 0.91 has an exact binary form, and the sum lands on the double just below the one nearest 0.91. Subtract 0.41 and you get 0.49999999999999994. One bit low, and the C keeps it, because it "
      "wrote it into the link."),
('fig', 'ulp', "The round trip, in decimal and in the 64 bits the machine actually holds. After the first call the value is stable."),
('p', "The first Rust translation called these functions on copies of the cross-section, the second way out of the borrow checker's objection. "
      "The copies came home with the bit and were thrown away; the link kept its original 0.5. On a test model with eight such "
      "pipes, {FP_DIFF} of {FP_VALS} values in the <code>.out</code> file differed from EPA's, the largest by {FP_MAX}, and one "
      "time of maximum depth in the report came out 01:04 instead of EPA's 01:06. One bit of a double, two minutes in a report."),
('p', "Run the lockstep tool on the engine as it stood before the fix and it names the spot. It stops at routing step 29, "
      "about 140 seconds into the storm, on conduit C0's new depth: 0.007244736162390254 ft in the c2rust copy of EPA's C, "
      "0.007244736162390255 ft in the Rust. The two agree to fifteen significant figures and differ in the sixteenth, and that is enough to move a reported "
      "time by two minutes an hour later."),
('p', "The fix is to do on purpose what C did by accident. Every place the engine asks a conduit's cross-section for an area, "
      "a depth or a radius now goes through a small getter that copies the section, asks the question, and writes the copy back. "
      "The independent review of this essay found one place that still used a copy, the velocity a control rule reads, and it was "
      "fixed the same way, though on the test model built for it the old path happened to give the same bytes. The bit is now lost exactly "
      "where EPA loses it, and the gate is clean. The lesson is old and it is worth repeating: the safe-looking change and the "
      "faithful change are not always the same change, and only the bytes can tell you which one you made."),

('h2', 'Bugs you keep', 'keep'),
('p', "A translator who finds a bug in the original has two choices, and only one of them is honest. Fixing it in the "
      "translation is the dishonest one, however good it feels, because then nobody can tell a fixed bug from a new one. Keeping "
      "it, writing it down and handing the note to the people who own the original is the honest one. Four defects in EPA 5.2.4 "
      "were kept that way. The <code>ccast</code> conversion and the FILLED_CIRCULAR bit you have met. Here are the other two."),
('p', "The first is in the storage units. To find the depth of water that holds a given volume, for some storage shapes SWMM "
      "solves an equation by Newton's method, and the function it hands the solver is this:"),
('code', 'c', "*f = storage_getVolume(k, y) - storageVol->v;\n*df = storage_getSurfArea(k, y);"),
('p', "<code>k</code> there is the storage unit's number among storage units. <code>storage_getVolume()</code> wants a node's "
      "number among all nodes, and looks the storage unit up from that. If node number <code>k</code> happens to be the same storage "
      "unit, all is well. If it is not, the solver uses some other unit's shape. Which unit it gets depends on the order of the "
      "sections in the input file. You can build a network where it matters, and we did: two storage units fed by identical "
      "subcatchments, one shaped like a funnel and one like a pyramid. List the outfall above the storage units and the pyramid "
      "takes the funnel's shape: its peak depth is {SO_PA} ft, the funnel's exactly. List the storage units first and the pyramid's "
      "peak is {SO_PB} ft. Same network. The app holds both orders, as samples on the Run tab and side by side on the Inside tab, so you can watch it happen in all three engines."),
('p', "Credit where it is due. The translator who wrote that function in Rust spotted the mismatch while reading the C and said "
      "so in a comment, but the OpenSWMM code review of 2026 had got there first, as NUM-55, and found a second fault on the same "
      "two lines: the trial depth is in the user's units while the functions expect feet, so in a metric model the solver settles "
      "on a depth about ten times too small. The review's own test decks for both faults are in the gate, and swmm5-rust matches "
      "EPA on them, wrong depths and all. That is the job."),
('fig', 'storage', "Depth in the two storage units, with the input sections in two orders. In order A the pyramid's curve lies "
                   "on the funnel's to within {SO_MAXD_A} ft, the leftovers of two Newton solves started from different guesses."),
('p', "The second is in the error messages for low-impact development controls. When a rain barrel or a bioretention cell has "
      "a bad parameter, <code>lid.c</code> writes the control's name into a global buffer called <code>Msg</code> and passes "
      "<code>Msg</code> to <code>report_writeErrorMsg()</code>. That function then copies the message template into the same "
      "<code>Msg</code> before printing it, so the name is gone and the template is printed in its place. The report reads "
      "<code>ERROR 185: invalid parameter value for LID</code>, and then the template a second time, <code>%s</code> "
      "and all. Rust will not let two names write the same buffer at once, so the translation could not reproduce this "
      "by accident. It reproduces it on purpose: the LID code passes the template itself. The report is byte-identical to EPA's, down to "
      "the missing name, and the defect register on the Inside tab says why."),

('h2', 'Where C has no answer', 'ub'),
('p', "Byte identity needs something to be identical to. For six models in the test set there is nothing. They come from "
      "the OpenSWMM code review of 2026, and each one makes EPA's C read or write memory it does not own. A unit-hydrograph group "
      "with no rain gage reads the first element of an empty gage array. A cross-section row placed above its weir writes past "
      "the end of the conduit array. A curve with no points is followed through a NULL pointer. And so on."),
('p', "What does EPA do with them? It depends on where you run it, which is what undefined means. Natively on x86 some run "
      "to the end on whatever they read, some crash with an empty report. In WebAssembly, where address zero is ordinary memory, "
      "the NULL pointer is followed without complaint and the run goes on. Same C, different answers. Safe Rust's answer does "
      "not move with the platform: it stops at the bad index and names it, and on the two CRASH-08 decks, where C's bad access "
      "comes only during cleanup, it writes the error report and finishes cleanly."),
('fig', 'memdecks', "The six decks in four builds. EPA's answer moves with the platform. swmm5-rust's does not, and where "
                    "the C bug only bites on close (CRASH-08), the Rust engine writes the error report EPA would have written."),
('p', "Safe Rust gives the translator no way to copy an out-of-bounds read, so at each such place somebody had to choose. "
      "Mostly the choice was to stop. In one place it was not. SWMM's cross-section tables are read through a function that "
      "turns a ratio into an index with no check, and a NaN or a huge ratio sends C sixteen gigabytes below the table. The "
      "corpus has decks for it, CRASH-07 in the same review. Natively EPA dies; in WebAssembly it reads whatever is there and "
      "runs on. The Rust reads 0.0 and runs on too, because for a NaN ratio the answer is NaN whatever is read. On one of those "
      "decks all three WebAssembly engines then agree byte for byte; on the other, EPA's WebAssembly build read something "
      "different. Neither counts as a match. The defect register lists it as RS-10, choice and all."),
('p', "These six, every corpus deck that crashes EPA natively, and the corpus's second deck for the CRASH-02 fault are counted apart in every swmm5-rust tally in this essay "
      "and in the app, and the Verify tab lists them by name. They are not failures of the translation and "
      "they are not matches either. They are the reason the translation was worth doing."),

('h2', 'The gate', 'gate'),
('p', "Now the numbers. The gate runs EPA 5.2.4, built from the official v5.2.4 tag with GCC at <code>-O2</code>, and "
      "swmm5-rust, built with Rust {RUSTC_NATIVE} at the same optimization level, on each model, and compares the files. Three sets of "
      "models went through it natively."),
('ul', ["<b>The smoke and coverage set</b>, {N221} models chosen to touch every cross-section shape, every routing method "
        "and the features found in the corpus, with the error decks and the six memory-error decks: {S221} byte-identical, {UB221} undefined in C, "
        "{D221} different.",
        "<b>Every corpus model with a FILLED_CIRCULAR conduit</b>, the shape that taught us about the bit: {FC_RESULT}.",
        "<b>The corpus set</b> that swmm5-cplus was gated on, {NCO} models from Bob's 1729-SWMM5-Models-2030 collection (every "
        "model under 125 KB and a seeded sample of larger ones): {CORPUS_RESULT}."]),
('fig', 'gate', "The smoke and coverage set by folder. Log scale, because the folders differ in size by two orders of magnitude."),
('fig', 'corpus', "The corpus set by folder."),
('p', "Then the browser. All three engines, EPA's C, swmm5-cplus and swmm5-rust, compile to WebAssembly, and the gate was run "
      "again on the {N221} models with all three running in node, the same JavaScript engine a Chrome tab uses. swmm5-rust "
      "matched EPA's WebAssembly build on {WRS} of the {WN} models whose behaviour is defined{WASM_STATE}. There is a "
      "control in that figure worth a look: EPA's own C, native against WebAssembly, agrees with itself on {WPLAT} of the {WN}. "
      "The source is the same, so where it disagrees the platform is the cause, a different compiler and a different C library; "
      "which functions, nobody has traced yet. A translation cannot be blamed for its platform."),
('fig', 'wasm', "Three engines in WebAssembly. The platform row is the control: EPA's C against itself."),

('h2', 'What it costs', 'cost'),
('p', "There ain't no such thing as a free lunch, and a checked array index costs a comparison. How much does safety cost "
      "here? The way to find out is to time it, and the way to time it honestly is to time the noise first. Each model ran in "
      "EPA, swmm5-cplus, swmm5-rust and then EPA again, interleaved, several times over. The spread between the two EPA runs is "
      "the noise floor; a ratio inside it means nothing."),
('fig', 'timing', "Native run time against EPA, per model. A dot inside the grey band is indistinguishable from noise."),
('fig', 'timing_wasm', "The same comparison in WebAssembly."),
('p?NO_TIMING', "Those runs are waiting for a quiet machine. Two cores busy with the gates make a poor stopwatch, and a number "
      "measured on one would be noise dressed up as a result. This section will carry the measurements when they are in."),
('p?T_RS', "Natively, swmm5-rust's median time is {T_RS} of EPA's, against a noise floor of {T_FLOOR}. Part of that is the bounds "
      "checks, and part is that the translation was written to match EPA's arithmetic, not to be fast: it copies cross-sections "
      "where C used a pointer, and it formats every number in the report through a printf written for exactness. No time was spent "
      "on speed. The engine is correct first; fast can come later, under the same gate."),

('h2', 'Three engines in one page', 'app'),
('p', "The app this essay lives in carries all three engines, compiled to WebAssembly and packed into the page itself. Its "
      "Run tab takes any sample or any <code>.inp</code> file of yours, runs it through all three in a background worker, and "
      "compares the results the same way the gate does: the <code>.out</code> files by SHA-256, every value in them one by one, "
      "and every number in the reports. The Many at once tab opens every sample in one swmm5-rust instance, one handle each, "
      "and steps them in turn, which EPA's engine, with its globals, cannot do. The Inside tab shows the C, the C++ and the "
      "Rust of the same functions side by side, and the defect register with the offending lines as they are in EPA's source."),
('p', "There is one small piece of engineering in the page worth mentioning. A WebAssembly program asks its host for files "
      "through an interface called WASI. The C engines need thirteen of its calls; Rust's standard library needs six more, to "
      "ask the size of a file or a path, where it stands in a file, how to make a directory, and what the environment holds. The page answers all nineteen from an "
      "in-memory file system about 150 lines long. That is the whole operating system the engines see."),

('h2', 'What this does not show', 'limits'),
('ul', ["It is not a new engine and not a better one. Every number is EPA 5.2.4's, its defects included. RS-1 to RS-4 in the "
        "register are reproduced on purpose.",
        "It does not reproduce EPA where EPA's behaviour is undefined. There, swmm5-rust stops with a message, or writes the error "
        "report EPA meant to write (RS-9), or reads 0.0 where C reads far outside a table (RS-10).",
        "Safe Rust rules out memory errors. It does not rule out wrong arithmetic. That is checked only by the gate, and only on "
        "the models the gate ran.",
        "It is not faster than the C.",
        "The C interface that lets C and JavaScript call the engine has {CAPI_UNSAFE} small unsafe blocks, where pointers come in "
        "from the caller. They are the only unsafe code in the engine and its interface; the test harnesses that call glibc "
        "and the c2rust copy for comparison use it too, and do not ship.",
        "Nobody has yet read the translation line by line against the C. The gate is mechanical and strict, but it is not a "
        "code review."]),

('h2', 'How this was made', 'made'),
('p', "Bob Dickinson asked for a Rust version of SWMM 5 after the C++ one, chose safe Rust with byte identity as the goal, and "
      "supplied the model corpus the gates ran on. Claude, Anthropic's AI model (Claude Opus 5.5), did the rest in one session on "
      "28 and 29 October 2026: ran c2rust and wrote the cast and stability passes over its output, wrote the skeleton and the rule "
      "book, ran the ten parallel translation batches as separate agents and integrated them, wrote <code>ccast</code>, "
      "<code>cio</code> and their glibc tests, found and fixed the FILLED_CIRCULAR drift, built the WebAssembly engine, wrote the "
      "gates, this app, its figures and this essay, in the manner of Robert A. Heinlein explaining a piece of engineering, at Bob's "
      "standing request. The checking was done by the gate, not by the AI's judgment. The translation has not yet been reviewed "
      "line by line by a person."),
]
