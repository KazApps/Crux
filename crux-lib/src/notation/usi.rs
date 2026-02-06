use std::fmt::Write;

use crate::{
    notation::Notation,
    shogi::{
        core::{Color, File, Piece, PieceType, Rank, Square},
        position::{hand::Hand, mv::Move, Position},
    },
};

#[derive(Debug, Copy, Clone)]
pub enum ParseSquareError {
    InvalidLength,
    InvalidFile,
    InvalidRank,
}

#[derive(Debug, Copy, Clone)]
pub enum ParseMoveError {
    InvalidFormat,
    InvalidFromSquare(ParseSquareError),
    InvalidToSquare(ParseSquareError),
    InvalidDropPieceType,
    InvalidPromotion,
}

#[derive(Debug, Copy, Clone)]
pub enum ParsePositionError {
    InvalidFormat,
    InvalidBoardRankCount,
    InvalidBoardFormat,
    InvalidBoardPiece,
    InvalidSideToMove,
    InvalidHandFormat,
    InvalidHandPieceType,
    InvalidPly,
}

pub struct Usi;

impl Notation for Usi {
    type ParseSquareError = ParseSquareError;
    type ParseMoveError = ParseMoveError;
    type ParsePositionError = ParsePositionError;

    fn parse_square(s: &str) -> Result<Square, Self::ParseSquareError> {
        let bytes = s.as_bytes();

        if bytes.len() != 2 {
            return Err(ParseSquareError::InvalidLength);
        }

        let file = bytes[0]
            .checked_sub(b'1')
            .ok_or(ParseSquareError::InvalidFile)?;

        if file >= 9 {
            return Err(ParseSquareError::InvalidFile);
        }

        let rank = bytes[1]
            .checked_sub(b'a')
            .ok_or(ParseSquareError::InvalidRank)?;

        if rank >= 9 {
            return Err(ParseSquareError::InvalidRank);
        }

        Ok(Square::new(File::from(file), Rank::from(rank)))
    }

    fn parse_move(s: &str) -> Result<Move, Self::ParseMoveError> {
        let bytes = s.as_bytes();

        if bytes.len() != 4 && (bytes.len() != 5 || bytes[4] != b'+') {
            return Err(ParseMoveError::InvalidFormat);
        }

        let to = Self::parse_square(&s[2..4]).map_err(ParseMoveError::InvalidToSquare)?;

        if bytes[1] == b'*' {
            let piece_type = match bytes[0] {
                b'P' => PieceType::Pawn,
                b'L' => PieceType::Lance,
                b'N' => PieceType::Knight,
                b'S' => PieceType::Silver,
                b'G' => PieceType::Gold,
                b'B' => PieceType::Bishop,
                b'R' => PieceType::Rook,
                _ => return Err(ParseMoveError::InvalidDropPieceType),
            };

            Ok(Move::drop(piece_type, to))
        } else {
            let from = Self::parse_square(&s[..2]).map_err(ParseMoveError::InvalidFromSquare)?;

            if bytes.len() == 4 {
                Ok(Move::normal(from, to))
            } else {
                if matches!(from.rank(), Rank::Rank4 | Rank::Rank5 | Rank::Rank6)
                    && matches!(to.rank(), Rank::Rank4 | Rank::Rank5 | Rank::Rank6)
                {
                    Err(ParseMoveError::InvalidPromotion)
                } else {
                    Ok(Move::promote(from, to))
                }
            }
        }
    }

