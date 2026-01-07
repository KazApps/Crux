use std::{
    mem::transmute,
    ops::{Index, IndexMut},
};

/// Maximum number of pawns that can exist in total,
/// including pieces on the board and in hand.
pub const MAX_PAWN: u32 = 18;

/// Maximum number of lances that can exist in total,
/// including pieces on the board and in hand.
pub const MAX_LANCE: u32 = 4;

/// Maximum number of knights that can exist in total,
/// including pieces on the board and in hand.
pub const MAX_KNIGHT: u32 = 4;

/// Maximum number of silvers that can exist in total,
/// including pieces on the board and in hand.
pub const MAX_SILVER: u32 = 4;

/// Maximum number of golds that can exist in total,
/// including pieces on the board and in hand.
pub const MAX_GOLD: u32 = 4;

/// Maximum number of bishops that can exist in total,
/// including pieces on the board and in hand.
pub const MAX_BISHOP: u32 = 2;

/// Maximum number of rooks that can exist in total,
/// including pieces on the board and in hand.
pub const MAX_ROOK: u32 = 2;

/// Maximum number of kings that can exist in total.
pub const MAX_KING: u32 = 2;

/// Represents the color of a player or piece in the game.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

impl Color {
    /// The number of colors.
    pub const COUNT: usize = 2;

    /// Returns the opposite color.
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }

    /// Returns the `Color` as a `u8`.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Returns the `Color` as a `usize`.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self as usize
    }
}

impl const From<u8> for Color {
    /// Creates a `Color` from the given raw `u8` value.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if `value` is greater than or equal to `COUNT`.
    fn from(value: u8) -> Self {
        debug_assert!(value < Self::COUNT as u8);

        unsafe { transmute(value) }
    }
}

impl const From<usize> for Color {
    /// Creates a `Color` from the given raw `usize` value.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if `value` is greater than or equal to `COUNT`.
    fn from(value: usize) -> Self {
        debug_assert!(value < Self::COUNT);

        unsafe { transmute(value as u8) }
    }
}

impl const PartialEq for Color {
    /// Compares two `Color` values for equality.
    fn eq(&self, other: &Self) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl const Eq for Color {}

impl<T> const Index<Color> for [T; Color::COUNT] {
    type Output = T;

    /// Indexes the array by `Color`.
    fn index(&self, color: Color) -> &Self::Output {
        &self[color.as_usize()]
    }
}

impl<T> const IndexMut<Color> for [T; Color::COUNT] {
    /// Mutably indexes the array by `Color`.
    fn index_mut(&mut self, color: Color) -> &mut Self::Output {
        &mut self[color.as_usize()]
    }
}

/// Represents the types of pieces.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
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
    /// The number of piece types.
    pub const COUNT: usize = 14;

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
        matches!(
            self,
            PieceType::ProPawn
                | PieceType::ProLance
                | PieceType::ProKnight
                | PieceType::ProSilver
                | PieceType::Horse
                | PieceType::Dragon
        )
    }

    /// Returns the `PieceType` as a `u8`.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Returns the `PieceType` as a `usize`.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self as usize
    }
}

impl const From<u8> for PieceType {
    /// Creates a `PieceType` from the given raw `u8` value.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if `value` is greater than or equal to `COUNT`.
    fn from(value: u8) -> Self {
        debug_assert!(value < Self::COUNT as u8);

        unsafe { transmute(value) }
    }
}

impl const From<usize> for PieceType {
    /// Creates a `PieceType` from the given raw `usize` value.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if `value` is greater than or equal to `COUNT`.
    fn from(value: usize) -> Self {
        debug_assert!(value < Self::COUNT);

        unsafe { transmute(value as u8) }
    }
}

impl const PartialEq for PieceType {
    /// Compares two `PieceType` values for equality.
    fn eq(&self, other: &Self) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl const Eq for PieceType {}

impl<T> const Index<PieceType> for [T; PieceType::COUNT] {
    type Output = T;

