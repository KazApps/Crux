use crux_lib::shogi::{
    core::{PieceType, Square},
    position::{hand::Hand, mv::Move},
};

#[test]
fn normal() {
    for from in Square::ALL {
        for to in Square::ALL {
            if from == to {
                continue;
            }

            let mv = Move::normal(from, to);

            assert_eq!(mv.from(), from);
            assert_eq!(mv.to(), to);
            assert!(!mv.is_promotion());
            assert!(!mv.is_drop());
        }
    }
}

#[test]
fn promote() {
    for from in Square::ALL {
        for to in Square::ALL {
            if from == to {
                continue;
            }

            let mv = Move::promote(from, to);

            assert_eq!(mv.from(), from);
            assert_eq!(mv.to(), to);
            assert!(mv.is_promotion());
            assert!(!mv.is_drop());
        }
    }
}

#[test]
fn drop() {
    for &piece_type in PieceType::ALL.iter().take(Hand::HAND_PIECE_TYPES) {
        for to in Square::ALL {
            let mv = Move::drop(piece_type, to);

            assert_eq!(mv.to(), to);
            assert!(!mv.is_promotion());
            assert!(mv.is_drop());
        }
    }
}
