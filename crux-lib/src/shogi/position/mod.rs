pub mod hand;
pub mod key;
pub mod mv;
pub mod zobrist;

use std::fmt::{Display, Formatter, Result};

use const_for::const_for;

use crate::shogi::{
    attacks::{
        bishop_pseudo_attacks, gold_attacks, king_attacks, knight_attacks, lance_pseudo_attacks,
        pawn_attacks, ray_between, rook_pseudo_attacks, silver_attacks,
    },
    bitboard::Bitboard,
    core::{Color, File, Piece, PieceType, Rank, Square, MAX_KING},
    position::{
        hand::Hand,
        key::Key,
        mv::Move,
        zobrist::{hand_key, piece_square_key, side_key},
    },
};

/// Represents a shogi position.
///
/// This struct stores both the primary game state (piece placement, side to move,
/// pieces in hand) and derived information used for fast move generation and
/// legality checks, such as checkers, pinners, and pinned pieces.
#[derive(Debug, Clone)]
pub struct Position {
    side_to_move: Color,
    mailbox: [Option<Piece>; Square::COUNT],
    hands: [Hand; Color::COUNT],
    color_bb: [Bitboard; Color::COUNT],
    piece_type_bb: [Bitboard; PieceType::COUNT],
    king_squares: [Option<Square>; Color::COUNT],
    checkers: Bitboard,
    pinners: Bitboard,
    pinned: Bitboard,
    ply: u32,
    key: Key,
}

impl const Default for Position {
    /// Same as [`Position::empty`].
    fn default() -> Self {
        Self::empty()
    }
}

