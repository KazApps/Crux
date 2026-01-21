use std::num::NonZeroU16;

use crate::shogi::{
    core::{PieceType, Square},
    position::hand::Hand,
};

/// Represents a move in the game.
///
/// `Move(u16)` is a compact representation of a shogi move.
///
/// Encoding layout:
///
/// [Normal move]
/// bits 0..=6  : destination square (7 bits)
/// bits 7..=13 : source square      (7 bits)
/// bit   14    : drop flag          (0 = normal move)
/// bit   15    : promotion flag     (1 = promotes)
///
/// [Drop move]
/// bits 0..=6   : destination square (7 bits)
/// bits 7..=9   : piece type         (3 bits)
/// bits 10..=13 : unused
/// bit   14     : drop flag          (1 = drop move)
/// bit   15     : unused
///
/// Notes:
/// - If bit 14 is set (drop move), bits 7..=9 store `PieceType` instead of a source square.
/// - Promotion flag (bit 15) is only meaningful for normal moves.
#[derive(Debug, Copy, Clone)]
pub struct Move(NonZeroU16);

impl Move {
    /// Creates a normal move from `from` square to `to` square.
    ///
    /// # Debug assertions
    /// In debug builds, panics if `from` == `to`.
    #[must_use]
    pub const fn normal(from: Square, to: Square) -> Self {
        debug_assert!(from != to);

        Self(NonZeroU16::new(((from as u16) << Self::FROM_SHIFT) | (to as u16)).unwrap())
    }

    /// Creates a promotion move from `from` square to `to` square.
    ///
    /// # Debug assertions
    /// In debug builds, panics if `from` == `to`.
    #[must_use]
    pub const fn promote(from: Square, to: Square) -> Self {
        debug_assert!(from != to);

        Self(
            NonZeroU16::new(
                Self::PROMOTION_FLAG_MASK | ((from as u16) << Self::FROM_SHIFT) | (to as u16),
            )
            .unwrap(),
        )
    }

    /// Creates a drop move of `piece_type` to `to` square.
    ///
    /// # Debug assertions
    /// In debug builds, panics if `piece_type` is invalid as a hand piece type.
    #[must_use]
    pub const fn drop(piece_type: PieceType, to: Square) -> Self {
        debug_assert!(piece_type.as_usize() < Hand::HAND_PIECE_TYPES);

        Self(
            NonZeroU16::new(
                Self::DROP_FLAG_MASK | (piece_type as u16) << Self::DROP_PIECE_SHIFT | (to as u16),
            )
            .unwrap(),
        )
    }

    /// Returns `true` if this move is a promotion.
    #[must_use]
    pub const fn is_promotion(self) -> bool {
        self.0.get() & Self::PROMOTION_FLAG_MASK != 0
    }

    /// Returns `true` if this move is a drop.
    #[must_use]
    pub const fn is_drop(self) -> bool {
        self.0.get() & Self::DROP_FLAG_MASK != 0
    }

    /// Returns the source square of a normal move.
    ///
    /// # Debug assertions
    /// In debug builds, panics if this is not a normal move.
    #[must_use]
    pub const fn from(self) -> Square {
        debug_assert!(!self.is_drop());

        Square::from(((self.as_u16() >> Self::FROM_SHIFT) & Self::SQUARE_MASK) as u8)
    }

    /// Returns the destination square of the move.
    ///
    /// # Debug assertions
    /// In debug builds, panics if this is a special move.
    #[must_use]
    pub const fn to(self) -> Square {
        Square::from((self.as_u16() & Self::SQUARE_MASK) as u8)
    }

    /// Returns the `PieceType` of a drop move.
    ///
    /// # Debug assertions
    /// In debug builds, panics if this is not a drop move.
    #[must_use]
    pub const fn drop_piece_type(self) -> PieceType {
        debug_assert!(self.is_drop());

        PieceType::from(((self.as_u16() >> Self::DROP_PIECE_SHIFT) & Self::PIECE_MASK) as u8)
    }

    /// Returns the underlying `u16` representation of the move.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0.get()
    }

    const FROM_SHIFT: u16 = 7;
    const DROP_PIECE_SHIFT: u16 = 7;
    const DROP_FLAG_MASK: u16 = 1 << 14;
    const PROMOTION_FLAG_MASK: u16 = 1 << 15;
    const SQUARE_MASK: u16 = 0b1111111;
    const PIECE_MASK: u16 = 0b111;
}

impl const PartialEq for Move {
    /// Compares two `Move` values for equality.
    fn eq(&self, other: &Self) -> bool {
        self.as_u16() == other.as_u16()
    }
}

impl const Eq for Move {}
