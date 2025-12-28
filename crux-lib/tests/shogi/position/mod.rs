use const_for::const_for;

use crux_lib::shogi::{
    bitboard::Bitboard,
    core::{Color, Piece, PieceType, Square},
    position::{key::Key, Position, PositionBuilder},
};

mod hand;
mod key;
mod mv;
mod zobrist;

const PIN_CHECK_POS1: Position = {
    let mut builder = Position::empty().builder();

    builder
        .place(Square::S11, Piece::WhiteKing)
        .place(Square::S12, Piece::WhitePawn)
        .place(Square::S13, Piece::BlackLance)
        .set_side_to_move(Color::White);

    builder.build()
};

const PIN_CHECK_POS2: Position = {
    let mut builder = PIN_CHECK_POS1.builder();

    builder
        .place(Square::S22, Piece::BlackPawn)
        .place(Square::S44, Piece::BlackBishop);

    builder.build()
};

const PIN_CHECK_POS3: Position = {
    let mut builder = PIN_CHECK_POS2.builder();

    builder
        .place(Square::S21, Piece::WhiteKnight)
        .place(Square::S41, Piece::BlackRook);

    builder.build()
};

const PIN_CHECK_POS4: Position = {
    let mut builder = PIN_CHECK_POS3.builder();

    builder
        .place(Square::S19, Piece::BlackLance)
        .place(Square::S55, Piece::BlackHorse)
        .place(Square::S61, Piece::BlackDragon);

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
        && lhs.ply() == rhs.ply()
        && lhs.key() == rhs.key()
}

#[test]
fn empty() {
    let pos = Position::empty();

    assert_eq!(pos.side_to_move(), Color::Black);
    assert_eq!(pos.occupancy(), Bitboard::empty());

    for piece in 0..Piece::COUNT {
        let piece = Piece::from(piece);

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
    let mut builder = Position::startpos().builder();

    builder
        .remove(Square::S27)
        .place(Square::S26, Piece::BlackPawn)
        .set_side_to_move(Color::White)
        .remove(Square::S83)
        .place(Square::S84, Piece::WhitePawn)
        .set_side_to_move(Color::Black);

    let pos1 = builder.build();

    builder = Position::default().builder();

    const fn place(builder: &mut PositionBuilder, squares: &[Square], piece_type: PieceType) {
        const_for!(i in 0..squares.len() => {
            builder
                .place(squares[i], piece_type.with_color(Color::Black))
                .place(squares[i].rotate180(), piece_type.with_color(Color::White));
        });
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

    let pos2 = builder.build();

    assert!(same_position(&pos1, &pos2));
}

#[test]
fn pinners() {
    assert_eq!(PIN_CHECK_POS1.pinners(), Square::S13.bit());
    assert_eq!(PIN_CHECK_POS2.pinners(), Square::S13.bit());
    assert_eq!(
        PIN_CHECK_POS3.pinners(),
        Square::S13.bit() | Square::S41.bit()
    );
    assert_eq!(
        PIN_CHECK_POS4.pinners(),
        Square::S13.bit() | Square::S41.bit() | Square::S19.bit() | Square::S61.bit()
    );
}

#[test]
fn pinned() {
    assert_eq!(PIN_CHECK_POS1.pinned(), Square::S12.bit());
    assert_eq!(PIN_CHECK_POS2.pinned(), Square::S12.bit());
    assert_eq!(
        PIN_CHECK_POS3.pinned(),
        Square::S12.bit() | Square::S21.bit()
    );
    assert_eq!(
        PIN_CHECK_POS4.pinned(),
        Square::S12.bit() | Square::S21.bit()
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

    let pos = builder.build();

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

    assert_eq!(pos.to_string(), expected);
}
