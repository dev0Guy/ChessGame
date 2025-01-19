#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum File {
    A, B, C, D, E, F, G, H,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Rank {
    One, Two, Three, Four, Five, Six, Seven, Eight,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Position {
    pub file: File,
    pub rank: Rank,
}

impl Position {
    /// Creates a new `Position` from `File` and `Rank`.
    pub fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }

    /// Iterates over all positions on the board.
    pub fn iter() -> impl Iterator<Item = Position> {
        File::iter().flat_map(|file| Rank::iter().map(move |rank| Position { file, rank }))
    }

    pub fn position_bitboard_index(&self) -> usize{
        usize::from((8 * usize::from(self.rank)) + (usize::from(self.file)))
    }
}

/// Conversion between `Position` and `usize`.
impl From<usize> for Position {
    fn from(index: usize) -> Self {
        let file = File::from(index % 8);
        let rank = Rank::from(index / 8);
        Position::new(file, rank)
    }
}

impl From<Position> for usize {
    fn from(pos: Position) -> Self {
        let file = pos.file as usize;
        let rank = pos.rank as usize;
        rank * 8 + file
    }
}

impl File {
    /// Iterates over all files (A–H).
    pub fn iter() -> impl Iterator<Item = File> {
        (0..8).map(File::from)
    }
}

impl Rank {
    /// Iterates over all ranks (1–8).
    pub fn iter() -> impl Iterator<Item = Rank> {
        (0..8).map(Rank::from)
    }
}

/// Conversion between `File` and `usize`.
impl From<usize> for File {
    fn from(index: usize) -> Self {
        match index {
            0 => File::A, 1 => File::B, 2 => File::C, 3 => File::D,
            4 => File::E, 5 => File::F, 6 => File::G, 7 => File::H,
            _ => panic!("Invalid file index: {}", index),
        }
    }
}

impl From<File> for usize {
    fn from(file: File) -> Self {
        match file {
            File::A => 0, File::B => 1, File::C => 2, File::D => 3,
            File::E => 4, File::F => 5, File::G => 6, File::H => 7,
        }
    }
}

impl From<usize> for Rank {
    fn from(index: usize) -> Self {
        match index {
            0 => Rank::One, 1 => Rank::Two, 2 => Rank::Three, 3 => Rank::Four,
            4 => Rank::Five, 5 => Rank::Six, 6 => Rank::Seven, 7 => Rank::Eight,
            _ => panic!("Invalid rank index: {}", index),
        }
    }
}

impl From<Rank> for usize {
    fn from(rank: Rank) -> Self {
        match rank {
            Rank::One => 0, Rank::Two => 1, Rank::Three => 2, Rank::Four => 3,
            Rank::Five => 4, Rank::Six => 5, Rank::Seven => 6, Rank::Eight => 7,
        }
    }
}