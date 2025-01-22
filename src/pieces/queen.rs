use crate::bitboard::BitBoard;
use crate::pieces::common::{Color, PossibleMoves};
use crate::square::Square;
use super::{Rock, Bishop};
/// Description
/// Combination of both bishop and rock (can or between each movement map)
pub(crate) struct Queen;


impl PossibleMoves for Queen {
    fn get_moves(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
        Rock::get_moves(piece, square, own_pieces, opponent_pieces, color)
        | Bishop::get_moves(piece, square, own_pieces, opponent_pieces, color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitboard::BitBoard;
    use crate::square::{File, Rank, Square};
    use crate::pieces::common::Color;

    #[test]
    fn test_queen_moves_center_unobstructed() {
        let queen = BitBoard::from(Square::new(File::D, Rank::Four));
        let own_pieces = BitBoard::new(0); // No friendly pieces
        let opponent_pieces = BitBoard::new(0); // No opponent pieces

        let moves = Queen::get_moves(&queen, Square::new(File::D, Rank::Four), &own_pieces, &opponent_pieces, &Color::White);

        let mut expected = BitBoard::empty();
        expected = expected | (BitBoard::from(File::D) & !queen);
        expected = expected | (BitBoard::from(Rank::Four) & !queen);
        let diagonals = vec![
            Square::new(File::C, Rank::Three),
            Square::new(File::B, Rank::Two),
            Square::new(File::A, Rank::One),
            Square::new(File::E, Rank::Five),
            Square::new(File::F, Rank::Six),
            Square::new(File::G, Rank::Seven),
            Square::new(File::H, Rank::Eight),

            Square::new(File::A, Rank::Seven),
            Square::new(File::B, Rank::Six),
            Square::new(File::C, Rank::Five),
            Square::new(File::E, Rank::Three),
            Square::new(File::F, Rank::Two),
            Square::new(File::G, Rank::One),
        ];
        for square in diagonals {
            expected = expected | BitBoard::from(square);
        }

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_queen_moves_in_corner_blocked(){
        let a1 = Square::new(File::A, Rank::One);
        let a2 = Square::new(File::A, Rank::Two);
        let b2 = Square::new(File::B, Rank::Two);
        let b1 = Square::new(File::B, Rank::One);

        let own_pieces = BitBoard::from(a2) | BitBoard::from(b1);
        let opponent_pieces = BitBoard::from(b2);

        let moves = Queen::get_moves(&BitBoard::from(a1), a1, &own_pieces, &opponent_pieces, &Color::White);

        let mut expected = opponent_pieces;
        assert_eq!(moves, expected);

    }

}