#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Prng(u64);

impl Prng {
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        debug_assert!(seed != 0);

        Self(seed)
    }

    pub const fn rand(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;

        self.0.wrapping_mul(2685821657736338717u64)
    }
}
