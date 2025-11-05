use const_for::const_for;
use std::{
    fmt::{Display, Formatter, Result},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use crate::shogi::core::{Color, File, Rank, Square};

/// Represents a bitboard.
///
/// ==============================================
///                    Mapping
/// ==============================================
///
///    upper                  lower
/// |---------|----------------------------------|
///    9    8    7    6    5    4    3    2    1
/// +----+----+----+----+----+----+----+----+----+
/// |  9 |  0 | 54 | 45 | 36 | 27 | 18 |  9 |  0 | 一
/// +----+----+----+----+----+----+----+----+----+
/// | 10 |  1 | 55 | 46 | 37 | 28 | 19 | 10 |  1 | 二
/// +----+----+----+----+----+----+----+----+----+
/// | 11 |  2 | 56 | 47 | 38 | 29 | 20 | 11 |  2 | 三
/// +----+----+----+----+----+----+----+----+----+
/// | 12 |  3 | 57 | 48 | 39 | 30 | 21 | 12 |  3 | 四
/// +----+----+----+----+----+----+----+----+----+
/// | 13 |  4 | 58 | 49 | 40 | 31 | 22 | 13 |  4 | 五
/// +----+----+----+----+----+----+----+----+----+
/// | 14 |  5 | 59 | 50 | 41 | 32 | 23 | 14 |  5 | 六
/// +----+----+----+----+----+----+----+----+----+
/// | 15 |  6 | 60 | 51 | 42 | 33 | 24 | 15 |  6 | 七
/// +----+----+----+----+----+----+----+----+----+
/// | 16 |  7 | 61 | 52 | 43 | 34 | 25 | 16 |  7 | 八
/// +----+----+----+----+----+----+----+----+----+
/// | 17 |  8 | 62 | 53 | 44 | 35 | 26 | 17 |  8 | 九
/// +----+----+----+----+----+----+----+----+----+
///
/// Note:
/// The 64th bit (bit index 63 in 0-based counting) is intentionally left unused.
/// This allows certain bitboard operations to be optimized.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(u128);

impl Bitboard {
    /// Returns a bitboard with all bits set.
    #[must_use]
    pub const fn all() -> Self {
        Self(Self::MASK)
    }

    /// Returns an empty bitboard with no bits set.
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns `true` if any bit in the bitboard is set.
    #[must_use]
    pub const fn is_any(self) -> bool {
        self.0 != 0
    }

    /// Returns `true` if the bitboard has no bits set.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns `true` if exactly one bit in the bitboard is set.
    #[must_use]
    pub const fn is_single(self) -> bool {
        self.is_any() && !self.is_multiple()
    }

    /// Returns `true` if more than one bit in the bitboard is set.
    #[must_use]
    pub const fn is_multiple(self) -> bool {
        self.0 & (self.0.wrapping_sub(1)) != 0
    }

    /// Returns the number of bits set in the bitboard.
    #[must_use]
    pub const fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the least significant bit (LSB) as a `Square`.
    ///
    /// # Panics
    ///
    /// Panics if the bitboard has no bits set.
    #[must_use]
    pub const fn lsb(self) -> Square {
        debug_assert!(self.is_any());

        let bit_pos = self.0.trailing_zeros() as u8;
        Square::from(if bit_pos < 64 { bit_pos } else { bit_pos - 1 })
    }

    /// Returns a bitboard with only the least significant bit set.
    ///
    /// # Panics
    ///
    /// Panics if the bitboard has no bits set.
    #[must_use]
    pub const fn isolate_lsb(self) -> Self {
        debug_assert!(self.is_any());

        Self(self.0 & self.0.wrapping_neg())
    }

    /// Returns the least significant bit (LSB) as a `Square` and clears it from the bitboard.
    ///
    /// # Panics
    ///
    /// Panics if the bitboard has no bits set.
    #[must_use]
    pub const fn pop_lsb(&mut self) -> Square {
        debug_assert!(self.is_any());

        let lsb = self.lsb();
        self.0 &= self.0.wrapping_sub(1);

        lsb
    }

    // Leave the 63rd bit empty to make some operations faster.
    const MASK: u128 = ((1u128 << (Square::COUNT + 1)) - 1) ^ (1u128 << 63);
}

impl const BitAnd for Bitboard {
    type Output = Self;

    /// Returns a new bitboard representing the bitwise AND of `self` and `other`.
    fn bitand(self, other: Self) -> Self::Output {
        Self(self.0 & other.0)
    }
}

impl const BitAndAssign for Bitboard {
    /// Performs a bitwise AND between `self` and `other`, storing the result in `self`.
    fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0
    }
}

impl const BitOr for Bitboard {
    type Output = Self;

    /// Returns a new bitboard representing the bitwise OR of `self` and `other`.
    fn bitor(self, other: Self) -> Self::Output {
        Self(self.0 | other.0)
    }
}

impl const BitOrAssign for Bitboard {
    /// Performs a bitwise OR between `self` and `other`, storing the result in `self`.
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0
    }
}

impl const BitXor for Bitboard {
    type Output = Self;

    /// Returns a new bitboard representing the bitwise XOR of `self` and `other`.
    fn bitxor(self, other: Self) -> Self::Output {
        Self(self.0 ^ other.0)
    }
}

impl const BitXorAssign for Bitboard {
    /// Performs a bitwise XOR between `self` and `other`, storing the result in `self`.
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0
    }
}

impl const Not for Bitboard {
    type Output = Self;

