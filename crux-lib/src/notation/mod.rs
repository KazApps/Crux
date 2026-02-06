use crate::shogi::{
    core::Square,
    position::{mv::Move, Position},
};

pub mod usi;

pub trait Notation {
    type ParseSquareError;
    type ParseMoveError;
    type ParsePositionError;

    fn parse_square(s: &str) -> Result<Square, Self::ParseSquareError>;
    fn parse_move(s: &str) -> Result<Move, Self::ParseMoveError>;
    fn parse_position(s: &str) -> Result<Position, Self::ParsePositionError>;

    fn format_square(square: Square) -> String;
    fn format_move(mv: Move) -> String;
    fn format_position(position: &Position) -> String;
}
