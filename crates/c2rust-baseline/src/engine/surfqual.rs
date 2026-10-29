#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    static mut Nobjects: [::core::ffi::c_int; 18];
    static mut Subcatch: *mut TSubcatch;
    static mut Pollut: *mut TPollut;
    static mut Landuse: *mut TLanduse;
    fn landuse_getInitBuildup(
        landFactor: *mut TLandFactor,
        initBuildup: *mut ::core::ffi::c_double,
        area: ::core::ffi::c_double,
        curb: ::core::ffi::c_double,
    );
    fn landuse_getBuildup(
        landuse: ::core::ffi::c_int,
        pollut: ::core::ffi::c_int,
        area: ::core::ffi::c_double,
        curb: ::core::ffi::c_double,
        buildup: ::core::ffi::c_double,
        tStep: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn landuse_getWashoffLoad(
        landuse: ::core::ffi::c_int,
        p: ::core::ffi::c_int,
        area: ::core::ffi::c_double,
        landFactor: *mut TLandFactor,
        runoff: ::core::ffi::c_double,
        vOutflow: ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn landuse_getAvgBmpEffic(
        j: ::core::ffi::c_int,
        p: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    fn massbal_updateLoadingTotals(
        type_0: ::core::ffi::c_int,
        pollut: ::core::ffi::c_int,
        w: ::core::ffi::c_double,
    );
    fn subcatch_getDepth(subcatch: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    static mut OutflowLoad: *mut ::core::ffi::c_double;
    static mut Vinfil: ::core::ffi::c_double;
    static mut Vinflow: ::core::ffi::c_double;
    static mut Voutflow: ::core::ffi::c_double;
    static mut VlidDrain: ::core::ffi::c_double;
    fn lid_addDrainLoads(
        subcatch: ::core::ffi::c_int,
        c: *mut ::core::ffi::c_double,
        tStep: ::core::ffi::c_double,
    );
}
pub type ObjectType = ::core::ffi::c_uint;
pub const MAX_OBJ_TYPES: ObjectType = 18;
pub const INLET: ObjectType = 17;
pub const STREET: ObjectType = 16;
pub const LID: ObjectType = 15;
pub const SHAPE: ObjectType = 14;
pub const SNOWMELT: ObjectType = 13;
pub const UNITHYD: ObjectType = 12;
pub const AQUIFER: ObjectType = 11;
pub const TRANSECT: ObjectType = 10;
pub const CONTROL: ObjectType = 9;
pub const TSERIES: ObjectType = 8;
pub const CURVE: ObjectType = 7;
pub const TIMEPATTERN: ObjectType = 6;
pub const LANDUSE: ObjectType = 5;
pub const POLLUT: ObjectType = 4;
pub const LINK: ObjectType = 3;
pub const NODE: ObjectType = 2;
pub const SUBCATCH: ObjectType = 1;
pub const GAGE: ObjectType = 0;
pub type ConversionType = ::core::ffi::c_uint;
pub const FLOW: ConversionType = 10;
pub const GWFLOW: ConversionType = 9;
pub const MASS: ConversionType = 8;
pub const TEMPERATURE: ConversionType = 7;
pub const WINDSPEED: ConversionType = 6;
pub const VOLUME: ConversionType = 5;
pub const LANDAREA: ConversionType = 4;
pub const LENGTH: ConversionType = 3;
pub const EVAPRATE: ConversionType = 2;
pub const RAINDEPTH: ConversionType = 1;
pub const RAINFALL: ConversionType = 0;
pub type LoadingType = ::core::ffi::c_uint;
pub const FINAL_LOAD: LoadingType = 6;
pub const RUNOFF_LOAD: LoadingType = 5;
pub const INFIL_LOAD: LoadingType = 4;
pub const BMP_REMOVAL_LOAD: LoadingType = 3;
pub const SWEEPING_LOAD: LoadingType = 2;
pub const DEPOSITION_LOAD: LoadingType = 1;
pub const BUILDUP_LOAD: LoadingType = 0;
pub type SubAreaType = ::core::ffi::c_uint;
pub const PERV: SubAreaType = 2;
pub const IMPERV1: SubAreaType = 1;
pub const IMPERV0: SubAreaType = 0;
pub type DateTime = ::core::ffi::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ExprNode {
    pub opcode: ::core::ffi::c_int,
    pub ivar: ::core::ffi::c_int,
    pub fvalue: ::core::ffi::c_double,
    pub prev: *mut ExprNode,
    pub next: *mut ExprNode,
}
pub type MathExpr = ExprNode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TGWaterStats {
    pub infil: ::core::ffi::c_double,
    pub evap: ::core::ffi::c_double,
    pub latFlow: ::core::ffi::c_double,
    pub deepFlow: ::core::ffi::c_double,
    pub avgUpperMoist: ::core::ffi::c_double,
    pub finalUpperMoist: ::core::ffi::c_double,
    pub avgWaterTable: ::core::ffi::c_double,
    pub finalWaterTable: ::core::ffi::c_double,
    pub maxFlow: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TGroundwater {
    pub aquifer: ::core::ffi::c_int,
    pub node: ::core::ffi::c_int,
    pub surfElev: ::core::ffi::c_double,
    pub a1: ::core::ffi::c_double,
    pub b1: ::core::ffi::c_double,
    pub a2: ::core::ffi::c_double,
    pub b2: ::core::ffi::c_double,
    pub a3: ::core::ffi::c_double,
    pub fixedDepth: ::core::ffi::c_double,
    pub nodeElev: ::core::ffi::c_double,
    pub bottomElev: ::core::ffi::c_double,
    pub waterTableElev: ::core::ffi::c_double,
    pub upperMoisture: ::core::ffi::c_double,
    pub theta: ::core::ffi::c_double,
    pub lowerDepth: ::core::ffi::c_double,
    pub oldFlow: ::core::ffi::c_double,
    pub newFlow: ::core::ffi::c_double,
    pub evapLoss: ::core::ffi::c_double,
    pub maxInfilVol: ::core::ffi::c_double,
    pub stats: TGWaterStats,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSnowpack {
    pub snowmeltIndex: ::core::ffi::c_int,
    pub fArea: [::core::ffi::c_double; 3],
    pub wsnow: [::core::ffi::c_double; 3],
    pub fw: [::core::ffi::c_double; 3],
    pub coldc: [::core::ffi::c_double; 3],
    pub ati: [::core::ffi::c_double; 3],
    pub sba: [::core::ffi::c_double; 3],
    pub awe: [::core::ffi::c_double; 3],
    pub sbws: [::core::ffi::c_double; 3],
    pub imelt: [::core::ffi::c_double; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSubarea {
    pub routeTo: ::core::ffi::c_int,
    pub fOutlet: ::core::ffi::c_double,
    pub N: ::core::ffi::c_double,
    pub fArea: ::core::ffi::c_double,
    pub dStore: ::core::ffi::c_double,
    pub alpha: ::core::ffi::c_double,
    pub inflow: ::core::ffi::c_double,
    pub runoff: ::core::ffi::c_double,
    pub depth: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLandFactor {
    pub fraction: ::core::ffi::c_double,
    pub buildup: *mut ::core::ffi::c_double,
    pub lastSwept: DateTime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSubcatch {
    pub ID: *mut ::core::ffi::c_char,
    pub rptFlag: ::core::ffi::c_int,
    pub gage: ::core::ffi::c_int,
    pub outNode: ::core::ffi::c_int,
    pub outSubcatch: ::core::ffi::c_int,
    pub infilModel: ::core::ffi::c_int,
    pub infil: ::core::ffi::c_int,
    pub subArea: [TSubarea; 3],
    pub width: ::core::ffi::c_double,
    pub area: ::core::ffi::c_double,
    pub fracImperv: ::core::ffi::c_double,
    pub slope: ::core::ffi::c_double,
    pub curbLength: ::core::ffi::c_double,
    pub initBuildup: *mut ::core::ffi::c_double,
    pub landFactor: *mut TLandFactor,
    pub groundwater: *mut TGroundwater,
    pub gwLatFlowExpr: *mut MathExpr,
    pub gwDeepFlowExpr: *mut MathExpr,
    pub snowpack: *mut TSnowpack,
    pub nPervPattern: ::core::ffi::c_int,
    pub dStorePattern: ::core::ffi::c_int,
    pub infilPattern: ::core::ffi::c_int,
    pub lidArea: ::core::ffi::c_double,
    pub rainfall: ::core::ffi::c_double,
    pub evapLoss: ::core::ffi::c_double,
    pub infilLoss: ::core::ffi::c_double,
    pub runon: ::core::ffi::c_double,
    pub oldRunoff: ::core::ffi::c_double,
    pub newRunoff: ::core::ffi::c_double,
    pub oldSnowDepth: ::core::ffi::c_double,
    pub newSnowDepth: ::core::ffi::c_double,
    pub oldQual: *mut ::core::ffi::c_double,
    pub newQual: *mut ::core::ffi::c_double,
    pub pondedQual: *mut ::core::ffi::c_double,
    pub totalLoad: *mut ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPollut {
    pub ID: *mut ::core::ffi::c_char,
    pub units: ::core::ffi::c_int,
    pub mcf: ::core::ffi::c_double,
    pub dwfConcen: ::core::ffi::c_double,
    pub pptConcen: ::core::ffi::c_double,
    pub gwConcen: ::core::ffi::c_double,
    pub rdiiConcen: ::core::ffi::c_double,
    pub initConcen: ::core::ffi::c_double,
    pub kDecay: ::core::ffi::c_double,
    pub coPollut: ::core::ffi::c_int,
    pub coFraction: ::core::ffi::c_double,
    pub snowOnly: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TBuildup {
    pub normalizer: ::core::ffi::c_int,
    pub funcType: ::core::ffi::c_int,
    pub coeff: [::core::ffi::c_double; 3],
    pub maxDays: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TWashoff {
    pub funcType: ::core::ffi::c_int,
    pub coeff: ::core::ffi::c_double,
    pub expon: ::core::ffi::c_double,
    pub sweepEffic: ::core::ffi::c_double,
    pub bmpEffic: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLanduse {
    pub ID: *mut ::core::ffi::c_char,
    pub sweepInterval: ::core::ffi::c_double,
    pub sweepRemoval: ::core::ffi::c_double,
    pub sweepDays0: ::core::ffi::c_double,
    pub buildupFunc: *mut TBuildup,
    pub washoffFunc: *mut TWashoff,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[no_mangle]
pub unsafe extern "C" fn surfqual_initState(mut j: ::core::ffi::c_int) {
    let mut p: ::core::ffi::c_int = 0;
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        *(*Subcatch.offset(j as isize)).oldQual.offset(p as isize) = 0.0f64;
        *(*Subcatch.offset(j as isize)).newQual.offset(p as isize) = 0.0f64;
        *(*Subcatch.offset(j as isize)).pondedQual.offset(p as isize) = 0.0f64;
        p += 1;
    }
    landuse_getInitBuildup(
        (*Subcatch.offset(j as isize)).landFactor,
        (*Subcatch.offset(j as isize)).initBuildup,
        (*Subcatch.offset(j as isize)).area,
        (*Subcatch.offset(j as isize)).curbLength,
    );
}
#[no_mangle]
pub unsafe extern "C" fn surfqual_getBuildup(
    mut j: ::core::ffi::c_int,
    mut tStep: ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut f: ::core::ffi::c_double = 0.;
    let mut area: ::core::ffi::c_double = 0.;
    let mut curb: ::core::ffi::c_double = 0.;
    let mut oldBuildup: ::core::ffi::c_double = 0.;
    let mut newBuildup: ::core::ffi::c_double = 0.;
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
        f = (*(*Subcatch.offset(j as isize)).landFactor.offset(i as isize)).fraction;
        if !(f == 0.0f64) {
            area = f * (*Subcatch.offset(j as isize)).area * UCF(LANDAREA as ::core::ffi::c_int);
            curb = f * (*Subcatch.offset(j as isize)).curbLength;
            p = 0 as ::core::ffi::c_int;
            while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                if !((*Pollut.offset(p as isize)).snowOnly != 0
                    && (*Subcatch.offset(j as isize)).newSnowDepth < 0.001f64 / 12.0f64)
                {
                    oldBuildup = *(*(*Subcatch.offset(j as isize)).landFactor.offset(i as isize))
                        .buildup
                        .offset(p as isize);
                    newBuildup = landuse_getBuildup(i, p, area, curb, oldBuildup, tStep);
                    newBuildup = if newBuildup >= oldBuildup {
                        newBuildup
                    } else {
                        oldBuildup
                    };
                    *(*(*Subcatch.offset(j as isize)).landFactor.offset(i as isize))
                        .buildup
                        .offset(p as isize) = newBuildup;
                    massbal_updateLoadingTotals(
                        BUILDUP_LOAD as ::core::ffi::c_int,
                        p,
                        newBuildup - oldBuildup,
                    );
                }
                p += 1;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn surfqual_sweepBuildup(mut j: ::core::ffi::c_int, mut aDate: DateTime) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut oldBuildup: ::core::ffi::c_double = 0.;
    let mut newBuildup: ::core::ffi::c_double = 0.;
    if !(*Subcatch.offset(j as isize)).snowpack.is_null()
        && (*(*Subcatch.offset(j as isize)).snowpack).wsnow[IMPERV0 as ::core::ffi::c_int as usize]
            > MIN_TOTAL_DEPTH
    {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
        if !((*(*Subcatch.offset(j as isize)).landFactor.offset(i as isize)).fraction == 0.0f64) {
            if !((*Landuse.offset(i as isize)).sweepInterval == 0.0f64) {
                if aDate - (*(*Subcatch.offset(j as isize)).landFactor.offset(i as isize)).lastSwept
                    >= (*Landuse.offset(i as isize)).sweepInterval
                {
                    (*(*Subcatch.offset(j as isize)).landFactor.offset(i as isize)).lastSwept =
                        aDate;
                    p = 0 as ::core::ffi::c_int;
                    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                        oldBuildup =
                            *(*(*Subcatch.offset(j as isize)).landFactor.offset(i as isize))
                                .buildup
                                .offset(p as isize);
                        newBuildup = oldBuildup
                            * (1.0f64
                                - (*Landuse.offset(i as isize)).sweepRemoval
                                    * (*(*Landuse.offset(i as isize))
                                        .washoffFunc
                                        .offset(p as isize))
                                    .sweepEffic);
                        newBuildup = if oldBuildup <= newBuildup {
                            oldBuildup
                        } else {
                            newBuildup
                        };
                        newBuildup = if 0.0f64 >= newBuildup {
                            0.0f64
                        } else {
                            newBuildup
                        };
                        *(*(*Subcatch.offset(j as isize)).landFactor.offset(i as isize))
                            .buildup
                            .offset(p as isize) = newBuildup;
                        massbal_updateLoadingTotals(
                            SWEEPING_LOAD as ::core::ffi::c_int,
                            p,
                            oldBuildup - newBuildup,
                        );
                        p += 1;
                    }
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn surfqual_getWashoff(
    mut j: ::core::ffi::c_int,
    mut runoff: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
) {
    let mut p: ::core::ffi::c_int = 0;
    let mut hasOutflow: ::core::ffi::c_int = 0;
    let mut cOut: ::core::ffi::c_double = 0.;
    let mut massLoad: ::core::ffi::c_double = 0.;
    let mut vLidRain: ::core::ffi::c_double = 0.;
    let mut vLidRunon: ::core::ffi::c_double = 0.;
    let mut vSurfOut: ::core::ffi::c_double = 0.;
    let mut vOut1: ::core::ffi::c_double = 0.;
    let mut vOut2: ::core::ffi::c_double = 0.;
    let mut area: ::core::ffi::c_double = 0.;
    area = (*Subcatch.offset(j as isize)).area;
    if Nobjects[POLLUT as ::core::ffi::c_int as usize] == 0 as ::core::ffi::c_int || area == 0.0f64
    {
        return;
    }
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        *OutflowLoad.offset(p as isize) = 0.0f64;
        p += 1;
    }
    findWashoffLoads(j, runoff);
    findPondedLoads(j, tStep);
    findLidLoads(j, tStep);
    vLidRain =
        (*Subcatch.offset(j as isize)).rainfall * (*Subcatch.offset(j as isize)).lidArea * tStep;
    vLidRunon = 0.0f64;
    if area == (*Subcatch.offset(j as isize)).lidArea {
        vLidRunon = (*Subcatch.offset(j as isize)).runon * area * tStep;
    }
    vOut1 = Voutflow + vLidRain + vLidRunon;
    vSurfOut = (*Subcatch.offset(j as isize)).newRunoff * tStep;
    vOut2 = vSurfOut + VlidDrain;
    hasOutflow = (vOut2 > MIN_RUNOFF * area * tStep) as ::core::ffi::c_int;
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        cOut = 0.0f64;
        if vOut1 > 0.0f64 && hasOutflow != 0 {
            cOut = *OutflowLoad.offset(p as isize) / vOut1;
        }
        if (*Subcatch.offset(j as isize)).lidArea > 0.0f64 {
            massLoad = cOut * (vOut1 - vOut2) * (*Pollut.offset(p as isize)).mcf;
            if massLoad > 0.0f64 {
                massbal_updateLoadingTotals(BMP_REMOVAL_LOAD as ::core::ffi::c_int, p, massLoad);
            }
        }
        massLoad = cOut * vOut2 * (*Pollut.offset(p as isize)).mcf;
        *(*Subcatch.offset(j as isize)).totalLoad.offset(p as isize) += massLoad;
        if (*Subcatch.offset(j as isize)).outNode >= 0 as ::core::ffi::c_int
            || (*Subcatch.offset(j as isize)).outSubcatch == j
        {
            massLoad = cOut * vSurfOut * (*Pollut.offset(p as isize)).mcf;
            massbal_updateLoadingTotals(RUNOFF_LOAD as ::core::ffi::c_int, p, massLoad);
        }
        *(*Subcatch.offset(j as isize)).newQual.offset(p as isize) = cOut / LperFT3;
        p += 1;
    }
    if (*Subcatch.offset(j as isize)).lidArea > 0.0f64 {
        lid_addDrainLoads(
            j,
            (*Subcatch.offset(j as isize)).newQual as *mut ::core::ffi::c_double,
            tStep,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn surfqual_getWtdWashoff(
    mut j: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
    mut f: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    return (1.0f64 - f)
        * (*Subcatch.offset(j as isize)).oldRunoff
        * *(*Subcatch.offset(j as isize)).oldQual.offset(p as isize)
        + f * (*Subcatch.offset(j as isize)).newRunoff
            * *(*Subcatch.offset(j as isize)).newQual.offset(p as isize);
}
unsafe extern "C" fn findPondedLoads(mut j: ::core::ffi::c_int, mut tStep: ::core::ffi::c_double) {
    let mut p: ::core::ffi::c_int = 0;
    let mut cPonded: ::core::ffi::c_double = 0.;
    let mut wPonded: ::core::ffi::c_double = 0.;
    let mut bmpRemoval: ::core::ffi::c_double = 0.;
    let mut vRain: ::core::ffi::c_double = 0.;
    let mut wRain: ::core::ffi::c_double = 0.;
    let mut wRunon: ::core::ffi::c_double = 0.;
    let mut wInfil: ::core::ffi::c_double = 0.;
    let mut wOutflow: ::core::ffi::c_double = 0.;
    let mut fullArea: ::core::ffi::c_double = 0.;
    let mut nonLidArea: ::core::ffi::c_double = 0.;
    if (*Subcatch.offset(j as isize)).area == (*Subcatch.offset(j as isize)).lidArea {
        return;
    }
    fullArea = (*Subcatch.offset(j as isize)).area;
    nonLidArea = fullArea - (*Subcatch.offset(j as isize)).lidArea;
    vRain = (*Subcatch.offset(j as isize)).rainfall * nonLidArea * tStep;
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        wRain = (*Pollut.offset(p as isize)).pptConcen * LperFT3 * vRain;
        massbal_updateLoadingTotals(
            DEPOSITION_LOAD as ::core::ffi::c_int,
            p,
            wRain * (*Pollut.offset(p as isize)).mcf,
        );
        if Vinflow == 0.0f64 {
            massbal_updateLoadingTotals(
                FINAL_LOAD as ::core::ffi::c_int,
                p,
                *(*Subcatch.offset(j as isize)).pondedQual.offset(p as isize)
                    * (*Pollut.offset(p as isize)).mcf,
            );
            *(*Subcatch.offset(j as isize)).pondedQual.offset(p as isize) = 0.0f64;
        } else {
            wRunon = *(*Subcatch.offset(j as isize)).newQual.offset(p as isize) * tStep;
            wPonded =
                *(*Subcatch.offset(j as isize)).pondedQual.offset(p as isize) + wRain + wRunon;
            cPonded = wPonded / Vinflow;
            wInfil = cPonded * Vinfil;
            wInfil = if wInfil <= wPonded { wInfil } else { wPonded };
            massbal_updateLoadingTotals(
                INFIL_LOAD as ::core::ffi::c_int,
                p,
                wInfil * (*Pollut.offset(p as isize)).mcf,
            );
            wPonded -= wInfil;
            wOutflow = cPonded * Voutflow;
            wOutflow = if wOutflow <= wPonded {
                wOutflow
            } else {
                wPonded
            };
            wPonded -= wOutflow;
            bmpRemoval = landuse_getAvgBmpEffic(j, p) * wOutflow;
            massbal_updateLoadingTotals(
                BMP_REMOVAL_LOAD as ::core::ffi::c_int,
                p,
                bmpRemoval * (*Pollut.offset(p as isize)).mcf,
            );
            wOutflow -= bmpRemoval;
            *(*Subcatch.offset(j as isize)).pondedQual.offset(p as isize) =
                cPonded * subcatch_getDepth(j) * nonLidArea;
            *OutflowLoad.offset(p as isize) += wOutflow;
        }
        p += 1;
    }
}
unsafe extern "C" fn findWashoffLoads(
    mut j: ::core::ffi::c_int,
    mut runoff: ::core::ffi::c_double,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_double = 0.;
    let mut area: ::core::ffi::c_double = (*Subcatch.offset(j as isize)).area;
    if runoff < MIN_RUNOFF {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < Nobjects[LANDUSE as ::core::ffi::c_int as usize] {
        if (*(*Subcatch.offset(j as isize)).landFactor.offset(i as isize)).fraction > 0.0f64 {
            p = 0 as ::core::ffi::c_int;
            while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
                *OutflowLoad.offset(p as isize) += landuse_getWashoffLoad(
                    i,
                    p,
                    area,
                    (*Subcatch.offset(j as isize)).landFactor as *mut TLandFactor,
                    runoff,
                    Voutflow,
                );
                p += 1;
            }
        }
        i += 1;
    }
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        k = (*Pollut.offset(p as isize)).coPollut;
        if k >= 0 as ::core::ffi::c_int {
            w = (*Pollut.offset(p as isize)).coFraction * *OutflowLoad.offset(k as isize);
            massbal_updateLoadingTotals(
                BUILDUP_LOAD as ::core::ffi::c_int,
                p,
                w * (*Pollut.offset(p as isize)).mcf,
            );
            *OutflowLoad.offset(p as isize) += w;
        }
        p += 1;
    }
}
unsafe extern "C" fn findLidLoads(mut j: ::core::ffi::c_int, mut tStep: ::core::ffi::c_double) {
    let mut p: ::core::ffi::c_int = 0;
    let mut useRunon: ::core::ffi::c_int = 0;
    let mut lidArea: ::core::ffi::c_double = 0.;
    let mut vLidRain: ::core::ffi::c_double = 0.;
    let mut wLidRain: ::core::ffi::c_double = 0.;
    let mut wLidRunon: ::core::ffi::c_double = 0.;
    lidArea = (*Subcatch.offset(j as isize)).lidArea;
    if lidArea == 0.0f64 {
        return;
    }
    vLidRain = (*Subcatch.offset(j as isize)).rainfall * lidArea * tStep;
    useRunon = (lidArea == (*Subcatch.offset(j as isize)).area) as ::core::ffi::c_int;
    p = 0 as ::core::ffi::c_int;
    while p < Nobjects[POLLUT as ::core::ffi::c_int as usize] {
        wLidRain = (*Pollut.offset(p as isize)).pptConcen * vLidRain * LperFT3;
        massbal_updateLoadingTotals(
            DEPOSITION_LOAD as ::core::ffi::c_int,
            p,
            wLidRain * (*Pollut.offset(p as isize)).mcf,
        );
        if useRunon != 0 {
            wLidRunon = *(*Subcatch.offset(j as isize)).newQual.offset(p as isize) * tStep;
        } else {
            wLidRunon = 0.0f64;
        }
        *OutflowLoad.offset(p as isize) += wLidRain + wLidRunon;
        p += 1;
    }
}
pub const MIN_TOTAL_DEPTH: ::core::ffi::c_double = 0.004167f64;
pub const MIN_RUNOFF: ::core::ffi::c_double = 2.31481e-8f64;
pub const LperFT3: ::core::ffi::c_double = 28.317f64;
