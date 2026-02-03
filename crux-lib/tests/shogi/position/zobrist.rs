use std::collections::HashSet;

use crux_lib::shogi::{
    core::{Color, Piece, PieceType, Square},
    position::{
        hand::Hand,
        zobrist::{hand_key, piece_square_key, side_key},
    },
};

#[test]
fn keys_are_unique() {
    let mut seen = HashSet::new();

    assert!(seen.insert(side_key().value()));

    for piece in 0..Piece::COUNT {
        for square in 0..Square::COUNT {
            let piece = Piece::from(piece);
            let square = Square::from(square);

            assert!(seen.insert(piece_square_key(piece, square).value()));
        }
    }

    for color in 0..Color::COUNT {
        for piece_type in 0..Hand::HAND_PIECE_TYPES {
            let piece_type = PieceType::from(piece_type);

            for count in 0..=Hand::max_piece_counts(piece_type) {
                let color = Color::from(color);

                if count == 0 {
                    assert_eq!(hand_key(color, piece_type, count).value(), 0);
                } else {
                    assert!(seen.insert(hand_key(color, piece_type, count).value()));
                }
            }
        }
    }
}
