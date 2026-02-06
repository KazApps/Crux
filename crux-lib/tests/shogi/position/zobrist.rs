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

    for piece in Piece::ALL {
        for square in Square::ALL {
            assert!(seen.insert(piece_square_key(piece, square).value()));
        }
    }

    for color in Color::ALL {
        for &piece_type in PieceType::ALL.iter().take(Hand::HAND_PIECE_TYPES) {
            for count in 0..=Hand::max_piece_counts(piece_type) {
                if count == 0 {
                    assert_eq!(hand_key(color, piece_type, count).value(), 0);
                } else {
                    assert!(seen.insert(hand_key(color, piece_type, count).value()));
                }
            }
        }
    }
}
