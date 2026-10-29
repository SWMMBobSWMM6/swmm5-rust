//! Differential tests live in tests/: each calls a function of the safe
//! engine and the same function of the c2rust baseline (EPA's C, translated
//! statement by statement) on many inputs and requires identical bits.
//! The baseline keeps its state in globals, so these tests run one at a
//! time (cargo test -- --test-threads=1).

/// A small deterministic generator for test inputs.
pub struct Rng(pub u64);
impl Rng {
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    /// uniform in [lo, hi)
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * ((self.next() >> 11) as f64 / (1u64 << 53) as f64)
    }
}

/// Bitwise equality that treats every NaN as equal to every NaN.
pub fn same(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan())
}