impl Position {
    /// Creates an empty `Position`.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            side_to_move: Color::Black,
            mailbox: [None; Square::COUNT],
            hands: [Hand::default(); Color::COUNT],
            color_bb: [Bitboard::empty(); Color::COUNT],
            piece_type_bb: [Bitboard::empty(); PieceType::COUNT],
            king_squares: [None; Color::COUNT],
            checkers: Bitboard::empty(),
            pinners: Bitboard::empty(),
            pinned: Bitboard::empty(),
            ply: 0,
            key: Key::default(),
        }
    }

    /// Creates the standard initial position.
    #[must_use]
    pub const fn startpos() -> Self {
        const STARTPOS: Position = {
            let mut builder = Position::default().builder();

            const fn place(
                builder: &mut PositionBuilder,
                squares: &[Square],
                piece_type: PieceType,
            ) {
                const_for!(i in 0..squares.len() => {
                    builder
                        .place(squares[i], piece_type.with_color(Color::Black))
                        .place(squares[i].rotate180(), piece_type.with_color(Color::White));
                });
            }

            const PAWN_SQUARES: [Square; 9] = [
                Square::S17,
                Square::S27,
                Square::S37,
                Square::S47,
                Square::S57,
                Square::S67,
                Square::S77,
                Square::S87,
                Square::S97,
            ];

            const LANCE_SQUARES: [Square; 2] = [Square::S19, Square::S99];
            const KNIGHT_SQUARES: [Square; 2] = [Square::S29, Square::S89];
            const SILVER_SQUARES: [Square; 2] = [Square::S39, Square::S79];
            const GOLD_SQUARES: [Square; 2] = [Square::S49, Square::S69];
            const BISHOP_SQUARE: [Square; 1] = [Square::S88];
            const ROOK_SQUARE: [Square; 1] = [Square::S28];
            const KING_SQUARE: [Square; 1] = [Square::S59];

            place(&mut builder, &PAWN_SQUARES, PieceType::Pawn);
            place(&mut builder, &LANCE_SQUARES, PieceType::Lance);
            place(&mut builder, &KNIGHT_SQUARES, PieceType::Knight);
            place(&mut builder, &SILVER_SQUARES, PieceType::Silver);
            place(&mut builder, &GOLD_SQUARES, PieceType::Gold);
            place(&mut builder, &BISHOP_SQUARE, PieceType::Bishop);
            place(&mut builder, &ROOK_SQUARE, PieceType::Rook);
            place(&mut builder, &KING_SQUARE, PieceType::King);

            builder.build()
        };

        STARTPOS
    }

    /// Returns a builder for modifying this `Position`.
    #[must_use]
    pub const fn builder(self) -> PositionBuilder {
        PositionBuilder(self)
    }

    /// Applies a non-special move to the position.
    ///
    /// Updates the board state, hands, side to move, ply, and checker-related state.
    /// /// Returns `Some(piece)` if a piece was captured, or `None` otherwise.
    ///
    /// # Debug assertions
    /// In debug builds, panics if the move is special or
    /// inconsistent with the current position.
    pub const fn make_move(&mut self, mv: Move) -> Option<Piece> {
        let stm = self.side_to_move();
        let nstm = stm.opposite();
        let to_piece = self.piece_at(mv.to());
        let moved_piece: Piece;

        if mv.is_drop() {
            debug_assert!(to_piece.is_none());

            moved_piece = mv.drop_piece_type().with_color(stm);
            self.decrement_hand_piece_count(stm, mv.drop_piece_type());
        } else {
            let moving_piece = self.piece_at(mv.from()).unwrap();
            debug_assert!(moving_piece.color() == stm);

            moved_piece = if mv.is_promotion() {
                moving_piece.promoted()
            } else {
                moving_piece
            };

            self.remove(mv.from());

            if let Some(captured) = to_piece {
                debug_assert!(captured.color() == nstm);

                self.increment_hand_piece_count(stm, captured.piece_type().unpromoted());
                self.remove(mv.to());
            }
        }

        self.place(mv.to(), moved_piece);
        self.set_side_to_move(nstm);
        self.ply += 1;

        // Update checker and pins.
        // For non-sliding pieces, only moves to `mv.to()` can give check,
        // so checking that square is sufficient.
        self.clear_checker_states();
        self.update_checkers_for(mv.to());
        self.update_sliding_checkers_and_pins();

        to_piece
    }

    /// Reverts a previously applied non-special move.
    ///
    /// Restores the board state, hands, side to move, ply, and checker-related state.
    /// The `captured` piece must be the value returned by [`Position::make_move`].
    ///
    /// # Debug assertions
    /// In debug builds, panics if the move is special or
    /// inconsistent with the current position.
    pub const fn unmake_move(&mut self, mv: Move, captured: Option<Piece>) {
        debug_assert!(self.ply > 0);

        let stm = self.side_to_move();
        let nstm = stm.opposite();
        let moved_piece = self.piece_at(mv.to()).unwrap();
        debug_assert!(moved_piece.color() == nstm);

        self.remove(mv.to());

        if mv.is_drop() {
            debug_assert!(captured.is_none());

            self.increment_hand_piece_count(nstm, moved_piece.piece_type().unpromoted());
        } else {
            let moving_piece = if mv.is_promotion() {
                moved_piece.unpromoted()
            } else {
                moved_piece
            };

            self.place(mv.from(), moving_piece);
        }

        if let Some(captured) = captured {
            debug_assert!(captured.color() == stm);

            self.place(mv.to(), captured);
            self.decrement_hand_piece_count(nstm, captured.piece_type().unpromoted());
        }

        self.set_side_to_move(nstm);
        self.ply -= 1;

        // Update checkers and pins.
        // Unlike make_move, if the king was moved, we must enumerate all
        // potential non-sliding piece checkers to recompute the state correctly.
        self.clear_checker_states();
        self.update_non_sliding_checkers();
        self.update_sliding_checkers_and_pins();
    }

    /// Returns the side to move.
    #[must_use]
    pub const fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    /// Returns `Some(piece)` if the given square is occupied, or `None` otherwise.
    #[must_use]
    pub const fn piece_at(&self, square: Square) -> Option<Piece> {
        self.mailbox[square]
    }

    /// Returns `true` if any piece occupies the given square.
    #[must_use]
    pub const fn has_any(&self, square: Square) -> bool {
        self.piece_at(square).is_some()
    }

    /// Returns `true` if the given square is empty.
    #[must_use]
    pub const fn is_empty(&self, square: Square) -> bool {
        self.piece_at(square).is_none()
    }

    /// Returns the hand of the given color.
    #[must_use]
    pub const fn hand(&self, color: Color) -> Hand {
        self.hands[color]
    }

    /// Returns a bitboard of all pieces of the given color.
    #[must_use]
    pub const fn color_bb(&self, color: Color) -> Bitboard {
        self.color_bb[color]
    }

    /// Returns a bitboard of all pieces of the given piece type.
    #[must_use]
    pub const fn piece_type_bb(&self, piece_type: PieceType) -> Bitboard {
        self.piece_type_bb[piece_type]
    }

    /// Returns a bitboard of all pieces of the given piece.
    #[must_use]
    pub const fn piece_bb(&self, piece: Piece) -> Bitboard {
        self.color_bb(piece.color()) & self.piece_type_bb(piece.piece_type())
    }

    /// Returns a bitboard of all occupied squares.
    #[must_use]
    pub const fn occupancy(&self) -> Bitboard {
        self.color_bb(Color::Black) | self.color_bb(Color::White)
    }

    /// Returns the square of the king of the given color, if present.
    #[must_use]
    pub const fn king_square(&self, color: Color) -> Option<Square> {
        self.king_squares[color]
    }

    /// Returns a bitboard of pieces currently giving check to
    /// the king of the side to move.
    #[must_use]
    pub const fn checkers(&self) -> Bitboard {
        self.checkers
    }

    /// Returns a bitboard of opponent sliding pieces that pin
    /// a piece of the side to move to its king.
    #[must_use]
    pub const fn pinners(&self) -> Bitboard {
        self.pinners
    }

    /// Returns a bitboard of pieces of the side to move that are
    /// pinned to their king by an opponent sliding piece.
    #[must_use]
    pub const fn pinned(&self) -> Bitboard {
        self.pinned
    }

    /// Returns the current ply count.
    #[must_use]
    pub const fn ply(&self) -> u32 {
        self.ply
    }

    /// Returns the Zobrist hash key of the position.
    #[must_use]
    pub const fn key(&self) -> Key {
        self.key
    }

    const fn set_side_to_move(&mut self, side_to_move: Color) {
        if self.side_to_move != side_to_move {
            self.side_to_move = side_to_move;
            self.key ^= side_key();
        }
    }

    const fn place(&mut self, square: Square, piece: Piece) {
        debug_assert!(self.is_empty(square));

        let color = piece.color();
        let pt = piece.piece_type();
        let bit = square.bit();

        debug_assert!(!self.color_bb(color).contains(square));
        debug_assert!(!self.piece_type_bb(pt).contains(square));

        self.mailbox[square] = Some(piece);
        self.color_bb[color] |= bit;
        self.piece_type_bb[pt] |= bit;

        if pt == PieceType::King {
            self.king_squares[color] = Some(square);
        }

        self.key ^= piece_square_key(piece, square);
    }

    const fn remove(&mut self, square: Square) {
        debug_assert!(self.has_any(square));

        let piece = self.mailbox[square].unwrap();
        let color = piece.color();
        let pt = piece.piece_type();
        let bit = square.bit();

        debug_assert!(self.color_bb(color).contains(square));
        debug_assert!(self.piece_type_bb(pt).contains(square));

        self.mailbox[square] = None;
        self.color_bb[color] ^= bit;
        self.piece_type_bb[pt] ^= bit;

        if pt == PieceType::King {
            self.king_squares[color] = None;
        }

        self.key ^= piece_square_key(piece, square);
    }

    const fn set_hand_piece_count(&mut self, color: Color, piece_type: PieceType, new_count: u32) {
        self.switch_hand_key(
            color,
            piece_type,
            self.hands[color].count(piece_type),
            new_count,
        );
        self.hands[color].set(piece_type, new_count);
    }

    const fn increment_hand_piece_count(&mut self, color: Color, piece_type: PieceType) {
        self.set_hand_piece_count(color, piece_type, self.hands[color].count(piece_type) + 1);
    }

    const fn decrement_hand_piece_count(&mut self, color: Color, piece_type: PieceType) {
        let current = self.hands[color].count(piece_type);
        debug_assert!(current > 0);

        self.set_hand_piece_count(color, piece_type, current - 1)
    }

    const fn switch_hand_key(
        &mut self,
        color: Color,
        piece_type: PieceType,
        old_count: u32,
        new_count: u32,
    ) {
        let diff = hand_key(color, piece_type, old_count) ^ hand_key(color, piece_type, new_count);

        self.key ^= diff;
    }

    const fn update_checkers_for(&mut self, square: Square) {
        // Note: Sliding attacks are handled separately in `update_non_sliding_checkers`,
        //       so they are ignored here.
        const fn adjacent_attacks(piece: Piece, square: Square) -> Bitboard {
            match piece.piece_type() {
                PieceType::Pawn | PieceType::Lance => pawn_attacks(piece.color(), square),
                PieceType::Knight => knight_attacks(piece.color(), square),
                PieceType::Silver => silver_attacks(piece.color(), square),
                PieceType::Gold
                | PieceType::ProPawn
                | PieceType::ProLance
                | PieceType::ProKnight
                | PieceType::ProSilver => gold_attacks(piece.color(), square),
                PieceType::Bishop | PieceType::Rook => Bitboard::empty(),
                PieceType::Horse | PieceType::Dragon | PieceType::King => king_attacks(square),
            }
        }

        let stm = self.side_to_move();
        let piece = self.piece_at(square).unwrap();

        debug_assert!(piece.color() != stm);

        if let Some(king_square) = self.king_square(stm)
            && adjacent_attacks(piece, square).contains(king_square)
        {
            self.checkers |= square.bit();
        }
    }

    const fn update_non_sliding_checkers(&mut self) {
        let stm = self.side_to_move();

        if let Some(king_square) = self.king_square(stm) {
            let pawns = self.piece_type_bb(PieceType::Pawn);
            let knights = self.piece_type_bb(PieceType::Knight);
            let silvers = self.piece_type_bb(PieceType::Silver);
            let golds = self.piece_type_bb(PieceType::Gold)
                | self.piece_type_bb(PieceType::ProPawn)
                | self.piece_type_bb(PieceType::ProLance)
                | self.piece_type_bb(PieceType::ProKnight)
                | self.piece_type_bb(PieceType::ProSilver);

            self.checkers |= ((pawns & pawn_attacks(stm, king_square))
                | (knights & knight_attacks(stm, king_square))
                | (silvers & silver_attacks(stm, king_square))
                | (golds & gold_attacks(stm, king_square)))
                & self.color_bb(stm.opposite());
        }
    }

    const fn update_sliding_checkers_and_pins(&mut self) {
        let stm = self.side_to_move();

        if let Some(king_square) = self.king_square(stm) {
            let lances = self.piece_type_bb(PieceType::Lance);
            let bishops =
                self.piece_type_bb(PieceType::Bishop) | self.piece_type_bb(PieceType::Horse);
            let rooks = self.piece_type_bb(PieceType::Rook) | self.piece_type_bb(PieceType::Dragon);

            let mut snipers = ((lances & lance_pseudo_attacks(stm, king_square))
                | (bishops & bishop_pseudo_attacks(king_square))
                | (rooks & rook_pseudo_attacks(king_square)))
                & self.color_bb(stm.opposite());

            let occ = self.occupancy() ^ snipers;

            while snipers.has_any() {
                let square = snipers.pop_lsb();
                let blockers = ray_between(king_square, square) & occ;

                if blockers.is_empty() {
                    self.checkers |= square.bit();
                } else if blockers.is_single() && (blockers & self.color_bb(stm)).has_any() {
                    self.pinners |= square.bit();
                    self.pinned |= blockers;
                }
            }
        }
    }

    const fn clear_checker_states(&mut self) {
        self.checkers = Bitboard::empty();
        self.pinners = Bitboard::empty();
        self.pinned = Bitboard::empty();
    }
}

