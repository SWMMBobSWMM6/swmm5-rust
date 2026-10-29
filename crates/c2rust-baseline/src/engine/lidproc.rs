#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn grnampt_getInfil(
        infil: *mut TGrnAmpt,
        tstep: ::core::ffi::c_double,
        irate: ::core::ffi::c_double,
        depth: ::core::ffi::c_double,
        modelType: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static mut HasWetLids: ::core::ffi::c_char;
    fn datetime_getTimeStamp(
        fmt: ::core::ffi::c_int,
        aDate: DateTime,
        stampSize: ::core::ffi::c_int,
        timeStamp: *mut ::core::ffi::c_char,
    );
    static mut OldRunoffTime: ::core::ffi::c_double;
    static mut NewRunoffTime: ::core::ffi::c_double;
    static mut Curve: *mut TTable;
    fn table_lookup(table: *mut TTable, x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn UCF(quantity: ::core::ffi::c_int) -> ::core::ffi::c_double;
    fn getDateTime(elapsedMsec: ::core::ffi::c_double) -> DateTime;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type InfilType = ::core::ffi::c_uint;
pub const CURVE_NUMBER: InfilType = 4;
pub const MOD_GREEN_AMPT: InfilType = 3;
pub const GREEN_AMPT: InfilType = 2;
pub const MOD_HORTON: InfilType = 1;
pub const HORTON: InfilType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TGrnAmpt {
    pub S: ::core::ffi::c_double,
    pub Ks: ::core::ffi::c_double,
    pub IMDmax: ::core::ffi::c_double,
    pub IMD: ::core::ffi::c_double,
    pub F: ::core::ffi::c_double,
    pub Fu: ::core::ffi::c_double,
    pub Lu: ::core::ffi::c_double,
    pub T: ::core::ffi::c_double,
    pub Sat: ::core::ffi::c_char,
}
pub type LidTypes = ::core::ffi::c_uint;
pub const ROOF_DISCON: LidTypes = 7;
pub const VEG_SWALE: LidTypes = 6;
pub const RAIN_BARREL: LidTypes = 5;
pub const POROUS_PAVEMENT: LidTypes = 4;
pub const INFIL_TRENCH: LidTypes = 3;
pub const GREEN_ROOF: LidTypes = 2;
pub const RAIN_GARDEN: LidTypes = 1;
pub const BIO_CELL: LidTypes = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSurfaceLayer {
    pub thickness: ::core::ffi::c_double,
    pub voidFrac: ::core::ffi::c_double,
    pub roughness: ::core::ffi::c_double,
    pub surfSlope: ::core::ffi::c_double,
    pub sideSlope: ::core::ffi::c_double,
    pub alpha: ::core::ffi::c_double,
    pub canOverflow: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TPavementLayer {
    pub thickness: ::core::ffi::c_double,
    pub voidFrac: ::core::ffi::c_double,
    pub impervFrac: ::core::ffi::c_double,
    pub kSat: ::core::ffi::c_double,
    pub clogFactor: ::core::ffi::c_double,
    pub regenDays: ::core::ffi::c_double,
    pub regenDegree: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TSoilLayer {
    pub thickness: ::core::ffi::c_double,
    pub porosity: ::core::ffi::c_double,
    pub fieldCap: ::core::ffi::c_double,
    pub wiltPoint: ::core::ffi::c_double,
    pub suction: ::core::ffi::c_double,
    pub kSat: ::core::ffi::c_double,
    pub kSlope: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TStorageLayer {
    pub thickness: ::core::ffi::c_double,
    pub voidFrac: ::core::ffi::c_double,
    pub kSat: ::core::ffi::c_double,
    pub clogFactor: ::core::ffi::c_double,
    pub covered: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TDrainLayer {
    pub coeff: ::core::ffi::c_double,
    pub expon: ::core::ffi::c_double,
    pub offset: ::core::ffi::c_double,
    pub delay: ::core::ffi::c_double,
    pub hOpen: ::core::ffi::c_double,
    pub hClose: ::core::ffi::c_double,
    pub qCurve: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TDrainMatLayer {
    pub thickness: ::core::ffi::c_double,
    pub voidFrac: ::core::ffi::c_double,
    pub roughness: ::core::ffi::c_double,
    pub alpha: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLidProc {
    pub ID: *mut ::core::ffi::c_char,
    pub lidType: ::core::ffi::c_int,
    pub surface: TSurfaceLayer,
    pub pavement: TPavementLayer,
    pub soil: TSoilLayer,
    pub storage: TStorageLayer,
    pub drain: TDrainLayer,
    pub drainMat: TDrainMatLayer,
    pub drainRmvl: *mut ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TWaterBalance {
    pub inflow: ::core::ffi::c_double,
    pub evap: ::core::ffi::c_double,
    pub infil: ::core::ffi::c_double,
    pub surfFlow: ::core::ffi::c_double,
    pub drainFlow: ::core::ffi::c_double,
    pub initVol: ::core::ffi::c_double,
    pub finalVol: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLidRptFile {
    pub file: *mut FILE,
    pub wasDry: ::core::ffi::c_int,
    pub results: [::core::ffi::c_char; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TLidUnit {
    pub lidIndex: ::core::ffi::c_int,
    pub number: ::core::ffi::c_int,
    pub area: ::core::ffi::c_double,
    pub fullWidth: ::core::ffi::c_double,
    pub botWidth: ::core::ffi::c_double,
    pub initSat: ::core::ffi::c_double,
    pub fromImperv: ::core::ffi::c_double,
    pub fromPerv: ::core::ffi::c_double,
    pub toPerv: ::core::ffi::c_int,
    pub drainSubcatch: ::core::ffi::c_int,
    pub drainNode: ::core::ffi::c_int,
    pub rptFile: *mut TLidRptFile,
    pub soilInfil: TGrnAmpt,
    pub surfaceDepth: ::core::ffi::c_double,
    pub paveDepth: ::core::ffi::c_double,
    pub soilMoisture: ::core::ffi::c_double,
    pub storageDepth: ::core::ffi::c_double,
    pub oldFluxRates: [::core::ffi::c_double; 4],
    pub dryTime: ::core::ffi::c_double,
    pub oldDrainFlow: ::core::ffi::c_double,
    pub newDrainFlow: ::core::ffi::c_double,
    pub volTreated: ::core::ffi::c_double,
    pub nextRegenDay: ::core::ffi::c_double,
    pub waterBalance: TWaterBalance,
}
pub const STOR: LidLayerTypes = 2;
pub const SOIL: LidLayerTypes = 1;
pub const PAVE: LidLayerTypes = 3;
pub const SURF: LidLayerTypes = 0;
pub const RAINFALL: ConversionType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TTable {
    pub ID: *mut ::core::ffi::c_char,
    pub curveType: ::core::ffi::c_int,
    pub refersTo: ::core::ffi::c_int,
    pub dxMin: ::core::ffi::c_double,
    pub lastDate: ::core::ffi::c_double,
    pub x1: ::core::ffi::c_double,
    pub x2: ::core::ffi::c_double,
    pub y1: ::core::ffi::c_double,
    pub y2: ::core::ffi::c_double,
    pub firstEntry: *mut TTableEntry,
    pub lastEntry: *mut TTableEntry,
    pub thisEntry: *mut TTableEntry,
    pub file: TFile,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TFile {
    pub name: [::core::ffi::c_char; 260],
    pub mode: ::core::ffi::c_char,
    pub state: ::core::ffi::c_char,
    pub file: *mut FILE,
}
pub type TTableEntry = TableEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TableEntry {
    pub x: ::core::ffi::c_double,
    pub y: ::core::ffi::c_double,
    pub next: *mut TableEntry,
}
pub const RAINDEPTH: ConversionType = 1;
pub type DateTime = ::core::ffi::c_double;
pub const STOR_DEPTH: LidRptVars = 11;
pub const SOIL_MOIST: LidRptVars = 10;
pub const PAVE_DEPTH: LidRptVars = 9;
pub const SURF_DEPTH: LidRptVars = 8;
pub const STOR_DRAIN: LidRptVars = 7;
pub const SURF_OUTFLOW: LidRptVars = 6;
pub const STOR_EXFIL: LidRptVars = 5;
pub const SOIL_PERC: LidRptVars = 4;
pub const PAVE_PERC: LidRptVars = 3;
pub const SURF_INFIL: LidRptVars = 2;
pub const TOTAL_EVAP: LidRptVars = 1;
pub const SURF_INFLOW: LidRptVars = 0;
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
pub type LidLayerTypes = ::core::ffi::c_uint;
pub const DRAIN: LidLayerTypes = 4;
pub type LidRptVars = ::core::ffi::c_uint;
pub const MAX_RPT_VARS: LidRptVars = 12;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BIG: ::core::ffi::c_double = 1.0E10f64;
pub const ZERO: ::core::ffi::c_double = 1.0E-10f64;
pub const SECperDAY: ::core::ffi::c_int = 86400 as ::core::ffi::c_int;
pub const MAX_LAYERS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const STOPTOL: ::core::ffi::c_double = 0.00328f64;
pub const MINFLOW: ::core::ffi::c_double = 2.3e-8f64;
static mut theLidUnit: *mut TLidUnit = ::core::ptr::null::<TLidUnit>() as *mut TLidUnit;
static mut theLidProc: *mut TLidProc = ::core::ptr::null::<TLidProc>() as *mut TLidProc;
static mut Tstep: ::core::ffi::c_double = 0.;
static mut EvapRate: ::core::ffi::c_double = 0.;
static mut MaxNativeInfil: ::core::ffi::c_double = 0.;
static mut SurfaceInflow: ::core::ffi::c_double = 0.;
static mut SurfaceInfil: ::core::ffi::c_double = 0.;
static mut SurfaceEvap: ::core::ffi::c_double = 0.;
static mut SurfaceOutflow: ::core::ffi::c_double = 0.;
static mut SurfaceVolume: ::core::ffi::c_double = 0.;
static mut PaveEvap: ::core::ffi::c_double = 0.;
static mut PavePerc: ::core::ffi::c_double = 0.;
static mut PaveVolume: ::core::ffi::c_double = 0.;
static mut SoilEvap: ::core::ffi::c_double = 0.;
static mut SoilPerc: ::core::ffi::c_double = 0.;
static mut SoilVolume: ::core::ffi::c_double = 0.;
static mut StorageInflow: ::core::ffi::c_double = 0.;
static mut StorageExfil: ::core::ffi::c_double = 0.;
static mut StorageEvap: ::core::ffi::c_double = 0.;
static mut StorageDrain: ::core::ffi::c_double = 0.;
static mut StorageVolume: ::core::ffi::c_double = 0.;
static mut Xold: [::core::ffi::c_double; 4] = [0.; 4];
#[no_mangle]
pub unsafe extern "C" fn lidproc_initWaterBalance(
    mut lidUnit: *mut TLidUnit,
    mut initVol: ::core::ffi::c_double,
) {
    (*lidUnit).waterBalance.inflow = 0.0f64;
    (*lidUnit).waterBalance.evap = 0.0f64;
    (*lidUnit).waterBalance.infil = 0.0f64;
    (*lidUnit).waterBalance.surfFlow = 0.0f64;
    (*lidUnit).waterBalance.drainFlow = 0.0f64;
    (*lidUnit).waterBalance.initVol = initVol;
    (*lidUnit).waterBalance.finalVol = initVol;
}
#[no_mangle]
pub unsafe extern "C" fn lidproc_getOutflow(
    mut lidUnit: *mut TLidUnit,
    mut lidProc: *mut TLidProc,
    mut inflow: ::core::ffi::c_double,
    mut evap: ::core::ffi::c_double,
    mut infil: ::core::ffi::c_double,
    mut maxInfil: ::core::ffi::c_double,
    mut tStep: ::core::ffi::c_double,
    mut lidEvap: *mut ::core::ffi::c_double,
    mut lidInfil: *mut ::core::ffi::c_double,
    mut lidDrain: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: [::core::ffi::c_double; 4] = [0.; 4];
    let mut xOld: [::core::ffi::c_double; 4] = [0.; 4];
    let mut xPrev: [::core::ffi::c_double; 4] = [0.; 4];
    let mut xMin: [::core::ffi::c_double; 4] = [0.; 4];
    let mut xMax: [::core::ffi::c_double; 4] = [0.; 4];
    let mut fOld: [::core::ffi::c_double; 4] = [0.; 4];
    let mut f: [::core::ffi::c_double; 4] = [0.; 4];
    let mut xTol: [::core::ffi::c_double; 4] = [STOPTOL, STOPTOL, STOPTOL, STOPTOL];
    let mut omega: ::core::ffi::c_double = 0.0f64;
    let mut fluxRates: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_double, *mut ::core::ffi::c_double) -> (),
    > = None;
    theLidProc = lidProc;
    theLidUnit = lidUnit;
    EvapRate = evap;
    MaxNativeInfil = maxInfil;
    Tstep = tStep;
    x[SURF as ::core::ffi::c_int as usize] = (*theLidUnit).surfaceDepth;
    x[SOIL as ::core::ffi::c_int as usize] = (*theLidUnit).soilMoisture;
    x[STOR as ::core::ffi::c_int as usize] = (*theLidUnit).storageDepth;
    x[PAVE as ::core::ffi::c_int as usize] = (*theLidUnit).paveDepth;
    SurfaceVolume = 0.0f64;
    PaveVolume = 0.0f64;
    SoilVolume = 0.0f64;
    StorageVolume = 0.0f64;
    SurfaceInflow = inflow;
    SurfaceInfil = 0.0f64;
    SurfaceEvap = 0.0f64;
    SurfaceOutflow = 0.0f64;
    PaveEvap = 0.0f64;
    PavePerc = 0.0f64;
    SoilEvap = 0.0f64;
    SoilPerc = 0.0f64;
    StorageInflow = 0.0f64;
    StorageExfil = 0.0f64;
    StorageEvap = 0.0f64;
    StorageDrain = 0.0f64;
    i = 0 as ::core::ffi::c_int;
    while i < MAX_LAYERS {
        f[i as usize] = 0.0f64;
        fOld[i as usize] = (*theLidUnit).oldFluxRates[i as usize];
        xMin[i as usize] = 0.0f64;
        xMax[i as usize] = BIG;
        Xold[i as usize] = x[i as usize];
        i += 1;
    }
    if (*theLidProc).lidType == POROUS_PAVEMENT as ::core::ffi::c_int {
        SurfaceInfil = 0.0f64;
    } else if (*theLidUnit).soilInfil.Ks > 0.0f64 {
        SurfaceInfil = grnampt_getInfil(
            &raw mut (*theLidUnit).soilInfil,
            Tstep,
            SurfaceInflow,
            (*theLidUnit).surfaceDepth,
            MOD_GREEN_AMPT as ::core::ffi::c_int,
        );
    } else {
        SurfaceInfil = infil;
    }
    if (*theLidProc).soil.thickness > 0.0f64 {
        xMin[SOIL as ::core::ffi::c_int as usize] = (*theLidProc).soil.wiltPoint;
        xMax[SOIL as ::core::ffi::c_int as usize] = (*theLidProc).soil.porosity;
    }
    if (*theLidProc).pavement.thickness > 0.0f64 {
        xMax[PAVE as ::core::ffi::c_int as usize] = (*theLidProc).pavement.thickness;
    }
    if (*theLidProc).storage.thickness > 0.0f64 {
        xMax[STOR as ::core::ffi::c_int as usize] = (*theLidProc).storage.thickness;
    }
    if (*theLidProc).lidType == GREEN_ROOF as ::core::ffi::c_int {
        xMax[STOR as ::core::ffi::c_int as usize] = (*theLidProc).drainMat.thickness;
    }
    match (*theLidProc).lidType {
        0 | 1 => {
            fluxRates = Some(
                biocellFluxRates
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
                >;
        }
        2 => {
            fluxRates = Some(
                greenRoofFluxRates
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
                >;
        }
        3 => {
            fluxRates = Some(
                trenchFluxRates
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
                >;
        }
        4 => {
            fluxRates = Some(
                pavementFluxRates
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
                >;
        }
        5 => {
            fluxRates = Some(
                barrelFluxRates
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
                >;
        }
        7 => {
            fluxRates = Some(
                roofFluxRates
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
                >;
        }
        6 => {
            fluxRates = Some(
                swaleFluxRates
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ::core::ffi::c_double,
                        *mut ::core::ffi::c_double,
                    ) -> (),
                >;
            omega = 0.5f64;
        }
        _ => return 0.0f64,
    }
    i = modpuls_solve(
        MAX_LAYERS,
        &raw mut x as *mut ::core::ffi::c_double,
        &raw mut xOld as *mut ::core::ffi::c_double,
        &raw mut xPrev as *mut ::core::ffi::c_double,
        &raw mut xMin as *mut ::core::ffi::c_double,
        &raw mut xMax as *mut ::core::ffi::c_double,
        &raw mut xTol as *mut ::core::ffi::c_double,
        &raw mut fOld as *mut ::core::ffi::c_double,
        &raw mut f as *mut ::core::ffi::c_double,
        tStep,
        omega,
        fluxRates,
    );
    if (*theLidProc).surface.canOverflow as ::core::ffi::c_int != 0
        || (*theLidUnit).fullWidth == 0.0f64
    {
        SurfaceOutflow += getSurfaceOverflowRate(
            (&raw mut x as *mut ::core::ffi::c_double).offset(SURF as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_double,
        );
    }
    (*theLidUnit).surfaceDepth = x[SURF as ::core::ffi::c_int as usize];
    (*theLidUnit).paveDepth = x[PAVE as ::core::ffi::c_int as usize];
    (*theLidUnit).soilMoisture = x[SOIL as ::core::ffi::c_int as usize];
    (*theLidUnit).storageDepth = x[STOR as ::core::ffi::c_int as usize];
    i = 0 as ::core::ffi::c_int;
    while i < MAX_LAYERS {
        (*theLidUnit).oldFluxRates[i as usize] = f[i as usize];
        i += 1;
    }
    *lidEvap = SurfaceEvap + PaveEvap + SoilEvap + StorageEvap;
    *lidInfil = StorageExfil;
    *lidDrain = StorageDrain;
    return SurfaceOutflow;
}
#[no_mangle]
pub unsafe extern "C" fn lidproc_saveResults(
    mut lidUnit: *mut TLidUnit,
    mut ucfRainfall: ::core::ffi::c_double,
    mut ucfRainDepth: ::core::ffi::c_double,
) {
    let mut ucf: ::core::ffi::c_double = 0.;
    let mut totalEvap: ::core::ffi::c_double = 0.;
    let mut totalVolume: ::core::ffi::c_double = 0.;
    let mut rptVars: [::core::ffi::c_double; 12] = [0.; 12];
    let mut isDry: ::core::ffi::c_int = FALSE;
    let mut timeStamp: [::core::ffi::c_char; 22] = [0; 22];
    let mut elapsedHrs: ::core::ffi::c_double = 0.;
    totalEvap = SurfaceEvap + PaveEvap + SoilEvap + StorageEvap;
    totalVolume = SurfaceVolume + PaveVolume + SoilVolume + StorageVolume;
    updateWaterBalance(
        theLidUnit,
        SurfaceInflow,
        totalEvap,
        StorageExfil,
        SurfaceOutflow,
        StorageDrain,
        totalVolume,
    );
    if SurfaceInflow < MINFLOW
        && SurfaceOutflow < MINFLOW
        && StorageDrain < MINFLOW
        && StorageExfil < MINFLOW
        && totalEvap < MINFLOW
    {
        isDry = TRUE;
    }
    if isDry == 0 {
        HasWetLids = TRUE as ::core::ffi::c_char;
    }
    if !(*lidUnit).rptFile.is_null() {
        ucf = ucfRainfall;
        rptVars[SURF_INFLOW as ::core::ffi::c_int as usize] = SurfaceInflow * ucf;
        rptVars[TOTAL_EVAP as ::core::ffi::c_int as usize] = totalEvap * ucf;
        rptVars[SURF_INFIL as ::core::ffi::c_int as usize] = SurfaceInfil * ucf;
        rptVars[PAVE_PERC as ::core::ffi::c_int as usize] = PavePerc * ucf;
        rptVars[SOIL_PERC as ::core::ffi::c_int as usize] = SoilPerc * ucf;
        rptVars[STOR_EXFIL as ::core::ffi::c_int as usize] = StorageExfil * ucf;
        rptVars[SURF_OUTFLOW as ::core::ffi::c_int as usize] = SurfaceOutflow * ucf;
        rptVars[STOR_DRAIN as ::core::ffi::c_int as usize] = StorageDrain * ucf;
        ucf = ucfRainDepth;
        rptVars[SURF_DEPTH as ::core::ffi::c_int as usize] = (*theLidUnit).surfaceDepth * ucf;
        rptVars[PAVE_DEPTH as ::core::ffi::c_int as usize] = (*theLidUnit).paveDepth * ucf;
        rptVars[SOIL_MOIST as ::core::ffi::c_int as usize] = (*theLidUnit).soilMoisture;
        rptVars[STOR_DEPTH as ::core::ffi::c_int as usize] = (*theLidUnit).storageDepth * ucf;
        if isDry == 0 && (*(*theLidUnit).rptFile).wasDry > 1 as ::core::ffi::c_int {
            fprintf(
                (*(*theLidUnit).rptFile).file,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*(*theLidUnit).rptFile).results as *mut ::core::ffi::c_char,
            );
        }
        elapsedHrs = NewRunoffTime / 1000.0f64 / 3600.0f64;
        datetime_getTimeStamp(
            M_D_Y,
            getDateTime(NewRunoffTime),
            TIME_STAMP_SIZE,
            &raw mut timeStamp as *mut ::core::ffi::c_char,
        );
        snprintf(
            &raw mut (*(*theLidUnit).rptFile).results as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as size_t,
            b"\n%20s\t %8.3f\t %8.3f\t %8.4f\t %8.3f\t %8.3f\t %8.3f\t %8.3f\t%8.3f\t %8.3f\t %8.3f\t %8.3f\t %8.3f\t %8.3f\0"
                as *const u8 as *const ::core::ffi::c_char,
            &raw mut timeStamp as *mut ::core::ffi::c_char,
            elapsedHrs,
            rptVars[0 as ::core::ffi::c_int as usize],
            rptVars[1 as ::core::ffi::c_int as usize],
            rptVars[2 as ::core::ffi::c_int as usize],
            rptVars[3 as ::core::ffi::c_int as usize],
            rptVars[4 as ::core::ffi::c_int as usize],
            rptVars[5 as ::core::ffi::c_int as usize],
            rptVars[6 as ::core::ffi::c_int as usize],
            rptVars[7 as ::core::ffi::c_int as usize],
            rptVars[8 as ::core::ffi::c_int as usize],
            rptVars[9 as ::core::ffi::c_int as usize],
            rptVars[10 as ::core::ffi::c_int as usize],
            rptVars[11 as ::core::ffi::c_int as usize],
        );
        if isDry != 0 {
            if (*(*theLidUnit).rptFile).wasDry == 0 as ::core::ffi::c_int {
                fprintf(
                    (*(*theLidUnit).rptFile).file,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    &raw mut (*(*theLidUnit).rptFile).results as *mut ::core::ffi::c_char,
                );
            }
            (*(*theLidUnit).rptFile).wasDry += 1;
        } else {
            fprintf(
                (*(*theLidUnit).rptFile).file,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut (*(*theLidUnit).rptFile).results as *mut ::core::ffi::c_char,
            );
            (*(*theLidUnit).rptFile).wasDry = 0 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn roofFluxRates(
    mut x: *mut ::core::ffi::c_double,
    mut f: *mut ::core::ffi::c_double,
) {
    let mut surfaceDepth: ::core::ffi::c_double = *x.offset(SURF as ::core::ffi::c_int as isize);
    getEvapRates(surfaceDepth, 0.0f64, 0.0f64, 0.0f64, 1.0f64);
    SurfaceVolume = surfaceDepth;
    SurfaceInfil = 0.0f64;
    if (*theLidProc).surface.alpha > 0.0f64 {
        SurfaceOutflow = getSurfaceOutflowRate(surfaceDepth);
    } else {
        getSurfaceOverflowRate(&raw mut surfaceDepth);
    }
    StorageDrain =
        if (*theLidProc).drain.coeff / UCF(RAINFALL as ::core::ffi::c_int) <= SurfaceOutflow {
            (*theLidProc).drain.coeff / UCF(RAINFALL as ::core::ffi::c_int)
        } else {
            SurfaceOutflow
        };
    SurfaceOutflow -= StorageDrain;
    *f.offset(SURF as ::core::ffi::c_int as isize) =
        SurfaceInflow - SurfaceEvap - StorageDrain - SurfaceOutflow;
}
unsafe extern "C" fn greenRoofFluxRates(
    mut x: *mut ::core::ffi::c_double,
    mut f: *mut ::core::ffi::c_double,
) {
    let mut surfaceDepth: ::core::ffi::c_double = 0.;
    let mut soilTheta: ::core::ffi::c_double = 0.;
    let mut storageDepth: ::core::ffi::c_double = 0.;
    let mut availVolume: ::core::ffi::c_double = 0.;
    let mut maxRate: ::core::ffi::c_double = 0.;
    let mut soilThickness: ::core::ffi::c_double = (*theLidProc).soil.thickness;
    let mut storageThickness: ::core::ffi::c_double = (*theLidProc).storage.thickness;
    let mut soilPorosity: ::core::ffi::c_double = (*theLidProc).soil.porosity;
    let mut storageVoidFrac: ::core::ffi::c_double = (*theLidProc).storage.voidFrac;
    let mut soilFieldCap: ::core::ffi::c_double = (*theLidProc).soil.fieldCap;
    let mut soilWiltPoint: ::core::ffi::c_double = (*theLidProc).soil.wiltPoint;
    surfaceDepth = *x.offset(SURF as ::core::ffi::c_int as isize);
    soilTheta = *x.offset(SOIL as ::core::ffi::c_int as isize);
    storageDepth = *x.offset(STOR as ::core::ffi::c_int as isize);
    SurfaceVolume = surfaceDepth * (*theLidProc).surface.voidFrac;
    SoilVolume = soilTheta * soilThickness;
    StorageVolume = storageDepth * storageVoidFrac;
    availVolume = SoilVolume - soilWiltPoint * soilThickness;
    getEvapRates(SurfaceVolume, 0.0f64, availVolume, StorageVolume, 1.0f64);
    if soilTheta >= soilPorosity {
        StorageEvap = 0.0f64;
    }
    SoilPerc = getSoilPercRate(soilTheta);
    availVolume = (soilTheta - soilFieldCap) * soilThickness;
    maxRate = (if availVolume >= 0.0f64 {
        availVolume
    } else {
        0.0f64
    }) / Tstep
        - SoilEvap;
    SoilPerc = if SoilPerc <= maxRate {
        SoilPerc
    } else {
        maxRate
    };
    SoilPerc = if SoilPerc >= 0.0f64 { SoilPerc } else { 0.0f64 };
    StorageExfil = 0.0f64;
    StorageDrain = getDrainMatOutflow(storageDepth);
    if soilTheta >= soilPorosity && storageDepth >= storageThickness {
        maxRate = if SoilPerc <= StorageDrain {
            SoilPerc
        } else {
            StorageDrain
        };
        SoilPerc = maxRate;
        StorageDrain = maxRate;
        SurfaceInfil = if SurfaceInfil <= maxRate {
            SurfaceInfil
        } else {
            maxRate
        };
    } else {
        maxRate = storageDepth * storageVoidFrac / Tstep - StorageEvap;
        if storageDepth >= storageThickness {
            maxRate += SoilPerc;
        }
        maxRate = if maxRate >= 0.0f64 { maxRate } else { 0.0f64 };
        StorageDrain = if StorageDrain <= maxRate {
            StorageDrain
        } else {
            maxRate
        };
        maxRate = (storageThickness - storageDepth) * storageVoidFrac / Tstep
            + StorageDrain
            + StorageEvap;
        SoilPerc = if SoilPerc <= maxRate {
            SoilPerc
        } else {
            maxRate
        };
        maxRate = (soilPorosity - soilTheta) * soilThickness / Tstep + SoilPerc + SoilEvap;
        SurfaceInfil = if SurfaceInfil <= maxRate {
            SurfaceInfil
        } else {
            maxRate
        };
    }
    SurfaceOutflow = getSurfaceOutflowRate(surfaceDepth);
    *f.offset(SURF as ::core::ffi::c_int as isize) =
        (SurfaceInflow - SurfaceEvap - SurfaceInfil - SurfaceOutflow)
            / (*theLidProc).surface.voidFrac;
    *f.offset(SOIL as ::core::ffi::c_int as isize) =
        (SurfaceInfil - SoilEvap - SoilPerc) / (*theLidProc).soil.thickness;
    *f.offset(STOR as ::core::ffi::c_int as isize) =
        (SoilPerc - StorageEvap - StorageDrain) / (*theLidProc).storage.voidFrac;
}
unsafe extern "C" fn biocellFluxRates(
    mut x: *mut ::core::ffi::c_double,
    mut f: *mut ::core::ffi::c_double,
) {
    let mut surfaceDepth: ::core::ffi::c_double = 0.;
    let mut soilTheta: ::core::ffi::c_double = 0.;
    let mut storageDepth: ::core::ffi::c_double = 0.;
    let mut availVolume: ::core::ffi::c_double = 0.;
    let mut maxRate: ::core::ffi::c_double = 0.;
    let mut soilThickness: ::core::ffi::c_double = (*theLidProc).soil.thickness;
    let mut soilPorosity: ::core::ffi::c_double = (*theLidProc).soil.porosity;
    let mut soilFieldCap: ::core::ffi::c_double = (*theLidProc).soil.fieldCap;
    let mut soilWiltPoint: ::core::ffi::c_double = (*theLidProc).soil.wiltPoint;
    let mut storageThickness: ::core::ffi::c_double = (*theLidProc).storage.thickness;
    let mut storageVoidFrac: ::core::ffi::c_double = (*theLidProc).storage.voidFrac;
    surfaceDepth = *x.offset(SURF as ::core::ffi::c_int as isize);
    soilTheta = *x.offset(SOIL as ::core::ffi::c_int as isize);
    storageDepth = *x.offset(STOR as ::core::ffi::c_int as isize);
    SurfaceVolume = surfaceDepth * (*theLidProc).surface.voidFrac;
    SoilVolume = soilTheta * soilThickness;
    StorageVolume = storageDepth * storageVoidFrac;
    availVolume = SoilVolume - soilWiltPoint * soilThickness;
    getEvapRates(SurfaceVolume, 0.0f64, availVolume, StorageVolume, 1.0f64);
    if soilTheta >= soilPorosity {
        StorageEvap = 0.0f64;
    }
    SoilPerc = getSoilPercRate(soilTheta);
    availVolume = (soilTheta - soilFieldCap) * soilThickness;
    maxRate = (if availVolume >= 0.0f64 {
        availVolume
    } else {
        0.0f64
    }) / Tstep
        - SoilEvap;
    SoilPerc = if SoilPerc <= maxRate {
        SoilPerc
    } else {
        maxRate
    };
    SoilPerc = if SoilPerc >= 0.0f64 { SoilPerc } else { 0.0f64 };
    StorageExfil = getStorageExfilRate();
    StorageDrain = 0.0f64;
    if (*theLidProc).drain.coeff > 0.0f64 {
        StorageDrain = getStorageDrainRate(storageDepth, soilTheta, 0.0f64, surfaceDepth);
    }
    if storageThickness == 0.0f64 {
        StorageEvap = 0.0f64;
        maxRate = if SoilPerc <= StorageExfil {
            SoilPerc
        } else {
            StorageExfil
        };
        SoilPerc = maxRate;
        StorageExfil = maxRate;
        maxRate = (soilPorosity - soilTheta) * soilThickness / Tstep + SoilPerc + SoilEvap;
        SurfaceInfil = if SurfaceInfil <= maxRate {
            SurfaceInfil
        } else {
            maxRate
        };
    } else if soilTheta >= soilPorosity && storageDepth >= storageThickness {
        maxRate = StorageExfil + StorageDrain;
        if SoilPerc < maxRate {
            maxRate = SoilPerc;
            if maxRate > StorageExfil {
                StorageDrain = maxRate - StorageExfil;
            } else {
                StorageExfil = maxRate;
                StorageDrain = 0.0f64;
            }
        } else {
            SoilPerc = maxRate;
        }
        SurfaceInfil = if SurfaceInfil <= maxRate {
            SurfaceInfil
        } else {
            maxRate
        };
    } else {
        maxRate = SoilPerc - StorageEvap + storageDepth * storageVoidFrac / Tstep;
        StorageExfil = if StorageExfil <= maxRate {
            StorageExfil
        } else {
            maxRate
        };
        StorageExfil = if StorageExfil >= 0.0f64 {
            StorageExfil
        } else {
            0.0f64
        };
        if StorageDrain > 0.0f64 {
            maxRate = -StorageExfil - StorageEvap;
            if storageDepth >= storageThickness {
                maxRate += SoilPerc;
            }
            if (*theLidProc).drain.offset <= storageDepth {
                maxRate += (storageDepth - (*theLidProc).drain.offset) * storageVoidFrac / Tstep;
            }
            maxRate = if maxRate >= 0.0f64 { maxRate } else { 0.0f64 };
            StorageDrain = if StorageDrain <= maxRate {
                StorageDrain
            } else {
                maxRate
            };
        }
        maxRate = StorageExfil
            + StorageDrain
            + StorageEvap
            + (storageThickness - storageDepth) * storageVoidFrac / Tstep;
        SoilPerc = if SoilPerc <= maxRate {
            SoilPerc
        } else {
            maxRate
        };
        maxRate = (soilPorosity - soilTheta) * soilThickness / Tstep + SoilPerc + SoilEvap;
        SurfaceInfil = if SurfaceInfil <= maxRate {
            SurfaceInfil
        } else {
            maxRate
        };
    }
    SurfaceOutflow = getSurfaceOutflowRate(surfaceDepth);
    *f.offset(SURF as ::core::ffi::c_int as isize) =
        (SurfaceInflow - SurfaceEvap - SurfaceInfil - SurfaceOutflow)
            / (*theLidProc).surface.voidFrac;
    *f.offset(SOIL as ::core::ffi::c_int as isize) =
        (SurfaceInfil - SoilEvap - SoilPerc) / (*theLidProc).soil.thickness;
    if storageThickness == 0.0f64 {
        *f.offset(STOR as ::core::ffi::c_int as isize) = 0.0f64;
    } else {
        *f.offset(STOR as ::core::ffi::c_int as isize) =
            (SoilPerc - StorageEvap - StorageExfil - StorageDrain) / (*theLidProc).storage.voidFrac;
    };
}
unsafe extern "C" fn trenchFluxRates(
    mut x: *mut ::core::ffi::c_double,
    mut f: *mut ::core::ffi::c_double,
) {
    let mut surfaceDepth: ::core::ffi::c_double = 0.;
    let mut storageDepth: ::core::ffi::c_double = 0.;
    let mut availVolume: ::core::ffi::c_double = 0.;
    let mut maxRate: ::core::ffi::c_double = 0.;
    let mut storageThickness: ::core::ffi::c_double = (*theLidProc).storage.thickness;
    let mut storageVoidFrac: ::core::ffi::c_double = (*theLidProc).storage.voidFrac;
    surfaceDepth = *x.offset(SURF as ::core::ffi::c_int as isize);
    storageDepth = *x.offset(STOR as ::core::ffi::c_int as isize);
    SurfaceVolume = surfaceDepth * (*theLidProc).surface.voidFrac;
    SoilVolume = 0.0f64;
    StorageVolume = storageDepth * storageVoidFrac;
    availVolume = (storageThickness - storageDepth) * storageVoidFrac;
    getEvapRates(SurfaceVolume, 0.0f64, 0.0f64, StorageVolume, 1.0f64);
    if surfaceDepth > 0.0f64 {
        StorageEvap = 0.0f64;
    }
    StorageInflow = SurfaceInflow + SurfaceVolume / Tstep;
    StorageExfil = getStorageExfilRate();
    StorageDrain = 0.0f64;
    if (*theLidProc).drain.coeff > 0.0f64 {
        StorageDrain = getStorageDrainRate(storageDepth, 0.0f64, 0.0f64, surfaceDepth);
    }
    maxRate = StorageInflow - StorageEvap + storageDepth * storageVoidFrac / Tstep;
    StorageExfil = if StorageExfil <= maxRate {
        StorageExfil
    } else {
        maxRate
    };
    StorageExfil = if StorageExfil >= 0.0f64 {
        StorageExfil
    } else {
        0.0f64
    };
    if StorageDrain > 0.0f64 {
        maxRate = -StorageExfil - StorageEvap;
        if storageDepth >= storageThickness {
            maxRate += StorageInflow;
        }
        if (*theLidProc).drain.offset <= storageDepth {
            maxRate += (storageDepth - (*theLidProc).drain.offset) * storageVoidFrac / Tstep;
        }
        maxRate = if maxRate >= 0.0f64 { maxRate } else { 0.0f64 };
        StorageDrain = if StorageDrain <= maxRate {
            StorageDrain
        } else {
            maxRate
        };
    }
    maxRate = (storageThickness - storageDepth) * storageVoidFrac / Tstep
        + StorageExfil
        + StorageEvap
        + StorageDrain;
    StorageInflow = if StorageInflow <= maxRate {
        StorageInflow
    } else {
        maxRate
    };
    SurfaceInfil = StorageInflow;
    SurfaceOutflow = getSurfaceOutflowRate(surfaceDepth);
    *f.offset(SURF as ::core::ffi::c_int as isize) =
        (SurfaceInflow - SurfaceEvap - StorageInflow - SurfaceOutflow)
            / (*theLidProc).surface.voidFrac;
    *f.offset(STOR as ::core::ffi::c_int as isize) =
        (StorageInflow - StorageEvap - StorageExfil - StorageDrain)
            / (*theLidProc).storage.voidFrac;
    *f.offset(SOIL as ::core::ffi::c_int as isize) = 0.0f64;
}
unsafe extern "C" fn pavementFluxRates(
    mut x: *mut ::core::ffi::c_double,
    mut f: *mut ::core::ffi::c_double,
) {
    let mut surfaceDepth: ::core::ffi::c_double = 0.;
    let mut paveDepth: ::core::ffi::c_double = 0.;
    let mut soilTheta: ::core::ffi::c_double = 0.;
    let mut storageDepth: ::core::ffi::c_double = 0.;
    let mut pervFrac: ::core::ffi::c_double = 1.0f64 - (*theLidProc).pavement.impervFrac;
    let mut storageInflow: ::core::ffi::c_double = 0.;
    let mut availVolume: ::core::ffi::c_double = 0.;
    let mut maxRate: ::core::ffi::c_double = 0.;
    let mut paveVoidFrac: ::core::ffi::c_double = (*theLidProc).pavement.voidFrac * pervFrac;
    let mut paveThickness: ::core::ffi::c_double = (*theLidProc).pavement.thickness;
    let mut soilThickness: ::core::ffi::c_double = (*theLidProc).soil.thickness;
    let mut soilPorosity: ::core::ffi::c_double = (*theLidProc).soil.porosity;
    let mut soilFieldCap: ::core::ffi::c_double = (*theLidProc).soil.fieldCap;
    let mut soilWiltPoint: ::core::ffi::c_double = (*theLidProc).soil.wiltPoint;
    let mut storageThickness: ::core::ffi::c_double = (*theLidProc).storage.thickness;
    let mut storageVoidFrac: ::core::ffi::c_double = (*theLidProc).storage.voidFrac;
    surfaceDepth = *x.offset(SURF as ::core::ffi::c_int as isize);
    paveDepth = *x.offset(PAVE as ::core::ffi::c_int as isize);
    soilTheta = *x.offset(SOIL as ::core::ffi::c_int as isize);
    storageDepth = *x.offset(STOR as ::core::ffi::c_int as isize);
    SurfaceVolume = surfaceDepth * (*theLidProc).surface.voidFrac;
    PaveVolume = paveDepth * paveVoidFrac;
    SoilVolume = soilTheta * soilThickness;
    StorageVolume = storageDepth * storageVoidFrac;
    availVolume = SoilVolume - soilWiltPoint * soilThickness;
    getEvapRates(
        SurfaceVolume,
        PaveVolume,
        availVolume,
        StorageVolume,
        pervFrac,
    );
    if paveDepth >= paveThickness || soilThickness > 0.0f64 && soilTheta >= soilPorosity {
        StorageEvap = 0.0f64;
    }
    SurfaceInfil = SurfaceInflow + SurfaceVolume / Tstep;
    PavePerc = getPavementPermRate() * pervFrac;
    SurfaceInfil = if SurfaceInfil <= PavePerc {
        SurfaceInfil
    } else {
        PavePerc
    };
    maxRate = PaveVolume / Tstep + SurfaceInfil - PaveEvap;
    maxRate = if maxRate >= 0.0f64 { maxRate } else { 0.0f64 };
    PavePerc = if PavePerc <= maxRate {
        PavePerc
    } else {
        maxRate
    };
    if soilThickness > 0.0f64 {
        SoilPerc = getSoilPercRate(soilTheta);
        availVolume = (soilTheta - soilFieldCap) * soilThickness;
        maxRate = (if availVolume >= 0.0f64 {
            availVolume
        } else {
            0.0f64
        }) / Tstep
            - SoilEvap;
        SoilPerc = if SoilPerc <= maxRate {
            SoilPerc
        } else {
            maxRate
        };
        SoilPerc = if SoilPerc >= 0.0f64 { SoilPerc } else { 0.0f64 };
    } else {
        SoilPerc = PavePerc;
    }
    StorageExfil = getStorageExfilRate();
    StorageDrain = 0.0f64;
    if (*theLidProc).drain.coeff > 0.0f64 {
        StorageDrain = getStorageDrainRate(storageDepth, soilTheta, paveDepth, surfaceDepth);
    }
    if soilThickness == 0.0f64 && storageDepth >= storageThickness && paveDepth >= paveThickness {
        maxRate = StorageEvap + StorageDrain + StorageExfil;
        if PavePerc > maxRate {
            PavePerc = maxRate;
        } else {
            StorageExfil = if StorageExfil <= PavePerc {
                StorageExfil
            } else {
                PavePerc
            };
            StorageDrain = PavePerc - StorageExfil;
        }
        SoilPerc = PavePerc;
        SurfaceInfil = if SurfaceInfil <= PavePerc {
            SurfaceInfil
        } else {
            PavePerc
        };
    } else if soilThickness > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        && storageDepth >= storageThickness
        && soilTheta >= soilPorosity
        && paveDepth >= paveThickness
    {
        maxRate = StorageExfil + StorageDrain;
        if SoilPerc < maxRate {
            maxRate = SoilPerc;
        } else {
            maxRate = if maxRate <= PavePerc {
                maxRate
            } else {
                PavePerc
            };
        }
        if maxRate > StorageExfil {
            StorageDrain = maxRate - StorageExfil;
        } else {
            StorageExfil = maxRate;
            StorageDrain = 0.0f64;
        }
        SoilPerc = maxRate;
        PavePerc = maxRate;
        SurfaceInfil = if SurfaceInfil <= PavePerc {
            SurfaceInfil
        } else {
            PavePerc
        };
    } else if soilThickness > 0.0f64
        && storageDepth >= storageThickness
        && soilTheta >= soilPorosity
    {
        maxRate = StorageDrain + StorageExfil;
        if SoilPerc > maxRate {
            SoilPerc = maxRate;
        } else {
            StorageExfil = if StorageExfil <= SoilPerc {
                StorageExfil
            } else {
                SoilPerc
            };
            StorageDrain = SoilPerc - StorageExfil;
        }
        PavePerc = if PavePerc <= SoilPerc {
            PavePerc
        } else {
            SoilPerc
        };
        availVolume = (paveThickness - paveDepth) * paveVoidFrac;
        maxRate = availVolume / Tstep + PavePerc + PaveEvap;
        SurfaceInfil = if SurfaceInfil <= maxRate {
            SurfaceInfil
        } else {
            maxRate
        };
    } else if soilThickness > 0.0f64 && paveDepth >= paveThickness && soilTheta >= soilPorosity {
        PavePerc = if PavePerc <= SoilPerc {
            PavePerc
        } else {
            SoilPerc
        };
        SoilPerc = PavePerc;
        SurfaceInfil = if SurfaceInfil <= PavePerc {
            SurfaceInfil
        } else {
            PavePerc
        };
        maxRate = if StorageVolume / Tstep + SoilPerc - StorageEvap >= 0.0f64 {
            StorageVolume / Tstep + SoilPerc - StorageEvap
        } else {
            0.0f64
        };
        StorageExfil = if StorageExfil <= maxRate {
            StorageExfil
        } else {
            maxRate
        };
    } else {
        maxRate = SoilPerc - StorageEvap + StorageVolume / Tstep;
        maxRate = if 0.0f64 >= maxRate { 0.0f64 } else { maxRate };
        StorageExfil = if StorageExfil <= maxRate {
            StorageExfil
        } else {
            maxRate
        };
        if StorageDrain > 0.0f64 {
            maxRate = -StorageExfil - StorageEvap;
            if storageDepth >= storageThickness {
                maxRate += SoilPerc;
            }
            if (*theLidProc).drain.offset <= storageDepth {
                maxRate += (storageDepth - (*theLidProc).drain.offset) * storageVoidFrac / Tstep;
            }
            maxRate = if maxRate >= 0.0f64 { maxRate } else { 0.0f64 };
            StorageDrain = if StorageDrain <= maxRate {
                StorageDrain
            } else {
                maxRate
            };
        }
        availVolume = (storageThickness - storageDepth) * storageVoidFrac;
        maxRate = availVolume / Tstep + StorageEvap + StorageDrain + StorageExfil;
        maxRate = if maxRate >= 0.0f64 { maxRate } else { 0.0f64 };
        if soilThickness > 0.0f64 {
            SoilPerc = if SoilPerc <= maxRate {
                SoilPerc
            } else {
                maxRate
            };
            maxRate = (soilPorosity - soilTheta) * soilThickness / Tstep + SoilPerc;
        }
        PavePerc = if PavePerc <= maxRate {
            PavePerc
        } else {
            maxRate
        };
        availVolume = (paveThickness - paveDepth) * paveVoidFrac;
        maxRate = availVolume / Tstep + PavePerc + PaveEvap;
        SurfaceInfil = if SurfaceInfil <= maxRate {
            SurfaceInfil
        } else {
            maxRate
        };
    }
    SurfaceOutflow = getSurfaceOutflowRate(surfaceDepth);
    *f.offset(SURF as ::core::ffi::c_int as isize) =
        SurfaceInflow - SurfaceEvap - SurfaceInfil - SurfaceOutflow;
    *f.offset(PAVE as ::core::ffi::c_int as isize) =
        (SurfaceInfil - PaveEvap - PavePerc) / paveVoidFrac;
    if (*theLidProc).soil.thickness > 0.0f64 {
        *f.offset(SOIL as ::core::ffi::c_int as isize) =
            (PavePerc - SoilEvap - SoilPerc) / soilThickness;
        storageInflow = SoilPerc;
    } else {
        *f.offset(SOIL as ::core::ffi::c_int as isize) = 0.0f64;
        storageInflow = PavePerc;
        SoilPerc = 0.0f64;
    }
    *f.offset(STOR as ::core::ffi::c_int as isize) =
        (storageInflow - StorageEvap - StorageExfil - StorageDrain) / storageVoidFrac;
}
unsafe extern "C" fn swaleFluxRates(
    mut x: *mut ::core::ffi::c_double,
    mut f: *mut ::core::ffi::c_double,
) {
    let mut depth: ::core::ffi::c_double = 0.;
    let mut topWidth: ::core::ffi::c_double = 0.;
    let mut botWidth: ::core::ffi::c_double = 0.;
    let mut length: ::core::ffi::c_double = 0.;
    let mut surfInflow: ::core::ffi::c_double = 0.;
    let mut surfWidth: ::core::ffi::c_double = 0.;
    let mut surfArea: ::core::ffi::c_double = 0.;
    let mut flowArea: ::core::ffi::c_double = 0.;
    let mut lidArea: ::core::ffi::c_double = 0.;
    let mut hydRadius: ::core::ffi::c_double = 0.;
    let mut slope: ::core::ffi::c_double = 0.;
    let mut volume: ::core::ffi::c_double = 0.;
    let mut dVdT: ::core::ffi::c_double = 0.;
    let mut dStore: ::core::ffi::c_double = 0.;
    let mut xDepth: ::core::ffi::c_double = 0.;
    depth = *x.offset(SURF as ::core::ffi::c_int as isize);
    depth = if depth <= (*theLidProc).surface.thickness {
        depth
    } else {
        (*theLidProc).surface.thickness
    };
    dStore = 0.0f64;
    slope = (*theLidProc).surface.sideSlope;
    topWidth = (*theLidUnit).fullWidth;
    topWidth = if topWidth >= 0.5f64 { topWidth } else { 0.5f64 };
    botWidth = topWidth - 2.0f64 * slope * (*theLidProc).surface.thickness;
    if botWidth < 0.5f64 {
        botWidth = 0.5f64;
        slope = 0.5f64 * (topWidth - 0.5f64) / (*theLidProc).surface.thickness;
    }
    lidArea = (*theLidUnit).area;
    length = lidArea / topWidth;
    surfWidth = botWidth + 2.0f64 * slope * depth;
    surfArea = length * surfWidth;
    flowArea = depth * (botWidth + slope * depth) * (*theLidProc).surface.voidFrac;
    volume = length * flowArea;
    surfInflow = SurfaceInflow * lidArea;
    SurfaceEvap = EvapRate * surfArea;
    SurfaceEvap = if SurfaceEvap <= volume / Tstep {
        SurfaceEvap
    } else {
        volume / Tstep
    };
    StorageExfil = SurfaceInfil * surfArea;
    xDepth = depth - dStore;
    if xDepth <= ZERO {
        SurfaceOutflow = 0.0f64;
    } else {
        flowArea -= dStore * (botWidth + slope * dStore) * (*theLidProc).surface.voidFrac;
        if flowArea < ZERO {
            SurfaceOutflow = 0.0f64;
        } else {
            botWidth = botWidth + 2.0f64 * dStore * slope;
            hydRadius = botWidth + 2.0f64 * xDepth * sqrt(1.0f64 + slope * slope);
            hydRadius = flowArea / hydRadius;
            SurfaceOutflow =
                (*theLidProc).surface.alpha * flowArea * pow(hydRadius, 2.0f64 / 3.0f64);
        }
    }
    dVdT = surfInflow - SurfaceEvap - StorageExfil - SurfaceOutflow;
    if depth == (*theLidProc).surface.thickness && dVdT > 0.0f64 {
        SurfaceOutflow += dVdT;
        dVdT = 0.0f64;
    }
    SurfaceEvap /= lidArea;
    StorageExfil /= lidArea;
    SurfaceOutflow /= lidArea;
    *f.offset(SURF as ::core::ffi::c_int as isize) = dVdT / surfArea;
    *f.offset(SOIL as ::core::ffi::c_int as isize) = 0.0f64;
    *f.offset(STOR as ::core::ffi::c_int as isize) = 0.0f64;
    SurfaceVolume = volume / lidArea;
    SoilVolume = 0.0f64;
    StorageVolume = 0.0f64;
}
unsafe extern "C" fn barrelFluxRates(
    mut x: *mut ::core::ffi::c_double,
    mut f: *mut ::core::ffi::c_double,
) {
    let mut storageDepth: ::core::ffi::c_double = *x.offset(STOR as ::core::ffi::c_int as isize);
    let mut head: ::core::ffi::c_double = 0.;
    let mut maxValue: ::core::ffi::c_double = 0.;
    SurfaceVolume = 0.0f64;
    SoilVolume = 0.0f64;
    StorageVolume = storageDepth;
    SurfaceInfil = 0.0f64;
    SurfaceOutflow = 0.0f64;
    StorageDrain = 0.0f64;
    if (*theLidProc).drain.delay == 0.0f64 || (*theLidUnit).dryTime >= (*theLidProc).drain.delay {
        head = storageDepth - (*theLidProc).drain.offset;
        if head > 0.0f64 {
            StorageDrain = getStorageDrainRate(storageDepth, 0.0f64, 0.0f64, 0.0f64);
            maxValue = head / Tstep;
            StorageDrain = if StorageDrain <= maxValue {
                StorageDrain
            } else {
                maxValue
            };
        }
    }
    StorageInflow = SurfaceInflow;
    maxValue = ((*theLidProc).storage.thickness - storageDepth) / Tstep + StorageDrain;
    StorageInflow = if StorageInflow <= maxValue {
        StorageInflow
    } else {
        maxValue
    };
    SurfaceInfil = StorageInflow;
    *f.offset(SURF as ::core::ffi::c_int as isize) = SurfaceInflow - StorageInflow;
    *f.offset(STOR as ::core::ffi::c_int as isize) = StorageInflow - StorageDrain;
    *f.offset(SOIL as ::core::ffi::c_int as isize) = 0.0f64;
}
unsafe extern "C" fn getSurfaceOutflowRate(
    mut depth: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut delta: ::core::ffi::c_double = 0.;
    let mut outflow: ::core::ffi::c_double = 0.;
    delta = depth - (*theLidProc).surface.thickness;
    if delta < 0.0f64 {
        return 0.0f64;
    }
    outflow = (*theLidProc).surface.alpha * pow(delta, 5.0f64 / 3.0f64) * (*theLidUnit).fullWidth
        / (*theLidUnit).area;
    outflow = if outflow <= delta / Tstep {
        outflow
    } else {
        delta / Tstep
    };
    return outflow;
}
unsafe extern "C" fn getPavementPermRate() -> ::core::ffi::c_double {
    let mut permReduction: ::core::ffi::c_double = 0.0f64;
    let mut clogFactor: ::core::ffi::c_double = (*theLidProc).pavement.clogFactor;
    let mut regenDays: ::core::ffi::c_double = (*theLidProc).pavement.regenDays;
    if clogFactor > 0.0f64 {
        if regenDays > 0.0f64 {
            if OldRunoffTime / 1000.0f64 / SECperDAY as ::core::ffi::c_double
                >= (*theLidUnit).nextRegenDay
            {
                (*theLidUnit).volTreated *= 1.0f64 - (*theLidProc).pavement.regenDegree;
                (*theLidUnit).nextRegenDay += regenDays;
            }
        }
        permReduction = (*theLidUnit).volTreated / clogFactor;
        permReduction = if permReduction <= 1.0f64 {
            permReduction
        } else {
            1.0f64
        };
    }
    return (*theLidProc).pavement.kSat * (1.0f64 - permReduction);
}
unsafe extern "C" fn getSoilPercRate(mut theta: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let mut delta: ::core::ffi::c_double = 0.;
    if theta <= (*theLidProc).soil.fieldCap {
        return 0.0f64;
    }
    delta = (*theLidProc).soil.porosity - theta;
    return (*theLidProc).soil.kSat * exp(-delta * (*theLidProc).soil.kSlope);
}
unsafe extern "C" fn getStorageExfilRate() -> ::core::ffi::c_double {
    let mut infil: ::core::ffi::c_double = 0.0f64;
    let mut clogFactor: ::core::ffi::c_double = 0.0f64;
    if (*theLidProc).storage.kSat == 0.0f64 {
        return 0.0f64;
    }
    if MaxNativeInfil == 0.0f64 {
        return 0.0f64;
    }
    clogFactor = (*theLidProc).storage.clogFactor;
    if clogFactor > 0.0f64 {
        clogFactor = (*theLidUnit).waterBalance.inflow / clogFactor;
        clogFactor = if clogFactor <= 1.0f64 {
            clogFactor
        } else {
            1.0f64
        };
    }
    infil = (*theLidProc).storage.kSat * (1.0f64 - clogFactor);
    return if infil <= MaxNativeInfil {
        infil
    } else {
        MaxNativeInfil
    };
}
unsafe extern "C" fn getStorageDrainRate(
    mut storageDepth: ::core::ffi::c_double,
    mut soilTheta: ::core::ffi::c_double,
    mut paveDepth: ::core::ffi::c_double,
    mut surfaceDepth: ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut curve: ::core::ffi::c_int = (*theLidProc).drain.qCurve;
    let mut head: ::core::ffi::c_double = storageDepth;
    let mut outflow: ::core::ffi::c_double = 0.0f64;
    let mut paveThickness: ::core::ffi::c_double = (*theLidProc).pavement.thickness;
    let mut soilThickness: ::core::ffi::c_double = (*theLidProc).soil.thickness;
    let mut soilPorosity: ::core::ffi::c_double = (*theLidProc).soil.porosity;
    let mut soilFieldCap: ::core::ffi::c_double = (*theLidProc).soil.fieldCap;
    let mut storageThickness: ::core::ffi::c_double = (*theLidProc).storage.thickness;
    if storageDepth >= storageThickness {
        if soilThickness > 0.0f64 {
            if soilTheta > soilFieldCap {
                head += (soilTheta - soilFieldCap) / (soilPorosity - soilFieldCap) * soilThickness;
                if soilTheta >= soilPorosity {
                    if paveThickness > 0.0f64 {
                        head += paveDepth;
                        if paveDepth >= paveThickness {
                            head += surfaceDepth;
                        }
                    } else {
                        head += surfaceDepth;
                    }
                }
            }
        } else if paveThickness > 0.0f64 {
            head += paveDepth;
            if paveDepth >= paveThickness {
                head += surfaceDepth;
            }
        }
    }
    if (*theLidUnit).oldDrainFlow == 0.0f64 && head <= (*theLidProc).drain.hOpen {
        return 0.0f64;
    }
    if (*theLidUnit).oldDrainFlow > 0.0f64 && head <= (*theLidProc).drain.hClose {
        return 0.0f64;
    }
    head -= (*theLidProc).drain.offset;
    if head > ZERO {
        head *= UCF(RAINDEPTH as ::core::ffi::c_int);
        outflow = (*theLidProc).drain.coeff * pow(head, (*theLidProc).drain.expon);
        if curve >= 0 as ::core::ffi::c_int {
            outflow *= table_lookup(Curve.offset(curve as isize) as *mut TTable, head);
        }
        outflow /= UCF(RAINFALL as ::core::ffi::c_int);
    }
    return outflow;
}
unsafe extern "C" fn getDrainMatOutflow(mut depth: ::core::ffi::c_double) -> ::core::ffi::c_double {
    let mut result: ::core::ffi::c_double = SoilPerc;
    if (*theLidProc).drainMat.alpha > 0.0f64 {
        result =
            (*theLidProc).drainMat.alpha * pow(depth, 5.0f64 / 3.0f64) * (*theLidUnit).fullWidth
                / (*theLidUnit).area
                * (*theLidProc).drainMat.voidFrac;
    }
    return result;
}
unsafe extern "C" fn getEvapRates(
    mut surfaceVol: ::core::ffi::c_double,
    mut paveVol: ::core::ffi::c_double,
    mut soilVol: ::core::ffi::c_double,
    mut storageVol: ::core::ffi::c_double,
    mut pervFrac: ::core::ffi::c_double,
) {
    let mut availEvap: ::core::ffi::c_double = 0.;
    availEvap = EvapRate;
    SurfaceEvap = if availEvap <= surfaceVol / Tstep {
        availEvap
    } else {
        surfaceVol / Tstep
    };
    SurfaceEvap = if 0.0f64 >= SurfaceEvap {
        0.0f64
    } else {
        SurfaceEvap
    };
    availEvap = if 0.0f64 >= availEvap - SurfaceEvap {
        0.0f64
    } else {
        availEvap - SurfaceEvap
    };
    availEvap *= pervFrac;
    if SurfaceInfil > 0.0f64 {
        PaveEvap = 0.0f64;
        SoilEvap = 0.0f64;
        StorageEvap = 0.0f64;
    } else {
        PaveEvap = if availEvap <= paveVol / Tstep {
            availEvap
        } else {
            paveVol / Tstep
        };
        availEvap = if 0.0f64 >= availEvap - PaveEvap {
            0.0f64
        } else {
            availEvap - PaveEvap
        };
        SoilEvap = if availEvap <= soilVol / Tstep {
            availEvap
        } else {
            soilVol / Tstep
        };
        availEvap = if 0.0f64 >= availEvap - SoilEvap {
            0.0f64
        } else {
            availEvap - SoilEvap
        };
        StorageEvap = if availEvap <= storageVol / Tstep {
            availEvap
        } else {
            storageVol / Tstep
        };
    };
}
unsafe extern "C" fn getSurfaceOverflowRate(
    mut surfaceDepth: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_double {
    let mut delta: ::core::ffi::c_double = *surfaceDepth - (*theLidProc).surface.thickness;
    if delta <= 0.0f64 {
        return 0.0f64;
    }
    *surfaceDepth = (*theLidProc).surface.thickness;
    return delta * (*theLidProc).surface.voidFrac / Tstep;
}
unsafe extern "C" fn updateWaterBalance(
    mut lidUnit: *mut TLidUnit,
    mut inflow: ::core::ffi::c_double,
    mut evap: ::core::ffi::c_double,
    mut infil: ::core::ffi::c_double,
    mut surfFlow: ::core::ffi::c_double,
    mut drainFlow: ::core::ffi::c_double,
    mut storage: ::core::ffi::c_double,
) {
    (*lidUnit).volTreated += inflow * Tstep;
    (*lidUnit).waterBalance.inflow += inflow * Tstep;
    (*lidUnit).waterBalance.evap += evap * Tstep;
    (*lidUnit).waterBalance.infil += infil * Tstep;
    (*lidUnit).waterBalance.surfFlow += surfFlow * Tstep;
    (*lidUnit).waterBalance.drainFlow += drainFlow * Tstep;
    (*lidUnit).waterBalance.finalVol = storage;
}
unsafe extern "C" fn modpuls_solve(
    mut n: ::core::ffi::c_int,
    mut x: *mut ::core::ffi::c_double,
    mut xOld: *mut ::core::ffi::c_double,
    mut xPrev: *mut ::core::ffi::c_double,
    mut xMin: *mut ::core::ffi::c_double,
    mut xMax: *mut ::core::ffi::c_double,
    mut xTol: *mut ::core::ffi::c_double,
    mut qOld: *mut ::core::ffi::c_double,
    mut q: *mut ::core::ffi::c_double,
    mut dt: ::core::ffi::c_double,
    mut omega: ::core::ffi::c_double,
    mut derivs: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_double, *mut ::core::ffi::c_double) -> (),
    >,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut canStop: ::core::ffi::c_int = 0;
    let mut steps: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut maxSteps: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *xOld.offset(i as isize) = *x.offset(i as isize);
        *xPrev.offset(i as isize) = *x.offset(i as isize);
        i += 1;
    }
    while steps < maxSteps {
        canStop = 1 as ::core::ffi::c_int;
        derivs.expect("non-null function pointer")(x, q);
        i = 0 as ::core::ffi::c_int;
        while i < n {
            *x.offset(i as isize) = *xOld.offset(i as isize)
                + (omega * *qOld.offset(i as isize) + (1.0f64 - omega) * *q.offset(i as isize))
                    * dt;
            *x.offset(i as isize) = if *x.offset(i as isize) <= *xMax.offset(i as isize) {
                *x.offset(i as isize)
            } else {
                *xMax.offset(i as isize)
            };
            *x.offset(i as isize) = if *x.offset(i as isize) >= *xMin.offset(i as isize) {
                *x.offset(i as isize)
            } else {
                *xMin.offset(i as isize)
            };
            if omega > 0.0f64
                && fabs(*x.offset(i as isize) - *xPrev.offset(i as isize))
                    > *xTol.offset(i as isize)
            {
                canStop = 0 as ::core::ffi::c_int;
            }
            *xPrev.offset(i as isize) = *x.offset(i as isize);
            i += 1;
        }
        if canStop != 0 {
            return steps;
        }
        steps += 1;
    }
    return 0 as ::core::ffi::c_int;
}
pub const M_D_Y: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TIME_STAMP_SIZE: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();

#[repr(C)]
pub struct _IO_wide_data {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _IO_codecvt {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _IO_marker {
    _opaque: [u8; 0],
}
