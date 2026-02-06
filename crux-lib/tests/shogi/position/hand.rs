use crux_lib::shogi::{core::PieceType, position::hand::Hand};

#[test]
fn default() {
    let hand = Hand::default();

    assert!(hand.is_empty());
}

#[test]
fn has_any() {
    for &piece_type in PieceType::ALL.iter().take(Hand::HAND_PIECE_TYPES) {
        let mut hand = Hand::default();
        assert!(!hand.has_any());
        hand.increment(piece_type);
        assert!(hand.has_any());
    }
}

#[test]
fn is_empty() {
    for &piece_type in PieceType::ALL.iter().take(Hand::HAND_PIECE_TYPES) {
        let mut hand = Hand::default();
        assert!(hand.is_empty());
        hand.increment(piece_type);
        assert!(!hand.is_empty());
    }
}

#[test]
fn count() {
    let mut hand = Hand::default();

    for &piece_type in PieceType::ALL.iter().take(Hand::HAND_PIECE_TYPES) {
        assert_eq!(hand.count(piece_type), 0);
    }

    hand.increment(PieceType::Pawn);
    assert_eq!(hand.count(PieceType::Pawn), 1);

    hand.set(PieceType::Gold, 2);

    assert_eq!(hand.count(PieceType::Pawn), 1);
    assert_eq!(hand.count(PieceType::Gold), 2);

    hand.decrement(PieceType::Gold);
    assert_eq!(hand.count(PieceType::Gold), 1);
}

#[test]
fn set() {
    let mut hand = Hand::default();

    for &piece_type in PieceType::ALL.iter().take(Hand::HAND_PIECE_TYPES) {
        let max_count = Hand::max_piece_counts(piece_type);

        hand.set(piece_type, max_count);
        assert_eq!(hand.count(piece_type), max_count);
    }
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn set_panics_if_count_is_greater_than_max() {
    let mut hand = Hand::default();
    hand.set(PieceType::Pawn, Hand::max_piece_counts(PieceType::Pawn) + 1);
}

#[test]
fn increment() {
    let mut hand = Hand::default();

    for &piece_type in PieceType::ALL.iter().take(Hand::HAND_PIECE_TYPES) {
        hand.increment(piece_type);
        assert_eq!(hand.count(piece_type), 1);
    }
}

#[test]
fn decrement() {
    let mut hand = Hand::default();

    for &piece_type in PieceType::ALL.iter().take(Hand::HAND_PIECE_TYPES) {
        let max_count = Hand::max_piece_counts(piece_type);

        hand.set(piece_type, max_count);
        hand.decrement(piece_type);
        assert_eq!(hand.count(piece_type), max_count - 1);
    }
}
