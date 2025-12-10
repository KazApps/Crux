use const_for::const_for;

use crate::shogi::core::PieceType;
use crate::shogi::{
    bitboard::Bitboard,
    core::{Color, File, Piece, Rank, Square},
};

type Attacks = [Bitboard; Square::COUNT];
type SidedAttacks = [Attacks; Color::COUNT];

// (direction0, direction1, direction2, direction3, all_directions)
type Masks = [(Bitboard, Bitboard, Bitboard, Bitboard, Bitboard); Square::COUNT];

macro_rules! generate_attacks {
    (|$square:ident| $body:expr) => {{
        let mut attacks = [Bitboard::empty(); Square::COUNT];

        const_for!(square_idx in 0..Square::COUNT => {
            let $square = Square::from(square_idx);

            attacks[square_idx] = $body;
        });

        attacks
    }};
}

macro_rules! generate_sided_attacks {
    (|$color:ident, $square:ident| $body:expr) => {{
        let mut attacks = [[Bitboard::empty(); Square::COUNT]; Color::COUNT];

        const_for!(color_idx in 0..Color::COUNT => {
            const_for!(square_idx in 0..Square::COUNT => {
                let $color = Color::from(color_idx);
                let $square = Square::from(square_idx);

                attacks[color_idx][square_idx] = $body;
            });
        });

        attacks
    }};
}

macro_rules! generate_masks {
    (|$square:ident| $body:expr) => {{
        let mut masks = [(Bitboard::empty(), Bitboard::empty(),
                            Bitboard::empty(), Bitboard::empty(),
                            Bitboard::empty()); Square::COUNT];

        const_for!(square_idx in 0..Square::COUNT => {
            let $square = Square::from(square_idx);

            masks[square_idx] = $body;
        });

        masks
    }};
}

/// Returns pawn attacks from the given color and square.
/// Always a single bit is set in the returned bitboard.
///
/// # Panics
///
/// Panics if the square is on the relative rank 1 for the given color.
pub const fn pawn_attacks(color: Color, square: Square) -> Bitboard {
    debug_assert!(!matches!(square.rank().relative(color), Rank::Rank1));

    square.relative_north(color).bit()
}

/// Returns pawn attacks for all squares set in `pawns_bb` for the given color.
///
/// # Panics
///
/// Panics if any square lies on relative rank 1 for the given color.
pub const fn multi_pawn_attacks(color: Color, pawns_bb: Bitboard) -> Bitboard {
    debug_assert!((pawns_bb & Rank::Rank1.relative(color).bit()).is_empty());

    if color.is_black() {
        pawns_bb.shr(1)
    } else {
        pawns_bb.shl(1)
    }
}

/// Returns the precomputed pseudo lance attacks for the given color and square.
///
/// This is equivalent to calling `lance_attacks` with `occupied = Bitboard::empty()`,
/// i.e., it ignores all pieces and assumes an empty board.
///
/// # Panics
///
/// Panics if the given square is on relative `Rank1`.
pub const fn lance_pseudo_attacks(color: Color, square: Square) -> Bitboard {
    debug_assert!(!matches!(square.rank().relative(color), Rank::Rank1));

    LANCE_PSEUDO_ATTACKS[color.as_usize()][square.as_usize()]
}

/// Returns lance attacks from the given color and square.
///
/// The `occupied` bitboard may include or exclude the given square.
///
/// # Panics
///
/// Panics if the given square is on relative `Rank1`.
pub const fn lance_attacks(color: Color, square: Square, occupied: Bitboard) -> Bitboard {
    debug_assert!(!matches!(square.rank().relative(color), Rank::Rank1));

    let pseudo_attacks = LANCE_PSEUDO_ATTACKS[color.as_usize()][square.as_usize()];

    if color.is_black() {
        sliding_backward(occupied, pseudo_attacks)
    } else {
        sliding_forward(occupied, pseudo_attacks)
    }
}

