use crux_lib::{
    notation::{usi::Usi, Notation},
    shogi::{
        bitboard::Bitboard,
        core::{Color, Piece, PieceType, Square},
        position::{
            key::Key,
            zobrist::{hand_key, piece_square_key, side_key},
            Position, PositionBuilder,
        },
    },
};

mod hand;
mod key;
mod mv;
mod zobrist;

const MAKE_MOVE_TEST_CASES: &[(&str, &str, &str, Option<Piece>)] = &[
    // Normal move
    (
        "9/9/9/9/4R4/9/9/9/9 b - 1",
        "5e5a",
        "4R4/9/9/9/9/9/9/9/9 w - 1",
        None,
    ),
    (
        "9/9/9/9/4r4/9/9/9/9 w - 1",
        "5e5i",
        "9/9/9/9/9/9/9/9/4r4 b - 1",
        None,
    ),
    // Promotion move
    (
        "9/9/9/9/4R4/9/9/9/9 b - 1",
        "5e5a+",
        "4+R4/9/9/9/9/9/9/9/9 w - 1",
        None,
    ),
    (
        "9/9/9/9/4r4/9/9/9/9 w - 1",
        "5e5i+",
        "9/9/9/9/9/9/9/9/4+r4 b - 1",
        None,
    ),
    // Drop move
    (
        "9/9/9/9/9/9/9/9/9 b P 1",
        "P*5e",
        "9/9/9/9/4P4/9/9/9/9 w - 1",
        None,
    ),
    (
        "9/9/9/9/9/9/9/9/9 w p 1",
        "P*5e",
        "9/9/9/9/4p4/9/9/9/9 b - 1",
        None,
    ),
    // Normal move with capture
    (
        "4p4/9/9/9/4R4/9/9/9/9 b - 1",
        "5e5a",
        "4R4/9/9/9/9/9/9/9/9 w P 1",
        Some(Piece::WhitePawn),
    ),
    (
        "9/9/9/9/4r4/9/9/9/4P4 w - 1",
        "5e5i",
        "9/9/9/9/9/9/9/9/4r4 b p 1",
        Some(Piece::BlackPawn),
    ),
    // Promotion move with capture
    (
        "4p4/9/9/9/4R4/9/9/9/9 b - 1",
        "5e5a+",
        "4+R4/9/9/9/9/9/9/9/9 w P 1",
        Some(Piece::WhitePawn),
    ),
    (
        "9/9/9/9/4r4/9/9/9/4P4 w - 1",
        "5e5i+",
        "9/9/9/9/9/9/9/9/4+r4 b p 1",
        Some(Piece::BlackPawn),
    ),
    // Normal move gives check
    (
        "8k/9/9/9/8N/9/9/9/8L b - 1",
        "1e2c",
        "8k/9/7N1/9/9/9/9/9/8L w - 1",
        None,
    ),
    (
        "l8/9/9/9/n8/9/9/9/K8 w - 1",
        "9e8g",
        "l8/9/9/9/9/9/1n7/9/K8 b - 1",
        None,
    ),
    // Promotion move gives check
    (
        "9/8k/9/9/8N/9/9/9/8L b - 1",
        "1e2c+",
        "9/8k/7+N1/9/9/9/9/9/8L w - 1",
        None,
    ),
    (
        "l8/9/9/9/n8/9/9/K8/9 w - 1",
        "9e8g+",
        "l8/9/9/9/9/9/1+n7/K8/9 b - 1",
        None,
    ),
    // Drop move gives check
    (
        "9/9/9/4k4/9/9/9/9/9 b P 1",
        "P*5e",
        "9/9/9/4k4/4P4/9/9/9/9 w - 1",
        None,
    ),
    (
        "9/9/9/9/9/4K4/9/9/9 w p 1",
        "P*5e",
        "9/9/9/9/4p4/4K4/9/9/9 b - 1",
        None,
    ),
    // Normal move with capture gives check
    (
        "8k/9/7p1/9/8N/9/9/9/8L b - 1",
        "1e2c",
        "8k/9/7N1/9/9/9/9/9/8L w P 1",
        Some(Piece::WhitePawn),
    ),
    (
        "l8/9/9/9/n8/9/1P7/9/K8 w - 1",
        "9e8g",
        "l8/9/9/9/9/9/1n7/9/K8 b p 1",
        Some(Piece::BlackPawn),
    ),
    // Promotion move with capture gives check
    (
        "9/8k/7p1/9/8N/9/9/9/8L b - 1",
        "1e2c+",
        "9/8k/7+N1/9/9/9/9/9/8L w P 1",
        Some(Piece::WhitePawn),
    ),
    (
        "l8/9/9/9/n8/9/1P7/K8/9 w - 1",
        "9e8g+",
        "l8/9/9/9/9/9/1+n7/K8/9 b p 1",
        Some(Piece::BlackPawn),
    ),
    // Evade check by moving the king
    (
        "l8/9/9/9/9/9/1n7/9/K8 b - 1",
        "9i8h",
        "l8/9/9/9/9/9/1n7/1K7/9 w - 1",
        None,
    ),
    (
        "8k/9/7N1/9/9/9/9/9/8L w - 1",
        "1a2b",
        "9/7k1/7N1/9/9/9/9/9/8L b - 1",
        None,
    ),
    // Evade check by moving the king with capture
    (
        "l8/9/9/9/9/9/1+n7/K8/9 b - 1",
        "9h8g",
        "l8/9/9/9/9/9/1K7/9/9 w N 1",
        Some(Piece::WhiteProKnight),
    ),
    (
        "9/8k/7+N1/9/9/9/9/9/8L w - 1",
        "1b2c",
        "9/9/7k1/9/9/9/9/9/8L b n 1",
        Some(Piece::BlackProKnight),
    ),
    // Evade check by capturing
    (
        "lR7/9/9/9/9/9/1+n7/9/K8 b - 1",
        "8a9a",
        "R8/9/9/9/9/9/1+n7/9/K8 w L 1",
        Some(Piece::WhiteLance),
    ),
    (
        "8k/9/7+N1/9/9/9/9/9/7rL w - 1",
        "2i1i",
        "8k/9/7+N1/9/9/9/9/9/8r b l 1",
        Some(Piece::BlackLance),
    ),
    // Evade check by capturing promotion
    (
        "lR7/9/9/9/9/9/1+n7/9/K8 b - 1",
        "8a9a+",
        "+R8/9/9/9/9/9/1+n7/9/K8 w L 1",
        Some(Piece::WhiteLance),
    ),
    (
        "8k/9/7+N1/9/9/9/9/9/7rL w - 1",
        "2i1i+",
        "8k/9/7+N1/9/9/9/9/9/8+r b l 1",
        Some(Piece::BlackLance),
    ),
    // Checkers and pins test
    (
        "8k/7pp/6R2/5B3/9/9/9/9/9 b - 1",
        "3c1c",
        "8k/7pp/8R/5B3/9/9/9/9/9 w - 1",
        None,
    ),
    (
        "9/9/9/9/9/3b5/2r6/PP7/K8 w - 1",
        "7g9g",
        "9/9/9/9/9/3b5/r8/PP7/K8 b - 1",
        None,
    ),
];

