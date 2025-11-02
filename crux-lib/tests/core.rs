use crux_lib::shogi::core::{Color, Piece, PieceType};

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
}
