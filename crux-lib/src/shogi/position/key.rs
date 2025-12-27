use std::ops::{BitXor, BitXorAssign};

/// Represents a 64-bit key used for hashing or unique identification.
#[derive(Debug, Copy, Clone)]
pub struct Key(u64);

impl Key {
    /// Returns the underlying `u64` value of the key.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}

impl const Default for Key {
    /// Returns a key with a value of zero.
    fn default() -> Self {
        Self(0)
    }
}

impl const From<u64> for Key {
    /// Creates a `Key` from a `u64` value.
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl const PartialEq for Key {
    /// Compares two `Key` values for equality.
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl const Eq for Key {}

impl const BitXor for Key {
    type Output = Self;

    /// Returns a new `Key` which is the bitwise XOR of `self` and `other`.
    fn bitxor(self, other: Self) -> Self::Output {
        Self(self.0 ^ other.0)
    }
}

impl const BitXorAssign for Key {
    /// Applies a bitwise XOR with `other` to `self` in place.
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0;
    }
}
