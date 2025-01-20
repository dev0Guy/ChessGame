use crate::bitboard::BitBoard;

/// Represents the files (columns) on a chessboard.
///
/// Files are labeled from `A` (leftmost column) to `H` (rightmost column), where `File::A`
/// corresponds to the `a`-file and `File::H` corresponds to the `h`-file.
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


impl From<File> for BitBoard {
    fn from(file: File) -> Self {
        match file {
            File::A => BitBoard::new(0x0101010101010101),
            File::B => BitBoard::new(0x0202020202020202),
            File::C => BitBoard::new(0x0404040404040404),
            File::D => BitBoard::new(0x0808080808080808),
            File::E => BitBoard::new(0x1010101010101010),
            File::F => BitBoard::new(0x2020202020202020),
            File::G => BitBoard::new(0x4040404040404040),
            File::H => BitBoard::new(0x8080808080808080),
        }
    }
}