const PIN_TEST_POS1: Position = {
    let mut builder = Position::empty().builder();

    builder
        .place(Square::S11, Piece::WhiteKing)
        .place(Square::S12, Piece::WhitePawn)
        .place(Square::S13, Piece::BlackLance)
        .set_side_to_move(Color::White);

    builder.build()
};

const PIN_TEST_POS2: Position = {
    let mut builder = Position::empty().builder();

    builder
        .place(Square::S99, Piece::BlackKing)
        .place(Square::S98, Piece::BlackPawn)
        .place(Square::S97, Piece::WhiteLance);

    builder.build()
};

const PIN_TEST_POS3: Position = {
    let mut builder = PIN_TEST_POS1.builder();

    builder
        .place(Square::S22, Piece::BlackPawn)
        .place(Square::S44, Piece::BlackBishop);

    builder.build()
};

const PIN_TEST_POS4: Position = {
    let mut builder = PIN_TEST_POS2.builder();

    builder
        .place(Square::S88, Piece::WhitePawn)
        .place(Square::S66, Piece::WhiteBishop);

    builder.build()
};

const PIN_TEST_POS5: Position = {
    let mut builder = PIN_TEST_POS3.builder();

    builder
        .place(Square::S21, Piece::WhiteKnight)
        .place(Square::S41, Piece::BlackRook);

    builder.build()
};

