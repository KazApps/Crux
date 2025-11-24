use const_for::const_for;

use crate::shogi::core::PieceType;
use crate::shogi::{
    bitboard::Bitboard,
    core::{Color, File, Piece, Rank, Square},
};

type Attacks = [Bitboard; Square::COUNT];
type SidedAttacks = [Attacks; Color::COUNT];

macro_rules! generate_attacks {
    (|$square:ident| $body:stmt) => {{
        let mut attacks = [Bitboard::empty(); Square::COUNT];

        const_for!(square_idx in 0..Square::COUNT => {
            let $square = Square::from(square_idx);

            attacks[square_idx] = { $body };
        });

        attacks
    }};
}

macro_rules! generate_sided_attacks {
    (|$color:ident, $square:ident| $body:stmt) => {{
        let mut attacks = [[Bitboard::empty(); Square::COUNT]; Color::COUNT];

        const_for!(color_idx in 0..Color::COUNT => {
            const_for!(square_idx in 0..Square::COUNT => {
                let $color = Color::from(color_idx);
                let $square = Square::from(square_idx);

                attacks[color_idx][square_idx] = { $body };
            });
        });

        attacks
    }};
}

pub const fn pawn_attacks(color: Color, square: Square) -> Bitboard {
    debug_assert!(square.rank().as_u8() != Rank::Rank1.relative(color).as_u8());

    square.relative_north(color).bit()
}

pub const fn multi_pawn_attacks(color: Color, pawns_bb: Bitboard) -> Bitboard {
    debug_assert!((pawns_bb & Rank::Rank1.relative(color).bit()).is_empty());

    if color.is_black() {
        pawns_bb.shr(1)
    } else {
        pawns_bb.shl(1)
    }
}

pub const fn lance_attacks(color: Color, square: Square, occupied: Bitboard) -> Bitboard {
    debug_assert!(square.rank().as_u8() != Rank::Rank1.relative(color).as_u8());
    debug_assert!((square.bit() & occupied).is_empty());

    let occupied = occupied | square.bit();

    if color.is_black() {
        const TABLE_SIZE: usize = 1 << (Rank::COUNT - 1);
        const TABLE: [u16; TABLE_SIZE] = {
            let mut results = [0u16; TABLE_SIZE];

            const_for!(idx in 1..TABLE_SIZE => {
                let mut b = 63 - idx.leading_zeros() as i32 - 1;
                let mut result = 0u16;

                while b >= 0 {
                    result |= 1u16 << b;

                    if (idx >> b) & 1 == 1 {
                        break;
                    }

                    b -= 1;
                }

                result = (result << 1) | u16::from(b < 0);
                results[idx] = result;
            });

            results
        };

        Bitboard(
            (TABLE[occupied.shr(square.file().as_usize() * Rank::COUNT + 1).0 as usize
                & ((1 << square.rank().as_u8()) - 1)] as u128)
                << (square.file().as_usize() * Rank::COUNT),
        )
    } else {
        let x = Rank::Rank9.bit() | occupied;
        x.sub(square.bit()).sub(square.bit()) ^ x
    }
}

pub const fn knight_attacks(color: Color, square: Square) -> Bitboard {
    debug_assert!((square.rank().bit()
        & (Rank::Rank1.relative(color).bit() | Rank::Rank2.relative(color).bit()))
    .is_empty());

    const KNIGHT_ATTACKS: SidedAttacks = generate_sided_attacks!(|color, square| {
        let north = square.bit().relative_north(color);

        if north.is_empty() {
            Bitboard::empty()
        } else {
            north.relative_north_east(color) | north.relative_north_west(color)
        }
    });

    KNIGHT_ATTACKS[color.as_usize()][square.as_usize()]
}

pub const fn multi_knight_attacks(color: Color, knights_bb: Bitboard) -> Bitboard {
    debug_assert!((knights_bb
        & (Rank::Rank1.relative(color).bit() | Rank::Rank2.relative(color).bit()))
    .is_empty());

    if color.is_black() {
        knights_bb.shr(11) | knights_bb.shl(7) & Bitboard::all()
    } else {
        knights_bb.shl(11) | knights_bb.shr(7) & Bitboard::all()
    }
}

pub const fn silver_attacks(color: Color, square: Square) -> Bitboard {
    const SILVER_ATTACKS: SidedAttacks = generate_sided_attacks!(|color, square| {
        let bb = square.bit();

        bb.relative_north(color)
            | bb.relative_north_east(color)
            | bb.relative_north_west(color)
            | bb.relative_south_east(color)
            | bb.relative_south_west(color)
    });

    SILVER_ATTACKS[color.as_usize()][square.as_usize()]
}

