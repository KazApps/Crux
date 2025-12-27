use crux_lib::shogi::{
    core::{PieceType, Square},
    position::{hand::Hand, mv::Move},
};

#[test]
fn null() {
    let mv = Move::null();

    assert!(mv.is_special());
    assert!(!mv.is_promotion());
    assert!(!mv.is_drop());
}

#[test]
fn win() {
    let mv = Move::win();

    assert!(mv.is_special());
    assert!(!mv.is_promotion());
    assert!(!mv.is_drop());
}

#[test]
fn resign() {
    let mv = Move::resign();

    assert!(mv.is_special());
    assert!(!mv.is_promotion());
    assert!(!mv.is_drop());
}

#[test]
fn normal() {
    for from in 0..Square::COUNT {
        for to in 0..Square::COUNT {
            let from = Square::from(from);
            let to = Square::from(to);

            if from == to {
                continue;
            }

            let mv = Move::normal(from, to);

            assert_eq!(mv.from(), from);
            assert_eq!(mv.to(), to);
            assert!(!mv.is_special());
            assert!(!mv.is_promotion());
            assert!(!mv.is_drop());
        }
    }
}

#[test]
fn promote() {
    for from in 0..Square::COUNT {
        for to in 0..Square::COUNT {
            let from = Square::from(from);
            let to = Square::from(to);

            if from == to {
                continue;
            }

            let mv = Move::promote(from, to);

            assert_eq!(mv.from(), from);
            assert_eq!(mv.to(), to);
            assert!(!mv.is_special());
            assert!(mv.is_promotion());
            assert!(!mv.is_drop());
        }
    }
}

#[test]
fn drop() {
    for piece_type in 0..Hand::HAND_PIECE_TYPES {
        for to in 0..Square::COUNT {
            let piece_type = PieceType::from(piece_type);
            let to = Square::from(to);

            let mv = Move::drop(piece_type, to);

            assert_eq!(mv.to(), to);
            assert!(!mv.is_special());
            assert!(!mv.is_promotion());
            assert!(mv.is_drop());
        }
    }
}