/// Returns knight attacks from the given color and square.
///
/// # Panics
///
/// Panics if the given square is on relative `Rank1` or `Rank2` for the given color.
pub const fn knight_attacks(color: Color, square: Square) -> Bitboard {
    debug_assert!(!matches!(
        square.rank().relative(color),
        Rank::Rank1 | Rank::Rank2
    ));

    const KNIGHT_ATTACKS: SidedAttacks = generate_sided_attacks!(|color, square| {
        if matches!(square.rank().relative(color), Rank::Rank1 | Rank::Rank2) {
            Bitboard::empty()
        } else {
            multi_knight_attacks(color, square.bit())
        }
    });

    KNIGHT_ATTACKS[color.as_usize()][square.as_usize()]
}

/// Returns knight attacks for all squares set in `knights_bb` for the given color.
///
/// # Panics
///
/// Panics if any square lies on relative ranks 1 or 2 for the given color.
pub const fn multi_knight_attacks(color: Color, knights_bb: Bitboard) -> Bitboard {
    debug_assert!((knights_bb
        & (Rank::Rank1.relative(color).bit() | Rank::Rank2.relative(color).bit()))
    .is_empty());

    if color.is_black() {
        knights_bb.shr(11) | knights_bb.shl(7)
    } else {
        knights_bb.shl(11) | knights_bb.shr(7)
    }
}

/// Returns silver attacks from the given color and square.
pub const fn silver_attacks(color: Color, square: Square) -> Bitboard {
    const SILVER_ATTACKS: SidedAttacks =
        generate_sided_attacks!(|color, square| multi_silver_attacks(color, square.bit()));

    SILVER_ATTACKS[color.as_usize()][square.as_usize()]
}

/// Returns silver attacks for all squares set in `silvers_bb` for the given color.
pub const fn multi_silver_attacks(color: Color, silvers_bb: Bitboard) -> Bitboard {
    let without_rank1 = silvers_bb & !Rank::Rank1.bit();
    let without_rank9 = silvers_bb & !Rank::Rank9.bit();

    if color.is_black() {
        without_rank1.shr(10)
            | without_rank1.shr(1)
            | without_rank1.shl(8)
            | without_rank9.shr(8)
            | without_rank9.shl(10)
    } else {
        without_rank9.shl(10)
            | without_rank9.shl(1)
            | without_rank9.shr(8)
            | without_rank1.shl(8)
            | without_rank1.shr(10)
    }
}

/// Returns gold attacks from the given color and square.
pub const fn gold_attacks(color: Color, square: Square) -> Bitboard {
    const GOLD_ATTACKS: SidedAttacks =
        generate_sided_attacks!(|color, square| multi_gold_attacks(color, square.bit()));

    GOLD_ATTACKS[color.as_usize()][square.as_usize()]
}

/// Returns gold attacks for all squares set in `golds_bb` for the given color.
pub const fn multi_gold_attacks(color: Color, golds_bb: Bitboard) -> Bitboard {
    let without_rank1 = golds_bb & !Rank::Rank1.bit();
    let without_rank9 = golds_bb & !Rank::Rank9.bit();

    if color.is_black() {
        without_rank1.shr(10)
            | without_rank1.shr(1)
            | without_rank1.shl(8)
            | golds_bb.shr(9)
            | golds_bb.shl(9)
            | without_rank9.shl(1)
    } else {
        without_rank9.shl(10)
            | without_rank9.shl(1)
            | without_rank9.shr(8)
            | golds_bb.shl(9)
            | golds_bb.shr(9)
            | without_rank1.shr(1)
    }
}

/// Returns the precomputed pseudo bishop attacks for the given square.
///
/// This is equivalent to calling `bishop_attacks` with `occupied = Bitboard::empty()`,
/// i.e., it ignores all pieces and assumes an empty board.
pub const fn bishop_pseudo_attacks(square: Square) -> Bitboard {
    BISHOP_MASKS[square.as_usize()].4
}