const PIN_TEST_POS6: Position = {
    let mut builder = PIN_TEST_POS4.builder();

    builder
        .place(Square::S89, Piece::BlackKnight)
        .place(Square::S69, Piece::WhiteRook);

    builder.build()
};

const PIN_TEST_POS7: Position = {
    let mut builder = PIN_TEST_POS5.builder();

    builder
        .place(Square::S19, Piece::BlackLance)
        .place(Square::S55, Piece::BlackHorse)
        .place(Square::S61, Piece::BlackDragon);

    builder.build()
};

const PIN_TEST_POS8: Position = {
    let mut builder = PIN_TEST_POS6.builder();

    builder
        .place(Square::S91, Piece::WhiteLance)
        .place(Square::S55, Piece::WhiteHorse)
        .place(Square::S49, Piece::WhiteDragon);

    builder.build()
};

const MATSURI_POS: Position = {
    let mut builder = Position::empty().builder();

    builder
        .place(Square::S11, Piece::WhiteLance)
        .place(Square::S12, Piece::WhiteKing)
        .place(Square::S14, Piece::WhitePawn)
        .place(Square::S16, Piece::BlackPawn)
        .place(Square::S19, Piece::BlackLance)
        .place(Square::S21, Piece::WhiteKnight)
        .place(Square::S22, Piece::WhiteGold)
        .place(Square::S24, Piece::BlackPawn)
        .place(Square::S25, Piece::WhitePawn)
        .place(Square::S27, Piece::BlackSilver)
        .place(Square::S29, Piece::BlackKing)
        .place(Square::S35, Piece::BlackSilver)
        .place(Square::S36, Piece::BlackPawn)
        .place(Square::S37, Piece::BlackGold)
        .place(Square::S39, Piece::WhiteBishop)
        .place(Square::S42, Piece::BlackProPawn)
        .place(Square::S43, Piece::BlackSilver)
        .place(Square::S63, Piece::WhitePawn)
        .place(Square::S65, Piece::BlackPawn)
        .place(Square::S66, Piece::WhiteBishop)
        .place(Square::S73, Piece::WhiteKnight)
        .place(Square::S74, Piece::WhitePawn)
        .place(Square::S76, Piece::BlackPawn)
        .place(Square::S86, Piece::BlackPawn)
        .place(Square::S89, Piece::BlackKnight)
        .place(Square::S91, Piece::WhiteLance)
        .place(Square::S94, Piece::WhitePawn)
        .place(Square::S97, Piece::BlackPawn)
        .place(Square::S98, Piece::BlackRook)
        .place(Square::S99, Piece::BlackLance)
        .set_hand_piece_count(Color::Black, PieceType::Rook, 1)
        .set_hand_piece_count(Color::Black, PieceType::Gold, 1)
        .set_hand_piece_count(Color::White, PieceType::Gold, 1)
        .set_hand_piece_count(Color::White, PieceType::Silver, 1)
        .set_hand_piece_count(Color::White, PieceType::Knight, 1)
        .set_hand_piece_count(Color::White, PieceType::Pawn, 5);

    builder.build()
};

fn same_position(lhs: &Position, rhs: &Position) -> bool {
    lhs.side_to_move() == rhs.side_to_move()
        && lhs.occupancy() == rhs.occupancy()
        && (0..Piece::COUNT).all(|piece| {
            let piece = Piece::from(piece);

            lhs.piece_bb(piece) == rhs.piece_bb(piece)
        })
        && lhs.king_square(Color::Black) == rhs.king_square(Color::Black)
        && lhs.king_square(Color::White) == rhs.king_square(Color::White)
        && lhs.checkers() == rhs.checkers()
        && lhs.pinners() == rhs.pinners()
        && lhs.pinned() == rhs.pinned()
        && lhs.key() == rhs.key()
}

