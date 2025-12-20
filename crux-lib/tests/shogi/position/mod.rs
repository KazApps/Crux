use const_for::const_for;
use crux_lib::shogi::{
    bitboard::Bitboard,
    core::{Color, Piece, PieceType, Square},
    position::{key::Key, Position, PositionBuilder},
};

mod hand;
mod key;
mod zobrist;

#[test]
fn default() {
    let pos = Position::default();

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

    assert_eq!(pos1, pos2);
}
