#[cfg(test)]
mod tests {
    use crux_lib::shogi::{
        attacks::{
            bishop_attacks, dragon_attacks, gold_attacks, horse_attacks, king_attacks,
            knight_attacks, lance_attacks, multi_gold_attacks, multi_knight_attacks,
            multi_pawn_attacks, multi_silver_attacks, pawn_attacks, piece_attacks, rook_attacks,
            silver_attacks,
        },
        bitboard::Bitboard,
        core::{Color, File, Piece, PieceType, Rank, Square},
    };

    #[test]
    fn pawn_attacks_respect_color_orientation() {
        let cases = [
            (Color::Black, Square::S55, Square::S54),
            (Color::White, Square::S55, Square::S56),
            (Color::Black, Square::S23, Square::S22),
            (Color::White, Square::S78, Square::S79),
        ];

        for (color, square, target) in cases {
            assert_eq!(pawn_attacks(color, square), target.bit());
        }
    }

    #[test]
    fn multi_pawn_attacks_matches_individual_pawns() {
        let black_pawns = [Square::S44, Square::S65, Square::S23];
        let black_bb = bb_from_squares(&black_pawns);
        let expected_black = black_pawns.iter().fold(Bitboard::empty(), |bb, &sq| {
            bb | pawn_attacks(Color::Black, sq)
        });

        let white_pawns = [Square::S55, Square::S47, Square::S68];
        let white_bb = bb_from_squares(&white_pawns);
        let expected_white = white_pawns.iter().fold(Bitboard::empty(), |bb, &sq| {
            bb | pawn_attacks(Color::White, sq)
        });

        let single_white = Square::S45.bit();

        let cases = [
            (Color::Black, black_bb, expected_black),
            (Color::White, white_bb, expected_white),
            (Color::Black, Bitboard::empty(), Bitboard::empty()),
            (
                Color::White,
                single_white,
                pawn_attacks(Color::White, Square::S45),
            ),
        ];

        for (color, pawns_bb, expected) in cases {
            assert_eq!(multi_pawn_attacks(color, pawns_bb), expected);
        }
    }

    #[test]
    fn knight_attacks_match_reference() {
        const KNIGHT_OFFSETS: [(i8, i8); 2] = [(-1, -2), (1, -2)];
        let cases = [
            (Color::Black, Square::S55),
            (Color::Black, Square::S24),
            (Color::White, Square::S77),
            (Color::White, Square::S46),
        ];

        for (color, square) in cases {
            let expected = relative_steps(square, color, &KNIGHT_OFFSETS);
            assert_eq!(knight_attacks(color, square), expected);
        }
    }

    #[test]
    fn multi_knight_attacks_matches_individual_knights() {
        let black_knights = [Square::S55, Square::S64, Square::S73];
        let white_knights = [Square::S55, Square::S46, Square::S37];
        let lone_black = [Square::S33];
        let white_edge = [Square::S64];

        let cases = [
            (Color::Black, &black_knights[..]),
            (Color::White, &white_knights[..]),
            (Color::Black, &lone_black[..]),
            (Color::White, &white_edge[..]),
        ];

        for (color, knights) in cases {
            let bb = bb_from_squares(knights);
            let expected = knights.iter().fold(Bitboard::empty(), |attacks, &sq| {
                attacks | knight_attacks(color, sq)
            });

            let actual = multi_knight_attacks(color, bb);
            assert_eq!(actual, expected, "color {:?} knights {:?}", color, knights);
        }
    }

    #[test]
    fn silver_attacks_match_reference() {
        const SILVER_OFFSETS: [(i8, i8); 5] = [(0, -1), (-1, -1), (1, -1), (-1, 1), (1, 1)];
        let cases = [
            (Color::Black, Square::S55),
            (Color::Black, Square::S21),
            (Color::White, Square::S87),
            (Color::White, Square::S25),
        ];

        for (color, square) in cases {
            let expected = relative_steps(square, color, &SILVER_OFFSETS);
            assert_eq!(silver_attacks(color, square), expected);
        }
    }

    #[test]
    fn multi_silver_attacks_matches_individual_silvers() {
        let black_silvers = [Square::S55, Square::S23, Square::S12];
        let white_silvers = [Square::S55, Square::S77, Square::S88];
        let edge_black = [Square::S72];

        let empty: &[Square] = &[];

        let cases = [
            (Color::Black, &black_silvers[..]),
            (Color::White, &white_silvers[..]),
            (Color::Black, &edge_black[..]),
            (Color::White, empty),
        ];

        for (color, silvers) in cases {
            let bb = bb_from_squares(silvers);
            let expected = silvers.iter().fold(Bitboard::empty(), |attacks, &sq| {
                attacks | silver_attacks(color, sq)
            });

            assert_eq!(multi_silver_attacks(color, bb), expected);
        }
    }