#[test]
fn empty() {
    let pos = Position::empty();

    assert_eq!(pos.side_to_move(), Color::Black);
    assert_eq!(pos.occupancy(), Bitboard::empty());

    for piece in Piece::ALL {
        assert_eq!(pos.piece_bb(piece), Bitboard::empty());
    }

    assert_eq!(pos.king_square(Color::Black), None);
    assert_eq!(pos.king_square(Color::White), None);
    assert_eq!(pos.checkers(), Bitboard::empty());
    assert_eq!(pos.pinners(), Bitboard::empty());
    assert_eq!(pos.pinned(), Bitboard::empty());
    assert_eq!(pos.ply(), 0);
    assert_eq!(pos.key(), Key::default());
}

#[test]
fn startpos() {
    let pos = Position::startpos();

    assert_eq!(pos.side_to_move(), Color::Black);
    assert_eq!(pos.occupancy().count_ones(), 40);
    assert_eq!(pos.color_bb(Color::Black).count_ones(), 20);
    assert_eq!(pos.color_bb(Color::White).count_ones(), 20);
    assert_eq!(pos.piece_type_bb(PieceType::Pawn).count_ones(), 18);
    assert_eq!(pos.piece_type_bb(PieceType::Lance).count_ones(), 4);
    assert_eq!(pos.piece_type_bb(PieceType::Knight).count_ones(), 4);
    assert_eq!(pos.piece_type_bb(PieceType::Silver).count_ones(), 4);
    assert_eq!(pos.piece_type_bb(PieceType::Gold).count_ones(), 4);
    assert_eq!(pos.piece_type_bb(PieceType::Bishop).count_ones(), 2);
    assert_eq!(pos.piece_type_bb(PieceType::Rook).count_ones(), 2);
    assert_eq!(pos.piece_type_bb(PieceType::King).count_ones(), 2);

    assert_eq!(pos.king_square(Color::Black), Some(Square::S59));
    assert_eq!(pos.king_square(Color::White), Some(Square::S51));
    assert_eq!(pos.checkers(), Bitboard::empty());
    assert_eq!(pos.pinners(), Bitboard::empty());
    assert_eq!(pos.pinned(), Bitboard::empty());
    assert_eq!(pos.ply(), 0);
    assert_ne!(pos.key(), Key::default());
}

#[test]
fn default() {
    assert!(same_position(&Position::default(), &Position::empty()));
}

#[test]
fn builder() {
    let pos1 = {
        let mut builder = Position::startpos().builder();

        builder
            .remove(Square::S27)
            .place(Square::S26, Piece::BlackPawn)
            .set_side_to_move(Color::White)
            .remove(Square::S83)
            .place(Square::S84, Piece::WhitePawn)
            .set_side_to_move(Color::Black);

        builder.build()
    };

    let pos2 = {
        let mut builder = Position::default().builder();

        fn place(builder: &mut PositionBuilder, squares: &[Square], piece_type: PieceType) {
            for &square in squares {
                builder
                    .place(square, piece_type.with_color(Color::Black))
                    .place(square.rotate180(), piece_type.with_color(Color::White));
            }
        }

        // S27 -> S26
        const PAWN_SQUARES: [Square; 9] = [
            Square::S17,
            Square::S26,
            Square::S37,
            Square::S47,
            Square::S57,
            Square::S67,
            Square::S77,
            Square::S87,
            Square::S97,
        ];

        const LANCE_SQUARES: [Square; 2] = [Square::S19, Square::S99];
        const KNIGHT_SQUARES: [Square; 2] = [Square::S29, Square::S89];
        const SILVER_SQUARES: [Square; 2] = [Square::S39, Square::S79];
        const GOLD_SQUARES: [Square; 2] = [Square::S49, Square::S69];
        const BISHOP_SQUARE: [Square; 1] = [Square::S88];
        const ROOK_SQUARE: [Square; 1] = [Square::S28];
        const KING_SQUARE: [Square; 1] = [Square::S59];

        place(&mut builder, &PAWN_SQUARES, PieceType::Pawn);
        place(&mut builder, &LANCE_SQUARES, PieceType::Lance);
        place(&mut builder, &KNIGHT_SQUARES, PieceType::Knight);
        place(&mut builder, &SILVER_SQUARES, PieceType::Silver);
        place(&mut builder, &GOLD_SQUARES, PieceType::Gold);
        place(&mut builder, &BISHOP_SQUARE, PieceType::Bishop);
        place(&mut builder, &ROOK_SQUARE, PieceType::Rook);
        place(&mut builder, &KING_SQUARE, PieceType::King);

        builder.build()
    };

    assert!(same_position(&pos1, &pos2));
}

