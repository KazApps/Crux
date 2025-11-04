use const_for::const_for;
use std::{
    fmt::{Display, Formatter, Result},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use crate::shogi::core::{Color, File, Rank, Square};

// ==============================================
//                    Mapping
// ==============================================
//
//    upper                  lower
// |---------|----------------------------------|
//    9    8    7    6    5    4    3    2    1
// +----+----+----+----+----+----+----+----+----+
// |  9 |  0 | 54 | 45 | 36 | 27 | 18 |  9 |  0 | 一
// +----+----+----+----+----+----+----+----+----+
// | 10 |  1 | 55 | 46 | 37 | 28 | 19 | 10 |  1 | 二
// +----+----+----+----+----+----+----+----+----+
// | 11 |  2 | 56 | 47 | 38 | 29 | 20 | 11 |  2 | 三
// +----+----+----+----+----+----+----+----+----+
// | 12 |  3 | 57 | 48 | 39 | 30 | 21 | 12 |  3 | 四
// +----+----+----+----+----+----+----+----+----+
// | 13 |  4 | 58 | 49 | 40 | 31 | 22 | 13 |  4 | 五
// +----+----+----+----+----+----+----+----+----+
// | 14 |  5 | 59 | 50 | 41 | 32 | 23 | 14 |  5 | 六
// +----+----+----+----+----+----+----+----+----+
// | 15 |  6 | 60 | 51 | 42 | 33 | 24 | 15 |  6 | 七
// +----+----+----+----+----+----+----+----+----+
// | 16 |  7 | 61 | 52 | 43 | 34 | 25 | 16 |  7 | 八
// +----+----+----+----+----+----+----+----+----+
// | 17 |  8 | 62 | 53 | 44 | 35 | 26 | 17 |  8 | 九
// +----+----+----+----+----+----+----+----+----+

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Bitboard(u128);

impl Bitboard {
    #[must_use]
    pub const fn all() -> Self {
        Self(Self::MASK)
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn is_any(self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn is_single(self) -> bool {
        self.is_any() && !self.is_multiple()
    }

    #[must_use]
    pub const fn is_multiple(self) -> bool {
        self.0 & (self.0 - 1) != 0
    }

    #[must_use]
    pub const fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    #[must_use]
    pub const fn lsb(self) -> Square {
        debug_assert!(self.is_any());

        let bit_pos = self.0.trailing_zeros() as u8;
        Square::from(if bit_pos < 64 { bit_pos } else { bit_pos - 1 })
    }

    #[must_use]
    pub const fn isolate_lsb(self) -> Self {
        debug_assert!(self.is_any());

        Self(self.0 & self.0.wrapping_neg())
    }

    #[must_use]
    pub const fn pop_lsb(&mut self) -> Square {
        debug_assert!(self.is_any());

        let lsb = self.lsb();
        self.0 &= self.0.wrapping_sub(1);

        lsb
    }

    const MASK: u128 = ((1u128 << (Square::COUNT + 1)) - 1) ^ (1u128 << 63);
}

impl const BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, other: Self) -> Self::Output {
        Self(self.0 & other.0)
    }
}

impl const BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, other: Self) {
        self.0 &= other.0
    }
}

impl const BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        Self(self.0 | other.0)
    }
}

impl const BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, other: Self) {
        self.0 |= other.0
    }
}

impl const BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        Self(self.0 ^ other.0)
    }
}

impl const BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, other: Self) {
        self.0 ^= other.0
    }
}

impl const Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(self.0 ^ Self::MASK)
    }
}

impl const From<File> for Bitboard {
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

pub const FILE1: Bitboard = Bitboard::from(File::File1);
pub const FILE2: Bitboard = Bitboard::from(File::File2);
pub const FILE3: Bitboard = Bitboard::from(File::File3);
pub const FILE4: Bitboard = Bitboard::from(File::File4);
pub const FILE5: Bitboard = Bitboard::from(File::File5);
pub const FILE6: Bitboard = Bitboard::from(File::File6);
pub const FILE7: Bitboard = Bitboard::from(File::File7);
pub const FILE8: Bitboard = Bitboard::from(File::File8);
pub const FILE9: Bitboard = Bitboard::from(File::File9);

pub const RANK1: Bitboard = Bitboard::from(Rank::Rank1);
pub const RANK2: Bitboard = Bitboard::from(Rank::Rank2);
pub const RANK3: Bitboard = Bitboard::from(Rank::Rank3);
pub const RANK4: Bitboard = Bitboard::from(Rank::Rank4);
pub const RANK5: Bitboard = Bitboard::from(Rank::Rank5);
pub const RANK6: Bitboard = Bitboard::from(Rank::Rank6);
pub const RANK7: Bitboard = Bitboard::from(Rank::Rank7);
pub const RANK8: Bitboard = Bitboard::from(Rank::Rank8);
pub const RANK9: Bitboard = Bitboard::from(Rank::Rank9);

#[must_use]
pub const fn promotion_area(color: Color) -> Bitboard {
    [RANK1 | RANK2 | RANK3, RANK7 | RANK8 | RANK9][color.as_usize()]
}

#[must_use]
pub const fn pawn_drop_mask(color: Color, pawn_bb: Bitboard) -> Bitboard {
    let mut bb = RANK9.0 - pawn_bb.0;

    if color.is_black() {
        bb = (bb & RANK9.0) >> 7;
        RANK9 ^ Bitboard(RANK9.0 - bb)
    } else {
        bb = (bb & RANK9.0) >> 8;
        Bitboard((!RANK9).0 & (RANK9.0 - bb))
    }
}

impl Display for Bitboard {
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
