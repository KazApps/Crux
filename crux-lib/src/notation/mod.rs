use crate::shogi::{
    core::Square,
    position::{mv::Move, Position},
};

pub mod usi;

/// A trait for parsing and formatting textual game notations.
///
/// This trait abstracts over different notation formats used to
/// represent squares, moves, and positions as text.
pub trait Notation {
    /// The error type returned when parsing a square fails.
    type ParseSquareError;

    /// The error type returned when parsing a move fails.
    type ParseMoveError;

    /// The error type returned when parsing a position fails.
    type ParsePositionError;

    /// Parses a square from its textual representation.
    fn parse_square(s: &str) -> Result<Square, Self::ParseSquareError>;

    /// Parses a move from its textual representation.
    fn parse_move(s: &str) -> Result<Move, Self::ParseMoveError>;

    /// Parses a position from its textual representation.
    fn parse_position(s: &str) -> Result<Position, Self::ParsePositionError>;

    /// Formats a square into its textual representation.
    #[must_use]
    fn format_square(square: Square) -> String;

    /// Formats a move into its textual representation.
    #[must_use]
    fn format_move(mv: Move) -> String;

    /// Formats a position into its textual representation.
    #[must_use]
    fn format_position(position: &Position) -> String;
}