/// Formats the position as a human-readable shogi board.
///
/// The board is shown as a 9x9 grid with files labeled 9..1 from left to right
/// and ranks labeled 一..九 on the right.
/// Each square displays the piece on it, if any, prefixed with its color
/// ('b' for Black, 'w' for White).
///
/// After the board, the side to move, pieces in hand for each side,
/// the current ply count, and the position hash key are displayed.
///
/// Example output:
/// ```text
///   9   8   7   6   5   4   3   2   1
/// +---+---+---+---+---+---+---+---+---+
/// |w香|w桂|w銀|w金|w玉|w金|w銀|w桂|w香| 一
/// +---+---+---+---+---+---+---+---+---+
/// |   |w飛|   |   |   |   |   |w角|   | 二
/// +---+---+---+---+---+---+---+---+---+
/// |w歩|w歩|w歩|w歩|w歩|w歩|w歩|w歩|w歩| 三
/// +---+---+---+---+---+---+---+---+---+
/// |   |   |   |   |   |   |   |   |   | 四
/// +---+---+---+---+---+---+---+---+---+
/// |   |   |   |   |   |   |   |   |   | 五
/// +---+---+---+---+---+---+---+---+---+
/// |   |   |   |   |   |   |   |   |   | 六
/// +---+---+---+---+---+---+---+---+---+
/// |b歩|b歩|b歩|b歩|b歩|b歩|b歩|b歩|b歩| 七
/// +---+---+---+---+---+---+---+---+---+
/// |   |b角|   |   |   |   |   |b飛|   | 八
/// +---+---+---+---+---+---+---+---+---+
/// |b香|b桂|b銀|b金|b玉|b金|b銀|b桂|b香| 九
/// +---+---+---+---+---+---+---+---+---+
///
/// Side to Move : Black
/// Hand (Black) : None
/// Hand (White) : None
/// Moves        : 0
/// Key          : 88abfff4d6167b4
/// ```
impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result {
        const COLOR_TO_STR: [&str; Color::COUNT] = ["Black", "White"];

        const RANK_TO_CHAR: [char; Rank::COUNT] =
            ['一', '二', '三', '四', '五', '六', '七', '八', '九'];

        const PIECE_TYPE_TO_CHAR: [char; PieceType::COUNT] = [
            '歩', '香', '桂', '銀', '金', '角', '飛', 'と', '杏', '圭', '全', '馬', '龍', '玉',
        ];

        const RANK_SEPARATOR: &str = "+---+---+---+---+---+---+---+---+---+";

        writeln!(f, "  9   8   7   6   5   4   3   2   1")?;
        writeln!(f, "{RANK_SEPARATOR}")?;

        for (&rank, rank_char) in Rank::ALL.iter().zip(RANK_TO_CHAR) {
            for &file in File::ALL.iter().rev() {
                let square = Square::new(file, rank);

                if let Some(piece) = self.piece_at(square) {
                    let color = piece.color();
                    let pt = piece.piece_type();
                    let pt_str = PIECE_TYPE_TO_CHAR[pt];

                    write!(
                        f,
                        "|{}{pt_str}",
                        if color == Color::Black { 'b' } else { 'w' }
                    )?;
                } else {
                    write!(f, "|   ")?;
                }
            }

            writeln!(f, "| {rank_char}")?;
            writeln!(f, "{RANK_SEPARATOR}")?;
        }

        writeln!(f)?;
        writeln!(f, "Side to Move : {}", COLOR_TO_STR[self.side_to_move()])?;

        for (&color, color_str) in Color::ALL.iter().zip(COLOR_TO_STR) {
            write!(f, "Hand ({color_str}) : ")?;

            let hand = self.hand(color);

            if hand.is_empty() {
                writeln!(f, "None")?;
                continue;
            }

            let mut parts = Vec::new();

            for (&piece_type, piece_type_char) in PieceType::ALL
                .iter()
                .zip(PIECE_TYPE_TO_CHAR)
                .take(Hand::HAND_PIECE_TYPES)
                .rev()
            {
                let count = hand.count(piece_type);

                if count != 0 {
                    parts.push(if count > 1 {
                        format!("{}x{}", piece_type_char, count)
                    } else {
                        format!("{}", piece_type_char)
                    });
                }
            }

            writeln!(f, "{}", parts.join(", "))?;
        }

        write!(f, "Moves        : {}", self.ply())?;
        writeln!(f)?;
        write!(f, "Key          : {:x}", self.key().value())?;

        Ok(())
    }
}

