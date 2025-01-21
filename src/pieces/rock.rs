use crate::{BitBoard};
use crate::square::{Square, File, Rank};
use super::common::{Color, PossibleMoves};

/// Description
/// Slide for each rank/ file.
/// stop movement when capture other piece or blocked by its own piece(exclusive)
struct Rock;

impl PossibleMoves for Rock {
    fn get_moves(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
        Self::get_vertical_moves(piece, square, own_pieces, opponent_pieces, color)
        | Self::get_horizontal_moves(piece, square, own_pieces, opponent_pieces, color)
    }
}


impl Rock{
    /// Calculates all possible horizontal moves for a piece located at the given square.
    /// # Parameters
    /// - `piece`: A [`BitBoard`] representing the single position of the piece whose horizontal moves are being calculated.
    /// - `square`: A [`Square`] representing the position of the piece.
    /// - `own_pieces`: A [`BitBoard`] representing the positions of all friendly pieces.
    /// - `opponent_pieces`: A [`BitBoard`] representing the positions of all opponent pieces.
    /// - `color`: The [`Color`] of the piece (`Color::White` or `Color::Black`).
    ///
    /// # Returns
    /// A [`BitBoard`] representing all valid horizontal moves for the piece.
    fn get_horizontal_moves(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard{
        let horizontal_mask= BitBoard::from(square.rank());
        let occupied_horizontal = Self::occupied(own_pieces, opponent_pieces) & horizontal_mask;
        let left_side = occupied_horizontal - (*piece * 2);
        let right_side = ((occupied_horizontal & horizontal_mask).reverse() - (piece.reverse() * 2)).reverse();
        let movement_with_capture  = (left_side ^ right_side) & horizontal_mask;
        movement_with_capture & !own_pieces
    }


    /// Calculates all possible vertical moves for a piece located at the given square.
    ///
    /// # Parameters
    /// - `piece`: A [`BitBoard`] representing the single position of the piece whose vertical moves are being calculated.
    /// - `square`: A [`Square`] representing the position of the piece.
    /// - `own_pieces`: A [`BitBoard`] representing the positions of all friendly pieces.
    /// - `opponent_pieces`: A [`BitBoard`] representing the positions of all opponent pieces.
    /// - `color`: The [`Color`] of the piece (`Color::White` or `Color::Black`).
    ///
    /// # Returns
    /// A [`BitBoard`] representing all valid vertical moves for the piece.
    fn get_vertical_moves(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard{
        let vertical_mask= BitBoard::from(square.file());
        let occupied_vertical = Self::occupied(own_pieces, opponent_pieces) & vertical_mask;
        let down = occupied_vertical - (*piece *2);
        let up = (occupied_vertical.reverse() - (piece.reverse() * 2)).reverse();
        let movement_with_capture  = (up ^ down) & vertical_mask;
        movement_with_capture & !own_pieces
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_horizontal_moves_unobstructed() {
        let square = Square::new(File::D, Rank::Two);
        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::new(0);

        let result = Rock::get_horizontal_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(Rank::Two) ^ BitBoard::from(square);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_horizontal_moves_blocked_by_friendly() {
        let square = Square::new(File::D, Rank::Two);
        let e2 = Square::new(File::E, Rank::Two);
        let b2 = Square::new(File::B, Rank::Two);
        let c2 = Square::new(File::C, Rank::Two);
        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::from(e2) | BitBoard::from(b2);
        let opponent_pieces = BitBoard::new(0);

        let result = Rock::get_horizontal_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(c2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_horizontal_moves_blocked_by_opponent() {
        let square = Square::new(File::D, Rank::Two);
        let b2 = Square::new(File::B, Rank::Two);
        let c2 = Square::new(File::C, Rank::Two);
        let e2 = Square::new(File::E, Rank::Two);
        let f2 = Square::new(File::F, Rank::Two);
        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::from(b2) | BitBoard::from(f2);

        let result = Rock::get_horizontal_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(c2) | BitBoard::from(e2) | BitBoard::from(f2);
    }

    #[test]
    fn test_get_horizontal_moves_on_edge() {
        let square = Square::new(File::H, Rank::Two);
        let a2 = Square::new(File::A, Rank::Two);
        let b2 = Square::new(File::B, Rank::Two);
        let c2 = Square::new(File::C, Rank::Two);
        let d2 = Square::new(File::D, Rank::Two);
        let e2 = Square::new(File::E, Rank::Two);
        let f2 = Square::new(File::F, Rank::Two);
        let g2 = Square::new(File::G, Rank::Two);
        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::new(0);

        let result = Rock::get_horizontal_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(a2)
            | BitBoard::from(b2)
            | BitBoard::from(c2)
            | BitBoard::from(d2)
            | BitBoard::from(e2)
            | BitBoard::from(f2)
            | BitBoard::from(g2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_horizontal_moves_opponent_at_end() {
        let square = Square::new(File::D, Rank::Two);
        let a2 = Square::new(File::A, Rank::Two);
        let b2 = Square::new(File::B, Rank::Two);
        let c2 = Square::new(File::C, Rank::Two);
        let e2 = Square::new(File::E, Rank::Two);
        let f2 = Square::new(File::F, Rank::Two);
        let g2 = Square::new(File::G, Rank::Two);
        let h2 = Square::new(File::H, Rank::Two);
        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::from(h2);

        let result = Rock::get_horizontal_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(a2)
            | BitBoard::from(b2)
            | BitBoard::from(c2)
            | BitBoard::from(e2)
            | BitBoard::from(f2)
            | BitBoard::from(g2)
            | BitBoard::from(h2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_vertical_moves_unobstructed() {
        let square = Square::new(File::D, Rank::Four);
        let d1 = Square::new(File::D, Rank::One);
        let d2 = Square::new(File::D, Rank::Two);
        let d3 = Square::new(File::D, Rank::Three);
        let d5 = Square::new(File::D, Rank::Five);
        let d6 = Square::new(File::D, Rank::Six);
        let d7 = Square::new(File::D, Rank::Seven);
        let d8 = Square::new(File::D, Rank::Eight);
        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::new(0);

        let result = Rock::get_vertical_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(d1)
            | BitBoard::from(d2)
            | BitBoard::from(d3)
            | BitBoard::from(d5)
            | BitBoard::from(d6)
            | BitBoard::from(d7)
            | BitBoard::from(d8);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_vertical_moves_blocked_by_friendly() {
        let square = Square::new(File::D, Rank::Four);
        let d2 = Square::new(File::D, Rank::Two);
        let d6 = Square::new(File::D, Rank::Six);
        let d5 = Square::new(File::D, Rank::Five);
        let d3 = Square::new(File::D, Rank::Three);
        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::from(d2) | BitBoard::from(d6);
        let opponent_pieces = BitBoard::new(0);

        let result = Rock::get_vertical_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(d3) | BitBoard::from(d5);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_vertical_moves_blocked_by_opponent() {
        let square = Square::new(File::D, Rank::Four);
        let d2 = Square::new(File::D, Rank::Two);
        let d6 = Square::new(File::D, Rank::Six);
        let d3 = Square::new(File::D, Rank::Three);
        let d5 = Square::new(File::D, Rank::Five);
        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::from(d2) | BitBoard::from(d6);

        let result = Rock::get_vertical_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(d3) | BitBoard::from(d5) | BitBoard::from(d2) | BitBoard::from(d6);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_vertical_moves_on_edge() {
        let square = Square::new(File::D, Rank::One);
        let d2 = Square::new(File::D, Rank::Two);
        let d3 = Square::new(File::D, Rank::Three);
        let d4 = Square::new(File::D, Rank::Four);
        let d5 = Square::new(File::D, Rank::Five);
        let d6 = Square::new(File::D, Rank::Six);
        let d7 = Square::new(File::D, Rank::Seven);
        let d8 = Square::new(File::D, Rank::Eight);
        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::new(0);

        let result = Rock::get_vertical_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(d2)
            | BitBoard::from(d3)
            | BitBoard::from(d4)
            | BitBoard::from(d5)
            | BitBoard::from(d6)
            | BitBoard::from(d7)
            | BitBoard::from(d8);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_vertical_moves_opponent_at_end() {
        let square = Square::new(File::D, Rank::Four);
        let d2 = Square::new(File::D, Rank::Two);
        let d3 = Square::new(File::D, Rank::Three);
        let d5 = Square::new(File::D, Rank::Five);
        let d6 = Square::new(File::D, Rank::Six);
        let d7 = Square::new(File::D, Rank::Seven);
        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::from(d7) | BitBoard::from(d2);

        let result = Rock::get_vertical_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(d2)
            | BitBoard::from(d3)
            | BitBoard::from(d5)
            | BitBoard::from(d6)
            | BitBoard::from(d7);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_moves_in_corner_unobstructed() {
        let a1 = Square::new(File::A, Rank::One);
        let piece = BitBoard::from(a1);
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::new(0);

        let result = Rock::get_moves(&piece, a1, &own_pieces, &opponent_pieces, &Color::White);
        let expected = (BitBoard::from(File::A) | BitBoard::from(Rank::One)) & !BitBoard::from(a1);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_rock_fully_blocked_by_own_pieces() {
        let square = Square::new(File::D, Rank::Four);
        let d1 = Square::new(File::D, Rank::One);
        let d2 = Square::new(File::D, Rank::Two);
        let d3 = Square::new(File::D, Rank::Three);
        let d5 = Square::new(File::D, Rank::Five);
        let d6 = Square::new(File::D, Rank::Six);
        let d7 = Square::new(File::D, Rank::Seven);
        let d8 = Square::new(File::D, Rank::Eight);
        let a4 = Square::new(File::A, Rank::Four);
        let b4 = Square::new(File::B, Rank::Four);
        let c4 = Square::new(File::C, Rank::Four);
        let e4 = Square::new(File::E, Rank::Four);
        let f4 = Square::new(File::F, Rank::Four);
        let g4 = Square::new(File::G, Rank::Four);
        let h4 = Square::new(File::H, Rank::Four);

        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::from(d1)
            | BitBoard::from(d2)
            | BitBoard::from(d3)
            | BitBoard::from(d5)
            | BitBoard::from(d6)
            | BitBoard::from(d7)
            | BitBoard::from(d8)
            | BitBoard::from(a4)
            | BitBoard::from(b4)
            | BitBoard::from(c4)
            | BitBoard::from(e4)
            | BitBoard::from(f4)
            | BitBoard::from(g4)
            | BitBoard::from(h4);
        let opponent_pieces = BitBoard::new(0);

        let result = Rock::get_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::new(0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rock_fully_blocked_but_can_capture_opponents() {
        let square = Square::new(File::D, Rank::Four);
        let d1 = Square::new(File::D, Rank::One);
        let d2 = Square::new(File::D, Rank::Two);
        let d3 = Square::new(File::D, Rank::Three);
        let d5 = Square::new(File::D, Rank::Five);
        let d6 = Square::new(File::D, Rank::Six);
        let d7 = Square::new(File::D, Rank::Seven);
        let d8 = Square::new(File::D, Rank::Eight);
        let a4 = Square::new(File::A, Rank::Four);
        let b4 = Square::new(File::B, Rank::Four);
        let c4 = Square::new(File::C, Rank::Four);
        let e4 = Square::new(File::E, Rank::Four);
        let f4 = Square::new(File::F, Rank::Four);
        let g4 = Square::new(File::G, Rank::Four);
        let h4 = Square::new(File::H, Rank::Four);

        let piece = BitBoard::from(square);
        let own_pieces = BitBoard::from(d1)
            | BitBoard::from(d2)
            | BitBoard::from(d3)
            | BitBoard::from(d5)
            | BitBoard::from(d6)
            | BitBoard::from(d7)
            | BitBoard::from(d8)
            | BitBoard::from(a4)
            | BitBoard::from(b4)
            | BitBoard::from(c4)
            | BitBoard::from(f4)
            | BitBoard::from(g4)
            | BitBoard::from(h4);
        let opponent_pieces = BitBoard::from(e4);

        let result = Rock::get_moves(&piece, square, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(e4);
        assert_eq!(result, expected);
    }
}