    /// Indexes the array by `PieceType`.
    fn index(&self, piece_type: PieceType) -> &Self::Output {
        &self[piece_type.as_usize()]
    }
}

impl<T> const IndexMut<PieceType> for [T; PieceType::COUNT] {
    /// Mutably indexes the array by `PieceType`.
    fn index_mut(&mut self, piece_type: PieceType) -> &mut Self::Output {
        &mut self[piece_type.as_usize()]
    }
}

/// Represents a piece in the game.
///
/// `Piece(u8)` is a structure that holds the color and types of pieces.
/// bit    0   : `Color`
/// bits 1..=4 : `PieceType` (4 bits)
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
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
    /// The total number of distinct `Piece` variants.
    pub const COUNT: usize = Color::COUNT * PieceType::COUNT;

    /// Returns `Piece` with the given color and type.
    #[must_use]
    pub const fn new(color: Color, piece_type: PieceType) -> Self {
        let raw = (piece_type.as_u8()) << 1 | (color.as_u8());

        debug_assert!(raw <= Self::WhiteKing.as_u8());

        unsafe { transmute(raw) }
    }

    /// Returns the color of this piece.
    #[must_use]
    pub const fn color(self) -> Color {
        unsafe { transmute(self.as_u8() & 1) }
    }

    /// Returns the type of this piece.
    #[must_use]
    pub const fn piece_type(self) -> PieceType {
        unsafe { transmute(self.as_u8() >> 1) }
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

    /// Returns the `Piece` as a `u8`.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Returns the `Piece` as a `usize`.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self as usize
    }
}

impl const From<u8> for Piece {
    /// Creates a `Piece` from the given raw `u8` value.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if `value` is greater than or equal to `COUNT`.
    fn from(value: u8) -> Self {
        debug_assert!(value < Self::COUNT as u8);

        unsafe { transmute(value) }
    }
}

impl const From<usize> for Piece {
    /// Creates a `Piece` from the given raw `usize` value.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if `value` is greater than or equal to `COUNT`.
    fn from(value: usize) -> Self {
        debug_assert!(value < Self::COUNT);

        unsafe { transmute(value as u8) }
    }
}

impl const PartialEq for Piece {
    /// Compares two `Piece` values for equality.
    fn eq(&self, other: &Self) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl const Eq for Piece {}

impl<T> const Index<Piece> for [T; Piece::COUNT] {
    type Output = T;

    /// Indexes the array by `Piece`.
    fn index(&self, piece: Piece) -> &Self::Output {
        &self[piece.as_usize()]
    }
}

impl<T> const IndexMut<Piece> for [T; Piece::COUNT] {
    /// Mutably indexes the array by `Piece`.
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self[piece.as_usize()]
    }
}

/// Represents a file on the board.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum File {
    File1,
    File2,
    File3,
    File4,
    File5,
    File6,
    File7,
    File8,
    File9,
}

impl File {
    /// The number of files.
    pub const COUNT: usize = 9;

    /// Returns the file to the right of this file.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if called on `File1`, since there is no file to the right.
    #[must_use]
    pub const fn east(self) -> Self {
        debug_assert!(self != File::File1);

        unsafe { transmute(self.as_u8() - 1) }
    }

    /// Returns the file to the left of this file.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if called on `File9`, since there is no file to the left.
    #[must_use]
    pub const fn west(self) -> Self {
        debug_assert!(self != File::File9);

        unsafe { transmute(self.as_u8() + 1) }
    }

    /// Returns the file to the right from the perspective of the given color.
    /// For black, this is the same as `east()`. For white, it's `west()`.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the file is at the board edge (`File1` for black, `File9` for white),
    /// because `east()` or `west()` would panic.
    #[must_use]
    pub const fn relative_east(self, color: Color) -> Self {
        if color == Color::Black {
            self.east()
        } else {
            self.west()
        }
    }

    /// Returns the file to the left from the perspective of the given color.
    /// For black, this is the same as `west()`. For white, it's `east()`.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the file is at the board edge (`File9` for black, `File1` for white),
    /// because `west()` or `east()` would panic.
    #[must_use]
    pub const fn relative_west(self, color: Color) -> Self {
        if color == Color::Black {
            self.west()
        } else {
            self.east()
        }
    }

    /// Returns the file mirrored horizontally across the center of the board.
    /// For example, `File1` becomes `File9` on the board.
    #[must_use]
    pub const fn flip(self) -> Self {
        unsafe { transmute(8 - self.as_u8()) }
    }

