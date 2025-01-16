use strum_macros::FromRepr;

/// Represents the files (columns) on a chessboard.
#[derive(FromRepr, Debug, Copy, Clone)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

/// Represents the rank (rows) of a chessboard.
#[derive(FromRepr, Debug, Copy, Clone)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

/// Represents a location on a chessboard.
///
/// A `Location` is defined by:
/// - A [`File`]: The column on the chessboard (e.g., A, B, C).
/// - A [`Rank`]: The row on the chessboard (e.g., 1, 2, 3).
///
/// Together, these components uniquely identify a square on the chessboard.
#[derive(Debug, Copy, Clone)]
pub(crate) struct Location {
    pub file: File,
    pub rank: Rank,
}

impl File {

    /// Converts a character representing a rank (e.g., 'A', 'B', 'C', 'D') into a [`File`] enum.
    fn from_char(c: char) -> Option<Self> {
        let index = (c.to_ascii_lowercase() as u8).checked_sub(b'a')? as usize;
        Self::from_repr(index)
    }
}

impl Rank {

    /// Converts a character representing a rank (e.g., '1', '2') into a [`Rank`] enum.
    fn from_char(c: char) -> Option<Self> {
       Self::from_repr((c.to_digit(10).unwrap() as usize) - 1)
    }
}

impl Location {
    /// Creates a new [`Location`] instance with the specified file and rank.
    pub(crate) const fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }

    /// Creates a [`Location`] from a string representation.
    ///
    /// This function parses a string slice (e.g., `"e4"`) and converts it into a
    /// corresponding [`Location`] instance by extracting the file and rank.
    ///
    /// ## Parameters
    /// - `value`: A string slice representing a location on the chessboard.
    ///   It should be in the format of a file followed by a rank (e.g., `"e4"`).
    ///
    /// ## Returns
    /// - `Ok(Location)`: If the string is valid and can be successfully converted into a `Location`.
    /// - `Err(())`: If the string is invalid or cannot be parsed.
    ///
    /// ## Panics
    /// This function uses `.unwrap()` when converting characters into [`File`] or [`Rank`]. If the
    /// provided string has invalid characters or format, it will panic.
    pub(crate) fn from(value: &str) -> Result<Self, ()> {
        match value.chars().collect::<Vec<char>>().as_slice() {
            [first, second, ..] => {
                let file = File::from_char(*first).unwrap();
                let rank = Rank::from_char(*second).unwrap();
                Ok(Location::new(file, rank))
            },
            _ => Err(())
        }
    }
}
