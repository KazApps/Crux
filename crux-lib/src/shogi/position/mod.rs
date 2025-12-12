pub mod hand;
pub mod key;
pub mod zobrist;

use crate::shogi::{
    bitboard::Bitboard,
    core::{Color, Piece, PieceType, Square},
    position::{hand::Hand, key::Key},
};

#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    stm: Color,
    mailbox: [Option<Piece>; Square::COUNT],
    hands: [Hand; Color::COUNT],
    color_bb: [Bitboard; Color::COUNT],
    piece_type_bb: [Bitboard; PieceType::COUNT],
    king_squares: [Option<Square>; Color::COUNT],
    board_key: Key,
    hand_key: Key,
    checkers: Bitboard,
    pinners: [Bitboard; Color::COUNT],
    pinned: [Bitboard; Color::COUNT],
    ply: u32,
}