/// Returns bishop attacks from the given square.
///
/// The `occupied` bitboard may include or exclude the given square.
pub const fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    let masks = BISHOP_MASKS[square.as_usize()];

    sliding_backward(occupied, masks.0)
        | sliding_backward(occupied, masks.1)
        | sliding_forward(occupied, masks.2)
        | sliding_forward(occupied, masks.3)
}

/// Returns the precomputed pseudo rook attacks for the given square.
///
/// This is equivalent to calling `rook_attacks` with `occupied = Bitboard::empty()`,
/// i.e., it ignores all pieces and assumes an empty board.
pub const fn rook_pseudo_attacks(square: Square) -> Bitboard {
    ROOK_MASKS[square.as_usize()].4
}

/// Returns rook attacks from the given square.
///
/// The `occupied` bitboard may include or exclude the given square.
pub const fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    let masks = ROOK_MASKS[square.as_usize()];

    sliding_backward(occupied, masks.0)
        | sliding_backward(occupied, masks.1)
        | sliding_forward(occupied, masks.2)
        | sliding_forward(occupied, masks.3)
}

/// Returns the precomputed pseudo horse attacks for the given square.
///
/// This is equivalent to calling `horse_attacks` with `occupied = Bitboard::empty()`,
/// i.e., it ignores all pieces and assumes an empty board.
pub const fn horse_pseudo_attacks(square: Square) -> Bitboard {
    bishop_pseudo_attacks(square) | king_attacks(square)
}

/// Returns horse attacks from the given square.
///
/// The `occupied` bitboard may include or exclude the given square.
pub const fn horse_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    bishop_attacks(square, occupied) | king_attacks(square)
}

/// Returns the precomputed pseudo dragon attacks for the given square.
///
/// This is equivalent to calling `dragon_attacks` with `occupied = Bitboard::empty()`,
/// i.e., it ignores all pieces and assumes an empty board.
pub const fn dragon_pseudo_attacks(square: Square) -> Bitboard {
    rook_pseudo_attacks(square) | king_attacks(square)
}

/// Returns dragon attacks from the given square.
///
/// The `occupied` bitboard may include or exclude the given square.
pub const fn dragon_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    rook_attacks(square, occupied) | king_attacks(square)
}

/// Returns king attacks from the given square.
pub const fn king_attacks(square: Square) -> Bitboard {
    const KING_ATTACKS: Attacks = generate_attacks!(
        |square| silver_attacks(Color::Black, square) | gold_attacks(Color::Black, square)
    );

    KING_ATTACKS[square.as_usize()]
}

/// Returns the pseudo attacks of the given piece type from the square.
///
/// # Panics
///
/// Panics if the square is invalid for the piece type:
/// - A pawn or lance on the relative rank 1 for its color.
/// - A knight on the last two ranks relative to its color.
pub const fn piece_pseudo_attacks(piece: Piece, square: Square) -> Bitboard {
    match piece.piece_type() {
        PieceType::Pawn => pawn_attacks(piece.color(), square),
        PieceType::Lance => lance_pseudo_attacks(piece.color(), square),
        PieceType::Knight => knight_attacks(piece.color(), square),
        PieceType::Silver => silver_attacks(piece.color(), square),
        PieceType::Gold
        | PieceType::ProPawn
        | PieceType::ProLance
        | PieceType::ProKnight
        | PieceType::ProSilver => gold_attacks(piece.color(), square),
        PieceType::Bishop => bishop_pseudo_attacks(square),
        PieceType::Rook => rook_pseudo_attacks(square),
        PieceType::Horse => horse_pseudo_attacks(square),
        PieceType::Dragon => dragon_pseudo_attacks(square),
        PieceType::King => king_attacks(square),
    }
}

