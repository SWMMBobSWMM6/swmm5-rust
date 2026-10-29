//! Float-to-integer conversion as EPA's C build performs it.
//!
//! In C, converting a double that does not fit the target integer is
//! undefined. Rust's `as` saturates (1e10 as i32 == i32::MAX). EPA's engine
//! built by GCC for x86-64 instead gets what the instruction `cvttsd2si`
//! gives: the "integer indefinite" value, i32::MIN; Clang's WebAssembly
//! build guards the conversion and substitutes the same value. Every
//! float-to-integer cast in the engine goes through these functions so the
//! Rust engine reproduces that. In range, they are exactly `as`.
#[inline]
pub fn f64_to_i32(x: f64) -> i32 {
    if x > -2147483649.0 && x < 2147483648.0 { x as i32 } else { i32::MIN }
}
#[inline]
pub fn f64_to_i64(x: f64) -> i64 {
    if x >= -9223372036854775808.0 && x < 9223372036854775808.0 { x as i64 } else { i64::MIN }
}
/// C converts double to char through a 32-bit conversion and keeps the low byte.
#[inline]
pub fn f64_to_i8(x: f64) -> i8 {
    f64_to_i32(x) as i8
}
#[inline]
pub fn f32_to_i64(x: f32) -> i64 {
    if x >= -9223372036854775808.0f32 && x < 9223372036854775808.0f32 { x as i64 } else { i64::MIN }
}
