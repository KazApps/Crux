pub mod hand;
pub mod key;
pub mod zobrist;

use const_for::const_for;

use crate::shogi::{
    attacks::{
        bishop_pseudo_attacks, gold_attacks, knight_attacks, lance_pseudo_attacks, pawn_attacks,
        ray_between, rook_pseudo_attacks, silver_attacks,
    },
    bitboard::Bitboard,
    core::{Color, File, Piece, PieceType, Rank, Square, MAX_KING},
    position::{
        hand::Hand,
        key::Key,
        zobrist::{hand_key, piece_square_key, side_key},
    },
};

#[derive(Debug, PartialEq, Eq)]
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
    fn default() -> Self {
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
}

impl Position {
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

    #[must_use]
    pub const fn builder(self) -> PositionBuilder {
        PositionBuilder(self)
    }

    #[must_use]
    pub const fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    #[must_use]
    pub const fn piece_at(&self, square: Square) -> Option<Piece> {
        self.mailbox[square.as_usize()]
    }

    #[must_use]
    pub const fn has_any(&self, square: Square) -> bool {
        self.piece_at(square).is_some()
    }

    #[must_use]
    pub const fn is_empty(&self, square: Square) -> bool {
        self.piece_at(square).is_none()
    }

    #[must_use]
    pub const fn hand(&self, color: Color) -> Hand {
        self.hands[color.as_usize()]
    }

    #[must_use]
    pub const fn color_bb(&self, color: Color) -> Bitboard {
        self.color_bb[color.as_usize()]
    }

    #[must_use]
    pub const fn piece_type_bb(&self, piece_type: PieceType) -> Bitboard {
        self.piece_type_bb[piece_type.as_usize()]
    }

    #[must_use]
    pub const fn piece_bb(&self, piece: Piece) -> Bitboard {
        self.color_bb(piece.color()) & self.piece_type_bb(piece.piece_type())
    }

    #[must_use]
    pub const fn occupancy(&self) -> Bitboard {
        self.color_bb(Color::Black) | self.color_bb(Color::White)
    }

    #[must_use]
    pub const fn king_square(&self, color: Color) -> Option<Square> {
        self.king_squares[color.as_usize()]
    }

    #[must_use]
    pub const fn checkers(&self) -> Bitboard {
        self.checkers
    }

    #[must_use]
    pub const fn pinners(&self) -> Bitboard {
        self.pinners
    }

    #[must_use]
    pub const fn pinned(&self) -> Bitboard {
        self.pinned
    }

    #[must_use]
    pub const fn ply(&self) -> u32 {
        self.ply
    }

    #[must_use]
    pub const fn key(&self) -> Key {
        self.key
    }

    const fn set_side_to_move(&mut self, side_to_move: Color) {
        if self.side_to_move.as_u8() != side_to_move.as_u8() {
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

        self.mailbox[square.as_usize()] = Some(piece);
        self.color_bb[color.as_usize()] |= bit;
        self.piece_type_bb[pt.as_usize()] |= bit;

        if pt == PieceType::King {
            self.king_squares[color.as_usize()] = Some(square);
        }

        self.key ^= piece_square_key(piece, square);
    }

    const fn remove(&mut self, square: Square) {
        debug_assert!(self.has_any(square));

        let piece = self.mailbox[square.as_usize()].unwrap();
        let color = piece.color();
        let pt = piece.piece_type();
        let bit = square.bit();

        debug_assert!(self.color_bb(color).contains(square));
        debug_assert!(self.piece_type_bb(pt).contains(square));

        self.mailbox[square.as_usize()] = None;
        self.color_bb[color.as_usize()] ^= bit;
        self.piece_type_bb[pt.as_usize()] ^= bit;

        if pt == PieceType::King {
            self.king_squares[color.as_usize()] = None;
        }

        self.key ^= piece_square_key(piece, square);
    }

    const fn set_hand_piece_count(&mut self, color: Color, piece_type: PieceType, count: u32) {
        self.switch_hand_key(
            color,
            piece_type,
            self.hands[color.as_usize()].count(piece_type),
            count,
        );
        self.hands[color.as_usize()].set(piece_type, count);
    }

    const fn increment_hand_piece_count(&mut self, color: Color, piece_type: PieceType) {
        self.set_hand_piece_count(
            color,
            piece_type,
            self.hands[color.as_usize()].count(piece_type) + 1,
        );
    }

    const fn decrement_hand_piece_count(&mut self, color: Color, piece_type: PieceType) {
        self.set_hand_piece_count(
            color,
            piece_type,
            self.hands[color.as_usize()].count(piece_type) - 1,
        )
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

    const fn update_non_sliding_checkers(&mut self) {
        let stm = self.side_to_move();

        if let Some(king_square) = self.king_square(stm) {
            let pawns = self.piece_type_bb(PieceType::Pawn);
            let knights = self.piece_type_bb(PieceType::Knight);
            let silvers = self.piece_type_bb(PieceType::Silver);
            let golds = self.piece_type_bb(PieceType::Gold)
                | self.piece_type_bb(PieceType::ProPawn)
                | self.piece_type_bb(PieceType::ProLance)
                | self.piece_type_bb(PieceType::ProSilver);

            self.checkers = ((pawns & pawn_attacks(stm, king_square))
                | (knights & knight_attacks(stm, king_square))
                | (silvers & silver_attacks(stm, king_square))
                | (golds & gold_attacks(stm, king_square)))
                & self.color_bb(stm.opposite());
        }
    }

    const fn update_sliding_checkers_and_pins(&mut self) {
        self.pinners = Bitboard::empty();
        self.pinned = Bitboard::empty();

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
}

pub struct PositionBuilder(Position);

impl PositionBuilder {
    pub const fn set_side_to_move(&mut self, side_to_move: Color) -> &mut Self {
        self.0.set_side_to_move(side_to_move);
        self
    }

    pub const fn place(&mut self, square: Square, piece: Piece) -> &mut Self {
        self.0.place(square, piece);
        self
    }

    pub const fn remove(&mut self, square: Square) -> &mut Self {
        self.0.remove(square);
        self
    }

    pub const fn set_hand_piece_count(
        &mut self,
        color: Color,
        piece_type: PieceType,
        count: u32,
    ) -> &mut Self {
        self.0.set_hand_piece_count(color, piece_type, count);
        self
    }

    pub const fn increment_hand_piece_count(
        &mut self,
        color: Color,
        piece_type: PieceType,
    ) -> &mut Self {
        self.0.increment_hand_piece_count(color, piece_type);
        self
    }

    pub const fn decrement_hand_piece_count(
        &mut self,
        color: Color,
        piece_type: PieceType,
    ) -> &mut Self {
        self.0.decrement_hand_piece_count(color, piece_type);
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

    pub const fn build(mut self) -> Position {
        debug_assert!(self.verify());

        self.0.update_non_sliding_checkers();
        self.0.update_sliding_checkers_and_pins();

        self.0
    }
}
