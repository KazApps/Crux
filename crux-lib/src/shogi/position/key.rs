use std::ops::{BitXor, BitXorAssign};

#[derive(Debug, Copy, Clone)]
pub struct Key(u64);

impl Key {
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

impl const From<u64> for Key {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl const PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl const Eq for Key {}

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
