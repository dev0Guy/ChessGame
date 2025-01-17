use strum_macros::FromRepr;

/// Represents the files (columns) on a chessboard.
#[derive(FromRepr, Debug, Copy, Clone, PartialEq)]
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
#[derive(FromRepr, Debug, Copy, Clone, PartialEq)]
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
#[derive(Debug, Copy, Clone, PartialEq)]
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

    /// Computes a new `Location` by applying the given rank and file offsets.
    ///
    /// This method adjusts the current location's rank and file based on the specified offsets
    /// and ensures that the resulting position remains within the bounds of a standard chessboard.
    ///
    /// ## Parameters
    /// - `rank_offset`: The vertical offset to apply to the rank. Positive values move upward,
    ///   while negative values move downward.
    /// - `file_offset`: The horizontal offset to apply to the file. Positive values move to the right,
    ///   while negative values move to the left.
    ///
    /// ## Returns
    /// - `Ok(Location)`: If the new position is valid and within the chessboard's bounds.
    /// - `Err(&str)`: If the new position is invalid due to being out of bounds. Specific errors include:
    ///     - `"Rank offset is out of bounds"`: The new rank is outside the range 1-8.
    ///     - `"File offset is out of bounds"`: The new file is outside the range A-H.
    ///     - `"Rank offset and File offset are out of bounds"`: Both rank and file offsets are invalid.
    pub(crate) fn offset(&self, rank_offset: i8, file_offset: i8) -> Result<Location, String> {
        let rank = Rank::from_repr((self.rank as i8 + rank_offset) as usize)
            .ok_or_else(|| "Rank offset is out of bounds".to_string())?;
        let file = File::from_repr((self.file as i8 + file_offset) as usize)
            .ok_or_else(|| "File offset is out of bounds".to_string())?;
        Ok(Location::new(file, rank))
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
