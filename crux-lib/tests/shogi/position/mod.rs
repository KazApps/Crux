use crux_lib::shogi::{
    bitboard::Bitboard,
    core::{Color, Piece, PieceType, Square},
    position::{
        key::Key,
        mv::Move,
        zobrist::{hand_key, piece_square_key, side_key},
        Position, PositionBuilder,
    },
};

mod hand;
mod key;
mod mv;
mod zobrist;

const MAKE_MOVE_TEST_CASES: &[(Position, Move, Position, Option<Piece>)] = &[
    // Normal move
    (
        {
            let mut builder = Position::empty().builder();

            builder.place(Square::S55, Piece::BlackRook);

            builder.build()
        },
        Move::normal(Square::S55, Square::S51),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S51, Piece::BlackRook)
                .set_side_to_move(Color::White);

            builder.build()
        },
        None,
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S55, Piece::WhiteRook)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::normal(Square::S55, Square::S59),
        {
            let mut builder = Position::empty().builder();

            builder.place(Square::S59, Piece::WhiteRook);

            builder.build()
        },
        None,
    ),
    // Promotion move
    (
        {
            let mut builder = Position::empty().builder();

            builder.place(Square::S55, Piece::BlackRook);

            builder.build()
        },
        Move::promote(Square::S55, Square::S51),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S51, Piece::BlackDragon)
                .set_side_to_move(Color::White);

            builder.build()
        },
        None,
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S55, Piece::WhiteRook)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::promote(Square::S55, Square::S59),
        {
            let mut builder = Position::empty().builder();

            builder.place(Square::S59, Piece::WhiteDragon);

            builder.build()
        },
        None,
    ),
    // Drop move
    (
        {
            let mut builder = Position::empty().builder();

            builder.increment_hand_piece_count(Color::Black, PieceType::Pawn);

            builder.build()
        },
        Move::drop(PieceType::Pawn, Square::S55),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S55, Piece::BlackPawn)
                .set_side_to_move(Color::White);

            builder.build()
        },
        None,
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .increment_hand_piece_count(Color::White, PieceType::Pawn)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::drop(PieceType::Pawn, Square::S55),
        {
            let mut builder = Position::empty().builder();

            builder.place(Square::S55, Piece::WhitePawn);

            builder.build()
        },
        None,
    ),
    // Normal move with capture
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S51, Piece::WhitePawn)
                .place(Square::S55, Piece::BlackRook);

            builder.build()
        },
        Move::normal(Square::S55, Square::S51),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S51, Piece::BlackRook)
                .increment_hand_piece_count(Color::Black, PieceType::Pawn)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Some(Piece::WhitePawn),
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S59, Piece::BlackPawn)
                .place(Square::S55, Piece::WhiteRook)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::normal(Square::S55, Square::S59),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S59, Piece::WhiteRook)
                .increment_hand_piece_count(Color::White, PieceType::Pawn);

            builder.build()
        },
        Some(Piece::BlackPawn),
    ),
    // Promotion move with capture
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S51, Piece::WhitePawn)
                .place(Square::S55, Piece::BlackRook);

            builder.build()
        },
        Move::promote(Square::S55, Square::S51),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S51, Piece::BlackDragon)
                .increment_hand_piece_count(Color::Black, PieceType::Pawn)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Some(Piece::WhitePawn),
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S59, Piece::BlackPawn)
                .place(Square::S55, Piece::WhiteRook)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::promote(Square::S55, Square::S59),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S59, Piece::WhiteDragon)
                .increment_hand_piece_count(Color::White, PieceType::Pawn);

            builder.build()
        },
        Some(Piece::BlackPawn),
    ),
    // Normal move gives check
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S11, Piece::WhiteKing)
                .place(Square::S19, Piece::BlackLance)
                .place(Square::S15, Piece::BlackKnight);

            builder.build()
        },
        Move::normal(Square::S15, Square::S23),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S11, Piece::WhiteKing)
                .place(Square::S19, Piece::BlackLance)
                .place(Square::S23, Piece::BlackKnight)
                .set_side_to_move(Color::White);

            builder.build()
        },
        None,
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S99, Piece::BlackKing)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S95, Piece::WhiteKnight)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::normal(Square::S95, Square::S87),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S99, Piece::BlackKing)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S87, Piece::WhiteKnight);

            builder.build()
        },
        None,
    ),
    // Promotion move gives check
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S12, Piece::WhiteKing)
                .place(Square::S19, Piece::BlackLance)
                .place(Square::S15, Piece::BlackKnight);

            builder.build()
        },
        Move::promote(Square::S15, Square::S23),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S12, Piece::WhiteKing)
                .place(Square::S19, Piece::BlackLance)
                .place(Square::S23, Piece::BlackProKnight)
                .set_side_to_move(Color::White);

            builder.build()
        },
        None,
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S98, Piece::BlackKing)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S95, Piece::WhiteKnight)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::promote(Square::S95, Square::S87),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S98, Piece::BlackKing)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S87, Piece::WhiteProKnight);

            builder.build()
        },
        None,
    ),
    // Drop move gives check
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S54, Piece::WhiteKing)
                .increment_hand_piece_count(Color::Black, PieceType::Pawn);

            builder.build()
        },
        Move::drop(PieceType::Pawn, Square::S55),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S54, Piece::WhiteKing)
                .place(Square::S55, Piece::BlackPawn)
                .set_side_to_move(Color::White);

            builder.build()
        },
        None,
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S56, Piece::BlackKing)
                .increment_hand_piece_count(Color::White, PieceType::Pawn)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::drop(PieceType::Pawn, Square::S55),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S56, Piece::BlackKing)
                .place(Square::S55, Piece::WhitePawn);

            builder.build()
        },
        None,
    ),
    // Normal move with capture gives check
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S11, Piece::WhiteKing)
                .place(Square::S19, Piece::BlackLance)
                .place(Square::S15, Piece::BlackKnight)
                .place(Square::S23, Piece::WhitePawn);

            builder.build()
        },
        Move::normal(Square::S15, Square::S23),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S11, Piece::WhiteKing)
                .place(Square::S19, Piece::BlackLance)
                .place(Square::S23, Piece::BlackKnight)
                .increment_hand_piece_count(Color::Black, PieceType::Pawn)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Some(Piece::WhitePawn),
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S99, Piece::BlackKing)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S95, Piece::WhiteKnight)
                .place(Square::S87, Piece::BlackPawn)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::normal(Square::S95, Square::S87),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S99, Piece::BlackKing)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S87, Piece::WhiteKnight)
                .increment_hand_piece_count(Color::White, PieceType::Pawn);

            builder.build()
        },
        Some(Piece::BlackPawn),
    ),
    // Promotion move with capture gives check
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S12, Piece::WhiteKing)
                .place(Square::S19, Piece::BlackLance)
                .place(Square::S15, Piece::BlackKnight)
                .place(Square::S23, Piece::WhitePawn);

            builder.build()
        },
        Move::promote(Square::S15, Square::S23),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S12, Piece::WhiteKing)
                .place(Square::S19, Piece::BlackLance)
                .place(Square::S23, Piece::BlackProKnight)
                .increment_hand_piece_count(Color::Black, PieceType::Pawn)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Some(Piece::WhitePawn),
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S98, Piece::BlackKing)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S95, Piece::WhiteKnight)
                .place(Square::S87, Piece::BlackPawn)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::promote(Square::S95, Square::S87),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S98, Piece::BlackKing)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S87, Piece::WhiteProKnight)
                .increment_hand_piece_count(Color::White, PieceType::Pawn);

            builder.build()
        },
        Some(Piece::BlackPawn),
    ),
    // Evade check by moving the king
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S99, Piece::BlackKing)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S87, Piece::WhiteKnight);

            builder.build()
        },
        Move::normal(Square::S99, Square::S88),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S88, Piece::BlackKing)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S87, Piece::WhiteKnight)
                .set_side_to_move(Color::White);

            builder.build()
        },
        None,
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
        Move::normal(Square::S11, Square::S22),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S22, Piece::WhiteKing)
                .place(Square::S19, Piece::BlackLance)
                .place(Square::S23, Piece::BlackKnight);

            builder.build()
        },
        None,
    ),
    // Evade check by moving the king with capture
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S98, Piece::BlackKing)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S87, Piece::WhiteProKnight);

            builder.build()
        },
        Move::normal(Square::S98, Square::S87),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S87, Piece::BlackKing)
                .place(Square::S91, Piece::WhiteLance)
                .increment_hand_piece_count(Color::Black, PieceType::Knight)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Some(Piece::WhiteProKnight),
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S12, Piece::WhiteKing)
                .place(Square::S19, Piece::BlackLance)
                .place(Square::S23, Piece::BlackProKnight)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::normal(Square::S12, Square::S23),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S23, Piece::WhiteKing)
                .place(Square::S19, Piece::BlackLance)
                .increment_hand_piece_count(Color::White, PieceType::Knight);

            builder.build()
        },
        Some(Piece::BlackProKnight),
    ),
    // Evade check by capturing
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S99, Piece::BlackKing)
                .place(Square::S81, Piece::BlackRook)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S87, Piece::WhiteProKnight);

            builder.build()
        },
        Move::normal(Square::S81, Square::S91),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S99, Piece::BlackKing)
                .place(Square::S91, Piece::BlackRook)
                .place(Square::S87, Piece::WhiteProKnight)
                .increment_hand_piece_count(Color::Black, PieceType::Lance)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Some(Piece::WhiteLance),
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S11, Piece::WhiteKing)
                .place(Square::S29, Piece::WhiteRook)
                .place(Square::S19, Piece::BlackLance)
                .place(Square::S23, Piece::BlackProKnight)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::normal(Square::S29, Square::S19),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S11, Piece::WhiteKing)
                .place(Square::S19, Piece::WhiteRook)
                .place(Square::S23, Piece::BlackProKnight)
                .increment_hand_piece_count(Color::White, PieceType::Lance);

            builder.build()
        },
        Some(Piece::BlackLance),
    ),
    // Evade check by capturing promotion
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S99, Piece::BlackKing)
                .place(Square::S81, Piece::BlackRook)
                .place(Square::S91, Piece::WhiteLance)
                .place(Square::S87, Piece::WhiteProKnight);

            builder.build()
        },
        Move::promote(Square::S81, Square::S91),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S99, Piece::BlackKing)
                .place(Square::S91, Piece::BlackDragon)
                .place(Square::S87, Piece::WhiteProKnight)
                .increment_hand_piece_count(Color::Black, PieceType::Lance)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Some(Piece::WhiteLance),
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S11, Piece::WhiteKing)
                .place(Square::S29, Piece::WhiteRook)
                .place(Square::S19, Piece::BlackLance)
                .place(Square::S23, Piece::BlackProKnight)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::promote(Square::S29, Square::S19),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S11, Piece::WhiteKing)
                .place(Square::S19, Piece::WhiteDragon)
                .place(Square::S23, Piece::BlackProKnight)
                .increment_hand_piece_count(Color::White, PieceType::Lance);

            builder.build()
        },
        Some(Piece::BlackLance),
    ),
    // Checkers and pins test
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S11, Piece::WhiteKing)
                .place(Square::S12, Piece::WhitePawn)
                .place(Square::S22, Piece::WhitePawn)
                .place(Square::S33, Piece::BlackRook)
                .place(Square::S44, Piece::BlackBishop);

            builder.build()
        },
        Move::normal(Square::S33, Square::S13),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S11, Piece::WhiteKing)
                .place(Square::S12, Piece::WhitePawn)
                .place(Square::S22, Piece::WhitePawn)
                .place(Square::S13, Piece::BlackRook)
                .place(Square::S44, Piece::BlackBishop)
                .set_side_to_move(Color::White);

            builder.build()
        },
        None,
    ),
    (
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S99, Piece::BlackKing)
                .place(Square::S98, Piece::BlackPawn)
                .place(Square::S88, Piece::BlackPawn)
                .place(Square::S77, Piece::WhiteRook)
                .place(Square::S66, Piece::WhiteBishop)
                .set_side_to_move(Color::White);

            builder.build()
        },
        Move::normal(Square::S77, Square::S97),
        {
            let mut builder = Position::empty().builder();

            builder
                .place(Square::S99, Piece::BlackKing)
                .place(Square::S98, Piece::BlackPawn)
                .place(Square::S88, Piece::BlackPawn)
                .place(Square::S97, Piece::WhiteRook)
                .place(Square::S66, Piece::WhiteBishop);

            builder.build()
        },
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
        let mut new_pos = pos.clone();
        let captured = new_pos.make_move(*mv);

        assert!(same_position(&new_pos, expected));
        assert_eq!(captured, *expected_captured);
    }
}

#[test]
fn unmake_move() {
    for (pos, mv, _, _) in MAKE_MOVE_TEST_CASES {
        let mut new_pos = pos.clone();
        let captured = new_pos.make_move(*mv);
        new_pos.unmake_move(*mv, captured);

        assert!(same_position(&new_pos, pos));
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
