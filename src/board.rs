use crate::bitboard::BitBoard;


/// Represents the ranks (rows) on a chessboard.
///
/// Ranks are numbered from 1 (bottom) to 8 (top), where `Rank::One` corresponds
/// to the bottom row (`a1` to `h1`) and `Rank::Eight` corresponds to the top row (`a8` to `h8`).
pub enum Rank{
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}


impl Rank{
    /// Converts the `Rank` into a `BitBoard` representing all squares in that rank.
    ///
    /// # Returns
    /// A `BitBoard` with all bits set for the corresponding rank.
    pub fn as_bitboard(&self) -> BitBoard{
        match self {
            Rank::One => BitBoard::new(0x00000000000000FF),
            Rank::Two => BitBoard::new(0x000000000000FF00),
            Rank::Three => BitBoard::new(0x0000000000FF0000),
            Rank::Four => BitBoard::new(0x00000000FF000000),
            Rank::Five => BitBoard::new(0x000000FF00000000),
            Rank::Six => BitBoard::new(0x0000FF0000000000),
            Rank::Seven => BitBoard::new(0x00FF000000000000),
            Rank::Eight => BitBoard::new(0xFF00000000000000),
        }
    }
}