    /// Returns the `File` relative to the given color.
    /// For black, this returns `self`.
    /// For white, this returns the `File` flipped horizontally (so that from white's
    /// perspective, `File1` corresponds to `File9`, `File2` to `File8`, and so on).
    #[must_use]
    pub const fn relative(self, color: Color) -> Self {
        if color == Color::Black {
            self
        } else {
            self.flip()
        }
    }

    /// Returns the `File` as a `u8`.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Returns the `File` as a `usize`.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self as usize
    }
}

impl const From<u8> for File {
    /// Creates a `File` from the given raw `u8` value.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if `value` is greater than or equal to `COUNT`.
    fn from(value: u8) -> Self {
        debug_assert!(value < Self::COUNT as u8);

        unsafe { transmute(value) }
    }
}

impl const From<usize> for File {
    /// Creates a `File` from the given raw `usize` value.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if `value` is greater than or equal to `COUNT`.
    fn from(value: usize) -> Self {
        debug_assert!(value < Self::COUNT);

        unsafe { transmute(value as u8) }
    }
}

impl const PartialEq for File {
    /// Compares two `File` values for equality.
    fn eq(&self, other: &Self) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl const Eq for File {}

impl<T> const Index<File> for [T; File::COUNT] {
    type Output = T;

    /// Indexes the array by `File`.
    fn index(&self, file: File) -> &Self::Output {
        &self[file.as_usize()]
    }
}

impl<T> const IndexMut<File> for [T; File::COUNT] {
    /// Mutably indexes the array by `File`.
    fn index_mut(&mut self, file: File) -> &mut Self::Output {
        &mut self[file.as_usize()]
    }
}

/// Represents a rank on the board.
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Rank {
    Rank1,
    Rank2,
    Rank3,
    Rank4,
    Rank5,
    Rank6,
    Rank7,
    Rank8,
    Rank9,
}

impl Rank {
    /// The number of ranks.
    pub const COUNT: usize = 9;

    /// Returns the rank above this rank.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if called on `Rank1`, as there is no rank above it.
    #[must_use]
    pub const fn north(self) -> Self {
        debug_assert!(self != Rank::Rank1);

        unsafe { transmute(self.as_u8() - 1) }
    }

    /// Returns the rank below this rank.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if called on `Rank9`, as there is no rank below it.
    #[must_use]
    pub const fn south(self) -> Self {
        debug_assert!(self != Rank::Rank9);

        unsafe { transmute(self.as_u8() + 1) }
    }

    /// Returns the rank above from the perspective of the given color.
    /// For black, this is the same as `north()`. For white, it's `south()`.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the rank is at the board edge (`Rank1` for black, `Rank9` for white),
    /// because `north()` or `south()` would panic.
    #[must_use]
    pub const fn relative_north(self, color: Color) -> Self {
        if color == Color::Black {
            self.north()
        } else {
            self.south()
        }
    }

    /// Returns the rank below from the perspective of the given color.
    /// For black, this is the same as `south()`. For white, it's `north()`.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the rank is at the board edge (`Rank9` for black, `Rank1` for white),
    /// because `south()` or `north()` would panic.
    #[must_use]
    pub const fn relative_south(self, color: Color) -> Self {
        if color == Color::Black {
            self.south()
        } else {
            self.north()
        }
    }

    /// Returns the rank flipped vertically (mirrored across the center of the board).
    /// For example, `Rank1` becomes `Rank9` on the board.
    #[must_use]
    pub const fn flip(self) -> Self {
        unsafe { transmute(8 - self.as_u8()) }
    }

    /// Returns the `Rank` relative to the given color.
    /// For black, this returns `self`.
    /// For white, this returns the `Rank` flipped vertically (so that from white's
    /// perspective, `Rank1` corresponds to `Rank9`, `Rank2` to `Rank8`, and so on).
    #[must_use]
    pub const fn relative(self, color: Color) -> Self {
        if color == Color::Black {
            self
        } else {
            self.flip()
        }
    }

    /// Returns `true` if a piece on this rank can promote for the given color.
    #[must_use]
    pub const fn can_promote(self, color: Color) -> bool {
        matches!(
            self.relative(color),
            Rank::Rank1 | Rank::Rank2 | Rank::Rank3
        )
    }

    /// Returns the `Rank` as a `u8`.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Returns the `Rank` as a `usize`.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self as usize
    }
}