#[test]
fn make_move() {
    for (pos, mv, expected, expected_captured) in MAKE_MOVE_TEST_CASES {
        let mut pos = Usi::parse_position(pos).unwrap();
        let expected = Usi::parse_position(expected).unwrap();
        let captured = pos.make_move(Usi::parse_move(mv).unwrap());

        assert!(same_position(&pos, &expected));
        assert_eq!(captured, *expected_captured);
    }
}

#[test]
fn unmake_move() {
    for (pos, mv, _, _) in MAKE_MOVE_TEST_CASES {
        let expected = Usi::parse_position(pos).unwrap();
        let mut pos = expected.clone();
        let mv = Usi::parse_move(mv).unwrap();
        let captured = pos.make_move(mv);
        pos.unmake_move(mv, captured);

        assert!(same_position(&pos, &expected));
    }
}

#[test]
fn checkers() {
    let cases = [
        (
            {
                let mut builder = Position::empty().builder();

                builder
                    .place(Square::S11, Piece::WhiteKing)
                    .place(Square::S12, Piece::BlackPawn)
                    .set_side_to_move(Color::White);

                builder.build()
            },
            Square::S12.bit(),
        ),
        (
            {
                let mut builder = Position::empty().builder();

                builder
                    .place(Square::S99, Piece::BlackKing)
                    .place(Square::S98, Piece::WhitePawn);

                builder.build()
            },
            Square::S98.bit(),
        ),
        (
            {
                let mut builder = Position::empty().builder();

                builder
                    .place(Square::S11, Piece::WhiteKing)
                    .place(Square::S13, Piece::BlackLance)
                    .set_side_to_move(Color::White);

                builder.build()
            },
            Square::S13.bit(),
        ),
        (
            {
                let mut builder = Position::empty().builder();

                builder
                    .place(Square::S99, Piece::BlackKing)
                    .place(Square::S97, Piece::WhiteLance);

                builder.build()
            },
            Square::S97.bit(),
        ),
        (
            {
                let mut builder = Position::empty().builder();

                builder
                    .place(Square::S11, Piece::WhiteKing)
                    .place(Square::S19, Piece::BlackLance)
                    .place(Square::S23, Piece::BlackKnight)
                    .set_side_to_move(Color::White);

                builder.build()
            },
            Square::S19.bit() | Square::S23.bit(),
        ),
        (
            {
                let mut builder = Position::empty().builder();

                builder
                    .place(Square::S99, Piece::BlackKing)
                    .place(Square::S91, Piece::WhiteLance)
                    .place(Square::S87, Piece::WhiteKnight);

                builder.build()
            },
            Square::S91.bit() | Square::S87.bit(),
        ),
    ];

    for (pos, checkers) in cases {
        assert_eq!(pos.checkers(), checkers);
    }

    assert_eq!(PIN_TEST_POS1.checkers(), Bitboard::empty());
    assert_eq!(PIN_TEST_POS2.checkers(), Bitboard::empty());
    assert_eq!(PIN_TEST_POS3.checkers(), Bitboard::empty());
    assert_eq!(PIN_TEST_POS4.checkers(), Bitboard::empty());
}