/// Returns the attacks of the given piece type from the square.
///
/// The `occupied` bitboard may include or exclude the given square.
///
/// # Panics
///
/// Panics if the square is invalid for the piece type:
/// - A pawn or lance on the relative rank 1 for its color.
/// - A knight on the last two ranks relative to its color.
pub const fn piece_attacks(piece: Piece, square: Square, occupied: Bitboard) -> Bitboard {
    match piece.piece_type() {
        PieceType::Pawn => pawn_attacks(piece.color(), square),
        PieceType::Lance => lance_attacks(piece.color(), square, occupied),
        PieceType::Knight => knight_attacks(piece.color(), square),
        PieceType::Silver => silver_attacks(piece.color(), square),
        PieceType::Gold
        | PieceType::ProPawn
        | PieceType::ProLance
        | PieceType::ProKnight
        | PieceType::ProSilver => gold_attacks(piece.color(), square),
        PieceType::Bishop => bishop_attacks(square, occupied),
        PieceType::Rook => rook_attacks(square, occupied),
        PieceType::Horse => horse_attacks(square, occupied),
        PieceType::Dragon => dragon_attacks(square, occupied),
        PieceType::King => king_attacks(square),
    }
}

const fn sliding_forward(occupied: Bitboard, mask: Bitboard) -> Bitboard {
    let tz = (occupied & mask | Square::S99.bit())
        .as_u128()
        .trailing_zeros();

    Bitboard::new(mask.as_u128() & ((1 << (tz + 1)) - 1))
}

const fn sliding_backward(occupied: Bitboard, mask: Bitboard) -> Bitboard {
    let lz = (occupied & mask | Square::S11.bit())
        .as_u128()
        .leading_zeros();

    Bitboard::new(mask.as_u128() & !((1 << (127 - lz)) - 1))
}

const LANCE_PSEUDO_ATTACKS: SidedAttacks = generate_sided_attacks! { |color, square| {
    let mut bb = square.bit();
    let mut res = Bitboard::empty();

    while (bb & Rank::Rank1.relative(color).bit()).is_empty() {
        bb = if color.is_black() { bb.shr(1) } else { bb.shl(1) };
        res |= bb;
    }

    res
}};

const BISHOP_MASKS: Masks = generate_masks!(|square| {
    let mut bb = square.bit();
    let mut right_up = Bitboard::empty();
    let mut right_down = Bitboard::empty();
    let mut left_up = Bitboard::empty();
    let mut left_down = Bitboard::empty();

    while (bb & (File::File1.bit() | Rank::Rank1.bit())).is_empty() {
        bb = bb.shr(10);
        right_up |= bb;
    }

    bb = square.bit();

    while (bb & (File::File1.bit() | Rank::Rank9.bit())).is_empty() {
        bb = bb.shr(8);
        right_down |= bb;
    }

    bb = square.bit();

    while (bb & (File::File9.bit() | Rank::Rank1.bit())).is_empty() {
        bb = bb.shl(8);
        left_up |= bb;
    }

    bb = square.bit();

    while (bb & (File::File9.bit() | Rank::Rank9.bit())).is_empty() {
        bb = bb.shl(10);
        left_down |= bb;
    }

    (
        right_up,
        right_down,
        left_up,
        left_down,
        right_up | right_down | left_up | left_down,
    )
});

const ROOK_MASKS: Masks = generate_masks!(|square| {
    let mut bb = square.bit();
    let mut right = Bitboard::empty();
    let mut up = Bitboard::empty();
    let mut left = Bitboard::empty();
    let mut down = Bitboard::empty();

    while (bb & File::File1.bit()).is_empty() {
        bb = bb.shr(9);
        right |= bb;
    }

    bb = square.bit();

    while (bb & Rank::Rank1.bit()).is_empty() {
        bb = bb.shr(1);
        up |= bb;
    }

    bb = square.bit();

    while (bb & File::File9.bit()).is_empty() {
        bb = bb.shl(9);
        left |= bb;
    }

    bb = square.bit();

    while (bb & Rank::Rank9.bit()).is_empty() {
        bb = bb.shl(1);
        down |= bb;
    }

    (right, up, left, down, right | up | left | down)
});