pub const fn multi_silver_attacks(color: Color, silvers_bb: Bitboard) -> Bitboard {
    let without_rank1 = silvers_bb & !Rank::Rank1.bit();
    let without_rank9 = silvers_bb & !Rank::Rank9.bit();

    if color.is_black() {
        (without_rank1.shr(10)
            | without_rank1.shr(1)
            | without_rank1.shl(8)
            | without_rank9.shr(8)
            | without_rank9.shl(10))
            & Bitboard::all()
    } else {
        (without_rank9.shl(10)
            | without_rank9.shl(1)
            | without_rank1.shr(8)
            | without_rank1.shl(8)
            | without_rank1.shr(10))
            & Bitboard::all()
    }
}

pub const fn gold_attacks(color: Color, square: Square) -> Bitboard {
    const GOLD_ATTACKS: SidedAttacks = generate_sided_attacks!(|color, square| {
        let bb = square.bit();

        bb.relative_north(color)
            | bb.relative_south(color)
            | bb.relative_east(color)
            | bb.relative_west(color)
            | bb.relative_north_east(color)
            | bb.relative_north_west(color)
    });

    GOLD_ATTACKS[color.as_usize()][square.as_usize()]
}

pub const fn multi_gold_attacks(color: Color, golds_bb: Bitboard) -> Bitboard {
    let without_rank1 = golds_bb & !Rank::Rank1.bit();
    let without_rank9 = golds_bb & !Rank::Rank9.bit();

    if color.is_black() {
        (without_rank1.shr(10)
            | without_rank1.shr(1)
            | without_rank1.shl(8)
            | golds_bb.shr(9)
            | golds_bb.shl(9)
            | without_rank9.shl(1))
            & Bitboard::all()
    } else {
        (without_rank9.shl(10)
            | without_rank9.shl(1)
            | without_rank9.shr(8)
            | golds_bb.shl(9)
            | golds_bb.shr(9)
            | without_rank1.shr(1))
            & Bitboard::all()
    }
}

const fn line_attacks(square: Square, occupied: Bitboard, mask: Bitboard) -> Bitboard {
    debug_assert!((square.bit() & occupied).is_empty());

    let occupied = occupied | square.bit();

    let forward = ((occupied & mask | Square::new(File::File9, Rank::Rank9).bit())
        & Bitboard(!((1u128 << (square.as_u8() + 1)) - 1)))
    .isolate_lsb()
    .shl(1);
    let backward = ((occupied & mask | Square::new(File::File1, Rank::Rank1).bit())
        & Bitboard((1u128 << square.as_u8()) - 1))
    .isolate_msb();
    (forward.sub(backward) & mask) & !square.bit()
}

pub const fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    debug_assert!((square.bit() & occupied).is_empty());

    const BISHOP_MASK1: [Bitboard; Square::COUNT] = {
        let mut results = [Bitboard::empty(); Square::COUNT];

        const_for!(idx in 0..Square::COUNT => {
            const_for!(idx2 in 0..Square::COUNT => {
                if idx / Rank::COUNT + idx % Rank::COUNT == idx2 / Rank::COUNT + idx2 % Rank::COUNT
                {
                    results[idx] |= Square::from(idx2).bit();
                }
            });
        });

        results
    };

    const BISHOP_MASK2: [Bitboard; Square::COUNT] = {
        let mut results = [Bitboard::empty(); Square::COUNT];

        const_for!(idx in 0..Square::COUNT => {
            const_for!(idx2 in 0..Square::COUNT => {
                if (idx / Rank::COUNT) as i32 - (idx % Rank::COUNT) as i32
                    == (idx2 / Rank::COUNT) as i32 - (idx2 % Rank::COUNT) as i32
                {
                    results[idx] |= Square::from(idx2).bit();
                }
            });
        });

        results
    };

    line_attacks(square, occupied, BISHOP_MASK1[square.as_usize()])
        | line_attacks(square, occupied, BISHOP_MASK2[square.as_usize()])
}

pub const fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    debug_assert!((square.bit() & occupied).is_empty());

    lance_attacks(Color::Black, square, occupied)
        | lance_attacks(Color::White, square, occupied)
        | line_attacks(square, occupied, square.rank().bit())
}

pub const fn horse_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    debug_assert!((square.bit() & occupied).is_empty());

    king_attacks(square) | bishop_attacks(square, occupied)
}

pub const fn dragon_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    debug_assert!((square.bit() & occupied).is_empty());

    king_attacks(square) | rook_attacks(square, occupied)
}

pub const fn king_attacks(square: Square) -> Bitboard {
    const KING_ATTACKS: Attacks = generate_attacks!(|square| {
        let bb = square.bit();

        bb.north()
            | bb.south()
            | bb.east()
            | bb.west()
            | bb.north_east()
            | bb.north_west()
            | bb.south_east()
            | bb.south_west()
    });

    KING_ATTACKS[square.as_usize()]
}

pub const fn piece_attacks(piece: Piece, square: Square, occupied: Bitboard) -> Bitboard {
    debug_assert!((square.bit() & occupied).is_empty());

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