#[test]
fn pinners() {
    assert_eq!(PIN_TEST_POS1.pinners(), Square::S13.bit());
    assert_eq!(PIN_TEST_POS2.pinners(), Square::S97.bit());
    assert_eq!(PIN_TEST_POS3.pinners(), Square::S13.bit());
    assert_eq!(PIN_TEST_POS4.pinners(), Square::S97.bit());
    assert_eq!(
        PIN_TEST_POS5.pinners(),
        Square::S13.bit() | Square::S41.bit()
    );
    assert_eq!(
        PIN_TEST_POS6.pinners(),
        Square::S97.bit() | Square::S69.bit()
    );
    assert_eq!(
        PIN_TEST_POS7.pinners(),
        Square::S13.bit() | Square::S41.bit() | Square::S19.bit() | Square::S61.bit()
    );
    assert_eq!(
        PIN_TEST_POS8.pinners(),
        Square::S97.bit() | Square::S69.bit() | Square::S91.bit() | Square::S49.bit()
    );
}

#[test]
fn pinned() {
    assert_eq!(PIN_TEST_POS1.pinned(), Square::S12.bit());
    assert_eq!(PIN_TEST_POS2.pinned(), Square::S98.bit());
    assert_eq!(PIN_TEST_POS3.pinned(), Square::S12.bit());
    assert_eq!(PIN_TEST_POS4.pinned(), Square::S98.bit());
    assert_eq!(
        PIN_TEST_POS5.pinned(),
        Square::S12.bit() | Square::S21.bit()
    );
    assert_eq!(
        PIN_TEST_POS6.pinned(),
        Square::S98.bit() | Square::S89.bit()
    );
    assert_eq!(
        PIN_TEST_POS7.pinned(),
        Square::S12.bit() | Square::S21.bit()
    );
    assert_eq!(
        PIN_TEST_POS8.pinned(),
        Square::S98.bit() | Square::S89.bit()
    );
}

#[test]
fn key() {
    let matsuri_key = piece_square_key(Piece::WhiteLance, Square::S11)
        ^ piece_square_key(Piece::WhiteKing, Square::S12)
        ^ piece_square_key(Piece::WhitePawn, Square::S14)
        ^ piece_square_key(Piece::BlackPawn, Square::S16)
        ^ piece_square_key(Piece::BlackLance, Square::S19)
        ^ piece_square_key(Piece::WhiteKnight, Square::S21)
        ^ piece_square_key(Piece::WhiteGold, Square::S22)
        ^ piece_square_key(Piece::BlackPawn, Square::S24)
        ^ piece_square_key(Piece::WhitePawn, Square::S25)
        ^ piece_square_key(Piece::BlackSilver, Square::S27)
        ^ piece_square_key(Piece::BlackKing, Square::S29)
        ^ piece_square_key(Piece::BlackSilver, Square::S35)
        ^ piece_square_key(Piece::BlackPawn, Square::S36)
        ^ piece_square_key(Piece::BlackGold, Square::S37)
        ^ piece_square_key(Piece::WhiteBishop, Square::S39)
        ^ piece_square_key(Piece::BlackProPawn, Square::S42)
        ^ piece_square_key(Piece::BlackSilver, Square::S43)
        ^ piece_square_key(Piece::WhitePawn, Square::S63)
        ^ piece_square_key(Piece::BlackPawn, Square::S65)
        ^ piece_square_key(Piece::WhiteBishop, Square::S66)
        ^ piece_square_key(Piece::WhiteKnight, Square::S73)
        ^ piece_square_key(Piece::WhitePawn, Square::S74)
        ^ piece_square_key(Piece::BlackPawn, Square::S76)
        ^ piece_square_key(Piece::BlackPawn, Square::S86)
        ^ piece_square_key(Piece::BlackKnight, Square::S89)
        ^ piece_square_key(Piece::WhiteLance, Square::S91)
        ^ piece_square_key(Piece::WhitePawn, Square::S94)
        ^ piece_square_key(Piece::BlackPawn, Square::S97)
        ^ piece_square_key(Piece::BlackRook, Square::S98)
        ^ piece_square_key(Piece::BlackLance, Square::S99)
        ^ hand_key(Color::Black, PieceType::Rook, 1)
        ^ hand_key(Color::Black, PieceType::Gold, 1)
        ^ hand_key(Color::White, PieceType::Gold, 1)
        ^ hand_key(Color::White, PieceType::Silver, 1)
        ^ hand_key(Color::White, PieceType::Knight, 1)
        ^ hand_key(Color::White, PieceType::Pawn, 5);

    assert_eq!(MATSURI_POS.key(), matsuri_key);
    assert_eq!(
        {
            let mut builder = MATSURI_POS.builder();

            builder.set_side_to_move(Color::White);

            builder.build()
        }
        .key(),
        matsuri_key ^ side_key()
    );
}

