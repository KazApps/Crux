use const_for::const_for;

use crate::shogi::{
    core::{Color, Piece, PieceType, Square, MAX_ROOK},
    position::{hand::Hand, key::Key},
};
use crate::utils::rng::Prng;

type SideKey = Key;
type PieceSquareKey = [[Key; Square::COUNT]; Piece::COUNT];
type HandKey = [Key; HAND_TOTAL];

const HAND_ENTRIES: usize = Color::COUNT * Hand::HAND_PIECE_TYPES;

const HAND_OFFSETS: [u32; HAND_ENTRIES] = {
    let mut offsets = [0; HAND_ENTRIES];

    const_for!(i in 1..HAND_ENTRIES => {
        offsets[i] =
            offsets[i - 1] + Hand::max_piece_counts(Piece::from(i - 1).piece_type()) + 1;
    });

    offsets
};

const HAND_TOTAL: usize = (HAND_OFFSETS[HAND_ENTRIES - 1] + MAX_ROOK + 1) as usize;

struct Zobrist {
    side: SideKey,
    piece_square: PieceSquareKey,
    hand: HandKey,
}

impl const Default for Zobrist {
    fn default() -> Self {
        Zobrist {
            side: Key::default(),
            piece_square: [[Key::default(); Square::COUNT]; Piece::COUNT],
            hand: [Key::default(); HAND_TOTAL],
        }
    }
}

impl Zobrist {
    const SIDE: SideKey = Self::ALL_KEYS.side;
    const PIECE_SQUARE: PieceSquareKey = Self::ALL_KEYS.piece_square;
    const HAND: HandKey = Self::ALL_KEYS.hand;

    const ALL_KEYS: Self = {
        let mut zobrist = Zobrist::default();
        let mut rng = Prng::new(20251212);

        zobrist.side = Key::from(rng.rand());

        const_for!(i in 0..Piece::COUNT => {
            const_for!(j in 0..Square::COUNT => {
                zobrist.piece_square[i][j] = Key::from(rng.rand());
            });
        });

        const_for!(color in 0..Color::COUNT => {
            const_for!(piece_type in 0..Hand::HAND_PIECE_TYPES => {
                let piece_type = PieceType::from(piece_type);

                const_for!(count in 0..Hand::max_piece_counts(piece_type) => {
                    zobrist.hand
                        [hand_index(Color::from(color), piece_type, count + 1)] =
                        Key::from(rng.rand());
                });
            });
        });

        zobrist
    };
}

/// Returns the Zobrist key for the side to move.
#[must_use]
pub const fn side_key() -> Key {
    Zobrist::SIDE
}

/// Returns the Zobrist key for the given piece on a given square.
#[must_use]
pub const fn piece_square_key(piece: Piece, square: Square) -> Key {
    Zobrist::PIECE_SQUARE[piece][square]
}

/// Returns the Zobrist key for a piece in hand.
///
/// # Debug assertions
/// In debug builds, panics if `piece_type` or `count` is invalid.
#[must_use]
pub const fn hand_key(color: Color, piece_type: PieceType, count: u32) -> Key {
    debug_assert!(piece_type.as_usize() < Hand::HAND_PIECE_TYPES);
    debug_assert!(count <= Hand::max_piece_counts(piece_type));

    Zobrist::HAND[hand_index(color, piece_type, count)]
}

const fn hand_index(color: Color, piece_type: PieceType, count: u32) -> usize {
    HAND_OFFSETS[piece_type.with_color(color).as_usize()] as usize + count as usize
}
