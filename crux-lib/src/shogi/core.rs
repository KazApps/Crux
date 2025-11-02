/// Represents the color of a player or piece in a game.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl Color {
    /// Returns `true` if the color is black.
    #[must_use]
    pub const fn is_black(self) -> bool {
        matches!(self, Color::Black)
    }

    /// Returns `true` if the color is white.
    #[must_use]
    pub const fn is_white(self) -> bool {
        matches!(self, Color::White)
    }

    /// Returns the opposite color.
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}

/// Represents the type of piece.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Lance,
    Knight,
    Silver,
    Gold,
    Bishop,
    Rook,
    ProPawn,
    ProLance,
    ProKnight,
    ProSilver,
    Horse,
    Dragon,
    King,
}

impl PieceType {
    /// Returns a `Piece` of this type with the specified color.
    #[must_use]
    pub const fn with_color(self, color: Color) -> Piece {
        Piece::new(color, self)
    }

    /// Returns the promoted version of this piece type, if any.
    #[must_use]
    pub const fn promoted(self) -> Self {
        match self {
            PieceType::Pawn => PieceType::ProPawn,
            PieceType::Lance => PieceType::ProLance,
            PieceType::Knight => PieceType::ProKnight,
            PieceType::Silver => PieceType::ProSilver,
            PieceType::Bishop => PieceType::Horse,
            PieceType::Rook => PieceType::Dragon,
            _ => self,
        }
    }

    /// Returns the unpromoted version of this piece type, if any.
    #[must_use]
    pub const fn unpromoted(self) -> Self {
        match self {
            PieceType::ProPawn => PieceType::Pawn,
            PieceType::ProLance => PieceType::Lance,
            PieceType::ProKnight => PieceType::Knight,
            PieceType::ProSilver => PieceType::Silver,
            PieceType::Horse => PieceType::Bishop,
            PieceType::Dragon => PieceType::Rook,
            _ => self,
        }
    }

    /// Returns `true` if this piece type is promoted.
    #[must_use]
    pub const fn is_promoted(self) -> bool {
        match self {
            PieceType::ProPawn
            | PieceType::ProLance
            | PieceType::ProKnight
            | PieceType::ProSilver
            | PieceType::Horse
            | PieceType::Dragon => true,
            _ => false,
        }
    }
}

/// Represents a piece in the game.
///
/// `Piece(u8)` encoding layout:
/// bit    0   : `Color`
/// bits 1..=4 : `PieceType` (4 bits)
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    BlackPawn,
    WhitePawn,
    BlackLance,
    WhiteLance,
    BlackKnight,
    WhiteKnight,
    BlackSilver,
    WhiteSilver,
    BlackGold,
    WhiteGold,
    BlackBishop,
    WhiteBishop,
    BlackRook,
    WhiteRook,
    BlackProPawn,
    WhiteProPawn,
    BlackProLance,
    WhiteProLance,
    BlackProKnight,
    WhiteProKnight,
    BlackProSilver,
    WhiteProSilver,
    BlackHorse,
    WhiteHorse,
    BlackDragon,
    WhiteDragon,
    BlackKing,
    WhiteKing,
}

impl Piece {
    /// Creates new `Piece` with the given color and type.
    #[must_use]
    pub const fn new(color: Color, piece_type: PieceType) -> Self {
        let raw = (piece_type as u8) << 1 | (color as u8);

        debug_assert!(raw <= Self::WhiteKing as u8);

        unsafe { std::mem::transmute(raw) }
    }

    /// Returns the color of this piece.
    #[must_use]
    pub const fn color(self) -> Color {
        unsafe { std::mem::transmute(self as u8 & 1) }
    }

    /// Returns the type of this piece.
    #[must_use]
    pub const fn piece_type(self) -> PieceType {
        unsafe { std::mem::transmute(self as u8 >> 1) }
    }

    /// Returns the promoted version of this piece.
    #[must_use]
    pub const fn promoted(self) -> Self {
        self.piece_type().promoted().with_color(self.color())
    }

    /// Returns the unpromoted version of this piece.
    #[must_use]
    pub const fn unpromoted(self) -> Self {
        self.piece_type().unpromoted().with_color(self.color())
    }

    /// Returns `true` if this piece is promoted.
    #[must_use]
    pub const fn is_promoted(self) -> bool {
        self.piece_type().is_promoted()
    }
}