/// A builder for constructing a `Position` incrementally.
pub struct PositionBuilder(Position);

impl PositionBuilder {
    /// Sets the side to move.
    pub const fn set_side_to_move(&mut self, side_to_move: Color) -> &mut Self {
        self.0.set_side_to_move(side_to_move);
        self
    }

    /// Places a piece on the given square.
    pub const fn place(&mut self, square: Square, piece: Piece) -> &mut Self {
        self.0.place(square, piece);
        self
    }

    /// Removes any piece from the given square.
    pub const fn remove(&mut self, square: Square) -> &mut Self {
        self.0.remove(square);
        self
    }

    /// Sets the number of pieces of the given piece type in hand for the given color.
    pub const fn set_hand_piece_count(
        &mut self,
        color: Color,
        piece_type: PieceType,
        count: u32,
    ) -> &mut Self {
        self.0.set_hand_piece_count(color, piece_type, count);
        self
    }

    /// Increments the hand piece count for the given color and piece type.
    pub const fn increment_hand_piece_count(
        &mut self,
        color: Color,
        piece_type: PieceType,
    ) -> &mut Self {
        self.0.increment_hand_piece_count(color, piece_type);
        self
    }

    /// Decrements the hand piece count for the given color and piece type.
    pub const fn decrement_hand_piece_count(
        &mut self,
        color: Color,
        piece_type: PieceType,
    ) -> &mut Self {
        self.0.decrement_hand_piece_count(color, piece_type);
        self
    }

