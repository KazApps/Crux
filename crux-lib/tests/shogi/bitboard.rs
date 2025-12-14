use rand::{rngs::StdRng, Rng, SeedableRng};

use crux_lib::shogi::{
    bitboard::{
        pawn_drop_mask as pawn_drop_mask_fn, promotion_area as promotion_area_fn, Bitboard,
    },
    core::{Color, File, Rank, Square},
};

#[test]
fn all() {
    let mut bb = Bitboard::all();

    assert_eq!(bb.count_ones(), 81);

    let mut square = 0u8;

    while bb.has_any() {
        let actual = bb.pop_lsb();

        assert_eq!(actual, Square::from(square));

        square += 1;
    }
}

#[test]
fn empty() {
    let empty_bb = Bitboard::empty();

    assert_eq!(empty_bb.count_ones(), 0);
    assert!(empty_bb.is_empty());
}

#[test]
fn default() {
    assert_eq!(Bitboard::default(), Bitboard::empty());
}

#[test]
fn has_any() {
    let cases = [
        (Bitboard::empty(), false),
        (Bitboard::all(), true),
        (Square::S11.bit(), true),
        (Square::S55.bit(), true),
        (Square::S99.bit(), true),
        (
            Square::S11.bit() | Square::S55.bit() | Square::S99.bit(),
            true,
        ),
    ];

    for (bb, has_any) in cases.iter() {
        assert_eq!(bb.has_any(), *has_any);
    }
}

#[test]
fn is_empty() {
    let cases = [
        (Bitboard::empty(), true),
        (Bitboard::all(), false),
        (Square::S11.bit(), false),
        (Square::S55.bit(), false),
        (Square::S99.bit(), false),
        (
            Square::S11.bit() | Square::S55.bit() | Square::S99.bit(),
            false,
        ),
    ];

    for (bb, is_empty) in cases.iter() {
        assert_eq!(bb.is_empty(), *is_empty);
    }
}

#[test]
fn is_single() {
    let cases = [
        (Bitboard::empty(), false),
        (Bitboard::all(), false),
        (Square::S11.bit(), true),
        (Square::S55.bit(), true),
        (Square::S99.bit(), true),
        (
            Square::S11.bit() | Square::S55.bit() | Square::S99.bit(),
            false,
        ),
    ];

    for (bb, is_single) in cases.iter() {
        assert_eq!(bb.is_single(), *is_single);
    }
}

#[test]
fn is_multiple() {
    let cases = [
        (Bitboard::empty(), false),
        (Bitboard::all(), true),
        (Square::S11.bit(), false),
        (Square::S55.bit(), false),
        (Square::S99.bit(), false),
        (
            Square::S11.bit() | Square::S55.bit() | Square::S99.bit(),
            true,
        ),
    ];

    for (bb, is_multiple) in cases.iter() {
        assert_eq!(bb.is_multiple(), *is_multiple);
    }
}

#[test]
fn count_ones() {
    let cases = [
        (Bitboard::empty(), 0),
        (Bitboard::all(), 81),
        (Square::S11.bit(), 1),
        (Square::S55.bit(), 1),
        (Square::S99.bit(), 1),
        (Square::S11.bit() | Square::S55.bit() | Square::S99.bit(), 3),
    ];

    for (bb, ones) in cases.iter() {
        assert_eq!(bb.count_ones(), *ones);
    }
}

