use crate::square::{Rank, File, Square};
use crate::bitboard::BitBoard;
use super::common::{Color, PossibleMoves};


pub(crate) struct Pawn;


impl PossibleMoves for Pawn {
    fn get_moves(piece: &BitBoard, _square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
        Self::possible_single_step(piece, own_pieces, opponent_pieces, color)
            | Self::possible_double_step(piece, own_pieces, opponent_pieces, color)
            | Self::possible_capture_step(piece, own_pieces, opponent_pieces, color)
    }

    fn get_capture(piece: &BitBoard, _square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
        Self::possible_capture_step(piece, own_pieces, opponent_pieces, color)
    }
}

impl Pawn {
    /// Calculates the possible single-step moves for pawns of the given color.
    /// Determines the squares to which a pawn can move forward by one rank.
    /// A pawn can move forward if the square is empty and it is not located on the 1/8'th rank
    /// (since pawns cannot move forward once they reach the promotion rank).
    /// # Parameters
    /// - `piece`: A &[`BitBoard`]  representing the positions of pawns to evaluate.
    /// - `own_pieces`: A &[`BitBoard`]  representing the positions of all friendly pieces.
    /// - `opponent_pieces`: A &[`BitBoard`] representing the positions of all opponent pieces.
    /// - `color`: The [`Color`]  of the pawns being evaluated (`Color::White` or `Color::Black`).
    ///
    /// # Returns
    /// A [`BitBoard`] where each set bit represents a valid single-step move for the pawns.
    #[inline]
    fn possible_single_step(piece: &BitBoard, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
        let empty = Self::empty(own_pieces, opponent_pieces);
        match color {
            Color::White => (piece << 8) & empty & !BitBoard::from(Rank::Eight),
            Color::Black => (piece >> 8) & empty & !BitBoard::from(Rank::One)
        }
    }

    /// Calculates the possible double-step moves for pawns of the given color.
    ///
    /// Pawns are only allowed to make a double-step move under specific conditions:
    /// - The pawn must be on its initial rank (`Rank::Two` for white pawns, `Rank::Seven` for black pawns).
    /// - Both the square directly in front of the pawn and the square two ranks ahead must be empty.
    ///
    /// # Parameters
    /// - `piece`: A &[`BitBoard`] representing the positions of pawns to evaluate.
    /// - `own_pieces`: A &[`BitBoard`] representing the positions of all friendly pieces.
    /// - `opponent_pieces`: A &[`BitBoard`] representing the positions of all opponent pieces.
    /// - `color`: The [`Color`] of the pawns being evaluated (`Color::White` or `Color::Black`).
    ///
    /// # Returns
    /// A [`BitBoard`] where each set bit represents a valid double-step move for the pawns.
    #[inline]
    fn possible_double_step(piece: &BitBoard, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
        let empty = Self::empty(own_pieces, opponent_pieces);
        // (empty << 8) to make sure doesn't pass throw occupied square
        match color {
            Color::White => (piece << (2 * 8)) & empty & (empty << 8) & BitBoard::from(Rank::Four),
            Color::Black => (piece >> (2 * 8)) & empty & (empty >> 8) & BitBoard::from(Rank::Five)
        }
    }

    /// Calculates the possible capture moves for pawns of the given color.
    ///
    /// Pawns can capture diagonally forward, either to the left or right, under the following conditions:
    /// - The target square must contain an opponent's piece.
    /// - The pawn must not be on the edge of the board where capturing diagonally would wrap around (e.g., `File::A` or `File::H`).
    /// - The pawn must not be on the promotion rank (`Rank::Eight` for white pawns, `Rank::One` for black pawns).
    ///
    /// # Parameters
    /// - `piece`: A &[`BitBoard`] representing the positions of pawns to evaluate.
    /// - `own_pieces`: A &[`BitBoard`] representing the positions of all friendly pieces (not used directly here but included for symmetry).
    /// - `opponent_pieces`: A &[`BitBoard`] representing the positions of all opponent pieces.
    /// - `color`: The &[`Color`] of the pawns being evaluated (`Color::White` or `Color::Black`).
    ///
    /// # Returns
    /// A [`BitBoard`] where each set bit represents a valid capture move for the pawns.
    #[inline]
    fn possible_capture_step(piece: &BitBoard, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
        // (8+1) for left capture (row and rank left)
        // (8-1) for right capture (row and rank left)
        let right_capture = match color {
            Color::White => (*piece & !BitBoard::from(Rank::Eight) & !BitBoard::from(File::H)) << (8 + 1),
            Color::Black => (*piece & !BitBoard::from(Rank::One) & !BitBoard::from(File::A)) >> (8 + 1),
        };
        let left_capture = match color {
            Color::White => (*piece & !BitBoard::from(Rank::Eight) & !BitBoard::from(File::A)) << (8 - 1),
            Color::Black => (*piece & !BitBoard::from(Rank::One) & !BitBoard::from(File::H)) >> (8 - 1),
        };
        ((left_capture | right_capture) & *opponent_pieces) & !own_pieces
    }

}


