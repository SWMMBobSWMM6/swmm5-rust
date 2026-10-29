#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const w_FLOW_UNITS: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"FLOW_UNITS\0") };
pub const w_INFIL_MODEL: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"INFILTRATION\0") };
pub const w_ROUTE_MODEL: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"FLOW_ROUTING\0") };
pub const w_START_DATE: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"START_DATE\0") };
pub const w_START_TIME: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"START_TIME\0") };
pub const w_END_DATE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"END_DATE\0") };
pub const w_END_TIME: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"END_TIME\0") };
pub const w_REPORT_START_DATE: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"REPORT_START_DATE\0")
};
pub const w_REPORT_START_TIME: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"REPORT_START_TIME\0")
};
pub const w_SWEEP_START: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"SWEEP_START\0") };
pub const w_SWEEP_END: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"SWEEP_END\0") };
pub const w_START_DRY_DAYS: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"DRY_DAYS\0") };
pub const w_WET_STEP: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"WET_STEP\0") };
pub const w_DRY_STEP: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"DRY_STEP\0") };
pub const w_ROUTE_STEP: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"ROUTING_STEP\0") };
pub const w_REPORT_STEP: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"REPORT_STEP\0") };
pub const w_RULE_STEP: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"RULE_STEP\0") };
pub const w_ALLOW_PONDING: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"ALLOW_PONDING\0") };
pub const w_INERT_DAMPING: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"INERTIAL_DAMPING\0")
};
pub const w_SLOPE_WEIGHTING: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"SLOPE_WEIGHTING\0") };
pub const w_VARIABLE_STEP: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"VARIABLE_STEP\0") };
pub const w_NORMAL_FLOW_LTD: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"NORMAL_FLOW_LIMITED\0")
};
pub const w_LENGTHENING_STEP: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"LENGTHENING_STEP\0")
};
pub const w_MIN_SURFAREA: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"MIN_SURFAREA\0") };
pub const w_COMPATIBILITY: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"COMPATIBILITY\0") };
pub const w_SKIP_STEADY_STATE: [::core::ffi::c_char; 18] = unsafe {
    ::core::mem::transmute::<[u8; 18], [::core::ffi::c_char; 18]>(*b"SKIP_STEADY_STATE\0")
};
pub const w_TEMPDIR: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"TEMPDIR\0") };
pub const w_IGNORE_RAINFALL: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"IGNORE_RAINFALL\0") };
pub const w_FORCE_MAIN_EQN: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"FORCE_MAIN_EQUATION\0")
};
pub const w_LINK_OFFSETS: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"LINK_OFFSETS\0") };
pub const w_MIN_SLOPE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"MIN_SLOPE\0") };
pub const w_IGNORE_SNOWMELT: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"IGNORE_SNOWMELT\0") };
pub const w_IGNORE_GWATER: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"IGNORE_GROUNDWATER\0")
};
pub const w_IGNORE_ROUTING: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"IGNORE_ROUTING\0") };
pub const w_IGNORE_QUALITY: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"IGNORE_QUALITY\0") };
pub const w_MAX_TRIALS: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"MAX_TRIALS\0") };
pub const w_HEAD_TOL: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"HEAD_TOLERANCE\0") };
pub const w_SYS_FLOW_TOL: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"SYS_FLOW_TOL\0") };
pub const w_LAT_FLOW_TOL: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"LAT_FLOW_TOL\0") };
pub const w_IGNORE_RDII: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"IGNORE_RDII\0") };
pub const w_MIN_ROUTE_STEP: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"MINIMUM_STEP\0") };
pub const w_NUM_THREADS: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"THREADS\0") };
pub const w_SURCHARGE_METHOD: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"SURCHARGE_METHOD\0")
};
pub const w_CFS: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"CFS\0") };
pub const w_GPM: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"GPM\0") };
pub const w_MGD: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"MGD\0") };
pub const w_CMS: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"CMS\0") };
pub const w_LPS: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"LPS\0") };
pub const w_MLD: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"MLD\0") };
pub const w_NF: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"NF\0") };
pub const w_KW: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"KW\0") };
pub const w_EKW: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"EKW\0") };
pub const w_DW: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"DW\0") };
pub const w_STEADY: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"STEADY\0") };
pub const w_KINWAVE: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"KINWAVE\0") };
pub const w_XKINWAVE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"XKINWAVE\0") };
pub const w_DYNWAVE: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"DYNWAVE\0") };
pub const w_EXTRAN: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"EXTRAN\0") };
pub const w_SLOT: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"SLOT\0") };
pub const w_HORTON: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"HORTON\0") };
pub const w_MOD_HORTON: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"MODIFIED_HORTON\0") };
pub const w_GREEN_AMPT: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"GREEN_AMPT\0") };
pub const w_MOD_GREEN_AMPT: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"MODIFIED_GREEN_AMPT\0")
};
pub const w_CURVE_NUMEBR: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"CURVE_NUMBER\0") };
pub const w_SLOPE: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"SLOPE\0") };
pub const w_FROUDE: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"FROUDE\0") };
pub const w_BOTH: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"BOTH\0") };
pub const w_WINDSPEED: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"WINDSPEED\0") };
pub const w_SNOWMELT: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"SNOWMELT\0") };
pub const w_ADC: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"ADC\0") };
pub const w_PLOWABLE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"PLOWABLE\0") };
pub const w_CONSTANT: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"CONSTANT\0") };
pub const w_TIMESERIES: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"TIMESERIES\0") };
pub const w_TEMPERATURE: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"TEMPERATURE\0") };
pub const w_FILE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"FILE\0") };
pub const w_RECOVERY: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"RECOVERY\0") };
pub const w_DRYONLY: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"DRY_ONLY\0") };
pub const w_MONTHLY: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"MONTHLY\0") };
pub const w_DAILY: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"DAILY\0") };
pub const w_HOURLY: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"HOURLY\0") };
pub const w_WEEKEND: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"WEEKEND\0") };
pub const w_INTENSITY: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"INTENSITY\0") };
pub const w_CUMULATIVE: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"CUMULATIVE\0") };
pub const w_SHORT: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"SHORT\0") };
pub const w_MEDIUM: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"MEDIUM\0") };
pub const w_LONG: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"LONG\0") };
pub const w_OUTLET: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"OUTLET\0") };
pub const w_IMPERV: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"IMPERV\0") };
pub const w_PERV: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"PERV\0") };
pub const w_FREE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"FREE\0") };
pub const w_FIXED: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"FIXED\0") };
pub const w_NORMAL: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"NORMAL\0") };
pub const w_FUNCTIONAL: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"FUNCTIONAL\0") };
pub const w_TABULAR: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"TABULAR\0") };
pub const w_CUTOFF: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"CUTOFF\0") };
pub const w_OVERFLOW: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"OVERFLOW\0") };
pub const w_CYLINDRICAL: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"CYLINDRICAL\0") };
pub const w_CONICAL: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"CONICAL\0") };
pub const w_PYRAMIDAL: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"PYRAMIDAL\0") };
pub const w_TYPE1: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"TYPE1\0") };
pub const w_TYPE2: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"TYPE2\0") };
pub const w_TYPE3: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"TYPE3\0") };
pub const w_TYPE4: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"TYPE4\0") };
pub const w_TYPE5: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"TYPE5\0") };
pub const w_IDEAL: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"IDEAL\0") };
pub const w_VOLUME: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"VOLUME\0") };
pub const w_DEPTH: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"DEPTH\0") };
pub const w_SIDE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"SIDE\0") };
pub const w_BOTTOM: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"BOTTOM\0") };
pub const w_TRANSVERSE: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"TRANSVERSE\0") };
pub const w_SIDEFLOW: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"SIDEFLOW\0") };
pub const w_VNOTCH: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"V-NOTCH\0") };
pub const w_ROADWAY: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"ROADWAY\0") };
pub const w_DUMMY: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"DUMMY\0") };
pub const w_CIRCULAR: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"CIRCULAR\0") };
pub const w_FILLED_CIRCULAR: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"FILLED_CIRCULAR\0") };
pub const w_RECT_CLOSED: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"RECT_CLOSED\0") };
pub const w_RECT_OPEN: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"RECT_OPEN\0") };
pub const w_TRAPEZOIDAL: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"TRAPEZOIDAL\0") };
pub const w_TRIANGULAR: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"TRIANGULAR\0") };
pub const w_PARABOLIC: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"PARABOLIC\0") };
pub const w_POWERFUNC: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"POWER\0") };
pub const w_STREET: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"STREET\0") };
pub const w_RECT_TRIANG: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"RECT_TRIANGULAR\0") };
pub const w_RECT_ROUND: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"RECT_ROUND\0") };
pub const w_MOD_BASKET: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"MODBASKETHANDLE\0") };
pub const w_HORIZELLIPSE: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"HORIZ_ELLIPSE\0") };
pub const w_VERTELLIPSE: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"VERT_ELLIPSE\0") };
pub const w_ARCH: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"ARCH\0") };
pub const w_EGGSHAPED: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"EGG\0") };
pub const w_HORSESHOE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"HORSESHOE\0") };
pub const w_GOTHIC: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"GOTHIC\0") };
pub const w_CATENARY: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"CATENARY\0") };
pub const w_SEMIELLIPTICAL: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"SEMIELLIPTICAL\0") };
pub const w_BASKETHANDLE: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"BASKETHANDLE\0") };
pub const w_SEMICIRCULAR: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"SEMICIRCULAR\0") };
pub const w_IRREGULAR: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"IRREGULAR\0") };
pub const w_CUSTOM: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"CUSTOM\0") };
pub const w_FORCE_MAIN: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"FORCE_MAIN\0") };
pub const w_H_W: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"H-W\0") };
pub const w_D_W: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"D-W\0") };
pub const w_ELEVATION: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"ELEVATION\0") };
pub const w_NC: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"NC\0") };
pub const w_X1: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"X1\0") };
pub const w_GR: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"GR\0") };
pub const w_INCHES: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"IN\0") };
pub const w_MMETER: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"MM\0") };
pub const w_MGAL: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"10^6 gal\0") };
pub const w_MLTRS: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"10^6 ltr\0") };
pub const w_GAL: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"gal\0") };
pub const w_LTR: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"ltr\0") };
pub const w_PONDED_FEET: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"Feet\0") };
pub const w_PONDED_METERS: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"Meters\0") };
pub const w_MGperL: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"MG/L\0") };
pub const w_UGperL: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"UG/L\0") };
pub const w_COUNTperL: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"#/L\0") };
pub const w_LBS: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"lbs\0") };
pub const w_KG: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"kg\0") };
pub const w_LOGN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"LogN\0") };
pub const w_POW: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"POW\0") };
pub const w_EXP: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"EXP\0") };
pub const w_SAT: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"SAT\0") };
pub const w_EXT: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"EXT\0") };
pub const w_PER_AREA: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"AREA\0") };
pub const w_PER_CURB: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"CURB\0") };
pub const w_RC: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"RC\0") };
pub const w_EMC: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"EMC\0") };
pub const w_REMOVAL: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"REMOVAL\0") };
pub const w_HRT: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"HRT\0") };
pub const w_DT: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"DT\0") };
pub const w_AREA: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"AREA\0") };
pub const w_DIVERSION: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"DIVERSION\0") };
pub const w_TIDAL: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"TIDAL\0") };
pub const w_RATING: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"RATING\0") };
pub const w_SHAPE: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"SHAPE\0") };
pub const w_PUMP1: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"PUMP1\0") };
pub const w_PUMP2: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"PUMP2\0") };
pub const w_PUMP3: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"PUMP3\0") };
pub const w_PUMP4: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"PUMP4\0") };
pub const w_PUMP5: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"PUMP5\0") };
pub const w_DISABLED: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"DISABLED\0") };
pub const w_INPUT: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"INPUT\0") };
pub const w_CONTINUITY: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"CONTINUITY\0") };
pub const w_FLOWSTATS: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"FLOWSTATS\0") };
pub const w_CONTROLS: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"CONTROL\0") };
pub const w_NODESTATS: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"NODESTATS\0") };
pub const w_AVERAGES: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"AVERAGES\0") };
pub const w_RAINFALL: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"RAINFALL\0") };
pub const w_RUNOFF: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"RUNOFF\0") };
pub const w_HOTSTART: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"HOTSTART\0") };
pub const w_RDII: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"RDII\0") };
pub const w_INFLOWS: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"INFLOWS\0") };
pub const w_OUTFLOWS: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"OUTFLOWS\0") };
pub const w_OFF: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"OFF\0") };
pub const w_ON: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"ON\0") };
pub const w_NO: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"NO\0") };
pub const w_YES: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"YES\0") };
pub const w_NONE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"NONE\0") };
pub const w_ALL: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"ALL\0") };
pub const w_SCRATCH: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"SCRATCH\0") };
pub const w_USE: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"USE\0") };
pub const w_SAVE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"SAVE\0") };
pub const w_FULL: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"FULL\0") };
pub const w_PARTIAL: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"PARTIAL\0") };
pub const w_SUBCATCH: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"SUBCATCH\0") };
pub const w_NODE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"NODE\0") };
pub const w_LINK: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"LINK\0") };
pub const w_JUNCTION: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"JUNCTION\0") };
pub const w_OUTFALL: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"OUTFALL\0") };
pub const w_STORAGE: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"STORAGE\0") };
pub const w_DIVIDER: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"DIVIDER\0") };
pub const w_CONDUIT: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"CONDUIT\0") };
pub const w_PUMP: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"PUMP\0") };
pub const w_ORIFICE: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"ORIFICE\0") };
pub const w_WEIR: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"WEIR\0") };
pub const w_RULE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"RULE\0") };
pub const w_IF: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"IF\0") };
pub const w_AND: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"AND\0") };
pub const w_OR: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"OR\0") };
pub const w_THEN: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"THEN\0") };
pub const w_ELSE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"ELSE\0") };
pub const w_PRIORITY: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"PRIORITY\0") };
pub const w_FLOW: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"FLOW\0") };
pub const w_CONCEN: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"CONCEN\0") };
pub const ws_TITLE: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"[TITLE\0") };
pub const ws_OPTION: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"[OPTION\0") };
pub const ws_FILE: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"[FILE\0") };
pub const ws_RAINGAGE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"[RAINGAGE\0") };
pub const ws_TEMP: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"[TEMPERATURE\0") };
pub const ws_EVAP: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"[EVAP\0") };
pub const ws_SUBCATCH: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"[SUBCATCHMENT\0") };
pub const ws_SUBAREA: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[SUBAREA\0") };
pub const ws_INFIL: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"[INFIL\0") };
pub const ws_AQUIFER: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[AQUIFER\0") };
pub const ws_GROUNDWATER: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"[GROUNDWATER\0") };
pub const ws_SNOWMELT: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"[SNOWPACK\0") };
pub const ws_JUNCTION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"[JUNC\0") };
pub const ws_OUTFALL: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[OUTFALL\0") };
pub const ws_STORAGE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[STORAGE\0") };
pub const ws_DIVIDER: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[DIVIDER\0") };
pub const ws_CONDUIT: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[CONDUIT\0") };
pub const ws_PUMP: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"[PUMP\0") };
pub const ws_ORIFICE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[ORIFICE\0") };
pub const ws_WEIR: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"[WEIR\0") };
pub const ws_OUTLET: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"[OUTLET\0") };
pub const ws_XSECTION: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"[XSECT\0") };
pub const ws_TRANSECT: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"[TRANSECT\0") };
pub const ws_LOSS: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"[LOSS\0") };
pub const ws_CONTROL: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[CONTROL\0") };
pub const ws_POLLUTANT: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"[POLLUT\0") };
pub const ws_LANDUSE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[LANDUSE\0") };
pub const ws_BUILDUP: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[BUILDUP\0") };
pub const ws_WASHOFF: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[WASHOFF\0") };
pub const ws_COVERAGE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"[COVERAGE\0") };
pub const ws_INFLOW: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"[INFLOW\0") };
pub const ws_DWF: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"[DWF\0") };
pub const ws_PATTERN: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[PATTERN\0") };
pub const ws_RDII: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"[RDII\0") };
pub const ws_UNITHYD: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"[HYDROGRAPH\0") };
pub const ws_LOADING: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[LOADING\0") };
pub const ws_TREATMENT: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"[TREATMENT\0") };
pub const ws_CURVE: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"[CURVE\0") };
pub const ws_TIMESERIES: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"[TIMESERIES\0") };
pub const ws_REPORT: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"[REPORT\0") };
pub const ws_MAP: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"[MAP\0") };
pub const ws_COORDINATE: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"[COORDINATE\0") };
pub const ws_VERTICES: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"[VERTICES\0") };
pub const ws_POLYGON: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[POLYGON\0") };
pub const ws_SYMBOL: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"[SYMBOL\0") };
pub const ws_LABEL: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"[LABEL\0") };
pub const ws_BACKDROP: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"[BACKDROP\0") };
pub const ws_TAG: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"[TAG\0") };
pub const ws_PROFILE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"[PROFILE\0") };
pub const ws_LID_CONTROL: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"[LID_CONTROL\0") };
pub const ws_LID_USAGE: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"[LID_USAGE\0") };
pub const ws_GWF: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"[GWF\0") };
pub const ws_ADJUST: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"[ADJUSTMENT\0") };
pub const ws_EVENT: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"[EVENT\0") };
pub const ws_STREET: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"[STREET\0") };
pub const ws_INLET: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"[INLET\0") };
pub const ws_INLET_USAGE: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"[INLET_USAGE\0") };
#[no_mangle]
pub static mut BuildupTypeWords: [*mut ::core::ffi::c_char; 6] = [
    w_NONE.as_ptr() as *mut ::core::ffi::c_char,
    w_POW.as_ptr() as *mut ::core::ffi::c_char,
    w_EXP.as_ptr() as *mut ::core::ffi::c_char,
    w_SAT.as_ptr() as *mut ::core::ffi::c_char,
    w_EXT.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut CurveTypeWords: [*mut ::core::ffi::c_char; 13] = [
    w_STORAGE.as_ptr() as *mut ::core::ffi::c_char,
    w_DIVERSION.as_ptr() as *mut ::core::ffi::c_char,
    w_TIDAL.as_ptr() as *mut ::core::ffi::c_char,
    w_RATING.as_ptr() as *mut ::core::ffi::c_char,
    w_CONTROLS.as_ptr() as *mut ::core::ffi::c_char,
    w_SHAPE.as_ptr() as *mut ::core::ffi::c_char,
    w_WEIR.as_ptr() as *mut ::core::ffi::c_char,
    w_PUMP1.as_ptr() as *mut ::core::ffi::c_char,
    w_PUMP2.as_ptr() as *mut ::core::ffi::c_char,
    w_PUMP3.as_ptr() as *mut ::core::ffi::c_char,
    w_PUMP4.as_ptr() as *mut ::core::ffi::c_char,
    w_PUMP5.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut DividerTypeWords: [*mut ::core::ffi::c_char; 5] = [
    w_CUTOFF.as_ptr() as *mut ::core::ffi::c_char,
    w_TABULAR.as_ptr() as *mut ::core::ffi::c_char,
    w_WEIR.as_ptr() as *mut ::core::ffi::c_char,
    w_OVERFLOW.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut EvapTypeWords: [*mut ::core::ffi::c_char; 8] = [
    w_CONSTANT.as_ptr() as *mut ::core::ffi::c_char,
    w_MONTHLY.as_ptr() as *mut ::core::ffi::c_char,
    w_TIMESERIES.as_ptr() as *mut ::core::ffi::c_char,
    w_TEMPERATURE.as_ptr() as *mut ::core::ffi::c_char,
    w_FILE.as_ptr() as *mut ::core::ffi::c_char,
    w_RECOVERY.as_ptr() as *mut ::core::ffi::c_char,
    w_DRYONLY.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut FileTypeWords: [*mut ::core::ffi::c_char; 7] = [
    w_RAINFALL.as_ptr() as *mut ::core::ffi::c_char,
    w_RUNOFF.as_ptr() as *mut ::core::ffi::c_char,
    w_HOTSTART.as_ptr() as *mut ::core::ffi::c_char,
    w_RDII.as_ptr() as *mut ::core::ffi::c_char,
    w_INFLOWS.as_ptr() as *mut ::core::ffi::c_char,
    w_OUTFLOWS.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut FileModeWords: [*mut ::core::ffi::c_char; 5] = [
    w_NO.as_ptr() as *mut ::core::ffi::c_char,
    w_SCRATCH.as_ptr() as *mut ::core::ffi::c_char,
    w_USE.as_ptr() as *mut ::core::ffi::c_char,
    w_SAVE.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut FlowUnitWords: [*mut ::core::ffi::c_char; 7] = [
    w_CFS.as_ptr() as *mut ::core::ffi::c_char,
    w_GPM.as_ptr() as *mut ::core::ffi::c_char,
    w_MGD.as_ptr() as *mut ::core::ffi::c_char,
    w_CMS.as_ptr() as *mut ::core::ffi::c_char,
    w_LPS.as_ptr() as *mut ::core::ffi::c_char,
    w_MLD.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut ForceMainEqnWords: [*mut ::core::ffi::c_char; 3] = [
    w_H_W.as_ptr() as *mut ::core::ffi::c_char,
    w_D_W.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut GageDataWords: [*mut ::core::ffi::c_char; 3] = [
    w_TIMESERIES.as_ptr() as *mut ::core::ffi::c_char,
    w_FILE.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut InfilModelWords: [*mut ::core::ffi::c_char; 6] = [
    w_HORTON.as_ptr() as *mut ::core::ffi::c_char,
    w_MOD_HORTON.as_ptr() as *mut ::core::ffi::c_char,
    w_GREEN_AMPT.as_ptr() as *mut ::core::ffi::c_char,
    w_MOD_GREEN_AMPT.as_ptr() as *mut ::core::ffi::c_char,
    w_CURVE_NUMEBR.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut InertDampingWords: [*mut ::core::ffi::c_char; 4] = [
    w_NONE.as_ptr() as *mut ::core::ffi::c_char,
    w_PARTIAL.as_ptr() as *mut ::core::ffi::c_char,
    w_FULL.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut LinkOffsetWords: [*mut ::core::ffi::c_char; 3] = [
    w_DEPTH.as_ptr() as *mut ::core::ffi::c_char,
    w_ELEVATION.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut LinkTypeWords: [*mut ::core::ffi::c_char; 5] = [
    w_CONDUIT.as_ptr() as *mut ::core::ffi::c_char,
    w_PUMP.as_ptr() as *mut ::core::ffi::c_char,
    w_ORIFICE.as_ptr() as *mut ::core::ffi::c_char,
    w_WEIR.as_ptr() as *mut ::core::ffi::c_char,
    w_OUTLET.as_ptr() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut LoadUnitsWords: [*mut ::core::ffi::c_char; 3] = [
    w_LBS.as_ptr() as *mut ::core::ffi::c_char,
    w_KG.as_ptr() as *mut ::core::ffi::c_char,
    w_LOGN.as_ptr() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut NodeTypeWords: [*mut ::core::ffi::c_char; 4] = [
    w_JUNCTION.as_ptr() as *mut ::core::ffi::c_char,
    w_OUTFALL.as_ptr() as *mut ::core::ffi::c_char,
    w_STORAGE.as_ptr() as *mut ::core::ffi::c_char,
    w_DIVIDER.as_ptr() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut NoneAllWords: [*mut ::core::ffi::c_char; 3] = [
    w_NONE.as_ptr() as *mut ::core::ffi::c_char,
    w_ALL.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut NormalFlowWords: [*mut ::core::ffi::c_char; 5] = [
    w_SLOPE.as_ptr() as *mut ::core::ffi::c_char,
    w_FROUDE.as_ptr() as *mut ::core::ffi::c_char,
    w_BOTH.as_ptr() as *mut ::core::ffi::c_char,
    w_NONE.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut NormalizerWords: [*mut ::core::ffi::c_char; 3] = [
    w_PER_AREA.as_ptr() as *mut ::core::ffi::c_char,
    w_PER_CURB.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut NoYesWords: [*mut ::core::ffi::c_char; 3] = [
    w_NO.as_ptr() as *mut ::core::ffi::c_char,
    w_YES.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut OffOnWords: [*mut ::core::ffi::c_char; 3] = [
    w_OFF.as_ptr() as *mut ::core::ffi::c_char,
    w_ON.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut OldRouteModelWords: [*mut ::core::ffi::c_char; 6] = [
    w_NONE.as_ptr() as *mut ::core::ffi::c_char,
    w_NF.as_ptr() as *mut ::core::ffi::c_char,
    w_KW.as_ptr() as *mut ::core::ffi::c_char,
    w_EKW.as_ptr() as *mut ::core::ffi::c_char,
    w_DW.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut OptionWords: [*mut ::core::ffi::c_char; 44] = [
    w_FLOW_UNITS.as_ptr() as *mut ::core::ffi::c_char,
    w_INFIL_MODEL.as_ptr() as *mut ::core::ffi::c_char,
    w_ROUTE_MODEL.as_ptr() as *mut ::core::ffi::c_char,
    w_START_DATE.as_ptr() as *mut ::core::ffi::c_char,
    w_START_TIME.as_ptr() as *mut ::core::ffi::c_char,
    w_END_DATE.as_ptr() as *mut ::core::ffi::c_char,
    w_END_TIME.as_ptr() as *mut ::core::ffi::c_char,
    w_REPORT_START_DATE.as_ptr() as *mut ::core::ffi::c_char,
    w_REPORT_START_TIME.as_ptr() as *mut ::core::ffi::c_char,
    w_SWEEP_START.as_ptr() as *mut ::core::ffi::c_char,
    w_SWEEP_END.as_ptr() as *mut ::core::ffi::c_char,
    w_START_DRY_DAYS.as_ptr() as *mut ::core::ffi::c_char,
    w_WET_STEP.as_ptr() as *mut ::core::ffi::c_char,
    w_DRY_STEP.as_ptr() as *mut ::core::ffi::c_char,
    w_ROUTE_STEP.as_ptr() as *mut ::core::ffi::c_char,
    w_RULE_STEP.as_ptr() as *mut ::core::ffi::c_char,
    w_REPORT_STEP.as_ptr() as *mut ::core::ffi::c_char,
    w_ALLOW_PONDING.as_ptr() as *mut ::core::ffi::c_char,
    w_INERT_DAMPING.as_ptr() as *mut ::core::ffi::c_char,
    w_SLOPE_WEIGHTING.as_ptr() as *mut ::core::ffi::c_char,
    w_VARIABLE_STEP.as_ptr() as *mut ::core::ffi::c_char,
    w_NORMAL_FLOW_LTD.as_ptr() as *mut ::core::ffi::c_char,
    w_LENGTHENING_STEP.as_ptr() as *mut ::core::ffi::c_char,
    w_MIN_SURFAREA.as_ptr() as *mut ::core::ffi::c_char,
    w_COMPATIBILITY.as_ptr() as *mut ::core::ffi::c_char,
    w_SKIP_STEADY_STATE.as_ptr() as *mut ::core::ffi::c_char,
    w_TEMPDIR.as_ptr() as *mut ::core::ffi::c_char,
    w_IGNORE_RAINFALL.as_ptr() as *mut ::core::ffi::c_char,
    w_FORCE_MAIN_EQN.as_ptr() as *mut ::core::ffi::c_char,
    w_LINK_OFFSETS.as_ptr() as *mut ::core::ffi::c_char,
    w_MIN_SLOPE.as_ptr() as *mut ::core::ffi::c_char,
    w_IGNORE_SNOWMELT.as_ptr() as *mut ::core::ffi::c_char,
    w_IGNORE_GWATER.as_ptr() as *mut ::core::ffi::c_char,
    w_IGNORE_ROUTING.as_ptr() as *mut ::core::ffi::c_char,
    w_IGNORE_QUALITY.as_ptr() as *mut ::core::ffi::c_char,
    w_MAX_TRIALS.as_ptr() as *mut ::core::ffi::c_char,
    w_HEAD_TOL.as_ptr() as *mut ::core::ffi::c_char,
    w_SYS_FLOW_TOL.as_ptr() as *mut ::core::ffi::c_char,
    w_LAT_FLOW_TOL.as_ptr() as *mut ::core::ffi::c_char,
    w_IGNORE_RDII.as_ptr() as *mut ::core::ffi::c_char,
    w_MIN_ROUTE_STEP.as_ptr() as *mut ::core::ffi::c_char,
    w_NUM_THREADS.as_ptr() as *mut ::core::ffi::c_char,
    w_SURCHARGE_METHOD.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut OrificeTypeWords: [*mut ::core::ffi::c_char; 3] = [
    w_SIDE.as_ptr() as *mut ::core::ffi::c_char,
    w_BOTTOM.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut OutfallTypeWords: [*mut ::core::ffi::c_char; 6] = [
    w_FREE.as_ptr() as *mut ::core::ffi::c_char,
    w_NORMAL.as_ptr() as *mut ::core::ffi::c_char,
    w_FIXED.as_ptr() as *mut ::core::ffi::c_char,
    w_TIDAL.as_ptr() as *mut ::core::ffi::c_char,
    w_TIMESERIES.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut PatternTypeWords: [*mut ::core::ffi::c_char; 5] = [
    w_MONTHLY.as_ptr() as *mut ::core::ffi::c_char,
    w_DAILY.as_ptr() as *mut ::core::ffi::c_char,
    w_HOURLY.as_ptr() as *mut ::core::ffi::c_char,
    w_WEEKEND.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut PondingUnitsWords: [*mut ::core::ffi::c_char; 2] = [
    w_PONDED_FEET.as_ptr() as *mut ::core::ffi::c_char,
    w_PONDED_METERS.as_ptr() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut ProcessVarWords: [*mut ::core::ffi::c_char; 6] = [
    w_HRT.as_ptr() as *mut ::core::ffi::c_char,
    w_DT.as_ptr() as *mut ::core::ffi::c_char,
    w_FLOW.as_ptr() as *mut ::core::ffi::c_char,
    w_DEPTH.as_ptr() as *mut ::core::ffi::c_char,
    w_AREA.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut PumpTypeWords: [*mut ::core::ffi::c_char; 6] = [
    w_TYPE1.as_ptr() as *mut ::core::ffi::c_char,
    w_TYPE2.as_ptr() as *mut ::core::ffi::c_char,
    w_TYPE3.as_ptr() as *mut ::core::ffi::c_char,
    w_TYPE4.as_ptr() as *mut ::core::ffi::c_char,
    w_TYPE5.as_ptr() as *mut ::core::ffi::c_char,
    w_IDEAL.as_ptr() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut QualUnitsWords: [*mut ::core::ffi::c_char; 4] = [
    w_MGperL.as_ptr() as *mut ::core::ffi::c_char,
    w_UGperL.as_ptr() as *mut ::core::ffi::c_char,
    w_COUNTperL.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut RainTypeWords: [*mut ::core::ffi::c_char; 4] = [
    w_INTENSITY.as_ptr() as *mut ::core::ffi::c_char,
    w_VOLUME.as_ptr() as *mut ::core::ffi::c_char,
    w_CUMULATIVE.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut RainUnitsWords: [*mut ::core::ffi::c_char; 3] = [
    w_INCHES.as_ptr() as *mut ::core::ffi::c_char,
    w_MMETER.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut RelationWords: [*mut ::core::ffi::c_char; 7] = [
    w_TABULAR.as_ptr() as *mut ::core::ffi::c_char,
    w_FUNCTIONAL.as_ptr() as *mut ::core::ffi::c_char,
    w_CYLINDRICAL.as_ptr() as *mut ::core::ffi::c_char,
    w_CONICAL.as_ptr() as *mut ::core::ffi::c_char,
    w_PARABOLIC.as_ptr() as *mut ::core::ffi::c_char,
    w_PYRAMIDAL.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut ReportWords: [*mut ::core::ffi::c_char; 11] = [
    w_DISABLED.as_ptr() as *mut ::core::ffi::c_char,
    w_INPUT.as_ptr() as *mut ::core::ffi::c_char,
    w_SUBCATCH.as_ptr() as *mut ::core::ffi::c_char,
    w_NODE.as_ptr() as *mut ::core::ffi::c_char,
    w_LINK.as_ptr() as *mut ::core::ffi::c_char,
    w_CONTINUITY.as_ptr() as *mut ::core::ffi::c_char,
    w_FLOWSTATS.as_ptr() as *mut ::core::ffi::c_char,
    w_CONTROLS.as_ptr() as *mut ::core::ffi::c_char,
    w_AVERAGES.as_ptr() as *mut ::core::ffi::c_char,
    w_NODESTATS.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut RouteModelWords: [*mut ::core::ffi::c_char; 6] = [
    w_NONE.as_ptr() as *mut ::core::ffi::c_char,
    w_STEADY.as_ptr() as *mut ::core::ffi::c_char,
    w_KINWAVE.as_ptr() as *mut ::core::ffi::c_char,
    w_XKINWAVE.as_ptr() as *mut ::core::ffi::c_char,
    w_DYNWAVE.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut RuleKeyWords: [*mut ::core::ffi::c_char; 8] = [
    w_RULE.as_ptr() as *mut ::core::ffi::c_char,
    w_IF.as_ptr() as *mut ::core::ffi::c_char,
    w_AND.as_ptr() as *mut ::core::ffi::c_char,
    w_OR.as_ptr() as *mut ::core::ffi::c_char,
    w_THEN.as_ptr() as *mut ::core::ffi::c_char,
    w_ELSE.as_ptr() as *mut ::core::ffi::c_char,
    w_PRIORITY.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut SectWords: [*mut ::core::ffi::c_char; 58] = [
    ws_TITLE.as_ptr() as *mut ::core::ffi::c_char,
    ws_OPTION.as_ptr() as *mut ::core::ffi::c_char,
    ws_FILE.as_ptr() as *mut ::core::ffi::c_char,
    ws_RAINGAGE.as_ptr() as *mut ::core::ffi::c_char,
    ws_TEMP.as_ptr() as *mut ::core::ffi::c_char,
    ws_EVAP.as_ptr() as *mut ::core::ffi::c_char,
    ws_SUBCATCH.as_ptr() as *mut ::core::ffi::c_char,
    ws_SUBAREA.as_ptr() as *mut ::core::ffi::c_char,
    ws_INFIL.as_ptr() as *mut ::core::ffi::c_char,
    ws_AQUIFER.as_ptr() as *mut ::core::ffi::c_char,
    ws_GROUNDWATER.as_ptr() as *mut ::core::ffi::c_char,
    ws_SNOWMELT.as_ptr() as *mut ::core::ffi::c_char,
    ws_JUNCTION.as_ptr() as *mut ::core::ffi::c_char,
    ws_OUTFALL.as_ptr() as *mut ::core::ffi::c_char,
    ws_STORAGE.as_ptr() as *mut ::core::ffi::c_char,
    ws_DIVIDER.as_ptr() as *mut ::core::ffi::c_char,
    ws_CONDUIT.as_ptr() as *mut ::core::ffi::c_char,
    ws_PUMP.as_ptr() as *mut ::core::ffi::c_char,
    ws_ORIFICE.as_ptr() as *mut ::core::ffi::c_char,
    ws_WEIR.as_ptr() as *mut ::core::ffi::c_char,
    ws_OUTLET.as_ptr() as *mut ::core::ffi::c_char,
    ws_XSECTION.as_ptr() as *mut ::core::ffi::c_char,
    ws_TRANSECT.as_ptr() as *mut ::core::ffi::c_char,
    ws_LOSS.as_ptr() as *mut ::core::ffi::c_char,
    ws_CONTROL.as_ptr() as *mut ::core::ffi::c_char,
    ws_POLLUTANT.as_ptr() as *mut ::core::ffi::c_char,
    ws_LANDUSE.as_ptr() as *mut ::core::ffi::c_char,
    ws_BUILDUP.as_ptr() as *mut ::core::ffi::c_char,
    ws_WASHOFF.as_ptr() as *mut ::core::ffi::c_char,
    ws_COVERAGE.as_ptr() as *mut ::core::ffi::c_char,
    ws_INFLOW.as_ptr() as *mut ::core::ffi::c_char,
    ws_DWF.as_ptr() as *mut ::core::ffi::c_char,
    ws_PATTERN.as_ptr() as *mut ::core::ffi::c_char,
    ws_RDII.as_ptr() as *mut ::core::ffi::c_char,
    ws_UNITHYD.as_ptr() as *mut ::core::ffi::c_char,
    ws_LOADING.as_ptr() as *mut ::core::ffi::c_char,
    ws_TREATMENT.as_ptr() as *mut ::core::ffi::c_char,
    ws_CURVE.as_ptr() as *mut ::core::ffi::c_char,
    ws_TIMESERIES.as_ptr() as *mut ::core::ffi::c_char,
    ws_REPORT.as_ptr() as *mut ::core::ffi::c_char,
    ws_COORDINATE.as_ptr() as *mut ::core::ffi::c_char,
    ws_VERTICES.as_ptr() as *mut ::core::ffi::c_char,
    ws_POLYGON.as_ptr() as *mut ::core::ffi::c_char,
    ws_LABEL.as_ptr() as *mut ::core::ffi::c_char,
    ws_SYMBOL.as_ptr() as *mut ::core::ffi::c_char,
    ws_BACKDROP.as_ptr() as *mut ::core::ffi::c_char,
    ws_TAG.as_ptr() as *mut ::core::ffi::c_char,
    ws_PROFILE.as_ptr() as *mut ::core::ffi::c_char,
    ws_MAP.as_ptr() as *mut ::core::ffi::c_char,
    ws_LID_CONTROL.as_ptr() as *mut ::core::ffi::c_char,
    ws_LID_USAGE.as_ptr() as *mut ::core::ffi::c_char,
    ws_GWF.as_ptr() as *mut ::core::ffi::c_char,
    ws_ADJUST.as_ptr() as *mut ::core::ffi::c_char,
    ws_EVENT.as_ptr() as *mut ::core::ffi::c_char,
    ws_STREET.as_ptr() as *mut ::core::ffi::c_char,
    ws_INLET_USAGE.as_ptr() as *mut ::core::ffi::c_char,
    ws_INLET.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut SnowmeltWords: [*mut ::core::ffi::c_char; 5] = [
    w_PLOWABLE.as_ptr() as *mut ::core::ffi::c_char,
    w_IMPERV.as_ptr() as *mut ::core::ffi::c_char,
    w_PERV.as_ptr() as *mut ::core::ffi::c_char,
    w_REMOVAL.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut SurchargeWords: [*mut ::core::ffi::c_char; 3] = [
    w_EXTRAN.as_ptr() as *mut ::core::ffi::c_char,
    w_SLOT.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut TempKeyWords: [*mut ::core::ffi::c_char; 6] = [
    w_TIMESERIES.as_ptr() as *mut ::core::ffi::c_char,
    w_FILE.as_ptr() as *mut ::core::ffi::c_char,
    w_WINDSPEED.as_ptr() as *mut ::core::ffi::c_char,
    w_SNOWMELT.as_ptr() as *mut ::core::ffi::c_char,
    w_ADC.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut TransectKeyWords: [*mut ::core::ffi::c_char; 4] = [
    w_NC.as_ptr() as *mut ::core::ffi::c_char,
    w_X1.as_ptr() as *mut ::core::ffi::c_char,
    w_GR.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut TreatTypeWords: [*mut ::core::ffi::c_char; 3] = [
    w_REMOVAL.as_ptr() as *mut ::core::ffi::c_char,
    w_CONCEN.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut UHTypeWords: [*mut ::core::ffi::c_char; 4] = [
    w_SHORT.as_ptr() as *mut ::core::ffi::c_char,
    w_MEDIUM.as_ptr() as *mut ::core::ffi::c_char,
    w_LONG.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut VolUnitsWords: [*mut ::core::ffi::c_char; 2] = [
    w_MGAL.as_ptr() as *mut ::core::ffi::c_char,
    w_MLTRS.as_ptr() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut VolUnitsWords2: [*mut ::core::ffi::c_char; 2] = [
    w_GAL.as_ptr() as *mut ::core::ffi::c_char,
    w_LTR.as_ptr() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut WashoffTypeWords: [*mut ::core::ffi::c_char; 5] = [
    w_NONE.as_ptr() as *mut ::core::ffi::c_char,
    w_EXP.as_ptr() as *mut ::core::ffi::c_char,
    w_RC.as_ptr() as *mut ::core::ffi::c_char,
    w_EMC.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut WeirTypeWords: [*mut ::core::ffi::c_char; 6] = [
    w_TRANSVERSE.as_ptr() as *mut ::core::ffi::c_char,
    w_SIDEFLOW.as_ptr() as *mut ::core::ffi::c_char,
    w_VNOTCH.as_ptr() as *mut ::core::ffi::c_char,
    w_TRAPEZOIDAL.as_ptr() as *mut ::core::ffi::c_char,
    w_ROADWAY.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
#[no_mangle]
pub static mut XsectTypeWords: [*mut ::core::ffi::c_char; 27] = [
    w_DUMMY.as_ptr() as *mut ::core::ffi::c_char,
    w_CIRCULAR.as_ptr() as *mut ::core::ffi::c_char,
    w_FILLED_CIRCULAR.as_ptr() as *mut ::core::ffi::c_char,
    w_RECT_CLOSED.as_ptr() as *mut ::core::ffi::c_char,
    w_RECT_OPEN.as_ptr() as *mut ::core::ffi::c_char,
    w_TRAPEZOIDAL.as_ptr() as *mut ::core::ffi::c_char,
    w_TRIANGULAR.as_ptr() as *mut ::core::ffi::c_char,
    w_PARABOLIC.as_ptr() as *mut ::core::ffi::c_char,
    w_POWERFUNC.as_ptr() as *mut ::core::ffi::c_char,
    w_RECT_TRIANG.as_ptr() as *mut ::core::ffi::c_char,
    w_RECT_ROUND.as_ptr() as *mut ::core::ffi::c_char,
    w_MOD_BASKET.as_ptr() as *mut ::core::ffi::c_char,
    w_HORIZELLIPSE.as_ptr() as *mut ::core::ffi::c_char,
    w_VERTELLIPSE.as_ptr() as *mut ::core::ffi::c_char,
    w_ARCH.as_ptr() as *mut ::core::ffi::c_char,
    w_EGGSHAPED.as_ptr() as *mut ::core::ffi::c_char,
    w_HORSESHOE.as_ptr() as *mut ::core::ffi::c_char,
    w_GOTHIC.as_ptr() as *mut ::core::ffi::c_char,
    w_CATENARY.as_ptr() as *mut ::core::ffi::c_char,
    w_SEMIELLIPTICAL.as_ptr() as *mut ::core::ffi::c_char,
    w_BASKETHANDLE.as_ptr() as *mut ::core::ffi::c_char,
    w_SEMICIRCULAR.as_ptr() as *mut ::core::ffi::c_char,
    w_IRREGULAR.as_ptr() as *mut ::core::ffi::c_char,
    w_CUSTOM.as_ptr() as *mut ::core::ffi::c_char,
    w_FORCE_MAIN.as_ptr() as *mut ::core::ffi::c_char,
    w_STREET.as_ptr() as *mut ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
];
