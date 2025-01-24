pub(crate) mod file;
pub(crate) mod rank;

pub(crate) use file::File;
pub(crate) use rank::Rank;
use crate::bitboard::BitBoard;


/// Represents a square on the chessboard, defined by a file (column) and rank (row).
///
/// A `Square` is an abstraction that combines a [`File`] and a [`Rank`] to represent a single
/// chessboard position. It is useful for mapping board positions to bitboards or other
/// representations.
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) struct Square {
    /// The file (column) of the square, such as `File::A` or `File::H`.
    file: File,
    /// The rank (row) of the square, such as `Rank::One` or `Rank::Eight`.
    rank: Rank,
}

impl Square{
    /// Creates a new `Square` from a given file and rank.
    ///
    /// # Parameters
    /// - `file`: The [`File`] of the square (e.g., `File::D`).
    /// - `rank`: The [`Rank`] of the square (e.g., `Rank::Four`).
    ///
    /// # Returns
    /// A new instance of `Square`.
    pub(crate) fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }

    /// Return square file (copy)
    pub(crate) fn file(&self) -> File{
        self.file
    }

    /// Return square rank (copy)
    pub(crate) fn rank(&self) -> Rank{
        self.rank
    }
}

/// Converts a [`Square`] into a [`BitBoard`].
impl From<Square> for BitBoard {
    fn from(square: Square) -> Self {
       BitBoard::from(square.file) & BitBoard::from(square.rank)
    }
}

impl TryFrom<String> for Square {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.chars().collect::<Vec<char>>().as_slice(){
            [first, second] => {
                let file = File::try_from(*first)?;
                let rank = Rank::try_from(*second)?;
                Ok(Self::new(file, rank))
            }
            _ => Err(())
        }
    }
}

impl From<Square> for usize{
    fn from(value: Square) -> Self {
        let [file, rank] = [usize::from(value.file), usize::from(value.rank)];
        rank * 8 + file
    }
}