impl const From<u8> for Rank {
    /// Creates a `Rank` from the given raw `u8` value.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if `value` is greater than or equal to `COUNT`.
    fn from(value: u8) -> Self {
        debug_assert!(value < Self::COUNT as u8);

        unsafe { transmute(value) }
    }
}

impl const From<usize> for Rank {
    /// Creates a `Rank` from the given raw `usize` value.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if `value` is greater than or equal to `COUNT`.
    fn from(value: usize) -> Self {
        debug_assert!(value < Self::COUNT);

        unsafe { transmute(value as u8) }
    }
}

impl const PartialEq for Rank {
    /// Compares two `Rank` values for equality.
    fn eq(&self, other: &Self) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl const Eq for Rank {}

impl<T> const Index<Rank> for [T; Rank::COUNT] {
    type Output = T;

    /// Indexes the array by `Rank`.
    fn index(&self, rank: Rank) -> &Self::Output {
        &self[rank.as_usize()]
    }
}

impl<T> const IndexMut<Rank> for [T; Rank::COUNT] {
    /// Mutably indexes the array by `Rank`.
    fn index_mut(&mut self, rank: Rank) -> &mut Self::Output {
        &mut self[rank.as_usize()]
    }
}

/// Represents a square on the board.
///
/// ==============================================
///                    Mapping
/// ==============================================
///
///    9    8    7    6    5    4    3    2    1
/// +----+----+----+----+----+----+----+----+----+
/// | 72 | 63 | 54 | 45 | 36 | 27 | 18 |  9 |  0 | 一
/// +----+----+----+----+----+----+----+----+----+
/// | 73 | 64 | 55 | 46 | 37 | 28 | 19 | 10 |  1 | 二
/// +----+----+----+----+----+----+----+----+----+
/// | 74 | 65 | 56 | 47 | 38 | 29 | 20 | 11 |  2 | 三
/// +----+----+----+----+----+----+----+----+----+
/// | 75 | 66 | 57 | 48 | 39 | 30 | 21 | 12 |  3 | 四
/// +----+----+----+----+----+----+----+----+----+
/// | 76 | 67 | 58 | 49 | 40 | 31 | 22 | 13 |  4 | 五
/// +----+----+----+----+----+----+----+----+----+
/// | 77 | 68 | 59 | 50 | 41 | 32 | 23 | 14 |  5 | 六
/// +----+----+----+----+----+----+----+----+----+
/// | 78 | 69 | 60 | 51 | 42 | 33 | 24 | 15 |  6 | 七
/// +----+----+----+----+----+----+----+----+----+
/// | 79 | 70 | 61 | 52 | 43 | 34 | 25 | 16 |  7 | 八
/// +----+----+----+----+----+----+----+----+----+
/// | 80 | 71 | 62 | 53 | 44 | 35 | 26 | 17 |  8 | 九
/// +----+----+----+----+----+----+----+----+----+
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
pub enum Square {
    S11, S12, S13, S14, S15, S16, S17, S18, S19,
    S21, S22, S23, S24, S25, S26, S27, S28, S29,
    S31, S32, S33, S34, S35, S36, S37, S38, S39,
    S41, S42, S43, S44, S45, S46, S47, S48, S49,
    S51, S52, S53, S54, S55, S56, S57, S58, S59,
    S61, S62, S63, S64, S65, S66, S67, S68, S69,
    S71, S72, S73, S74, S75, S76, S77, S78, S79,
    S81, S82, S83, S84, S85, S86, S87, S88, S89,
    S91, S92, S93, S94, S95, S96, S97, S98, S99,
}

impl Square {
    /// The number of squares on the board.
    pub const COUNT: usize = File::COUNT * Rank::COUNT;

    /// Returns a `Square` from the given `File` and `Rank`.
    #[must_use]
    pub const fn new(file: File, rank: Rank) -> Self {
        unsafe { transmute(file.as_u8() * 9 + rank.as_u8()) }
    }

    /// Returns the `File` of this square.
    #[must_use]
    pub const fn file(self) -> File {
        unsafe { transmute(self.as_u8() / 9) }
    }

    /// Returns the `Rank` of this square.
    #[must_use]
    pub const fn rank(self) -> Rank {
        unsafe { transmute(self.as_u8() % 9) }
    }

    /// Returns the square directly above.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the rank of this square is `Rank1`, since there is no square above it.
    #[must_use]
    pub const fn north(self) -> Self {
        Self::new(self.file(), self.rank().north())
    }