    fn parse_position(s: &str) -> Result<Position, Self::ParsePositionError> {
        let mut builder = Position::empty().builder();

        let mut it = s.split_whitespace();
        let board = it.next().ok_or(ParsePositionError::InvalidFormat)?;
        let stm = it.next().ok_or(ParsePositionError::InvalidFormat)?;
        let hand = it.next().ok_or(ParsePositionError::InvalidFormat)?;
        let ply = it.next().unwrap_or("1");

        let ranks = board.split('/').collect::<Vec<_>>();

        if ranks.len() != 9 {
            return Err(ParsePositionError::InvalidBoardRankCount);
        }

        for (&rank, rank_str) in Rank::ALL.iter().zip(ranks) {
            let bytes = rank_str.as_bytes();
            let mut file = 9;
            let mut i = 0;

            while i < bytes.len() {
                match bytes[i] {
                    b'1'..=b'9' => {
                        let empty_squares = bytes[i] - b'0';
                        debug_assert!(empty_squares > 0 && file >= empty_squares);
                        file -= empty_squares;
                        i += 1;
                    }

                    b'+' => {
                        if i + 1 >= bytes.len() {
                            return Err(ParsePositionError::InvalidBoardFormat);
                        }

                        let piece = match bytes[i + 1] {
                            b'P' => Piece::BlackProPawn,
                            b'p' => Piece::WhiteProPawn,
                            b'L' => Piece::BlackProLance,
                            b'l' => Piece::WhiteProLance,
                            b'N' => Piece::BlackProKnight,
                            b'n' => Piece::WhiteProKnight,
                            b'S' => Piece::BlackProSilver,
                            b's' => Piece::WhiteProSilver,
                            b'B' => Piece::BlackHorse,
                            b'b' => Piece::WhiteHorse,
                            b'R' => Piece::BlackDragon,
                            b'r' => Piece::WhiteDragon,
                            _ => return Err(ParsePositionError::InvalidBoardPiece),
                        };

                        file -= 1;
                        i += 2;
                        builder.place(Square::new(File::from(file), rank), piece);
                    }

                    piece => {
                        let piece = match piece {
                            b'P' => Piece::BlackPawn,
                            b'p' => Piece::WhitePawn,
                            b'L' => Piece::BlackLance,
                            b'l' => Piece::WhiteLance,
                            b'N' => Piece::BlackKnight,
                            b'n' => Piece::WhiteKnight,
                            b'S' => Piece::BlackSilver,
                            b's' => Piece::WhiteSilver,
                            b'G' => Piece::BlackGold,
                            b'g' => Piece::WhiteGold,
                            b'B' => Piece::BlackBishop,
                            b'b' => Piece::WhiteBishop,
                            b'R' => Piece::BlackRook,
                            b'r' => Piece::WhiteRook,
                            b'K' => Piece::BlackKing,
                            b'k' => Piece::WhiteKing,
                            _ => return Err(ParsePositionError::InvalidBoardPiece),
                        };

                        file -= 1;
                        i += 1;
                        builder.place(Square::new(File::from(file), rank), piece);
                    }
                }
            }

            debug_assert_eq!(file, 0);
        }

        builder.set_side_to_move(match stm {
            "b" => Color::Black,
            "w" => Color::White,
            _ => return Err(ParsePositionError::InvalidSideToMove),
        });

        if hand != "-" {
            let bytes = hand.as_bytes();
            let mut i = 0;

            while i < bytes.len() {
                let mut count = 0;

                while bytes[i].is_ascii_digit() {
                    count = count * 10 + (bytes[i] - b'0') as u32;
                    i += 1;

                    if i >= bytes.len() {
                        return Err(ParsePositionError::InvalidHandFormat);
                    }
                }

                count = count.max(1);

                let piece = match bytes[i] {
                    b'P' => Piece::BlackPawn,
                    b'p' => Piece::WhitePawn,
                    b'L' => Piece::BlackLance,
                    b'l' => Piece::WhiteLance,
                    b'N' => Piece::BlackKnight,
                    b'n' => Piece::WhiteKnight,
                    b'S' => Piece::BlackSilver,
                    b's' => Piece::WhiteSilver,
                    b'G' => Piece::BlackGold,
                    b'g' => Piece::WhiteGold,
                    b'B' => Piece::BlackBishop,
                    b'b' => Piece::WhiteBishop,
                    b'R' => Piece::BlackRook,
                    b'r' => Piece::WhiteRook,
                    _ => return Err(ParsePositionError::InvalidHandPieceType),
                };

                builder.set_hand_piece_count(piece.color(), piece.piece_type(), count);

                i += 1;
            }
        }

        let ply = ply
            .parse::<u32>()
            .map_err(|_| ParsePositionError::InvalidPly)?;
        builder.set_ply(ply - 1);

        Ok(builder.build())
    }

    fn format_square(square: Square) -> String {
        let file_char = (b'1' + square.file().as_u8()) as char;
        let rank_char = (b'a' + square.rank().as_u8()) as char;

        format!("{}{}", file_char, rank_char)
    }

    fn format_move(mv: Move) -> String {
        if mv.is_drop() {
            format!(
                "{}*{}",
                PIECE_TYPE_TO_STR[mv.drop_piece_type()],
                Self::format_square(mv.to())
            )
        } else {
            format!(
                "{}{}{}",
                Self::format_square(mv.from()),
                Self::format_square(mv.to()),
                if mv.is_promotion() { "+" } else { "" }
            )
        }
    }

    fn format_position(position: &Position) -> String {
        let mut result = String::with_capacity(128);

        for rank in Rank::ALL {
            let mut empty_count = 0;

            if rank != Rank::Rank1 {
                result.push('/');
            }

            for &file in File::ALL.iter().rev() {
                if let Some(piece) = position.piece_at(Square::new(file, rank)) {
                    if empty_count != 0 {
                        write!(result, "{}", empty_count).unwrap();
                    }

                    empty_count = 0;

                    result.push_str(PIECE_TO_STR[piece])
                } else {
                    empty_count += 1;
                }
            }

            if empty_count != 0 {
                write!(result, "{}", empty_count).unwrap();
            }
        }

        result.push_str(if position.side_to_move() == Color::Black {
            " b "
        } else {
            " w "
        });

        let black_hand = position.hand(Color::Black);
        let white_hand = position.hand(Color::White);

        if black_hand.is_empty() && white_hand.is_empty() {
            result.push('-');
        } else {
            for (&color, hand) in Color::ALL.iter().zip([black_hand, white_hand]) {
                for &piece_type in PieceType::ALL.iter().take(Hand::HAND_PIECE_TYPES).rev() {
                    let count = hand.count(piece_type);

                    if count == 0 {
                        continue;
                    }

                    if count > 1 {
                        write!(result, "{}", count).unwrap();
                    }

                    result.push_str(PIECE_TO_STR[piece_type.with_color(color)]);
                }
            }
        }

        write!(result, " {}", position.ply() + 1).unwrap();

        result
    }
}

const PIECE_TYPE_TO_STR: [&str; PieceType::COUNT] = [
    "P", "L", "N", "S", "G", "B", "R", "+P", "+L", "+N", "+S", "+B", "+R", "K",
];

const PIECE_TO_STR: [&str; Piece::COUNT] = [
    "P", "p", "L", "l", "N", "n", "S", "s", "G", "g", "B", "b", "R", "r", "+P", "+p", "+L", "+l",
    "+N", "+n", "+S", "+s", "+B", "+b", "+R", "+r", "K", "k",
];