// TODO: rewrite test with Square format to bitboard for more clear representation
// TODO: add test for all together (get moves)
#[cfg(test)]
mod tests {
    use crate::square::Square;
    use super::*;

    /// [from](https://lichess.org/editor/8/8/8/8/8/8/3P4/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/8/8/8/3P4/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_pawn_single_step_white() {
        let piece = BitBoard::from(Square::new(File::D, Rank::Two));
        let own_pieces = BitBoard::from(Square::new(File::D, Rank::Two));
        let opponent_pieces = BitBoard::empty();

        let result = Pawn::possible_single_step(&piece, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(Square::new(File::D, Rank::Three));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_for_guy(){
        let piece = BitBoard::new(0x400000000000);
        let own_pieces = BitBoard::new(0x408000000000);
        let opponent_pieces = BitBoard::new(0xff9f000000000000);
        println!("own {:?}", own_pieces);
        println!("opponent pieces {:?}", opponent_pieces);
        println!("Location {:?}", piece);
        let result = Pawn::possible_capture_step(&piece, &own_pieces, &opponent_pieces, &Color::White);
        let expected = BitBoard::from(Square::new(File::H, Rank::Seven));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_for_guy_black(){
        let piece = BitBoard::new(0x80000000000000);
        let own_pieces = BitBoard::new(0xff9f000000000000);
        let opponent_pieces = BitBoard::new(0x400000000000);
        println!("own {:?}", own_pieces);
        println!("opponent pieces {:?}", opponent_pieces);
        println!("Location {:?}", piece);
        let result = Pawn::possible_capture_step(&piece, &own_pieces, &opponent_pieces, &Color::Black);
        let expected = BitBoard::from(Square::new(File::G, Rank::Six));
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/3p4/8/8/8/8/8/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/3p4/8/8/8/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_pawn_single_step_black() {
        let piece = BitBoard::new(0x0008000000000000);
        let own_pieces = BitBoard::new(0x0008000000000000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_single_step(&piece, &own_pieces, &opponent_pieces, &Color::Black);

        let expected = BitBoard::new(0x0000080000000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/8/8/8/3N4/3P4/8_w_HAha_-_0_1?color=white) -> X
    #[test]
    fn test_possible_pawn_single_step_white_blocked() {
        let piece = BitBoard::new(0x0000000000000800);
        let own_pieces = BitBoard::new(0x0000000000080000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_single_step(&piece, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/3p4/3n4/8/8/8/8/8_w_HAha_-_0_1?color=white) -> X
    #[test]
    fn test_possible_pawn_single_step_black_blocked() {
        let piece = BitBoard::new(0x0008000000000000);
        let own_pieces = BitBoard::new(0x0000080000000000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_single_step(&piece, &own_pieces, &opponent_pieces, &Color::Black);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/8/8/8/8/3P4/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/8/8/3P4/8/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_double_step_white() {
        let piece = BitBoard::new(0x0000000000000800);
        let own_pieces = BitBoard::new(0x0000000000000800);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_double_step(&piece, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::new(0x0000000008000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/3p4/8/8/8/8/8/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/8/3p4/8/8/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_double_step_black() {
        let piece = BitBoard::new(0x0008000000000000);
        let own_pieces = BitBoard::new(0x0008000000000000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_double_step(&piece, &own_pieces, &opponent_pieces, &Color::Black);

        let expected = BitBoard::new(0x0000000800000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/8/8/8/3P4/3P4/8_w_HAha_-_0_1?color=white) -> X
    #[test]
    fn test_possible_double_step_white_blocked_first_square() {
        let piece = BitBoard::new(0x0000000000000800);
        let own_pieces = BitBoard::new(0x0000000000080000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_double_step(&piece, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/8/8/3P4/8/3P4/8_w_HAha_-_0_1?color=white) -> [to]X
    #[test]
    fn test_possible_double_step_white_blocked_second_square() {
        let piece = BitBoard::new(0x0000000000000800);
        let own_pieces = BitBoard::new(0x0000000008000000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_double_step(&piece, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/3p4/3p4/8/8/8/8/8_w_HAha_-_0_1?color=white) -> X
    #[test]
    fn test_possible_double_step_black_blocked_first_square() {
        let piece = BitBoard::new(0x0008000000000000);
        let own_pieces = BitBoard::new(0x0000080000000000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_double_step(&piece, &own_pieces, &opponent_pieces, &Color::Black);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/3p4/8/3p4/8/8/8/8_w_HAha_-_0_1?color=white) -> X
    #[test]
    fn test_possible_double_step_black_blocked_second_square() {
        let piece = BitBoard::new(0x0008000000000000);
        let own_pieces = BitBoard::new(0x0000000800000000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_double_step(&piece, &own_pieces, &opponent_pieces, &Color::Black);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }


    /// [from](https://lichess.org/editor/8/8/8/8/8/2b5/3P4/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/8/8/8/2P5/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_capture_step_white_left() {
        let piece = BitBoard::new(0x0000000000000800);
        let own_pieces = BitBoard::new(0x0000000000000800);
        let opponent_pieces = BitBoard::new(0x0000000000100000);

        let result = Pawn::possible_capture_step(&piece, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::new(0x0000000000100000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/8/8/8/4b3/3P4/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/8/8/8/4P3/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_capture_step_white_right() {
        let d2 = Square::new(File::D, Rank::Two);
        let e3 = Square::new(File::E, Rank::Three);
        let piece = BitBoard::from(d2);
        let own_pieces = BitBoard::empty();
        let opponent_pieces = BitBoard::from(e3);

        let result = Pawn::possible_capture_step(&piece, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(e3);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/3p4/2B5/8/8/8/8/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/2p5/8/8/8/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_capture_step_black_left() {
        let d7 = Square::new(File::D, Rank::Seven);
        let c6 = Square::new(File::C, Rank::Six);
        let piece = BitBoard::from(d7);
        let own_pieces = BitBoard::empty();
        let opponent_pieces = BitBoard::from(c6);

        let result = Pawn::possible_capture_step(&piece, &own_pieces, &opponent_pieces, &Color::Black);

        let expected = BitBoard::from(c6);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/3p4/4B3/8/8/8/8/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/4p3/8/8/8/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_capture_step_black_right() {
        let d7 = Square::new(File::D, Rank::Seven);
        let d6 = Square::new(File::D, Rank::Six);
        let e6 = Square::new(File::E, Rank::Six);

        let piece = BitBoard::from(d7);
        let own_pieces = BitBoard::from(d6);
        let opponent_pieces = BitBoard::from(e6);

        let result = Pawn::possible_capture_step(&piece, &own_pieces, &opponent_pieces, &Color::Black);

        let expected = BitBoard::from(e6);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/8/8/8/1b6/P7/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/8/8/8/1P6/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_capture_step_edge_file_a() {
        let piece = BitBoard::new(0x0000000000000100);
        let own_pieces = BitBoard::new(0x0000000000000100);
        let opponent_pieces = BitBoard::new(0x0000000000020000);

        let result = Pawn::possible_capture_step(&piece, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::new(0x0000000000020000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/8/8/8/6p1/7P/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/8/8/8/6P1/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_capture_step_edge_file_h() {
        let piece = BitBoard::new(0x0000000000008000);
        let own_pieces = BitBoard::new(0x0000000000008000);
        let opponent_pieces = BitBoard::new(0x0000000000400000);

        let result = Pawn::possible_capture_step(&piece, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::new(0x0000000000400000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/8/8/8/8/3P4/8_w_HAha_-_0_1?color=white) -> X
    #[test]
    fn test_possible_capture_step_no_opponent() {
        let piece = BitBoard::new(0x0000000000000800);
        let own_pieces = BitBoard::new(0x0000000000000800);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_capture_step(&piece, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }

}