    /// Returns the square directly below.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the rank of this square is `Rank9`, since there is no square below it.
    #[must_use]
    pub const fn south(self) -> Self {
        Self::new(self.file(), self.rank().south())
    }

    /// Returns the square directly to the right.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the file of this square is `File1`, since there is no square to the right.
    #[must_use]
    pub const fn east(self) -> Self {
        Self::new(self.file().east(), self.rank())
    }

    /// Returns the square directly to the left.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the file of this square is `File9`, since there is no square to the left.
    #[must_use]
    pub const fn west(self) -> Self {
        Self::new(self.file().west(), self.rank())
    }

    /// Returns the square diagonally up-right.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the square is on the top rank (`Rank1`) or the rightmost file (`File1`),
    /// since there is no square above or to the right.
    #[must_use]
    pub const fn north_east(self) -> Self {
        Self::new(self.file().east(), self.rank().north())
    }

    /// Returns the square diagonally up-left.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the square is on the top rank (`Rank1`) or the leftmost file (`File9`),
    /// since there is no square above or to the left.
    #[must_use]
    pub const fn north_west(self) -> Self {
        Self::new(self.file().west(), self.rank().north())
    }

    /// Returns the square diagonally down-right.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the square is on the bottom rank (`Rank9`) or the rightmost file (`File1`),
    /// since there is no square below or to the right.
    #[must_use]
    pub const fn south_east(self) -> Self {
        Self::new(self.file().east(), self.rank().south())
    }

    /// Returns the square diagonally down-left.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the square is on the bottom rank (`Rank9`) or the leftmost file (`File9`),
    /// since there is no square below or to the left.
    #[must_use]
    pub const fn south_west(self) -> Self {
        Self::new(self.file().west(), self.rank().south())
    }

    /// Returns the square directly above from the perspective of the given color.
    /// For black, this is the same as `north()`. For white, it's `south()`.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the resulting square would be outside the board:
    /// - For black, if the square is on the top rank (`Rank1`)
    /// - For white, if the square is on the bottom rank (`Rank9`)
    #[must_use]
    pub const fn relative_north(self, color: Color) -> Self {
        if color == Color::Black {
            self.north()
        } else {
            self.south()
        }
    }

    /// Returns the square directly below from the perspective of the given color.
    /// For black, this is the same as `south()`. For white, it's `north()`.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the resulting square would be outside the board:
    /// - For black, if the square is on the bottom rank (`Rank9`)
    /// - For white, if the square is on the top rank (`Rank1`)
    #[must_use]
    pub const fn relative_south(self, color: Color) -> Self {
        if color == Color::Black {
            self.south()
        } else {
            self.north()
        }
    }

    /// Returns the square to the right from the perspective of the given color.
    /// For black, this is the same as `east()`. For white, it's `west()`.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the resulting square would be outside the board:
    /// - For black, if the square is on the rightmost file (`File1`)
    /// - For white, if the square is on the leftmost file (`File9`)
    #[must_use]
    pub const fn relative_east(self, color: Color) -> Self {
        if color == Color::Black {
            self.east()
        } else {
            self.west()
        }
    }

    /// Returns the square to the left from the perspective of the given color.
    /// For black, this is the same as `west()`. For white, it's `east()`.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the resulting square would be outside the board:
    /// - For black, if the square is on the leftmost file (`File9`)
    /// - For white, if the square is on the rightmost file (`File1`)
    #[must_use]
    pub const fn relative_west(self, color: Color) -> Self {
        if color == Color::Black {
            self.west()
        } else {
            self.east()
        }
    }

    /// Returns the square diagonally up-right from the perspective of the given color.
    /// For black, this is the same as `north_east()`. For white, it's `south_west()`.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the resulting square would be outside the board:
    /// - For black, if the square is on the top rank (`Rank1`) or rightmost file (`File1`)
    /// - For white, if the square is on the bottom rank (`Rank9`) or leftmost file (`File9`)
    #[must_use]
    pub const fn relative_north_east(self, color: Color) -> Self {
        if color == Color::Black {
            self.north_east()
        } else {
            self.south_west()
        }
    }

