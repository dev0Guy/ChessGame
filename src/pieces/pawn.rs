use std::num::IntErrorKind::Empty;
use crate::square::{Rank, File};
use crate::bitboard::BitBoard;
use super::common::{Color, PossibleMoves};


/// Description
/// - [X] (White) pawn can make a single step forward (UP) for each rank expect 8 rank and only if free.
/// - [X] (White) pawn can make a double step forward (UP) to only  rank 4 and only if free.
/// - (White) pawn can capture (LEFT) for all file expect rank 8 to all file expect H and if not empty with enemy piece
/// - (White) pawn can capture (RIGHT) for all file expect rank 8 to all file expect A and if not empty with enemy piece
/// - (White) pawn promotion: one square forward or a diagonal capture results in it landing on rank 8
/// - [X] (Black) pawn can make a single step forward (DOWN) for each rank expect 1 rank and only if free.
/// - [X] (Black) pawn can make a double step forward (DOWN) to only  rank 5 and only if free.
/// - (Black) pawn can capture (LEFT) for all file expect rank 1 to all file expect H and if not empty with enemy piece
/// - (Black) pawn can capture (RIGHT) for all file expect rank 1 to all file expect A and if not empty with enemy piece
/// - (BLACK) pawn promotion: one square forward or a diagonal capture results in it landing on rank 1
struct Pawn;


impl PossibleMoves for Pawn {
    fn get_moves(piece: &BitBoard, own_pieces: &BitBoard, opponent_pics: &BitBoard, color: Color) -> BitBoard {
        todo!()
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
    fn possible_single_step(piece:&BitBoard, own_pieces: &BitBoard, opponent_pics: &BitBoard, color: Color) -> BitBoard{
        let empty = Self::empty(own_pieces, opponent_pics);
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
    fn possible_double_step(piece:&BitBoard, own_pieces: &BitBoard, opponent_pics: &BitBoard, color: Color) -> BitBoard{
        let empty = Self::empty(own_pieces, opponent_pics);
        // (empty << 8) to make sure doesn't pass throw occupied square
        match color {
            Color::White => (piece << (2*8)) & empty & (empty << 8) & !BitBoard::from(Rank::Four),
            Color::Black => (piece >> (2*8)) & empty & (empty >> 8) &!BitBoard::from(Rank::Five)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// [from](https://lichess.org/editor/8/8/8/8/8/8/3P4/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/8/8/8/3P4/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_pawn_single_step_white() {
        let piece = BitBoard::new(0x0000000000000800);
        let own_pieces = BitBoard::new(0x0000000000000800);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_single_step(&piece, &own_pieces, &opponent_pieces, Color::White);

        let expected = BitBoard::new(0x0000000000080000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/3p4/8/8/8/8/8/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/3p4/8/8/8/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_pawn_single_step_black() {
        let piece = BitBoard::new(0x0008000000000000);
        let own_pieces = BitBoard::new(0x0008000000000000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_single_step(&piece, &own_pieces, &opponent_pieces, Color::Black);

        let expected = BitBoard::new(0x0000080000000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/8/8/8/3N4/3P4/8_w_HAha_-_0_1?color=white) -> X
    #[test]
    fn test_possible_pawn_single_step_white_blocked() {
        let piece = BitBoard::new(0x0000000000000800);
        let own_pieces = BitBoard::new(0x0000000000080000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_single_step(&piece, &own_pieces, &opponent_pieces, Color::White);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/3p4/3n4/8/8/8/8/8_w_HAha_-_0_1?color=white) -> X
    #[test]
    fn test_possible_pawn_single_step_black_blocked() {
        let piece = BitBoard::new(0x0008000000000000);
        let own_pieces = BitBoard::new(0x0000080000000000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_single_step(&piece, &own_pieces, &opponent_pieces, Color::Black);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/8/8/8/8/3P4/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/8/8/3P4/8/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_double_step_white() {
        let piece = BitBoard::new(0x0000000000000800);
        let own_pieces = BitBoard::new(0x0000000000000800);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_double_step(&piece, &own_pieces, &opponent_pieces, Color::White);

        let expected = BitBoard::new(0x0000000008000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/3p4/8/8/8/8/8/8_w_HAha_-_0_1?color=white) -> [to](https://lichess.org/editor/8/8/8/3p4/8/8/8/8_w_HAha_-_0_1?color=white)
    #[test]
    fn test_possible_double_step_black() {
        let piece = BitBoard::new(0x0008000000000000);
        let own_pieces = BitBoard::new(0x0008000000000000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_double_step(&piece, &own_pieces, &opponent_pieces, Color::Black);

        let expected = BitBoard::new(0x0000000800000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/8/8/8/3P4/3P4/8_w_HAha_-_0_1?color=white) -> X
    #[test]
    fn test_possible_double_step_white_blocked_first_square() {
        let piece = BitBoard::new(0x0000000000000800);
        let own_pieces = BitBoard::new(0x0000000000080000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_double_step(&piece, &own_pieces, &opponent_pieces, Color::White);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/8/8/8/3P4/8/3P4/8_w_HAha_-_0_1?color=white) -> [to]X
    #[test]
    fn test_possible_double_step_white_blocked_second_square() {
        let piece = BitBoard::new(0x0000000000000800);
        let own_pieces = BitBoard::new(0x0000000008000000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_double_step(&piece, &own_pieces, &opponent_pieces, Color::White);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/3p4/3p4/8/8/8/8/8_w_HAha_-_0_1?color=white) -> X
    #[test]
    fn test_possible_double_step_black_blocked_first_square() {
        let piece = BitBoard::new(0x0008000000000000);
        let own_pieces = BitBoard::new(0x0000080000000000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_double_step(&piece, &own_pieces, &opponent_pieces, Color::Black);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }

    /// [from](https://lichess.org/editor/8/3p4/8/3p4/8/8/8/8_w_HAha_-_0_1?color=white) -> X
    #[test]
    fn test_possible_double_step_black_blocked_second_square() {
        let piece = BitBoard::new(0x0008000000000000);
        let own_pieces = BitBoard::new(0x0000000800000000);
        let opponent_pieces = BitBoard::new(0x0000000000000000);

        let result = Pawn::possible_double_step(&piece, &own_pieces, &opponent_pieces, Color::Black);

        let expected = BitBoard::new(0x0000000000000000);
        assert_eq!(result, expected);
    }

}