    /// Returns a new bitboard with all bits flipped, within the valid mask.
    fn not(self) -> Self::Output {
        Self(self.0 ^ Self::MASK)
    }
}

impl const From<File> for Bitboard {
    /// Returns a bitboard with all squares in the given file set.
    fn from(value: File) -> Self {
        const TABLE: [Bitboard; File::COUNT] = {
            let mut table = [Bitboard::empty(); File::COUNT];

            const_for!(file in 0..File::COUNT => {
                const_for!(rank in 0..Rank::COUNT => {
                    let file = File::from(file);
                    let rank = Rank::from(rank);
                    let square = Square::new(file, rank);

                    table[file.as_usize()] |= Bitboard::from(square);
                });
            });

            table
        };

        TABLE[value.as_usize()]
    }
}

impl const From<Rank> for Bitboard {
    /// Returns a bitboard with all squares in the given rank set.
    fn from(value: Rank) -> Self {
        const TABLE: [Bitboard; Rank::COUNT] = {
            let mut table = [Bitboard::empty(); Rank::COUNT];

            const_for!(file in 0..File::COUNT => {
                const_for!(rank in 0..Rank::COUNT => {
                    let file = File::from(file);
                    let rank = Rank::from(rank);
                    let square = Square::new(file, rank);

                    table[rank.as_usize()] |= Bitboard::from(square);
                });
            });

            table
        };

        TABLE[value.as_usize()]
    }
}

impl const From<Square> for Bitboard {
    /// Returns a bitboard with only the given square set.
    fn from(value: Square) -> Self {
        const TABLE: [Bitboard; Square::COUNT] = {
            let mut table = [Bitboard::empty(); Square::COUNT];

            const_for!(square in 0..Square::COUNT => {
                if square < Square::S81.as_usize() {
                    table[square] = Bitboard(1 << square);
                } else {
                    table[square] = Bitboard(1 << (square + 1));
                }
            });

            table
        };

        TABLE[value.as_usize()]
    }
}

/// Returns the promotion area for the given color as a `Bitboard`.
///
/// For black, this is the first three ranks (RANK1..=RANK3),
/// and for white, the last three ranks (RANK7..=RANK9).
#[must_use]
pub const fn promotion_area(color: Color) -> Bitboard {
    [
        Rank::Rank1.bit() | Rank::Rank2.bit() | Rank::Rank3.bit(),
        Rank::Rank7.bit() | Rank::Rank8.bit() | Rank::Rank9.bit(),
    ][color.as_usize()]
}

/// Returns a `Bitboard` representing squares where a pawn can be dropped for the given color.
///
/// This function takes a bitboard of existing pawns (`pawns_bb`) and computes
/// all valid drop squares, ensuring:
/// - No doubled pawns occur.
/// - Pawns are not dropped on the first rank (1st rank for black, 9th rank for white).
///
/// # Panics
///
/// If `pawns_bb` contains a state that would result in doubled pawns, this function may panic.
#[must_use]
pub const fn pawn_drop_mask(color: Color, pawns_bb: Bitboard) -> Bitboard {
    const RANK9: Bitboard = Rank::Rank9.bit();

    let mut bb = RANK9.0 - pawns_bb.0;

    if color.is_black() {
        bb = (bb & RANK9.0) >> 7;
        RANK9 ^ Bitboard(RANK9.0 - bb)
    } else {
        bb = (bb & RANK9.0) >> 8;
        Bitboard((!RANK9).0 & (RANK9.0 - bb))
    }
}

impl File {
    /// Returns a `Bitboard` representing this file.
    pub const fn bit(self) -> Bitboard {
        Bitboard::from(self)
    }
}

impl Rank {
    /// Returns a `Bitboard` representing this rank.
    pub const fn bit(self) -> Bitboard {
        Bitboard::from(self)
    }
}

impl Square {
    /// Returns a `Bitboard` representing this square.
    pub const fn bit(self) -> Bitboard {
        Bitboard::from(self)
    }
}

impl Display for Bitboard {
    /// Formats the bitboard as a human-readable 9x9 board.
    ///
    /// Squares that are set in the bitboard are shown as 'X', and empty squares as ' '.
    /// Files are labeled 9..1 from left to right, and ranks are labeled 一..九 on the right.
    ///
    /// Example output:
    /// ```
    ///   9   8   7   6   5   4   3   2   1
    /// +---+---+---+---+---+---+---+---+---+
    /// | X |   |   |   |   |   |   |   |   | 一
    /// +---+---+---+---+---+---+---+---+---+
    /// ...
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        const RANK_TO_CHAR: [char; Rank::COUNT] =
            ['一', '二', '三', '四', '五', '六', '七', '八', '九'];
        const RANK_SEPARATOR: &str = "+---+---+---+---+---+---+---+---+---+";

        writeln!(f, "  9   8   7   6   5   4   3   2   1")?;
        writeln!(f, "{}", RANK_SEPARATOR)?;

        for rank in 0..Rank::COUNT {
            let rank = Rank::from(rank);

            for file in (0..File::COUNT).rev() {
                let file = File::from(file);
                let square = Square::new(file, rank);

                write!(
                    f,
                    "| {} ",
                    if (*self & Bitboard::from(square)).is_any() {
                        'X'
                    } else {
                        ' '
                    }
                )?;
            }

            writeln!(f, "| {}", RANK_TO_CHAR[rank.as_usize()])?;
            write!(f, "{}", RANK_SEPARATOR)?;

            if rank != Rank::Rank9 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
