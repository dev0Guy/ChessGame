use crate::bitboard::BitBoard;
use crate::pieces::common::{Color, PossibleMoves};
use crate::square::{File, Square};

pub(crate) struct King;

impl PossibleMoves for King{
    fn get_moves(piece: &BitBoard, _square: Square, own_pieces: &BitBoard, _opponent_pieces: &BitBoard, _color: &Color) -> BitBoard {
        let horizontal_movement = (piece << 1) | (piece >> 1) | *piece;
        let movement = horizontal_movement | horizontal_movement << 8 | horizontal_movement >> 8;
        movement & !own_pieces
    }
}



#[cfg(test)]
mod tests {
    use crate::square::Rank;
    use super::*;

    #[test]
    fn test_king_moves_center_blocked() {
        let d3 = Square::new(File::D, Rank::Three);
        let d4 = Square::new(File::D, Rank::Four);
        let d5 = Square::new(File::D, Rank::Five);
        let c3 = Square::new(File::C, Rank::Three);
        let c4 = Square::new(File::C, Rank::Four);
        let c5 = Square::new(File::C, Rank::Five);
        let e3 = Square::new(File::E, Rank::Three);
        let e4 = Square::new(File::E, Rank::Four);
        let e5 = Square::new(File::E, Rank::Five);
        let own_pieces = BitBoard::from(c3) | BitBoard::from(e3) | BitBoard::from(d4);
        let opponent_pieces = BitBoard::from(d5);

        let king_moves = King::get_moves(&BitBoard::from(d4), d4, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(d5) | BitBoard::from(e5)
            | BitBoard::from(c4) | BitBoard::from(e4) | BitBoard::from(c5)
            | BitBoard::from(d3);
        assert_eq!(king_moves, expected);
    }

    #[test]
    fn test_king_moves_left_corner_blocked() {
        let a1 = Square::new(File::A, Rank::One);
        let a2 = Square::new(File::A, Rank::Two);
        let b1 = Square::new(File::B, Rank::One);
        let b2 = Square::new(File::B, Rank::Two);

        let own_pieces = BitBoard::from(a2) | BitBoard::from(b1) | BitBoard::from(a1);
        let opponent_pieces = BitBoard::empty();

        let king_moves = King::get_moves(&BitBoard::from(a1), a1, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::from(b2);
        assert_eq!(king_moves, expected);
    }

    #[test]
    fn test_king_moves_left_blocked_from_all_direction() {
        let a1 = Square::new(File::A, Rank::One);
        let a2 = Square::new(File::A, Rank::Two);
        let b1 = Square::new(File::B, Rank::One);
        let b2 = Square::new(File::B, Rank::Two);

        let own_pieces = BitBoard::from(a2) | BitBoard::from(b1) | BitBoard::from(b2) | BitBoard::from(a1);
        let opponent_pieces = BitBoard::empty();

        let king_moves = King::get_moves(&BitBoard::from(a1), a1, &own_pieces, &opponent_pieces, &Color::White);

        let expected = BitBoard::empty();
        assert_eq!(king_moves, expected);
    }


}