use crate::shogi::core::PieceType;
use crate::shogi::{
    core::{Color, Piece, Square, MAX_ROOK},
    position::{hand::Hand, key::Key},
};
use crate::utils::rng::Prng;
use const_for::const_for;

type SideKey = Key;
type PieceSquareKey = [[Key; Square::COUNT]; Piece::COUNT];
type HandKey = [Key; HAND_TOTAL];

const HAND_ENTRIES: usize = Hand::HAND_PIECE_TYPES * Color::COUNT;

const HAND_OFFSETS: [u32; HAND_ENTRIES] = {
    let mut offsets = [0; HAND_ENTRIES];

    const_for!(i in 1..HAND_ENTRIES => {
        offsets[i] = offsets[i - 1] + Hand::MAX_PIECE_COUNTS[Piece::from(i).piece_type().as_usize()];
    });

    offsets
};

const HAND_TOTAL: usize = (HAND_OFFSETS[HAND_ENTRIES - 1] + MAX_ROOK + 1) as usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    pub const SIDE: SideKey = Self::ALL_KEYS.side;
    pub const PIECE_SQUARE: PieceSquareKey = Self::ALL_KEYS.piece_square;
    pub const HAND: HandKey = Self::ALL_KEYS.hand;

    const ALL_KEYS: Self = {
        let mut zobrist = Zobrist::default();
        let mut rng = Prng::new(20251212);

        zobrist.side = Key::new(rng.rand());

        const_for!(i in 0..Piece::COUNT => {
            const_for!(j in 0..Square::COUNT => {
                zobrist.piece_square[i][j] = Key::new(rng.rand());
            });
        });

        const_for!(i in 0..HAND_TOTAL => {
            zobrist.hand[i] = Key::new(rng.rand());
        });

        zobrist
    };
}

#[must_use]
pub const fn side_key() -> Key {
    Zobrist::SIDE
}

#[must_use]
pub const fn piece_square_key(piece: Piece, square: Square) -> Key {
    Zobrist::PIECE_SQUARE[piece.as_usize()][square.as_usize()]
}

#[must_use]
pub const fn hand_key(color: Color, piece_type: PieceType, count: u32) -> Key {
    debug_assert!(piece_type.as_usize() < Hand::HAND_PIECE_TYPES);
    debug_assert!(count <= Hand::MAX_PIECE_COUNTS[piece_type.as_usize()]);

    Zobrist::HAND[HAND_OFFSETS[piece_type.with_color(color).as_usize()] as usize + count as usize]
}