    /// Set the ply.
    pub const fn set_ply(&mut self, ply: u32) -> &mut Self {
        self.0.ply = ply;
        self
    }

    /// Verifies that the position is structurally valid.
    ///
    /// This function checks only *basic invariants* of a shogi position:
    ///
    /// - Total piece counts do not exceed the maximum allowed
    ///   (including promoted pieces and pieces in hand)
    /// - There are no illegal pieces with no legal moves
    ///   (pawns/lances on the last rank, knights on the last two ranks)
    /// - There are no double pawns
    ///
    /// It does NOT check dynamic legality, such as whether the non-side-to-move king is in check.
    #[must_use]
    pub const fn verify(&self) -> bool {
        let black_hand = self.0.hand(Color::Black);
        let white_hand = self.0.hand(Color::White);

        const_for!(piece_type in 0..Hand::HAND_PIECE_TYPES => {
            let piece_type = PieceType::from(piece_type);

            let mut total = self.0.piece_type_bb(piece_type).count_ones() + black_hand.count(piece_type) + white_hand.count(piece_type);

            if piece_type != piece_type.promoted() {
                total += self.0.piece_type_bb(piece_type.promoted()).count_ones();
            }

            if total > Hand::max_piece_counts(piece_type) {
                return false;
            }
        });

        let kings_count = self.0.piece_type_bb(PieceType::King).count_ones();

        if kings_count > MAX_KING {
            return false;
        }

        const_for!(color in 0..Color::COUNT => {
            let color = Color::from(color);

            let pawns = self.0.piece_bb(PieceType::Pawn.with_color(color));

            if ((pawns
                | self.0.piece_bb(PieceType::Lance.with_color(color)))
                & Rank::Rank1.relative(color).bit())
            .has_any()
            {
                return false;
            }

            if (self.0.piece_bb(PieceType::Knight.with_color(color))
                & (Rank::Rank1.relative(color).bit() | Rank::Rank2.relative(color).bit()))
            .has_any()
            {
                return false;
            }

            const_for!(file in 0..File::COUNT => {
                let file = File::from(file);

                if (pawns & file.bit()).is_multiple() {
                    return false;
                }
            });
        });

        true
    }

    /// Builds the `Position`.
    ///
    /// # Debug assertions
    /// In debug builds, this function panics if the position fails structural validation.
    /// See [`PositionBuilder::verify`] for the details of the checks performed.
    #[must_use]
    pub const fn build(mut self) -> Position {
        debug_assert!(self.verify());

        self.0.clear_checker_states();
        self.0.update_non_sliding_checkers();
        self.0.update_sliding_checkers_and_pins();

        self.0
    }
}
