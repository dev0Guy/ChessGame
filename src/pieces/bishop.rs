use crate::bitboard::BitBoard;
use crate::pieces::common::{Color, PossibleMoves};
use crate::square::Square;

const DIAGONAL_MASK: [u64; 15] =  [
    0x100000000000000,
    0x201000000000000,
    0x402010000000000,
    0x804020100000000,
    0x1008040201000000,
    0x2010080402010000,
    0x4020100804020100,
    0x8040201008040201,
    0x80402010080402,
    0x804020100804,
    0x8040201008,
    0x80402010,
    0x804020,
    0x8040,
    0x80,
];

const ANTI_DIAGONAL_MASK: [u64; 15] = [
    0x0000000000000001, // a1
    0x0000000000000102, // a2-b1
    0x0000000000010204, // a3-b2-c1
    0x0000000001020408, // a4-b3-c2-d1
    0x0000000102040810, // a5-b4-c3-d2-e1
    0x0000010204081020, // a6-b5-c4-d3-e2-f1
    0x0001020408102040, // a7-b6-c5-d4-e3-f2-g1
    0x0102040810204080, // a8-b7-c6-d5-e4-f3-g2-h1
    0x2040810204080000, // b8-c7-d6-e5-f4-g3-h2
    0x4081020408000000, // c8-d7-e6-f5-g4-h3
    0x8102040800000000, // d8-e7-f6-g5-h4
    0x1020408000000000, // e8-f7-g6-h5
    0x2040800000000000, // f8-g7-h6
    0x4080000000000000, // g8-h7
    0x8000000000000000, // h8
];


/// Description
/// Slide in diagonal or anti-diagonal
/// stop movement when capture other piece or blocked by its own piece(exclusive)
struct Bishop;


impl PossibleMoves for Bishop {
    fn get_moves(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
       Self::get_diagonal_moves(piece, square, own_pieces, opponent_pieces, color)
        | Self::get_anti_diagonal_moves(piece, square, own_pieces, opponent_pieces, color)
    }
}

impl Bishop {

    /// Computes the diagonal mask for the given square.
    /// # Parameters
    /// - `square`: The [`Square`] for which to calculate the diagonal mask.
    /// # Returns
    /// A [`BitBoard`] containing the mask for the diagonal.
    fn get_diagonal_mask(square: Square) -> BitBoard{
        let rank = square.rank() as usize;
        let file = square.file() as usize;
        BitBoard::new(DIAGONAL_MASK[7 + (file - rank)])
    }

    /// Computes the anti-diagonal mask for the given square.
    /// # Parameters
    /// - `square`: The [`Square`] for which to calculate the anti-diagonal mask.
    /// # Returns
    /// A [`BitBoard`] containing the mask for the anti-diagonal.
    fn get_anti_diagonal_mask(square: Square) -> BitBoard{
        let rank = square.rank() as usize;
        let file = square.file() as usize;
        BitBoard::new(DIAGONAL_MASK[file + rank])
    }

    /// Computes all possible diagonal moves for a piece located on the given square.
    /// # Parameters
    /// - `piece`: A [`BitBoard`] representing the single position of the piece.
    /// - `square`: The [`Square`] where the piece is located.
    /// - `own_pieces`: A [`BitBoard`] representing the positions of all friendly pieces.
    /// - `opponent_pieces`: A [`BitBoard`] representing the positions of all opponent pieces.
    /// - `color`: The [`Color`] of the piece (`Color::White` or `Color::Black`).
    /// # Returns
    /// A [`BitBoard`] representing all valid diagonal moves for the piece.
    fn get_diagonal_moves(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
        let diagonal_mask = Self::get_diagonal_mask(square);
        let occupied_diagonal = Self::occupied(own_pieces, opponent_pieces) & diagonal_mask;
        let diagonal_up = occupied_diagonal - (*piece * 2);
        let diagonal_down = (occupied_diagonal.reverse() - ((*piece).reverse() * 2)).reverse();
        ((diagonal_up ^ diagonal_down) & diagonal_mask) & !own_pieces
    }