    #[test]
    fn gold_attacks_match_reference() {
        const GOLD_OFFSETS: [(i8, i8); 6] = [(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (1, -1)];

        let cases = [
            (Color::Black, Square::S55),
            (Color::Black, Square::S19),
            (Color::White, Square::S85),
            (Color::White, Square::S23),
        ];

        for (color, square) in cases {
            let expected = relative_steps(square, color, &GOLD_OFFSETS);
            assert_eq!(gold_attacks(color, square), expected);
        }
    }

    #[test]
    fn multi_gold_attacks_matches_individual_golds() {
        let black_golds = [Square::S55, Square::S13, Square::S87];
        let white_golds = [Square::S55, Square::S24, Square::S68];
        let single_white = [Square::S52];
        let empty: &[Square] = &[];

        let cases = [
            (Color::Black, &black_golds[..]),
            (Color::White, &white_golds[..]),
            (Color::White, &single_white[..]),
            (Color::Black, empty),
        ];

        for (color, golds) in cases {
            let bb = bb_from_squares(golds);
            let expected = golds.iter().fold(Bitboard::empty(), |attacks, &sq| {
                attacks | gold_attacks(color, sq)
            });

            assert_eq!(multi_gold_attacks(color, bb), expected);
        }
    }

    #[test]
    fn lance_attacks_stop_at_blockers() {
        let black_square = Square::S55;
        let black_occupied = bb_from_squares(&[Square::S54, Square::S53, Square::S72]);

        assert_eq!(
            lance_attacks(Color::Black, black_square, black_occupied),
            reference_lance(Color::Black, black_square, black_occupied)
        );

        let white_square = Square::S45;
        let white_occupied = bb_from_squares(&[Square::S46, Square::S48, Square::S54]);

        assert_eq!(
            lance_attacks(Color::White, white_square, white_occupied),
            reference_lance(Color::White, white_square, white_occupied)
        );

        let cramped_square = Square::S43;
        let cramped_occupied = bb_from_squares(&[Square::S42, Square::S49, Square::S31]);

        assert_eq!(
            lance_attacks(Color::Black, cramped_square, cramped_occupied),
            reference_lance(Color::Black, cramped_square, cramped_occupied)
        );
    }

    #[test]
    fn bishop_attacks_follow_diagonals() {
        let cases = [
            (
                Square::S55,
                bb_from_squares(&[Square::S33, Square::S73, Square::S37, Square::S77]),
            ),
            (
                Square::S22,
                bb_from_squares(&[Square::S11, Square::S44, Square::S64]),
            ),
            (Square::S88, Bitboard::empty()),
        ];

        for (square, occupied) in cases {
            assert_eq!(
                bishop_attacks(square, occupied),
                reference_bishop(square, occupied)
            );
        }
    }

    #[test]
    fn rook_attacks_follow_ranks_and_files() {
        let cases = [
            (
                Square::S55,
                bb_from_squares(&[Square::S53, Square::S57, Square::S45, Square::S65]),
            ),
            (
                Square::S22,
                bb_from_squares(&[Square::S23, Square::S24, Square::S62]),
            ),
            (Square::S77, Bitboard::empty()),
        ];

        for (square, occupied) in cases {
            assert_eq!(
                rook_attacks(square, occupied),
                reference_rook(square, occupied)
            );
        }
    }

    #[test]
    fn horse_and_dragon_include_king_moves() {
        let cases = [
            (
                Square::S55,
                bb_from_squares(&[
                    Square::S33,
                    Square::S73,
                    Square::S37,
                    Square::S77,
                    Square::S53,
                    Square::S57,
                    Square::S45,
                    Square::S65,
                ]),
            ),
            (
                Square::S22,
                bb_from_squares(&[Square::S23, Square::S24, Square::S44]),
            ),
            (Square::S88, Bitboard::empty()),
        ];

        for (square, occupied) in cases {
            assert_eq!(
                horse_attacks(square, occupied),
                reference_bishop(square, occupied) | reference_king(square)
            );
            assert_eq!(
                dragon_attacks(square, occupied),
                reference_rook(square, occupied) | reference_king(square)
            );
        }
    }

    #[test]
    fn king_attacks_cover_adjacent_squares() {
        let cases = [Square::S55, Square::S11, Square::S59];

        for square in cases {
            assert_eq!(king_attacks(square), reference_king(square));
        }
    }

    #[test]
    fn piece_attacks_dispatches_to_specialized_implementation() {
        let square = Square::S55;
        let occupied = bb_from_squares(&[
            Square::S54,
            Square::S57,
            Square::S45,
            Square::S65,
            Square::S33,
            Square::S37,
            Square::S73,
            Square::S77,
        ]);

        assert_eq!(
            piece_attacks(Piece::new(Color::Black, PieceType::Pawn), square, occupied),
            pawn_attacks(Color::Black, square)
        );

        assert_eq!(
            piece_attacks(Piece::new(Color::White, PieceType::Pawn), square, occupied),
            pawn_attacks(Color::White, square)
        );

        assert_eq!(
            piece_attacks(Piece::new(Color::Black, PieceType::Lance), square, occupied),
            lance_attacks(Color::Black, square, occupied)
        );

        assert_eq!(
            piece_attacks(Piece::new(Color::White, PieceType::Lance), square, occupied),
            lance_attacks(Color::White, square, occupied)
        );

        assert_eq!(
            piece_attacks(
                Piece::new(Color::Black, PieceType::Knight),
                square,
                occupied
            ),
            knight_attacks(Color::Black, square)
        );

        assert_eq!(
            piece_attacks(
                Piece::new(Color::White, PieceType::Knight),
                square,
                occupied
            ),
            knight_attacks(Color::White, square)
        );

        assert_eq!(
            piece_attacks(
                Piece::new(Color::Black, PieceType::Silver),
                square,
                occupied
            ),
            silver_attacks(Color::Black, square)
        );

        assert_eq!(
            piece_attacks(
                Piece::new(Color::White, PieceType::Silver),
                square,
                occupied
            ),
            silver_attacks(Color::White, square)
        );

        let gold_like = [
            PieceType::Gold,
            PieceType::ProPawn,
            PieceType::ProLance,
            PieceType::ProKnight,
            PieceType::ProSilver,
        ];

        for piece_type in gold_like {
            let piece = Piece::new(Color::White, piece_type);
            assert_eq!(
                piece_attacks(piece, square, occupied),
                gold_attacks(Color::White, square)
            );
        }

        assert_eq!(
            piece_attacks(
                Piece::new(Color::Black, PieceType::Bishop),
                square,
                occupied
            ),
            bishop_attacks(square, occupied)
        );

        assert_eq!(
            piece_attacks(Piece::new(Color::Black, PieceType::Rook), square, occupied),
            rook_attacks(square, occupied)
        );

        assert_eq!(
            piece_attacks(Piece::new(Color::Black, PieceType::Horse), square, occupied),
            horse_attacks(square, occupied)
        );

        assert_eq!(
            piece_attacks(
                Piece::new(Color::Black, PieceType::Dragon),
                square,
                occupied
            ),
            dragon_attacks(square, occupied)
        );

        assert_eq!(
            piece_attacks(Piece::new(Color::Black, PieceType::King), square, occupied),
            king_attacks(square)
        );
    }

    fn bb_from_squares(squares: &[Square]) -> Bitboard {
        squares
            .iter()
            .fold(Bitboard::empty(), |bb, &square| bb | square.bit())
    }

    fn offset_square(square: Square, file_delta: i8, rank_delta: i8) -> Option<Square> {
        let file_idx = square.file().as_u8() as i8 + file_delta;
        let rank_idx = square.rank().as_u8() as i8 + rank_delta;

        if file_idx < 0
            || file_idx >= File::COUNT as i8
            || rank_idx < 0
            || rank_idx >= Rank::COUNT as i8
        {
            None
        } else {
            Some(Square::new(
                File::from(file_idx as u8),
                Rank::from(rank_idx as u8),
            ))
        }
    }

    fn relative_steps(square: Square, color: Color, offsets: &[(i8, i8)]) -> Bitboard {
        offsets.iter().fold(Bitboard::empty(), |bb, &(df, dr)| {
            let (df, dr) = if color.is_black() {
                (df, dr)
            } else {
                (-df, -dr)
            };

            if let Some(target) = offset_square(square, df, dr) {
                bb | target.bit()
            } else {
                bb
            }
        })
    }

    fn slider_ray(square: Square, file_delta: i8, rank_delta: i8, occupied: Bitboard) -> Bitboard {
        let mut bb = Bitboard::empty();
        let mut file_idx = square.file().as_u8() as i8;
        let mut rank_idx = square.rank().as_u8() as i8;

        loop {
            file_idx += file_delta;
            rank_idx += rank_delta;

            if file_idx < 0
                || file_idx >= File::COUNT as i8
                || rank_idx < 0
                || rank_idx >= Rank::COUNT as i8
            {
                break;
            }

            let next = Square::new(File::from(file_idx as u8), Rank::from(rank_idx as u8));
            let bit = next.bit();
            bb |= bit;

            if (occupied & bit).is_any() {
                break;
            }
        }

        bb
    }

    fn reference_lance(color: Color, square: Square, occupied: Bitboard) -> Bitboard {
        let rank_delta = if color.is_black() { -1 } else { 1 };
        slider_ray(square, 0, rank_delta, occupied)
    }

    fn reference_bishop(square: Square, occupied: Bitboard) -> Bitboard {
        slider_ray(square, -1, -1, occupied)
            | slider_ray(square, -1, 1, occupied)
            | slider_ray(square, 1, -1, occupied)
            | slider_ray(square, 1, 1, occupied)
    }

    fn reference_rook(square: Square, occupied: Bitboard) -> Bitboard {
        slider_ray(square, 0, -1, occupied)
            | slider_ray(square, 0, 1, occupied)
            | slider_ray(square, -1, 0, occupied)
            | slider_ray(square, 1, 0, occupied)
    }

    fn reference_king(square: Square) -> Bitboard {
        let deltas = [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];

        deltas.iter().fold(Bitboard::empty(), |bb, &(df, dr)| {
            if let Some(target) = offset_square(square, df, dr) {
                bb | target.bit()
            } else {
                bb
            }
        })
    }
}