#[test]
fn lsb() {
    let cases = [
        (Bitboard::all(), Square::S11),
        (Square::S11.bit(), Square::S11),
        (Square::S55.bit(), Square::S55),
        (Square::S99.bit(), Square::S99),
        (
            Square::S11.bit() | Square::S55.bit() | Square::S99.bit(),
            Square::S11,
        ),
    ];

    for (bb, lsb) in cases.iter() {
        assert_eq!(bb.lsb(), *lsb);
    }
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn lsb_panics_on_empty() {
    let _ = Bitboard::empty().lsb();
}

#[test]
fn isolate_lsb() {
    let cases = [
        (Bitboard::all(), Square::S11.bit()),
        (Square::S11.bit(), Square::S11.bit()),
        (Square::S55.bit(), Square::S55.bit()),
        (Square::S99.bit(), Square::S99.bit()),
        (
            Square::S11.bit() | Square::S55.bit() | Square::S99.bit(),
            Square::S11.bit(),
        ),
    ];

    for (bb, lsb_bb) in cases.iter() {
        assert_eq!(bb.isolate_lsb(), *lsb_bb);
    }
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn isolate_lsb_panics_on_empty() {
    let _ = Bitboard::empty().isolate_lsb();
}

#[test]
fn pop_lsb() {
    let mut bb = Square::S11.bit() | Square::S55.bit() | Square::S99.bit();

    assert_eq!(bb.pop_lsb(), Square::S11);
    assert_eq!(bb, Square::S55.bit() | Square::S99.bit());
    assert_eq!(bb.pop_lsb(), Square::S55);
    assert_eq!(bb, Square::S99.bit());
    assert_eq!(bb.pop_lsb(), Square::S99);
    assert_eq!(bb, Bitboard::empty());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn pop_panics_on_empty() {
    let _ = Bitboard::empty().pop_lsb();
}

#[test]
fn bit_ops() {
    assert_eq!(
        (File::File1.bit() & Rank::Rank1.bit()
            | Rank::Rank2.bit()
            | File::File9.bit()
            | Square::S55.bit())
            & !File::File9.bit()
            & !Rank::Rank2.bit(),
        Square::S11.bit() | Square::S55.bit()
    )
}

#[test]
fn bit_ops_assign() {
    let mut bb = Bitboard::all();
    bb &= File::File1.bit();
    bb &= Rank::Rank1.bit();
    bb |= File::File9.bit();
    bb |= Square::S55.bit();
    bb &= !File::File9.bit();
    bb &= !Rank::Rank2.bit();

    assert_eq!(bb, Square::S11.bit() | Square::S55.bit());
}

#[test]
fn from_file() {
    for file in 0..File::COUNT {
        let file = File::from(file);
        let mut bb = file.bit();

        assert_eq!(bb.count_ones(), 9);

        let mut rank = 0u8;

        while bb.has_any() {
            let square = bb.pop_lsb();

            assert_eq!(square.file(), file);
            assert_eq!(square.rank(), Rank::from(rank));

            rank += 1;
        }
    }
}

#[test]
fn from_rank() {
    for rank in 0..Rank::COUNT {
        let rank = Rank::from(rank);
        let mut bb = rank.bit();

        assert_eq!(bb.count_ones(), 9);

        let mut file = 0u8;

        while bb.has_any() {
            let square = bb.pop_lsb();

            assert_eq!(square.file(), File::from(file));
            assert_eq!(square.rank(), rank);

            file += 1;
        }
    }
}

#[test]
fn from_square() {
    for square in 0..Square::COUNT {
        let square = Square::from(square);
        let mut bb = square.bit();

        assert_eq!(bb.count_ones(), 1);
        assert_eq!(bb.pop_lsb(), square);
    }
}

#[test]
fn promotion_area() {
    for color in 0..Color::COUNT {
        let color = Color::from(color);

        let mut promotion_are = promotion_area_fn(color);

        while promotion_are.has_any() {
            let square = promotion_are.pop_lsb();

            assert!(square.can_promote(color));
        }
    }
}

#[test]
fn pawn_drop_mask() {
    let sample_count = 100_000;
    let seed = 42;
    let mut rng = StdRng::seed_from_u64(seed);

    for color in 0..Color::COUNT {
        let color = Color::from(color);

        for _ in 0..sample_count {
            let ranks: [u8; 9] = [
                rng.random_range(0..9),
                rng.random_range(0..9),
                rng.random_range(0..9),
                rng.random_range(0..9),
                rng.random_range(0..9),
                rng.random_range(0..9),
                rng.random_range(0..9),
                rng.random_range(0..9),
                rng.random_range(0..9),
            ];

            let mut bb = Bitboard::empty();
            let mut expected = Bitboard::empty();

            for (i, rank) in ranks.iter().enumerate() {
                let file = File::from(i);

                if Rank::from(*rank) != Rank::Rank1.relative(color) {
                    bb |= Square::new(file, Rank::from(*rank)).bit();
                } else {
                    expected |= file.bit();
                }
            }

            expected &= !Rank::Rank1.relative(color).bit();

            assert_eq!(pawn_drop_mask_fn(color, bb), expected);
        }
    }
}

#[cfg_attr(debug_assertions, ignore)]
#[test]
fn pawn_drop_mask_all() {
    fn recurse(color: Color, first_rank: Rank, depth: usize, current: &mut [u8; 9]) {
        if depth == 9 {
            let mut bb = Bitboard::empty();
            let mut expected = Bitboard::empty();

            for (i, rank) in current.iter().enumerate() {
                let file = File::from(i);
                let rank = Rank::from(*rank);

                if rank != first_rank {
                    bb |= Square::new(file, rank).bit();
                } else {
                    expected |= file.bit();
                }
            }

            expected &= !first_rank.bit();
            assert_eq!(pawn_drop_mask_fn(color, bb), expected);
            return;
        }

        for rank in 0..9 {
            current[depth] = rank;
            recurse(color, first_rank, depth + 1, current);
        }
    }

    for color_idx in 0..Color::COUNT {
        let color = Color::from(color_idx);
        let first_rank = if color == Color::Black {
            Rank::Rank1
        } else {
            Rank::Rank9
        };

        let mut current = [0u8; 9];
        recurse(color, first_rank, 0, &mut current);
    }
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn pawn_drop_mask_black_panics_on_doubled_pawns() {
    let _ = pawn_drop_mask_fn(Color::Black, Rank::Rank9.bit() | Square::S55.bit());
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn pawn_drop_mask_white_panics_on_doubled_pawns() {
    let _ = pawn_drop_mask_fn(Color::White, Rank::Rank9.bit() | Square::S55.bit());
}

#[test]
fn display_empty_board() {
    let bb = Bitboard::empty();

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
+---+---+---+---+---+---+---+---+---+";

    assert_eq!(bb.to_string(), expected);
}

#[test]
fn display_board_single_square() {
    let bb = Square::S55.bit();

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
|   |   |   |   | X |   |   |   |   | 五
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 六
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 七
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 八
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   |   |   |   |   | 九
+---+---+---+---+---+---+---+---+---+";

    assert_eq!(bb.to_string(), expected);
}

#[test]
fn display_board_multiple_squares() {
    let bb = File::File4.bit()
        | Rank::Rank2.bit()
        | Square::S11.bit()
        | Square::S55.bit()
        | Square::S99.bit();

    let expected = "  9   8   7   6   5   4   3   2   1
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   | X |   |   | X | 一
+---+---+---+---+---+---+---+---+---+
| X | X | X | X | X | X | X | X | X | 二
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   | X |   |   |   | 三
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   | X |   |   |   | 四
+---+---+---+---+---+---+---+---+---+
|   |   |   |   | X | X |   |   |   | 五
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   | X |   |   |   | 六
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   | X |   |   |   | 七
+---+---+---+---+---+---+---+---+---+
|   |   |   |   |   | X |   |   |   | 八
+---+---+---+---+---+---+---+---+---+
| X |   |   |   |   | X |   |   |   | 九
+---+---+---+---+---+---+---+---+---+";

    assert_eq!(bb.to_string(), expected);
}
