#[cfg(test)]
mod tests {
    use crux_lib::shogi::{
        attacks,
        bitboard::Bitboard,
        core::{Color, File, Rank, Square},
    };

    #[test]
    fn pawn_attacks() {
        let cases = [
            (Color::Black, Square::S12, Square::S11.bit()),
            (Color::Black, Square::S55, Square::S54.bit()),
            (Color::Black, Square::S99, Square::S98.bit()),
            (Color::White, Square::S11, Square::S12.bit()),
            (Color::White, Square::S55, Square::S56.bit()),
            (Color::White, Square::S98, Square::S99.bit()),
        ];

        for (color, square, pawn_attacks) in cases.iter() {
            assert_eq!(attacks::pawn_attacks(*color, *square), *pawn_attacks);
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn pawn_attacks_black_panics_on_rank1() {
        let _ = attacks::pawn_attacks(Color::Black, Square::S51);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn pawn_attacks_white_panics_on_rank9() {
        let _ = attacks::pawn_attacks(Color::White, Square::S59);
    }

    #[test]
    fn multi_pawn_attacks() {
        let cases = [
            (
                Color::Black,
                Square::S12.bit() | Square::S55.bit() | Square::S99.bit(),
                Square::S11.bit() | Square::S54.bit() | Square::S98.bit(),
            ),
            (
                Color::White,
                Square::S11.bit() | Square::S55.bit() | Square::S98.bit(),
                Square::S12.bit() | Square::S56.bit() | Square::S99.bit(),
            ),
        ];

        for (color, pawns_bb, pawns_attacks) in cases.iter() {
            assert_eq!(
                attacks::multi_pawn_attacks(*color, *pawns_bb),
                *pawns_attacks
            );
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn multi_pawn_attacks_black_panics_if_contains_rank1() {
        let _ = attacks::multi_pawn_attacks(Color::Black, Square::S51.bit());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn multi_pawn_attacks_white_panics_if_contains_rank9() {
        let _ = attacks::multi_pawn_attacks(Color::White, Square::S59.bit());
    }

    #[test]
    fn lance_pseudo_attacks() {
        let cases = [
            (Color::Black, Square::S12, Square::S11.bit()),
            (
                Color::Black,
                Square::S55,
                Square::S54.bit() | Square::S53.bit() | Square::S52.bit() | Square::S51.bit(),
            ),
            (
                Color::Black,
                Square::S99,
                File::File9.bit() & !Square::S99.bit(),
            ),
            (
                Color::White,
                Square::S11,
                File::File1.bit() & !Square::S11.bit(),
            ),
            (
                Color::White,
                Square::S55,
                Square::S56.bit() | Square::S57.bit() | Square::S58.bit() | Square::S59.bit(),
            ),
            (Color::White, Square::S98, Square::S99.bit()),
        ];

        for (color, square, lance_pseudo_attacks) in cases.iter() {
            assert_eq!(
                attacks::lance_pseudo_attacks(*color, *square),
                *lance_pseudo_attacks
            );
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn lance_pseudo_attacks_black_panics_on_rank1() {
        let _ = attacks::lance_pseudo_attacks(Color::Black, Square::S51);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn lance_pseudo_attacks_white_panics_on_rank9() {
        let _ = attacks::lance_pseudo_attacks(Color::White, Square::S59);
    }

    #[test]
    fn lance_attacks() {
        for color in 0..Color::COUNT {
            for square in 0..Square::COUNT {
                let color = Color::from(color);
                let square = Square::from(square);

                if matches!(square.rank().relative(color), Rank::Rank1) {
                    continue;
                }

                assert_eq!(
                    attacks::lance_attacks(color, square, Bitboard::empty()),
                    attacks::lance_pseudo_attacks(color, square)
                );

                assert_eq!(
                    attacks::lance_attacks(color, square, Bitboard::all()),
                    attacks::pawn_attacks(color, square)
                );
            }
        }

        let cases = [
            (
                Color::Black,
                Square::S19,
                Square::S15.bit(),
                Square::S18.bit() | Square::S17.bit() | Square::S16.bit() | Square::S15.bit(),
            ),
            (
                Color::Black,
                Square::S59,
                Square::S57.bit() | Square::S53.bit(),
                Square::S58.bit() | Square::S57.bit(),
            ),
            (
                Color::Black,
                Square::S99,
                Square::S91.bit(),
                File::File9.bit() & !Square::S99.bit(),
            ),
            (
                Color::White,
                Square::S11,
                Square::S15.bit(),
                Square::S12.bit() | Square::S13.bit() | Square::S14.bit() | Square::S15.bit(),
            ),
            (
                Color::White,
                Square::S51,
                Square::S53.bit() | Square::S57.bit(),
                Square::S52.bit() | Square::S53.bit(),
            ),
            (
                Color::White,
                Square::S91,
                Square::S99.bit(),
                File::File9.bit() & !Square::S91.bit(),
            ),
        ];

        for (color, square, occupied, lance_attacks) in cases.iter() {
            assert_eq!(
                attacks::lance_attacks(*color, *square, *occupied),
                *lance_attacks
            );
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn lance_attacks_black_panics_on_rank1() {
        let _ = attacks::lance_attacks(Color::Black, Square::S51, Bitboard::empty());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn lance_attacks_white_panics_on_rank9() {
        let _ = attacks::lance_attacks(Color::White, Square::S59, Bitboard::empty());
    }

    #[test]
    fn knight_attacks() {
        let cases = [
            (Color::Black, Square::S13, Square::S21.bit()),
            (
                Color::Black,
                Square::S55,
                Square::S43.bit() | Square::S63.bit(),
            ),
            (Color::Black, Square::S99, Square::S87.bit()),
            (Color::White, Square::S11, Square::S23.bit()),
            (
                Color::White,
                Square::S55,
                Square::S47.bit() | Square::S67.bit(),
            ),
            (Color::White, Square::S97, Square::S89.bit()),
        ];

        for (color, square, knight_attacks) in cases.iter() {
            assert_eq!(attacks::knight_attacks(*color, *square), *knight_attacks);
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn knight_attacks_black_panics_on_rank1() {
        let _ = attacks::knight_attacks(Color::Black, Square::S51);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn knight_attacks_white_panics_on_rank9() {
        let _ = attacks::knight_attacks(Color::White, Square::S59);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn knight_attacks_black_panics_on_rank2() {
        let _ = attacks::knight_attacks(Color::Black, Square::S52);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn knight_attacks_white_panics_on_rank8() {
        let _ = attacks::knight_attacks(Color::White, Square::S58);
    }

    #[test]
    fn multi_knight_attacks() {
        let cases = [
            (
                Color::Black,
                Square::S13.bit() | Square::S55.bit() | Square::S99.bit(),
                Square::S21.bit() | Square::S43.bit() | Square::S63.bit() | Square::S87.bit(),
            ),
            (
                Color::White,
                Square::S11.bit() | Square::S55.bit() | Square::S97.bit(),
                Square::S23.bit() | Square::S47.bit() | Square::S67.bit() | Square::S89.bit(),
            ),
        ];

        for (color, knights_bb, knights_attacks) in cases.iter() {
            assert_eq!(
                attacks::multi_knight_attacks(*color, *knights_bb),
                *knights_attacks
            );
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn multi_knight_attacks_black_panics_if_contains_rank1() {
        let _ = attacks::multi_knight_attacks(Color::Black, Square::S51.bit());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn multi_knight_attacks_white_panics_if_contains_rank9() {
        let _ = attacks::multi_knight_attacks(Color::White, Square::S59.bit());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn multi_knight_attacks_black_panics_if_contains_rank2() {
        let _ = attacks::multi_knight_attacks(Color::Black, Square::S52.bit());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn multi_knight_attacks_white_panics_if_contains_rank8() {
        let _ = attacks::multi_knight_attacks(Color::White, Square::S58.bit());
    }

    #[test]
    fn silver_attacks() {
        let cases = [
            (Color::Black, Square::S11, Square::S22.bit()),
            (
                Color::Black,
                Square::S55,
                Square::S44.bit()
                    | Square::S46.bit()
                    | Square::S54.bit()
                    | Square::S64.bit()
                    | Square::S66.bit(),
            ),
            (
                Color::Black,
                Square::S99,
                Square::S88.bit() | Square::S98.bit(),
            ),
            (
                Color::White,
                Square::S11,
                Square::S12.bit() | Square::S22.bit(),
            ),
            (
                Color::White,
                Square::S55,
                Square::S44.bit()
                    | Square::S46.bit()
                    | Square::S56.bit()
                    | Square::S64.bit()
                    | Square::S66.bit(),
            ),
            (Color::White, Square::S99, Square::S88.bit()),
        ];

        for (color, square, silver_attacks) in cases.iter() {
            assert_eq!(attacks::silver_attacks(*color, *square), *silver_attacks);
        }
    }

    #[test]
    fn multi_silver_attacks() {
        let cases = [
            (
                Color::Black,
                Square::S11.bit() | Square::S55.bit() | Square::S99.bit(),
                Square::S22.bit()
                    | Square::S44.bit()
                    | Square::S46.bit()
                    | Square::S54.bit()
                    | Square::S64.bit()
                    | Square::S66.bit()
                    | Square::S88.bit()
                    | Square::S98.bit(),
            ),
            (
                Color::White,
                Square::S11.bit() | Square::S55.bit() | Square::S99.bit(),
                Square::S12.bit()
                    | Square::S22.bit()
                    | Square::S44.bit()
                    | Square::S46.bit()
                    | Square::S56.bit()
                    | Square::S64.bit()
                    | Square::S66.bit()
                    | Square::S88.bit(),
            ),
        ];

        for (color, silvers_bb, silvers_attacks) in cases.iter() {
            assert_eq!(
                attacks::multi_silver_attacks(*color, *silvers_bb),
                *silvers_attacks
            );
        }
    }

    #[test]
    fn gold_attacks() {
        let cases = [
            (
                Color::Black,
                Square::S11,
                Square::S12.bit() | Square::S21.bit(),
            ),
            (
                Color::Black,
                Square::S55,
                Square::S44.bit()
                    | Square::S45.bit()
                    | Square::S54.bit()
                    | Square::S56.bit()
                    | Square::S64.bit()
                    | Square::S65.bit(),
            ),
            (
                Color::Black,
                Square::S99,
                Square::S88.bit() | Square::S89.bit() | Square::S98.bit(),
            ),
            (
                Color::White,
                Square::S11,
                Square::S12.bit() | Square::S21.bit() | Square::S22.bit(),
            ),
            (
                Color::White,
                Square::S55,
                Square::S45.bit()
                    | Square::S46.bit()
                    | Square::S54.bit()
                    | Square::S56.bit()
                    | Square::S65.bit()
                    | Square::S66.bit(),
            ),
            (
                Color::White,
                Square::S99,
                Square::S89.bit() | Square::S98.bit(),
            ),
        ];

        for (color, square, gold_attacks) in cases.iter() {
            assert_eq!(attacks::gold_attacks(*color, *square), *gold_attacks);
        }
    }

    #[test]
    fn multi_gold_attacks() {
        let cases = [
            (
                Color::Black,
                Square::S11.bit() | Square::S55.bit() | Square::S99.bit(),
                Square::S12.bit()
                    | Square::S21.bit()
                    | Square::S44.bit()
                    | Square::S45.bit()
                    | Square::S54.bit()
                    | Square::S56.bit()
                    | Square::S64.bit()
                    | Square::S65.bit()
                    | Square::S88.bit()
                    | Square::S89.bit()
                    | Square::S98.bit(),
            ),
            (
                Color::White,
                Square::S11.bit() | Square::S55.bit() | Square::S99.bit(),
                Square::S12.bit()
                    | Square::S21.bit()
                    | Square::S22.bit()
                    | Square::S45.bit()
                    | Square::S46.bit()
                    | Square::S54.bit()
                    | Square::S56.bit()
                    | Square::S65.bit()
                    | Square::S66.bit()
                    | Square::S89.bit()
                    | Square::S98.bit(),
            ),
        ];

        for (color, golds_bb, golds_attacks) in cases.iter() {
            assert_eq!(
                attacks::multi_gold_attacks(*color, *golds_bb),
                *golds_attacks
            );
        }
    }

    #[test]
    fn bishop_pseudo_attacks() {
        let cases = [
            (
                Square::S11,
                Square::S22.bit()
                    | Square::S33.bit()
                    | Square::S44.bit()
                    | Square::S55.bit()
                    | Square::S66.bit()
                    | Square::S77.bit()
                    | Square::S88.bit()
                    | Square::S99.bit(),
            ),
            (
                Square::S15,
                Square::S24.bit()
                    | Square::S26.bit()
                    | Square::S33.bit()
                    | Square::S37.bit()
                    | Square::S42.bit()
                    | Square::S48.bit()
                    | Square::S51.bit()
                    | Square::S59.bit(),
            ),
            (
                Square::S19,
                Square::S28.bit()
                    | Square::S37.bit()
                    | Square::S46.bit()
                    | Square::S55.bit()
                    | Square::S64.bit()
                    | Square::S73.bit()
                    | Square::S82.bit()
                    | Square::S91.bit(),
            ),
            (
                Square::S51,
                Square::S15.bit()
                    | Square::S24.bit()
                    | Square::S33.bit()
                    | Square::S42.bit()
                    | Square::S62.bit()
                    | Square::S73.bit()
                    | Square::S84.bit()
                    | Square::S95.bit(),
            ),
            (
                Square::S55,
                Square::S11.bit()
                    | Square::S11.bit()
                    | Square::S19.bit()
                    | Square::S22.bit()
                    | Square::S28.bit()
                    | Square::S33.bit()
                    | Square::S37.bit()
                    | Square::S44.bit()
                    | Square::S46.bit()
                    | Square::S64.bit()
                    | Square::S66.bit()
                    | Square::S73.bit()
                    | Square::S77.bit()
                    | Square::S82.bit()
                    | Square::S88.bit()
                    | Square::S91.bit()
                    | Square::S99.bit(),
            ),
            (
                Square::S59,
                Square::S15.bit()
                    | Square::S26.bit()
                    | Square::S37.bit()
                    | Square::S48.bit()
                    | Square::S68.bit()
                    | Square::S77.bit()
                    | Square::S86.bit()
                    | Square::S95.bit(),
            ),
            (
                Square::S91,
                Square::S19.bit()
                    | Square::S28.bit()
                    | Square::S37.bit()
                    | Square::S46.bit()
                    | Square::S55.bit()
                    | Square::S64.bit()
                    | Square::S73.bit()
                    | Square::S82.bit(),
            ),
            (
                Square::S95,
                Square::S51.bit()
                    | Square::S59.bit()
                    | Square::S62.bit()
                    | Square::S68.bit()
                    | Square::S73.bit()
                    | Square::S77.bit()
                    | Square::S84.bit()
                    | Square::S86.bit(),
            ),
            (
                Square::S99,
                Square::S11.bit()
                    | Square::S22.bit()
                    | Square::S33.bit()
                    | Square::S44.bit()
                    | Square::S55.bit()
                    | Square::S66.bit()
                    | Square::S77.bit()
                    | Square::S88.bit(),
            ),
        ];

        for (square, bishop_pseudo_attacks) in cases.iter() {
            assert_eq!(
                attacks::bishop_pseudo_attacks(*square),
                *bishop_pseudo_attacks
            );
        }
    }

    #[test]
    fn bishop_attacks() {
        for square in 0..Square::COUNT {
            let square = Square::from(square);

            assert_eq!(
                attacks::bishop_attacks(square, Bitboard::empty()),
                attacks::bishop_pseudo_attacks(square)
            );

            assert_eq!(
                attacks::bishop_attacks(square, Bitboard::all()),
                attacks::silver_attacks(Color::Black, square)
                    & if matches!(square.rank(), Rank::Rank1) {
                        Bitboard::all()
                    } else {
                        !attacks::pawn_attacks(Color::Black, square)
                    }
            );
        }

        let cases = [
            (
                Square::S11,
                Square::S55.bit(),
                Square::S22.bit() | Square::S33.bit() | Square::S44.bit() | Square::S55.bit(),
            ),
            (
                Square::S15,
                Square::S33.bit() | Square::S37.bit(),
                Square::S24.bit() | Square::S26.bit() | Square::S33.bit() | Square::S37.bit(),
            ),
            (
                Square::S19,
                Square::S73.bit(),
                Square::S28.bit()
                    | Square::S37.bit()
                    | Square::S46.bit()
                    | Square::S55.bit()
                    | Square::S64.bit()
                    | Square::S73.bit(),
            ),
            (
                Square::S51,
                Square::S24.bit() | Square::S84.bit(),
                Square::S24.bit()
                    | Square::S33.bit()
                    | Square::S42.bit()
                    | Square::S62.bit()
                    | Square::S73.bit()
                    | Square::S84.bit(),
            ),
            (
                Square::S55,
                Square::S11.bit() | Square::S37.bit() | Square::S64.bit() | Square::S88.bit(),
                Square::S11.bit()
                    | Square::S11.bit()
                    | Square::S22.bit()
                    | Square::S33.bit()
                    | Square::S37.bit()
                    | Square::S44.bit()
                    | Square::S46.bit()
                    | Square::S64.bit()
                    | Square::S66.bit()
                    | Square::S77.bit()
                    | Square::S88.bit(),
            ),
            (
                Square::S59,
                Square::S48.bit() | Square::S86.bit(),
                Square::S48.bit() | Square::S68.bit() | Square::S77.bit() | Square::S86.bit(),
            ),
            (
                Square::S91,
                Square::S19.bit() | Square::S55.bit(),
                Square::S55.bit() | Square::S64.bit() | Square::S73.bit() | Square::S82.bit(),
            ),
            (
                Square::S95,
                Square::S51.bit() | Square::S77.bit(),
                Square::S51.bit()
                    | Square::S62.bit()
                    | Square::S73.bit()
                    | Square::S77.bit()
                    | Square::S84.bit()
                    | Square::S86.bit(),
            ),
            (
                Square::S99,
                Square::S22.bit() | Square::S88.bit(),
                Square::S88.bit(),
            ),
        ];

        for (square, occupied, bishop_attacks) in cases.iter() {
            assert_eq!(attacks::bishop_attacks(*square, *occupied), *bishop_attacks);
        }
    }

    #[test]
    fn rook_pseudo_attacks() {
        for square in 0..Square::COUNT {
            let square = Square::from(square);

            assert_eq!(attacks::rook_pseudo_attacks(square).count_ones(), 16);
        }

        let cases = [
            (
                Square::S11,
                (File::File1.bit() | Rank::Rank1.bit()) & !Square::S11.bit(),
            ),
            (
                Square::S15,
                (File::File1.bit() | Rank::Rank5.bit()) & !Square::S15.bit(),
            ),
            (
                Square::S19,
                (File::File1.bit() | Rank::Rank9.bit()) & !Square::S19.bit(),
            ),
            (
                Square::S51,
                (File::File5.bit() | Rank::Rank1.bit()) & !Square::S51.bit(),
            ),
            (
                Square::S55,
                (File::File5.bit() | Rank::Rank5.bit()) & !Square::S55.bit(),
            ),
            (
                Square::S59,
                (File::File5.bit() | Rank::Rank9.bit()) & !Square::S59.bit(),
            ),
            (
                Square::S91,
                (File::File9.bit() | Rank::Rank1.bit()) & !Square::S91.bit(),
            ),
            (
                Square::S95,
                (File::File9.bit() | Rank::Rank5.bit()) & !Square::S95.bit(),
            ),
            (
                Square::S99,
                (File::File9.bit() | Rank::Rank9.bit()) & !Square::S99.bit(),
            ),
        ];

        for (square, rook_pseudo_attacks) in cases.iter() {
            assert_eq!(attacks::rook_pseudo_attacks(*square), *rook_pseudo_attacks);
        }
    }

    #[test]
    fn rook_attacks() {
        for square in 0..Square::COUNT {
            let square = Square::from(square);

            assert_eq!(
                attacks::rook_attacks(square, Bitboard::empty()),
                attacks::rook_pseudo_attacks(square)
            );

            assert_eq!(
                attacks::rook_attacks(square, Bitboard::all()),
                attacks::king_attacks(square) & !attacks::bishop_attacks(square, Bitboard::all())
            );
        }

        let cases = [
            (
                Square::S11,
                Square::S15.bit() | Square::S51.bit(),
                Square::S12.bit()
                    | Square::S13.bit()
                    | Square::S14.bit()
                    | Square::S15.bit()
                    | Square::S21.bit()
                    | Square::S31.bit()
                    | Square::S41.bit()
                    | Square::S51.bit(),
            ),
            (
                Square::S15,
                Square::S13.bit() | Square::S17.bit() | Square::S55.bit(),
                Square::S13.bit()
                    | Square::S14.bit()
                    | Square::S16.bit()
                    | Square::S17.bit()
                    | Square::S25.bit()
                    | Square::S35.bit()
                    | Square::S45.bit()
                    | Square::S55.bit(),
            ),
            (
                Square::S19,
                Square::S14.bit() | Square::S69.bit(),
                Square::S14.bit()
                    | Square::S15.bit()
                    | Square::S16.bit()
                    | Square::S17.bit()
                    | Square::S18.bit()
                    | Square::S29.bit()
                    | Square::S39.bit()
                    | Square::S49.bit()
                    | Square::S59.bit()
                    | Square::S69.bit(),
            ),
            (
                Square::S51,
                Square::S11.bit()
                    | Square::S31.bit()
                    | Square::S53.bit()
                    | Square::S57.bit()
                    | Square::S71.bit()
                    | Square::S91.bit(),
                Square::S52.bit()
                    | Square::S53.bit()
                    | Square::S31.bit()
                    | Square::S41.bit()
                    | Square::S61.bit()
                    | Square::S71.bit(),
            ),
            (
                Square::S55,
                Square::S15.bit() | Square::S52.bit() | Square::S57.bit() | Square::S65.bit(),
                Square::S52.bit()
                    | Square::S53.bit()
                    | Square::S54.bit()
                    | Square::S56.bit()
                    | Square::S57.bit()
                    | Square::S15.bit()
                    | Square::S25.bit()
                    | Square::S35.bit()
                    | Square::S45.bit()
                    | Square::S65.bit(),
            ),
            (
                Square::S59,
                Square::S49.bit() | Square::S53.bit() | Square::S57.bit() | Square::S69.bit(),
                Square::S57.bit() | Square::S58.bit() | Square::S49.bit() | Square::S69.bit(),
            ),
            (
                Square::S91,
                Square::S71.bit() | Square::S97.bit(),
                Square::S71.bit()
                    | Square::S81.bit()
                    | Square::S92.bit()
                    | Square::S93.bit()
                    | Square::S94.bit()
                    | Square::S95.bit()
                    | Square::S96.bit()
                    | Square::S97.bit(),
            ),
            (
                Square::S95,
                Square::S35.bit() | Square::S65.bit() | Square::S92.bit() | Square::S99.bit(),
                Square::S65.bit()
                    | Square::S75.bit()
                    | Square::S85.bit()
                    | Square::S92.bit()
                    | Square::S93.bit()
                    | Square::S94.bit()
                    | Square::S96.bit()
                    | Square::S97.bit()
                    | Square::S98.bit()
                    | Square::S99.bit(),
            ),
            (
                Square::S99,
                Square::S79.bit() | Square::S92.bit() | Square::S96.bit(),
                Square::S79.bit()
                    | Square::S89.bit()
                    | Square::S96.bit()
                    | Square::S97.bit()
                    | Square::S98.bit(),
            ),
        ];

        for (square, occupied, rook_attacks) in cases.iter() {
            assert_eq!(attacks::rook_attacks(*square, *occupied), *rook_attacks);
        }
    }

    #[test]
    fn king_attacks() {
        for square in 0..Square::COUNT {
            let square = Square::from(square);

            assert_eq!(
                attacks::king_attacks(square),
                attacks::silver_attacks(Color::Black, square)
                    | attacks::gold_attacks(Color::Black, square)
            );
        }
    }
}