    /// Computes all possible anti-diagonal moves for a piece located on the given square.
    /// # Parameters
    /// - `piece`: A [`BitBoard`] representing the single position of the piece.
    /// - `square`: The [`Square`] where the piece is located.
    /// - `own_pieces`: A [`BitBoard`] representing the positions of all friendly pieces.
    /// - `opponent_pieces`: A [`BitBoard`] representing the positions of all opponent pieces.
    /// - `color`: The [`Color`] of the piece (`Color::White` or `Color::Black`).
    /// # Returns
    /// A [`BitBoard`] representing all valid anti-diagonal moves for the piece.
    fn get_anti_diagonal_moves(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
        let anti_diagonal_mask = Self::get_diagonal_mask(square);
        let occupied_anti_diagonal = Self::occupied(own_pieces, opponent_pieces) & anti_diagonal_mask;
        let ant_diagonal_up = occupied_anti_diagonal - (*piece * 2);
        let anti_diagonal_down = (occupied_anti_diagonal.reverse() - ((*piece).reverse() * 2)).reverse();
        (ant_diagonal_up ^ anti_diagonal_down) & anti_diagonal_mask
    }
}

#[cfg(test)]
mod tests {
    use crate::square::{File, Rank};
    use super::*;

    #[test]
    fn test_get_diagonal_moves_unobstructed() {
        let square = Square::new(File::D, Rank::Four);
        let a1 = Square::new(File::A, Rank::One);
        let b2 = Square::new(File::B, Rank::Two);
        let c3 = Square::new(File::C, Rank::Three);
        let e5 = Square::new(File::E, Rank::Five);
        let f6 = Square::new(File::F, Rank::Six);
        let g7 = Square::new(File::G, Rank::Seven);
        let h8 = Square::new(File::H, Rank::Eight);

        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::new(0);

        let result = Bishop::get_diagonal_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(a1)
            | BitBoard::from(b2)
            | BitBoard::from(c3)
            | BitBoard::from(e5)
            | BitBoard::from(f6)
            | BitBoard::from(g7)
            | BitBoard::from(h8);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/5P2/4P3/3B4/2P5/1P6/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_get_diagonal_moves_blocked_by_own_pieces() {
        let square = Square::new(File::D, Rank::Four);
        let b2 = Square::new(File::B, Rank::Two);
        let f6 = Square::new(File::F, Rank::Six);
        let c3 = Square::new(File::C, Rank::Three);
        let e5 = Square::new(File::E, Rank::Five);

        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::from(b2) | BitBoard::from(f6) | BitBoard::from(c3) | BitBoard::from(e5);
        let opponent_pieces = BitBoard::new(0);

        let result = Bishop::get_diagonal_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::new(0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_diagonal_moves_blocked_by_opponent_pieces() {
        let square = Square::new(File::D, Rank::Four);
        let c3 = Square::new(File::C, Rank::Three);
        let e5 = Square::new(File::E, Rank::Five);

        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::from(c3) | BitBoard::from(e5);

        let result = Bishop::get_diagonal_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(c3) | BitBoard::from(e5);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_diagonal_moves_corner_case() {
        let square = Square::new(File::A, Rank::One);
        let b2 = Square::new(File::B, Rank::Two);
        let c3 = Square::new(File::C, Rank::Three);
        let d4 = Square::new(File::D, Rank::Four);
        let e5 = Square::new(File::E, Rank::Five);
        let f6 = Square::new(File::F, Rank::Six);
        let g7 = Square::new(File::G, Rank::Seven);
        let h8 = Square::new(File::H, Rank::Eight);

        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::new(0);

        let result = Bishop::get_diagonal_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(b2)
            | BitBoard::from(c3)
            | BitBoard::from(d4)
            | BitBoard::from(e5)
            | BitBoard::from(f6)
            | BitBoard::from(g7)
            | BitBoard::from(h8);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_diagonal_moves_corner_blocked() {
        let square = Square::new(File::A, Rank::One);
        let c3 = Square::new(File::C, Rank::Three);
        let b2 = Square::new(File::B, Rank::Two);
        let d4 = Square::new(File::D, Rank::Four);

        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::from(c3);
        let opponent_pieces = BitBoard::from(d4);

        let result = Bishop::get_diagonal_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(b2);
        assert_eq!(result, expected);
    }
}