    /// Returns the square diagonally up-left from the perspective of the given color.
    /// For black, this is the same as `north_west()`. For white, it's `south_east()`.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the resulting square would be outside the board:
    /// - For black, if the square is on the top rank (`Rank1`) or leftmost file (`File9`)
    /// - For white, if the square is on the bottom rank (`Rank9`) or rightmost file (`File1`)
    #[must_use]
    pub const fn relative_north_west(self, color: Color) -> Self {
        if color == Color::Black {
            self.north_west()
        } else {
            self.south_east()
        }
    }

    /// Returns the square diagonally down-right from the perspective of the given color.
    /// For black, this is the same as `south_east()`. For white, it's `north_west()`.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the resulting square would be outside the board:
    /// - For black, if the square is on the bottom rank (`Rank9`) or rightmost file (`File1`)
    /// - For white, if the square is on the top rank (`Rank1`) or leftmost file (`File9`)
    #[must_use]
    pub const fn relative_south_east(self, color: Color) -> Self {
        if color == Color::Black {
            self.south_east()
        } else {
            self.north_west()
        }
    }

    /// Returns the square diagonally down-left from the perspective of the given color.
    /// For black, this is the same as `south_west()`. For white, it's `north_east()`.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if the resulting square would be outside the board:
    /// - For black, if the square is on the bottom rank (`Rank9`) or leftmost file (`File9`)
    /// - For white, if the square is on the top rank (`Rank1`) or rightmost file (`File1`)
    #[must_use]
    pub const fn relative_south_west(self, color: Color) -> Self {
        if color == Color::Black {
            self.south_west()
        } else {
            self.north_east()
        }
    }

    /// Returns a new square with the given `File`, keeping the current `Rank`.
    pub const fn with_file(self, file: File) -> Self {
        Self::new(file, self.rank())
    }

    /// Returns a new square with the given `Rank`, keeping the current `File`.
    pub const fn with_rank(self, rank: Rank) -> Self {
        Self::new(self.file(), rank)
    }

    /// Returns the square flipped horizontally across the center of the board.
    /// For example, `Square::S11` becomes `Square::S91` on the board.
    #[must_use]
    pub const fn flip_file(self) -> Self {
        self.with_file(self.file().flip())
    }

    /// Returns the square flipped vertically (mirrored across the center of the board).
    /// For example, `Square::S11` becomes `Square::S19` on the board.
    #[must_use]
    pub const fn flip_rank(self) -> Self {
        self.with_rank(self.rank().flip())
    }

    /// Returns the square flipped horizontally and vertically (mirrored across the center of the board).
    /// For example, `Square::S11` becomes `Square::S99` on the board.
    #[must_use]
    pub const fn rotate180(self) -> Self {
        self.flip_file().flip_rank()
    }

    /// Returns `true` if a piece on this square can promote for the given color.
    #[must_use]
    pub const fn can_promote(self, color: Color) -> bool {
        self.rank().can_promote(color)
    }

    /// Returns the `Square` as a `u8`.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Returns the `Square` as a `usize`.
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self as usize
    }
}

impl const From<u8> for Square {
    /// Creates a `Square` from the given raw `u8` value.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if `value` is greater than or equal to `COUNT`.
    fn from(value: u8) -> Self {
        debug_assert!(value < Self::COUNT as u8);

        unsafe { transmute(value) }
    }
}

impl const From<usize> for Square {
    /// Creates a `Square` from the given raw `usize` value.
    ///
    /// # Debug assertions
    ///
    /// In debug builds, panics if `value` is greater than or equal to `COUNT`.
    fn from(value: usize) -> Self {
        debug_assert!(value < Self::COUNT);

        unsafe { transmute(value as u8) }
    }
}

impl const PartialEq for Square {
    /// Compares two `Square` values for equality.
    fn eq(&self, other: &Self) -> bool {
        self.as_u8() == other.as_u8()
    }
}

impl const Eq for Square {}

impl<T> const Index<Square> for [T; Square::COUNT] {
    type Output = T;

    /// Indexes the array by `Square`.
    fn index(&self, square: Square) -> &Self::Output {
        &self[square.as_usize()]
    }
}

impl<T> const IndexMut<Square> for [T; Square::COUNT] {
    /// Mutably indexes the array by `Square`.
    fn index_mut(&mut self, square: Square) -> &mut Self::Output {
        &mut self[square.as_usize()]
    }
}
