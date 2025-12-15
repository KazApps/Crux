use const_for::const_for;

use crate::shogi::core::{
    PieceType, MAX_BISHOP, MAX_GOLD, MAX_KNIGHT, MAX_LANCE, MAX_PAWN, MAX_ROOK, MAX_SILVER,
};

/// Represents a hand.
///
/// `Hand(u32)` is a structure that encodes the number of pieces in hand.
///
/// Bit layout (from LSB to MSB):
/// bits  0..=4  : Pawn   count (max 18, 5 bits)
/// bits  5..=7  : Lance  count (max  4, 3 bits)
/// bits  8..=10 : Knight count (max  4, 3 bits)
/// bits 11..=13 : Silver count (max  4, 3 bits)
/// bits 14..=16 : Gold   count (max  4, 3 bits)
/// bits 17..=18 : Bishop count (max  2, 2 bits)
/// bits 19..=20 : Rook   count (max  2, 2 bits)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Hand(u32);

impl Hand {
    #[must_use]
    pub const fn has_any(self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn count(self, piece_type: PieceType) -> u32 {
        debug_assert!(piece_type.as_usize() < Self::HAND_PIECE_TYPES);

        let offset = Self::OFFSETS[piece_type.as_usize()];
        let mask = Self::MASKS[piece_type.as_usize()];

        (self.0 & mask) >> offset
    }

    pub const fn set(&mut self, piece_type: PieceType, count: u32) {
        debug_assert!(piece_type.as_usize() < Self::HAND_PIECE_TYPES);
        debug_assert!(count <= Self::max_piece_counts(piece_type));

        let offset = Self::OFFSETS[piece_type.as_usize()];
        let mask = Self::MASKS[piece_type.as_usize()];

        self.0 = (self.0 & !mask) | (count << offset);
    }

    pub const fn increment(&mut self, piece_type: PieceType) {
        debug_assert!(piece_type.as_usize() < Self::HAND_PIECE_TYPES);

        self.set(piece_type, self.count(piece_type) + 1);
    }

    pub const fn decrement(&mut self, piece_type: PieceType) {
        debug_assert!(piece_type.as_usize() < Self::HAND_PIECE_TYPES);
        debug_assert!(self.count(piece_type) > 0);

        self.set(piece_type, self.count(piece_type) - 1);
    }

    pub const fn max_piece_counts(piece_type: PieceType) -> u32 {
        debug_assert!(piece_type.as_usize() < Self::HAND_PIECE_TYPES);

        Self::MAX_PIECE_COUNTS[piece_type.as_usize()]
    }

    pub const HAND_PIECE_TYPES: usize = PieceType::Rook.as_usize() + 1;

    const MAX_PIECE_COUNTS: [u32; Self::HAND_PIECE_TYPES] = [
        MAX_PAWN, MAX_LANCE, MAX_KNIGHT, MAX_SILVER, MAX_GOLD, MAX_BISHOP, MAX_ROOK,
    ];

    const BITS: [u32; Self::HAND_PIECE_TYPES] = {
        let mut bits = [0; Self::HAND_PIECE_TYPES];

        const_for!(i in 0..Self::HAND_PIECE_TYPES => {
            bits[i] = bit_width(Self::MAX_PIECE_COUNTS[i]);
        });

        bits
    };

    const OFFSETS: [u32; Self::HAND_PIECE_TYPES] = {
        let mut offsets = [0; Self::HAND_PIECE_TYPES];

        const_for!(i in 1..Self::HAND_PIECE_TYPES => {
            offsets[i] = offsets[i - 1] + Self::BITS[i - 1];
        });

        offsets
    };

    const MASKS: [u32; Self::HAND_PIECE_TYPES] = {
        let mut masks = [0; Self::HAND_PIECE_TYPES];

        const_for!(i in 0..Self::HAND_PIECE_TYPES => {
            masks[i] = ((1u32 << Self::BITS[i]) - 1) << Self::OFFSETS[i];
        });

        masks
    };

    const _TOTAL_BITS: u32 =
        Self::OFFSETS[Self::HAND_PIECE_TYPES - 1] + Self::BITS[Self::HAND_PIECE_TYPES - 1];
    const _HAND_FITS_IN_U32: () = assert!(Self::_TOTAL_BITS <= 32);
}

impl const Default for Hand {
    fn default() -> Self {
        Self(0)
    }
}

const fn bit_width(max: u32) -> u32 {
    (max + 1).ilog2() + 1
}
