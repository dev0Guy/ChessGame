use crate::bitboard::BitBoard;


/// Represents the ranks (rows) on a chessboard.
///
/// Ranks are numbered from 1 (bottom) to 8 (top), where `Rank::One` corresponds
/// to the bottom row (`a1` to `h1`) and `Rank::Eight` corresponds to the top row (`a8` to `h8`).
#[derive(Copy, Clone, PartialEq, Debug)]
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


impl From<Rank> for BitBoard {
    fn from(rank: Rank) -> Self {
        match rank {
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


impl TryFrom<char> for Rank {
    type Error = ();

    fn try_from(rank: char) -> Result<Self, Self::Error> {
        match rank.to_digit(10) {
            Some(1) => Ok(Rank::One),
            Some(2) => Ok(Rank::Two),
            Some(3) => Ok(Rank::Three),
            Some(4) => Ok(Rank::Four),
            Some(5) => Ok(Rank::Five),
            Some(6) => Ok(Rank::Six),
            Some(7) => Ok(Rank::Seven),
            Some(8) => Ok(Rank::Eight),
            _ => Err(())
        }
    }
}

impl From<Rank> for usize {
    fn from(rank: Rank) -> Self {
        match rank {
            Rank::One => 0,
            Rank::Two => 1,
            Rank::Three => 2,
            Rank::Four => 3,
            Rank::Five => 4,
            Rank::Six => 5,
            Rank::Seven => 6,
            Rank::Eight => 7,
        }
    }
}