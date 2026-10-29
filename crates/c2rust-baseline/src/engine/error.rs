#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
}
#[no_mangle]
pub static mut ErrString: [::core::ffi::c_char; 256] = [0; 256];
#[no_mangle]
pub unsafe extern "C" fn error_getMsg(
    mut errCode: ::core::ffi::c_int,
    mut msg: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    match errCode {
        101 => {
            strcpy(
                msg,
                b"\n  ERROR 101: memory allocation error.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        103 => {
            strcpy(
                msg,
                b"\n  ERROR 103: cannot solve KW equations for Link %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        105 => {
            strcpy(
                msg,
                b"\n  ERROR 105: cannot open ODE solver.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        107 => {
            strcpy(
                msg,
                b"\n  ERROR 107: cannot compute a valid time step.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        108 => {
            strcpy(
                msg,
                b"\n  ERROR 108: ambiguous outlet ID name for Subcatchment %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        109 => {
            strcpy(
                msg,
                b"\n  ERROR 109: invalid parameter values for Aquifer %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        110 => {
            strcpy(
                msg,
                b"\n  ERROR 110: ground elevation is below water table for Subcatchment %s.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        111 => {
            strcpy(
                msg,
                b"\n  ERROR 111: invalid length for Conduit %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        112 => {
            strcpy(
                msg,
                b"\n  ERROR 112: elevation drop exceeds length for Conduit %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        113 => {
            strcpy(
                msg,
                b"\n  ERROR 113: invalid roughness for Conduit %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        114 => {
            strcpy(
                msg,
                b"\n  ERROR 114: invalid number of barrels for Conduit %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        115 => {
            strcpy(
                msg,
                b"\n  ERROR 115: adverse slope for Conduit %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        117 => {
            strcpy(
                msg,
                b"\n  ERROR 117: no cross section defined for Link %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        119 => {
            strcpy(
                msg,
                b"\n  ERROR 119: invalid cross section for Link %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        121 => {
            strcpy(
                msg,
                b"\n  ERROR 121: missing or invalid pump curve assigned to Pump %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        122 => {
            strcpy(
                msg,
                b"\n  ERROR 122: startup depth not higher than shutoff depth for Pump %s.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        131 => {
            strcpy(
                msg,
                b"\n  ERROR 131: the following links form cyclic loops in the drainage system:\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        133 => {
            strcpy(
                msg,
                b"\n  ERROR 133: Node %s has more than one outlet link.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        134 => {
            strcpy(
                msg,
                b"\n  ERROR 134: Node %s has illegal DUMMY link connections.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        135 => {
            strcpy(
                msg,
                b"\n  ERROR 135: Divider %s does not have two outlet links.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        136 => {
            strcpy(
                msg,
                b"\n  ERROR 136: Divider %s has invalid diversion link.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        137 => {
            strcpy(
                msg,
                b"\n  ERROR 137: Weir Divider %s has invalid parameters.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        138 => {
            strcpy(
                msg,
                b"\n  ERROR 138: Node %s has initial depth greater than maximum depth.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        139 => {
            strcpy(
                msg,
                b"\n  ERROR 139: Regulator %s is the outlet of a non-storage node.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        140 => {
            strcpy(
                msg,
                b"\n  ERROR 140: Storage node %s has negative volume at full depth.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        141 => {
            strcpy(
                msg,
                b"\n  ERROR 141: Outfall %s has more than 1 inlet link or an outlet link.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        143 => {
            strcpy(
                msg,
                b"\n  ERROR 143: Regulator %s has invalid cross-section shape.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        145 => {
            strcpy(
                msg,
                b"\n  ERROR 145: Drainage system has no acceptable outlet nodes.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        151 => {
            strcpy(
                msg,
                b"\n  ERROR 151: a Unit Hydrograph in set %s has invalid time base.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        153 => {
            strcpy(
                msg,
                b"\n  ERROR 153: a Unit Hydrograph in set %s has invalid response ratios.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        155 => {
            strcpy(
                msg,
                b"\n  ERROR 155: invalid sewer area for RDII at node %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        156 => {
            strcpy(
                msg,
                b"\n  ERROR 156: ambiguous station ID for Rain Gage %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        157 => {
            strcpy(
                msg,
                b"\n  ERROR 157: inconsistent rainfall format for Rain Gage %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        158 => {
            strcpy(
                msg,
                b"\n  ERROR 158: time series for Rain Gage %s is also used by another object.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        159 => {
            strcpy(
                msg,
                b"\n  ERROR 159: recording interval greater than time series interval for Rain Gage %s.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        161 => {
            strcpy(
                msg,
                b"\n  ERROR 161: cyclic dependency in treatment functions at node %s.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        171 => {
            strcpy(
                msg,
                b"\n  ERROR 171: Curve %s has invalid or out of sequence data.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        173 => {
            strcpy(
                msg,
                b"\n  ERROR 173: Time Series %s has its data out of sequence.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        181 => {
            strcpy(
                msg,
                b"\n  ERROR 181: invalid Snow Melt Climatology parameters.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        182 => {
            strcpy(
                msg,
                b"\n  ERROR 182: invalid parameters for Snow Pack %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        183 => {
            strcpy(
                msg,
                b"\n  ERROR 183: no type specified for LID %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        184 => {
            strcpy(
                msg,
                b"\n  ERROR 184: missing layer for LID %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        185 => {
            strcpy(
                msg,
                b"\n  ERROR 185: invalid parameter value for LID %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        187 => {
            strcpy(
                msg,
                b"\n  ERROR 187: LID area exceeds total area for Subcatchment %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        188 => {
            strcpy(
                msg,
                b"\n  ERROR 188: LID capture area exceeds total impervious area for Subcatchment %s.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        191 => {
            strcpy(
                msg,
                b"\n  ERROR 191: simulation start date comes after ending date.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        193 => {
            strcpy(
                msg,
                b"\n  ERROR 193: report start date comes after ending date.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        195 => {
            strcpy(
                msg,
                b"\n  ERROR 195: reporting time step or duration is less than routing time step.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        200 => {
            strcpy(
                msg,
                b"\n  ERROR 200: one or more errors in input file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        201 => {
            strcpy(
                msg,
                b"\n  ERROR 201: too many characters in input line \0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        203 => {
            strcpy(
                msg,
                b"\n  ERROR 203: too few items \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        205 => {
            strcpy(
                msg,
                b"\n  ERROR 205: invalid keyword %s \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        207 => {
            strcpy(
                msg,
                b"\n  ERROR 207: duplicate ID name %s \0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        209 => {
            strcpy(
                msg,
                b"\n  ERROR 209: undefined object %s \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        211 => {
            strcpy(
                msg,
                b"\n  ERROR 211: invalid number %s \0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        213 => {
            strcpy(
                msg,
                b"\n  ERROR 213: invalid date/time %s \0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        217 => {
            strcpy(
                msg,
                b"\n  ERROR 217: control rule clause invalid or out of sequence \0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        219 => {
            strcpy(
                msg,
                b"\n  ERROR 219: data provided for unidentified transect \0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        221 => {
            strcpy(
                msg,
                b"\n  ERROR 221: transect station out of sequence \0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        223 => {
            strcpy(
                msg,
                b"\n  ERROR 223: Transect %s has too few stations.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        225 => {
            strcpy(
                msg,
                b"\n  ERROR 225: Transect %s has too many stations.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        227 => {
            strcpy(
                msg,
                b"\n  ERROR 227: Transect %s has no Manning's N.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        229 => {
            strcpy(
                msg,
                b"\n  ERROR 229: Transect %s has invalid overbank locations.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        231 => {
            strcpy(
                msg,
                b"\n  ERROR 231: Transect %s has no depth.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        233 => {
            strcpy(
                msg,
                b"\n  ERROR 233: invalid math expression \0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        235 => {
            strcpy(
                msg,
                b"\n  ERROR 235: invalid infiltration parameters \0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        301 => {
            strcpy(
                msg,
                b"\n  ERROR 301: files share same names.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        303 => {
            strcpy(
                msg,
                b"\n  ERROR 303: cannot open input file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        305 => {
            strcpy(
                msg,
                b"\n  ERROR 305: cannot open report file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        307 => {
            strcpy(
                msg,
                b"\n  ERROR 307: cannot open binary results file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        308 => {
            strcpy(
                msg,
                b"\n  ERROR 308: amount of output produced will exceed maximum file size.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        309 => {
            strcpy(
                msg,
                b"\n  ERROR 309: error writing to binary results file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        311 => {
            strcpy(
                msg,
                b"\n  ERROR 311: error reading from binary results file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        313 => {
            strcpy(
                msg,
                b"\n  ERROR 313: cannot open scratch rainfall interface file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        315 => {
            strcpy(
                msg,
                b"\n  ERROR 315: cannot open rainfall interface file %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        317 => {
            strcpy(
                msg,
                b"\n  ERROR 317: cannot open rainfall data file %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        318 => {
            strcpy(
                msg,
                b"\n  ERROR 318: the following line is out of sequence in rainfall data file %s.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        319 => {
            strcpy(
                msg,
                b"\n  ERROR 319: unknown format for rainfall data file %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        320 => {
            strcpy(
                msg,
                b"\n  ERROR 320: invalid format for rainfall interface file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        321 => {
            strcpy(
                msg,
                b"\n  ERROR 321: no data in rainfall interface file for gage %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        323 => {
            strcpy(
                msg,
                b"\n  ERROR 323: cannot open runoff interface file %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        325 => {
            strcpy(
                msg,
                b"\n  ERROR 325: incompatible data found in runoff interface file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        327 => {
            strcpy(
                msg,
                b"\n  ERROR 327: attempting to read beyond end of runoff interface file.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        329 => {
            strcpy(
                msg,
                b"\n  ERROR 329: error in reading from runoff interface file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        331 => {
            strcpy(
                msg,
                b"\n  ERROR 331: cannot open hot start interface file %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        333 => {
            strcpy(
                msg,
                b"\n  ERROR 333: incompatible data found in hot start interface file.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        335 => {
            strcpy(
                msg,
                b"\n  ERROR 335: error in reading from hot start interface file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        336 => {
            strcpy(
                msg,
                b"\n  ERROR 336: no climate file specified for evaporation and/or wind speed.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        337 => {
            strcpy(
                msg,
                b"\n  ERROR 337: cannot open climate file %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        338 => {
            strcpy(
                msg,
                b"\n  ERROR 338: error in reading from climate file %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        339 => {
            strcpy(
                msg,
                b"\n  ERROR 339: attempt to read beyond end of climate file %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        341 => {
            strcpy(
                msg,
                b"\n  ERROR 341: cannot open scratch RDII interface file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        343 => {
            strcpy(
                msg,
                b"\n  ERROR 343: cannot open RDII interface file %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        345 => {
            strcpy(
                msg,
                b"\n  ERROR 345: invalid format for RDII interface file.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        351 => {
            strcpy(
                msg,
                b"\n  ERROR 351: cannot open routing interface file %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        353 => {
            strcpy(
                msg,
                b"\n  ERROR 353: invalid format for routing interface file %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        355 => {
            strcpy(
                msg,
                b"\n  ERROR 355: mis-matched names in routing interface file %s.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        357 => {
            strcpy(
                msg,
                b"\n  ERROR 357: inflows and outflows interface files have same name.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        361 => {
            strcpy(
                msg,
                b"\n  ERROR 361: could not open external file used for Time Series %s.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        363 => {
            strcpy(
                msg,
                b"\n  ERROR 363: invalid data in external file used for Time Series %s.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
        500 => {
            strcpy(
                msg,
                b"\n  ERROR 500: System exception thrown.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        501 => {
            strcpy(
                msg,
                b"\n  API Error 501: project not opened.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        502 => {
            strcpy(
                msg,
                b"\n  API Error 502: simulation not started.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        503 => {
            strcpy(
                msg,
                b"\n  API Error 503: simulation not ended.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        504 => {
            strcpy(
                msg,
                b"\n  API Error 504: invalid object type.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        505 => {
            strcpy(
                msg,
                b"\n  API Error 505: invalid object index.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        506 => {
            strcpy(
                msg,
                b"\n  API Error 506: invalid object name.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        507 => {
            strcpy(
                msg,
                b"\n  API Error 507: invalid property type.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        508 => {
            strcpy(
                msg,
                b"\n  API Error 508: invalid property value.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        509 => {
            strcpy(
                msg,
                b"\n  API Error 509: invalid time period.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        _ => {
            strcpy(msg, b"\0" as *const u8 as *const ::core::ffi::c_char);
        }
    }
    return msg;
}
#[no_mangle]
pub unsafe extern "C" fn error_setInpError(
    mut errcode: ::core::ffi::c_int,
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    strcpy(&raw mut ErrString as *mut ::core::ffi::c_char, s);
    return errcode;
}
