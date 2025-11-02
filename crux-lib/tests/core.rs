use crux_lib::shogi::core::{Color, File, Piece, PieceType, Rank, Square};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_is_black() {
        assert!(Color::Black.is_black());
        assert!(!Color::White.is_black());
    }

    #[test]
    fn color_is_white() {
        assert!(Color::White.is_white());
        assert!(!Color::Black.is_white());
    }

    #[test]
    fn color_opposite() {
        assert_eq!(Color::Black.opposite(), Color::White);
        assert_eq!(Color::White.opposite(), Color::Black);
    }

    #[test]
    fn piece_type_with_color() {
        let cases = [
            (PieceType::Pawn, Piece::BlackPawn, Piece::WhitePawn),
            (PieceType::ProPawn, Piece::BlackProPawn, Piece::WhiteProPawn),
            (PieceType::Dragon, Piece::BlackDragon, Piece::WhiteDragon),
            (PieceType::King, Piece::BlackKing, Piece::WhiteKing),
        ];

        for (piece_type, black, white) in cases.iter() {
            assert_eq!(piece_type.with_color(Color::Black), *black);
            assert_eq!(piece_type.with_color(Color::White), *white);
        }
    }

    #[test]
    fn piece_type_promoted() {
        let cases = [
            (PieceType::Pawn, PieceType::ProPawn),
            (PieceType::Lance, PieceType::ProLance),
            (PieceType::Knight, PieceType::ProKnight),
            (PieceType::Silver, PieceType::ProSilver),
            (PieceType::Bishop, PieceType::Horse),
            (PieceType::Rook, PieceType::Dragon),
        ];

        for (piece_type, promoted) in cases.iter() {
            assert_eq!(piece_type.promoted(), *promoted);
        }

        for piece_type in [
            PieceType::ProPawn,
            PieceType::ProLance,
            PieceType::ProKnight,
            PieceType::ProSilver,
            PieceType::Horse,
            PieceType::Dragon,
        ]
        .iter()
        {
            assert_eq!(piece_type.promoted(), *piece_type);
        }
    }

    #[test]
    fn piece_type_unpromoted() {
        let cases = [
            (PieceType::ProPawn, PieceType::Pawn),
            (PieceType::ProLance, PieceType::Lance),
            (PieceType::ProKnight, PieceType::Knight),
            (PieceType::ProSilver, PieceType::Silver),
            (PieceType::Horse, PieceType::Bishop),
            (PieceType::Dragon, PieceType::Rook),
        ];

        for (piece_type, unpromoted) in cases.iter() {
            assert_eq!(piece_type.unpromoted(), *unpromoted);
        }

        for piece_type in [
            PieceType::Pawn,
            PieceType::Lance,
            PieceType::Knight,
            PieceType::Silver,
            PieceType::Gold,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::King,
        ]
        .iter()
        {
            assert_eq!(piece_type.unpromoted(), *piece_type);
        }
    }

    #[test]
    fn piece_type_is_promoted() {
        for piece_type in [
            PieceType::ProPawn,
            PieceType::ProLance,
            PieceType::ProKnight,
            PieceType::ProSilver,
            PieceType::Horse,
            PieceType::Dragon,
        ] {
            assert!(piece_type.is_promoted());
        }

        for piece_type in [
            PieceType::Pawn,
            PieceType::Lance,
            PieceType::Knight,
            PieceType::Silver,
            PieceType::Gold,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::King,
        ] {
            assert!(!piece_type.is_promoted());
        }
    }

    #[test]
    fn piece_get_color() {
        let cases = [
            (Piece::BlackPawn, Color::Black),
            (Piece::WhitePawn, Color::White),
            (Piece::BlackProPawn, Color::Black),
            (Piece::WhiteProPawn, Color::White),
            (Piece::BlackDragon, Color::Black),
            (Piece::WhiteDragon, Color::White),
            (Piece::BlackKing, Color::Black),
            (Piece::WhiteKing, Color::White),
        ];

        for (piece, color) in cases.iter() {
            assert_eq!(piece.color(), *color);
        }
    }

    #[test]
    fn piece_get_type() {
        let cases = [
            (Piece::BlackPawn, PieceType::Pawn),
            (Piece::WhitePawn, PieceType::Pawn),
            (Piece::BlackProPawn, PieceType::ProPawn),
            (Piece::WhiteProPawn, PieceType::ProPawn),
            (Piece::BlackDragon, PieceType::Dragon),
            (Piece::WhiteDragon, PieceType::Dragon),
            (Piece::BlackKing, PieceType::King),
            (Piece::WhiteKing, PieceType::King),
        ];

        for (piece, piece_type) in cases.iter() {
            assert_eq!(piece.piece_type(), *piece_type);
        }
    }

    #[test]
    fn file_east() {
        let cases = [
            (File::File2, File::File1),
            (File::File5, File::File4),
            (File::File9, File::File8),
        ];

        for (file, east) in cases.iter() {
            assert_eq!(file.east(), *east);
        }
    }

    #[test]
    #[should_panic]
    fn file_east_file1() {
        let _ = File::File1.east();
    }

    #[test]
    fn file_west() {
        let cases = [
            (File::File1, File::File2),
            (File::File5, File::File6),
            (File::File8, File::File9),
        ];

        for (file, west) in cases.iter() {
            assert_eq!(file.west(), *west);
        }
    }

    #[test]
    #[should_panic]
    fn file_west_file9() {
        let _ = File::File9.west();
    }

    #[test]
    fn file_relative_east() {
        let cases = [
            (Color::Black, File::File2, File::File1),
            (Color::Black, File::File5, File::File4),
            (Color::Black, File::File9, File::File8),
            (Color::White, File::File1, File::File2),
            (Color::White, File::File5, File::File6),
            (Color::White, File::File8, File::File9),
        ];

        for (color, file, relative_east) in cases.iter() {
            assert_eq!(file.relative_east(*color), *relative_east);
        }
    }

    #[test]
    #[should_panic]
    fn relative_east_black_panics_on_file1() {
        let _ = File::File1.relative_east(Color::Black);
    }

    #[test]
    #[should_panic]
    fn relative_east_white_panics_on_file9() {
        let _ = File::File9.relative_east(Color::White);
    }


    #[test]
    fn file_relative_west() {
        let cases = [
            (Color::Black, File::File1, File::File2),
            (Color::Black, File::File5, File::File6),
            (Color::Black, File::File8, File::File9),
            (Color::White, File::File2, File::File1),
            (Color::White, File::File5, File::File4),
            (Color::White, File::File9, File::File8),
        ];

        for (color, file, relative_west) in cases.iter() {
            assert_eq!(file.relative_west(*color), *relative_west);
        }
    }

    #[test]
    #[should_panic]
    fn relative_west_black_panics_on_file9() {
        let _ = File::File9.relative_west(Color::Black);
    }

    #[test]
    #[should_panic]
    fn relative_west_white_panics_on_file1() {
        let _ = File::File1.relative_west(Color::White);
    }

    #[test]
    fn file_flip() {
        let cases = [
            (File::File1, File::File9),
            (File::File2, File::File8),
            (File::File3, File::File7),
            (File::File4, File::File6),
            (File::File5, File::File5),
            (File::File6, File::File4),
            (File::File7, File::File3),
            (File::File8, File::File2),
            (File::File9, File::File1),
        ];

        for (file, flipped) in cases.iter() {
            assert_eq!(file.flip(), *flipped);
        }
    }

    #[test]
    fn rank_north() {
        let cases = [
            (Rank::Rank2, Rank::Rank1),
            (Rank::Rank5, Rank::Rank4),
            (Rank::Rank9, Rank::Rank8),
        ];

        for (rank, north) in cases.iter() {
            assert_eq!(rank.north(), *north);
        }
    }

    #[test]
    #[should_panic]
    fn rank_north_rank1() {
        let _ = Rank::Rank1.north();
    }

    #[test]
    fn rank_south() {
        let cases = [
            (Rank::Rank1, Rank::Rank2),
            (Rank::Rank5, Rank::Rank6),
            (Rank::Rank8, Rank::Rank9),
        ];

        for (rank, south) in cases.iter() {
            assert_eq!(rank.south(), *south);
        }
    }

    #[test]
    #[should_panic]
    fn rank_south_rank9() {
        let _ = Rank::Rank9.south();
    }

    #[test]
    fn rank_relative_north() {
        let cases = [
            (Color::Black, Rank::Rank2, Rank::Rank1),
            (Color::Black, Rank::Rank5, Rank::Rank4),
            (Color::Black, Rank::Rank9, Rank::Rank8),
            (Color::White, Rank::Rank1, Rank::Rank2),
            (Color::White, Rank::Rank5, Rank::Rank6),
            (Color::White, Rank::Rank8, Rank::Rank9),
        ];

        for (color, rank, relative_north) in cases.iter() {
            assert_eq!(rank.relative_north(*color), *relative_north);
        }
    }

    #[test]
    #[should_panic]
    fn relative_north_black_panics_on_rank1() {
        let _ = Rank::Rank1.relative_north(Color::Black);
    }

    #[test]
    #[should_panic]
    fn relative_north_white_panics_on_rank9() {
        let _ = Rank::Rank9.relative_north(Color::White);
    }


    #[test]
    fn rank_relative_south() {
        let cases = [
            (Color::Black, Rank::Rank1, Rank::Rank2),
            (Color::Black, Rank::Rank5, Rank::Rank6),
            (Color::Black, Rank::Rank8, Rank::Rank9),
            (Color::White, Rank::Rank2, Rank::Rank1),
            (Color::White, Rank::Rank5, Rank::Rank4),
            (Color::White, Rank::Rank9, Rank::Rank8),
        ];

        for (color, rank, relative_south) in cases.iter() {
            assert_eq!(rank.relative_south(*color), *relative_south);
        }
    }

    #[test]
    #[should_panic]
    fn relative_south_black_panics_on_rank9() {
        let _ = Rank::Rank9.relative_south(Color::Black);
    }

    #[test]
    #[should_panic]
    fn relative_south_white_panics_on_rank1() {
        let _ = Rank::Rank1.relative_south(Color::White);
    }

    #[test]
    fn rank_flip() {
        let cases = [
            (Rank::Rank1, Rank::Rank9),
            (Rank::Rank2, Rank::Rank8),
            (Rank::Rank3, Rank::Rank7),
            (Rank::Rank4, Rank::Rank6),
            (Rank::Rank5, Rank::Rank5),
            (Rank::Rank6, Rank::Rank4),
            (Rank::Rank7, Rank::Rank3),
            (Rank::Rank8, Rank::Rank2),
            (Rank::Rank9, Rank::Rank1),
        ];

        for (rank, flipped) in cases.iter() {
            assert_eq!(rank.flip(), *flipped);
        }
    }

    #[test]
    fn rank_can_promote() {
        let cases = [
            (Color::Black, Rank::Rank1, true),
            (Color::Black, Rank::Rank2, true),
            (Color::Black, Rank::Rank3, true),
            (Color::Black, Rank::Rank4, false),
            (Color::Black, Rank::Rank5, false),
            (Color::Black, Rank::Rank6, false),
            (Color::Black, Rank::Rank7, false),
            (Color::Black, Rank::Rank8, false),
            (Color::Black, Rank::Rank9, false),
            (Color::White, Rank::Rank1, false),
            (Color::White, Rank::Rank2, false),
            (Color::White, Rank::Rank3, false),
            (Color::White, Rank::Rank4, false),
            (Color::White, Rank::Rank5, false),
            (Color::White, Rank::Rank6, false),
            (Color::White, Rank::Rank7, true),
            (Color::White, Rank::Rank8, true),
            (Color::White, Rank::Rank9, true),
        ];

        for (color, rank, can_promote) in cases.iter() {
            assert_eq!(rank.can_promote(*color), *can_promote);
        }
    }

    #[test]
    fn square_from_file_rank() {
        for file in 0..File::COUNT {
            for rank in 0..Rank::COUNT {
                let file: File = File::from_raw(file as u8);
                let rank: Rank = Rank::from_raw(rank as u8);

                let square = Square::new(file, rank);

                assert_eq!(square.file(), file);
                assert_eq!(square.rank(), rank);
            }
        }
    }

    #[test]
    pub fn square_file() {
        let cases = [
            (Square::new(File::File1, Rank::Rank9), File::File1),
            (Square::new(File::File5, Rank::Rank1), File::File5),
             (Square::new(File::File9, Rank::Rank5), File::File9),
        ];

        for (square, file) in cases.iter() {
            assert_eq!(square.file(), *file);
        }
    }

    #[test]
    pub fn square_rank() {
        let cases = [
            (Square::new(File::File1, Rank::Rank9), Rank::Rank9),
            (Square::new(File::File5, Rank::Rank1), Rank::Rank1),
            (Square::new(File::File9, Rank::Rank5), Rank::Rank5),
        ];

        for (square, rank) in cases.iter() {
            assert_eq!(square.rank(), *rank);
        }
    }

    #[test]
    pub fn square_north() {
        let cases = [
            (Square::new(File::File1, Rank::Rank9), Square::new(File::File1, Rank::Rank8)),
            (Square::new(File::File5, Rank::Rank2), Square::new(File::File5, Rank::Rank1)),
            (Square::new(File::File9, Rank::Rank5), Square::new(File::File9, Rank::Rank4)),
        ];

        for (square, north) in cases.iter() {
            assert_eq!(square.north(), *north);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_north_panics_on_rank1() {
        let _ = Square::new(File::File5, Rank::Rank1).north();
    }

    #[test]
    pub fn square_south() {
        let cases = [
            (Square::new(File::File1, Rank::Rank8), Square::new(File::File1, Rank::Rank9)),
            (Square::new(File::File5, Rank::Rank1), Square::new(File::File5, Rank::Rank2)),
            (Square::new(File::File9, Rank::Rank5), Square::new(File::File9, Rank::Rank6)),
        ];

        for (square, south) in cases.iter() {
            assert_eq!(square.south(), *south);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_south_panics_on_rank9() {
        let _ = Square::new(File::File5, Rank::Rank9).south();
    }

    #[test]
    pub fn square_east() {
        let cases = [
            (Square::new(File::File2, Rank::Rank9), Square::new(File::File1, Rank::Rank9)),
            (Square::new(File::File5, Rank::Rank1), Square::new(File::File4, Rank::Rank1)),
            (Square::new(File::File9, Rank::Rank5), Square::new(File::File8, Rank::Rank5)),
        ];

        for (square, east) in cases.iter() {
            assert_eq!(square.east(), *east);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_east_panics_on_file1() {
        let _ = Square::new(File::File1, Rank::Rank5).east();
    }

    #[test]
    pub fn square_west() {
        let cases = [
            (Square::new(File::File1, Rank::Rank9), Square::new(File::File2, Rank::Rank9)),
            (Square::new(File::File5, Rank::Rank1), Square::new(File::File6, Rank::Rank1)),
            (Square::new(File::File8, Rank::Rank5), Square::new(File::File9, Rank::Rank5)),
        ];

        for (square, west) in cases.iter() {
            assert_eq!(square.west(), *west);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_west_panics_on_file9() {
        let _ = Square::new(File::File9, Rank::Rank5).west();
    }

    #[test]
    pub fn square_north_east() {
        let cases = [
            (Square::new(File::File2, Rank::Rank9), Square::new(File::File1, Rank::Rank8)),
            (Square::new(File::File5, Rank::Rank2), Square::new(File::File4, Rank::Rank1)),
            (Square::new(File::File9, Rank::Rank5), Square::new(File::File8, Rank::Rank4)),
        ];

        for (square, north_east) in cases.iter() {
            assert_eq!(square.north_east(), *north_east);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_north_east_panic_on_rank1() {
        let _ = Square::new(File::File5, Rank::Rank1).north_east();
    }

    #[test]
    #[should_panic]
    pub fn square_north_east_panic_on_file1() {
        let _ = Square::new(File::File1, Rank::Rank5).north_east();
    }

    #[test]
    pub fn square_north_west() {
        let cases = [
            (Square::new(File::File1, Rank::Rank9), Square::new(File::File2, Rank::Rank8)),
            (Square::new(File::File5, Rank::Rank2), Square::new(File::File6, Rank::Rank1)),
            (Square::new(File::File8, Rank::Rank5), Square::new(File::File9, Rank::Rank4)),
        ];

        for (square, north_west) in cases.iter() {
            assert_eq!(square.north_west(), *north_west);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_north_west_panic_on_rank1() {
        let _ = Square::new(File::File5, Rank::Rank1).north_west();
    }

    #[test]
    #[should_panic]
    pub fn square_north_west_panic_on_file9() {
        let _ = Square::new(File::File9, Rank::Rank5).north_west();
    }

    #[test]
    pub fn square_south_east() {
        let cases = [
            (Square::new(File::File2, Rank::Rank8), Square::new(File::File1, Rank::Rank9)),
            (Square::new(File::File5, Rank::Rank1), Square::new(File::File4, Rank::Rank2)),
            (Square::new(File::File9, Rank::Rank5), Square::new(File::File8, Rank::Rank6)),
        ];

        for (square, south_east) in cases.iter() {
            assert_eq!(square.south_east(), *south_east);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_south_east_panic_on_rank9() {
        let _ = Square::new(File::File5, Rank::Rank9).south_east();
    }

    #[test]
    #[should_panic]
    pub fn square_south_east_panic_on_file1() {
        let _ = Square::new(File::File1, Rank::Rank5).south_east();
    }

    #[test]
    pub fn square_south_west() {
        let cases = [
            (Square::new(File::File1, Rank::Rank8), Square::new(File::File2, Rank::Rank9)),
            (Square::new(File::File5, Rank::Rank1), Square::new(File::File6, Rank::Rank2)),
            (Square::new(File::File8, Rank::Rank5), Square::new(File::File9, Rank::Rank6)),
        ];

        for (square, south_west) in cases.iter() {
            assert_eq!(square.south_west(), *south_west);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_south_west_panic_on_rank9() {
        let _ = Square::new(File::File5, Rank::Rank9).south_west();
    }

    #[test]
    #[should_panic]
    pub fn square_south_west_panic_on_file9() {
        let _ = Square::new(File::File9, Rank::Rank5).south_west();
    }

    #[test]
    pub fn square_relative_north() {
        let cases = [
            (Color::Black, Square::new(File::File1, Rank::Rank9), Square::new(File::File1, Rank::Rank8)),
            (Color::Black, Square::new(File::File5, Rank::Rank2), Square::new(File::File5, Rank::Rank1)),
            (Color::Black, Square::new(File::File9, Rank::Rank5), Square::new(File::File9, Rank::Rank4)),
            (Color::White, Square::new(File::File1, Rank::Rank8), Square::new(File::File1, Rank::Rank9)),
            (Color::White, Square::new(File::File5, Rank::Rank1), Square::new(File::File5, Rank::Rank2)),
            (Color::White, Square::new(File::File9, Rank::Rank5), Square::new(File::File9, Rank::Rank6)),
        ];

        for (color, square, relative_north) in cases.iter() {
            assert_eq!(square.relative_north(*color), *relative_north);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_relative_north_black_panics_on_rank1() {
        let _ = Square::new(File::File5, Rank::Rank1).relative_north(Color::Black);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_north_white_panics_on_rank9() {
        let _ = Square::new(File::File5, Rank::Rank9).relative_north(Color::White);
    }

    #[test]
    pub fn square_relative_south() {
        let cases = [
            (Color::Black, Square::new(File::File1, Rank::Rank8), Square::new(File::File1, Rank::Rank9)),
            (Color::Black, Square::new(File::File5, Rank::Rank1), Square::new(File::File5, Rank::Rank2)),
            (Color::Black, Square::new(File::File9, Rank::Rank5), Square::new(File::File9, Rank::Rank6)),
            (Color::White, Square::new(File::File1, Rank::Rank9), Square::new(File::File1, Rank::Rank8)),
            (Color::White, Square::new(File::File5, Rank::Rank2), Square::new(File::File5, Rank::Rank1)),
            (Color::White, Square::new(File::File9, Rank::Rank5), Square::new(File::File9, Rank::Rank4)),
        ];

        for (color, square, relative_south) in cases.iter() {
            assert_eq!(square.relative_south(*color), *relative_south);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_relative_south_black_panics_on_rank9() {
        let _ = Square::new(File::File5, Rank::Rank9).relative_south(Color::Black);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_south_white_panics_on_rank1() {
        let _ = Square::new(File::File5, Rank::Rank1).relative_south(Color::White);
    }

    #[test]
    pub fn square_relative_east() {
        let cases = [
            (Color::Black, Square::new(File::File2, Rank::Rank9), Square::new(File::File1, Rank::Rank9)),
            (Color::Black, Square::new(File::File5, Rank::Rank1), Square::new(File::File4, Rank::Rank1)),
            (Color::Black, Square::new(File::File9, Rank::Rank5), Square::new(File::File8, Rank::Rank5)),
            (Color::White, Square::new(File::File1, Rank::Rank9), Square::new(File::File2, Rank::Rank9)),
            (Color::White, Square::new(File::File5, Rank::Rank1), Square::new(File::File6, Rank::Rank1)),
            (Color::White, Square::new(File::File8, Rank::Rank5), Square::new(File::File9, Rank::Rank5)),
        ];

        for (color, square, relative_east) in cases.iter() {
            assert_eq!(square.relative_east(*color), *relative_east);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_relative_east_black_panics_on_file1() {
        let _ = Square::new(File::File1, Rank::Rank5).relative_east(Color::Black);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_east_white_panics_on_file9() {
        let _ = Square::new(File::File9, Rank::Rank5).relative_east(Color::White);
    }

    #[test]
    pub fn square_relative_west() {
        let cases = [
            (Color::Black, Square::new(File::File1, Rank::Rank9), Square::new(File::File2, Rank::Rank9)),
            (Color::Black, Square::new(File::File5, Rank::Rank1), Square::new(File::File6, Rank::Rank1)),
            (Color::Black, Square::new(File::File8, Rank::Rank5), Square::new(File::File9, Rank::Rank5)),
            (Color::White, Square::new(File::File2, Rank::Rank9), Square::new(File::File1, Rank::Rank9)),
            (Color::White, Square::new(File::File5, Rank::Rank1), Square::new(File::File4, Rank::Rank1)),
            (Color::White, Square::new(File::File9, Rank::Rank5), Square::new(File::File8, Rank::Rank5)),
        ];

        for (color, square, relative_west) in cases.iter() {
            assert_eq!(square.relative_west(*color), *relative_west);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_relative_west_black_panics_on_file9() {
        let _ = Square::new(File::File9, Rank::Rank5).relative_west(Color::Black);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_west_white_panics_on_file1() {
        let _ = Square::new(File::File1, Rank::Rank5).relative_west(Color::White);
    }

    #[test]
    pub fn square_relative_north_east() {
        let cases = [
            (Color::Black, Square::new(File::File2, Rank::Rank9), Square::new(File::File1, Rank::Rank8)),
            (Color::Black, Square::new(File::File5, Rank::Rank2), Square::new(File::File4, Rank::Rank1)),
            (Color::Black, Square::new(File::File9, Rank::Rank5), Square::new(File::File8, Rank::Rank4)),
            (Color::White, Square::new(File::File1, Rank::Rank8), Square::new(File::File2, Rank::Rank9)),
            (Color::White, Square::new(File::File5, Rank::Rank1), Square::new(File::File6, Rank::Rank2)),
            (Color::White, Square::new(File::File8, Rank::Rank5), Square::new(File::File9, Rank::Rank6)),
        ];

        for (color, square, relative_north_east) in cases.iter() {
            assert_eq!(square.relative_north_east(*color), *relative_north_east);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_relative_north_east_black_panics_on_rank1() {
        let _ = Square::new(File::File5, Rank::Rank1).relative_north_east(Color::Black);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_north_east_black_panics_on_file1() {
        let _ = Square::new(File::File1, Rank::Rank5).relative_north_east(Color::Black);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_north_east_white_panics_on_rank9() {
        let _ = Square::new(File::File5, Rank::Rank9).relative_north_east(Color::White);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_north_east_white_panics_on_file9() {
        let _ = Square::new(File::File9, Rank::Rank5).relative_north_east(Color::White);
    }

    #[test]
    pub fn square_relative_north_west() {
        let cases = [
            (Color::Black, Square::new(File::File1, Rank::Rank9), Square::new(File::File2, Rank::Rank8)),
            (Color::Black, Square::new(File::File5, Rank::Rank2), Square::new(File::File6, Rank::Rank1)),
            (Color::Black, Square::new(File::File8, Rank::Rank5), Square::new(File::File9, Rank::Rank4)),
            (Color::White, Square::new(File::File2, Rank::Rank8), Square::new(File::File1, Rank::Rank9)),
            (Color::White, Square::new(File::File5, Rank::Rank1), Square::new(File::File4, Rank::Rank2)),
            (Color::White, Square::new(File::File9, Rank::Rank5), Square::new(File::File8, Rank::Rank6)),
        ];

        for (color, square, relative_north_west) in cases.iter() {
            assert_eq!(square.relative_north_west(*color), *relative_north_west);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_relative_north_west_black_panics_on_rank1() {
        let _ = Square::new(File::File5, Rank::Rank1).relative_north_west(Color::Black);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_north_west_black_panics_on_file9() {
        let _ = Square::new(File::File9, Rank::Rank5).relative_north_west(Color::Black);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_north_west_white_panics_on_rank9() {
        let _ = Square::new(File::File5, Rank::Rank9).relative_north_west(Color::White);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_north_west_white_panics_on_file1() {
        let _ = Square::new(File::File1, Rank::Rank5).relative_north_west(Color::White);
    }

    #[test]
    pub fn square_relative_south_east() {
        let cases = [
            (Color::Black, Square::new(File::File2, Rank::Rank8), Square::new(File::File1, Rank::Rank9)),
            (Color::Black, Square::new(File::File5, Rank::Rank1), Square::new(File::File4, Rank::Rank2)),
            (Color::Black, Square::new(File::File9, Rank::Rank5), Square::new(File::File8, Rank::Rank6)),
            (Color::White, Square::new(File::File1, Rank::Rank9), Square::new(File::File2, Rank::Rank8)),
            (Color::White, Square::new(File::File5, Rank::Rank2), Square::new(File::File6, Rank::Rank1)),
            (Color::White, Square::new(File::File8, Rank::Rank5), Square::new(File::File9, Rank::Rank4)),
        ];

        for (color, square, relative_south_east) in cases.iter() {
            assert_eq!(square.relative_south_east(*color), *relative_south_east);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_relative_south_east_black_panics_on_rank9() {
        let _ = Square::new(File::File5, Rank::Rank9).relative_south_east(Color::Black);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_south_east_black_panics_on_file1() {
        let _ = Square::new(File::File1, Rank::Rank5).relative_south_east(Color::Black);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_south_east_white_panics_on_rank1() {
        let _ = Square::new(File::File5, Rank::Rank1).relative_south_east(Color::White);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_south_east_white_panics_on_file9() {
        let _ = Square::new(File::File9, Rank::Rank5).relative_south_east(Color::White);
    }

    #[test]
    pub fn square_relative_south_west() {
        let cases = [
            (Color::Black, Square::new(File::File1, Rank::Rank8), Square::new(File::File2, Rank::Rank9)),
            (Color::Black, Square::new(File::File5, Rank::Rank1), Square::new(File::File6, Rank::Rank2)),
            (Color::Black, Square::new(File::File8, Rank::Rank5), Square::new(File::File9, Rank::Rank6)),
            (Color::White, Square::new(File::File2, Rank::Rank9), Square::new(File::File1, Rank::Rank8)),
            (Color::White, Square::new(File::File5, Rank::Rank2), Square::new(File::File4, Rank::Rank1)),
            (Color::White, Square::new(File::File9, Rank::Rank5), Square::new(File::File8, Rank::Rank4)),
        ];

        for (color, square, relative_south_west) in cases.iter() {
            assert_eq!(square.relative_south_west(*color), *relative_south_west);
        }
    }

    #[test]
    #[should_panic]
    pub fn square_relative_south_west_black_panics_on_rank9() {
        let _ = Square::new(File::File5, Rank::Rank9).relative_south_west(Color::Black);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_south_west_black_panics_on_file9() {
        let _ = Square::new(File::File9, Rank::Rank5).relative_south_west(Color::Black);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_south_west_white_panics_on_rank1() {
        let _ = Square::new(File::File5, Rank::Rank1).relative_south_west(Color::White);
    }

    #[test]
    #[should_panic]
    pub fn square_relative_south_west_white_panics_on_file1() {
        let _ = Square::new(File::File1, Rank::Rank5).relative_south_west(Color::White);
    }

    #[test]
    pub fn square_with_file() {
        let cases = [
            (Square::new(File::File1, Rank::Rank9), File::File5, Square::new(File::File5, Rank::Rank9)),
            (Square::new(File::File5, Rank::Rank1), File::File9, Square::new(File::File9, Rank::Rank1)),
            (Square::new(File::File9, Rank::Rank5), File::File1, Square::new(File::File1, Rank::Rank5)),
        ];

        for (square, file, flipped) in cases.iter() {
            assert_eq!(square.with_file(*file), *flipped);
        }
    }

    #[test]
    pub fn square_with_rank() {
        let cases = [
            (Square::new(File::File1, Rank::Rank9), Rank::Rank1, Square::new(File::File1, Rank::Rank1)),
            (Square::new(File::File5, Rank::Rank1), Rank::Rank5, Square::new(File::File5, Rank::Rank5)),
            (Square::new(File::File9, Rank::Rank5), Rank::Rank9, Square::new(File::File9, Rank::Rank9)),
        ];

        for (square, rank, flipped) in cases.iter() {
            assert_eq!(square.with_rank(*rank), *flipped);
        }
    }

    #[test]
    pub fn square_flip_file() {
        let cases = [
            (Square::new(File::File1, Rank::Rank9), Square::new(File::File9, Rank::Rank9)),
            (Square::new(File::File5, Rank::Rank1), Square::new(File::File5, Rank::Rank1)),
            (Square::new(File::File9, Rank::Rank5), Square::new(File::File1, Rank::Rank5)),
        ];

        for (square, flipped) in cases.iter() {
            assert_eq!(square.flip_file(), *flipped);
        }
    }

    #[test]
    pub fn square_flip_rank() {
        let cases = [
            (Square::new(File::File1, Rank::Rank9), Square::new(File::File1, Rank::Rank1)),
            (Square::new(File::File5, Rank::Rank1), Square::new(File::File5, Rank::Rank9)),
            (Square::new(File::File9, Rank::Rank5), Square::new(File::File9, Rank::Rank5)),
        ];

        for (square, flipped) in cases.iter() {
            assert_eq!(square.flip_rank(), *flipped);
        }
    }

    #[test]
    pub fn square_rotate() {
        let cases = [
            (Square::new(File::File1, Rank::Rank9), Square::new(File::File9, Rank::Rank1)),
            (Square::new(File::File5, Rank::Rank1), Square::new(File::File5, Rank::Rank9)),
            (Square::new(File::File9, Rank::Rank5), Square::new(File::File1, Rank::Rank5)),
        ];

        for (square, rotated) in cases.iter() {
            assert_eq!(square.rotate(), *rotated);
        }
    }

    #[test]
    pub fn can_promote() {
        let cases = [
            (Color::Black, Square::new(File::File1, Rank::Rank9), false),
            (Color::Black, Square::new(File::File5, Rank::Rank1), true),
            (Color::Black, Square::new(File::File9, Rank::Rank5), false),
            (Color::White, Square::new(File::File1, Rank::Rank9), true),
            (Color::White, Square::new(File::File5, Rank::Rank1), false),
            (Color::White, Square::new(File::File9, Rank::Rank5), false),
        ];

        for (color, square, can_promote) in cases.iter() {
            assert_eq!(square.can_promote(*color), *can_promote);
        }
    }
}