#[test]
fn display_empty() {
    let pos = Position::empty();

    let expected = "  9   8   7   6   5   4   3   2   1
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 一
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 二
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 三
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 四
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 五
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 六
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 七
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 八
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 九
+---+---+---+---+---+---+---+---+---+

Side to Move : Black
Hand (Black) : None
Hand (White) : None
Moves        : 0
Key          : 0";

    assert_eq!(pos.to_string(), expected);
}

#[test]
fn display_startpos() {
    let pos = Position::startpos();

    let expected = "  9   8   7   6   5   4   3   2   1
+---+---+---+---+---+---+---+---+---+
|w香|w桂|w銀|w金|w玉|w金|w銀|w桂|w香| 一
+---+---+---+---+---+---+---+---+---+
|   |w飛|   |   |   |   |   |w角|   | 二
+---+---+---+---+---+---+---+---+---+
|w歩|w歩|w歩|w歩|w歩|w歩|w歩|w歩|w歩| 三
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 四
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 五
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 六
+---+---+---+---+---+---+---+---+---+
|b歩|b歩|b歩|b歩|b歩|b歩|b歩|b歩|b歩| 七
+---+---+---+---+---+---+---+---+---+
|   |b角|   |   |   |   |   |b飛|   | 八
+---+---+---+---+---+---+---+---+---+
|b香|b桂|b銀|b金|b玉|b金|b銀|b桂|b香| 九
+---+---+---+---+---+---+---+---+---+

Side to Move : Black
Hand (Black) : None
Hand (White) : None
Moves        : 0
Key          : 88abfff4d6167b4";

    assert_eq!(pos.to_string(), expected);
}

#[test]
fn display_matsuri_pos() {
    let expected = "  9   8   7   6   5   4   3   2   1
+---+---+---+---+---+---+---+---+---+
|w香|   |   |   |   |   |   |w桂|w香| 一
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |bと|   |w金|w玉| 二
+---+---+---+---+---+---+---+---+---+
|   |   |w桂|w歩|   |b銀|   |   |   | 三
+---+---+---+---+---+---+---+---+---+
|w歩|   |w歩|   |   |   |   |b歩|w歩| 四
+---+---+---+---+---+---+---+---+---+
|   |   |   |b歩|   |   |b銀|w歩|   | 五
+---+---+---+---+---+---+---+---+---+
|   |b歩|b歩|w角|   |   |b歩|   |b歩| 六
+---+---+---+---+---+---+---+---+---+
|b歩|   |   |   |   |   |b金|b銀|   | 七
+---+---+---+---+---+---+---+---+---+
|b飛|   |   |   |   |   |   |   |   | 八
+---+---+---+---+---+---+---+---+---+
|b香|b桂|   |   |   |   |w角|b玉|b香| 九
+---+---+---+---+---+---+---+---+---+

Side to Move : Black
Hand (Black) : 飛, 金
Hand (White) : 金, 銀, 桂, 歩x5
Moves        : 0
Key          : 5ed2639a48bb4076";

    assert_eq!(MATSURI_POS.to_string(), expected);
}
