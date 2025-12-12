use std::ops::{BitXor, BitXorAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Key(u64);

impl Key {
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl const Default for Key {
    fn default() -> Self {
        Self(0)
    }
}

impl const BitXor for Key {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        Self(self.0 ^ other.0)
    }
}

impl const BitXorAssign for Key {
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0;
    }
}
