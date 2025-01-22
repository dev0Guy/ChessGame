use crate::bitboard::BitBoard;
use crate::pieces::common::{Color, PossibleMoves};
use crate::square::{File, Rank, Square};


pub(crate) struct Knight;

impl PossibleMoves for Knight{

    fn get_moves(piece: &BitBoard, _square: Square, own_pieces: &BitBoard, _opponent_pieces: &BitBoard, _color: &Color) -> BitBoard {
        let file_a =  BitBoard::from(File::A);
        let file_b = BitBoard::from(File::B);
        let file_g = BitBoard::from(File::G);
        let file_h = BitBoard::from(File::H);
        let rank_1 = BitBoard::from(Rank::One);
        let rank_2 = BitBoard::from(Rank::Two);
        let rank_7 = BitBoard::from(Rank::Seven);
        let rank_8 = BitBoard::from(Rank::Eight);
        let down_2_constraint = !(rank_1 | rank_2);
        let down_1_constraint = !(rank_1);
        let up_2_constraint = !(rank_8 | rank_7);
        let up_1_constraint = !(rank_8);
        let left_2_constraint = !(file_a | file_b);
        let left_1_constraint = !(file_a);
        let right_2_constraint = !(file_g | file_h);
        let right_1_constraint = !(file_h);
        let moves = BitBoard::empty();
        let moves = moves | ((*piece & up_2_constraint & left_1_constraint) << 15); // Up 2, Left 1
        let moves = moves | ((*piece & up_2_constraint & right_1_constraint) << 17); // Up 2, Right 1
        let moves = moves | ((*piece & down_2_constraint & right_1_constraint) >> 15); // Down 2, Right 1
        let moves = moves | ((*piece & down_2_constraint & left_1_constraint) >> 17); // Down 2, Left 1 XXX
        let moves = moves | ((*piece & left_2_constraint & up_1_constraint) << 6); // Left 2, Up 1
        let moves = moves | ((*piece & left_2_constraint & down_1_constraint) >> 10); // Left 2, Down 1
        let moves = moves | ((*piece & right_2_constraint & up_1_constraint) << 10); // Right 2, Up 1
        let moves = moves | ((*piece & right_2_constraint & down_1_constraint) >> 6); // Right 2, Down 1
        moves & !(piece | own_pieces)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitboard::BitBoard;
    use crate::square::{File, Rank, Square};
    use crate::pieces::common::Color;

    #[test]
    fn test_knight_moves_center() {
        let b3 = Square::new(File::B, Rank::Three);
        let b5 = Square::new(File::B, Rank::Five);
        let c2 = Square::new(File::C, Rank::Two);
        let c6 = Square::new(File::C, Rank::Six);
        let d4 = Square::new(File::D, Rank::Four);
        let e2 = Square::new(File::E, Rank::Two);
        let e6 = Square::new(File::E, Rank::Six);
        let f3 = Square::new(File::F, Rank::Three);
        let f5 = Square::new(File::F, Rank::Five);
        let own_pieces = BitBoard::empty();
        let opponent_pieces = BitBoard::empty();

        let expected = BitBoard::from(b3) | BitBoard::from(b3) | BitBoard::from(b5)
            | BitBoard::from(c2) | BitBoard::from(c6) | BitBoard::from(e2)
            | BitBoard::from(e6) | BitBoard::from(f3) | BitBoard::from(f5);

        let moves = Knight::get_moves(&BitBoard::from(d4), d4, &own_pieces, &opponent_pieces, &Color::White);
        assert_eq!(moves, expected);
    }

    #[test]
    fn test_knight_moves_corner_a1() {
        let a1 = BitBoard::from(Square::new(File::A, Rank::One));
        let own_pieces = BitBoard::empty();
        let opponent_pieces = BitBoard::empty();
        let b3 = BitBoard::from(Square::new(File::B, Rank::Three));
        let c2 = BitBoard::from(Square::new(File::C, Rank::Two));


        let expected = BitBoard::from(b3) | BitBoard::from(c2);

        let moves = Knight::get_moves(&a1, Square::new(File::A, Rank::One), &own_pieces, &opponent_pieces, &Color::White);

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_knight_moves_corner_h8() {
        // Knight at H8
        let knight = BitBoard::from(Square::new(File::H, Rank::Eight));
        let own_pieces = BitBoard::new(0);
        let opponent_pieces = BitBoard::from(Square::new(File::F, Rank::Seven));
        let f7 = Square::new(File::F, Rank::Seven);
        let g6 = Square::new(File::G, Rank::Six);

        let expected = BitBoard::from(f7) | BitBoard::from(g6);

        let moves = Knight::get_moves(&knight, Square::new(File::H, Rank::Eight), &own_pieces, &opponent_pieces, &Color::Black);

        assert_eq!(moves, expected);
    }

    #[test]
    fn test_knight_moves_blocked_by_friendly() {
        let d2 = Square::new(File::D, Rank::Two);
        let d6 = Square::new(File::D, Rank::Six);
        let f2 = Square::new(File::F, Rank::Two);
        let f6 = Square::new(File::F, Rank::Six);
        let g5 = Square::new(File::G, Rank::Five);
        let g3 = Square::new(File::G, Rank::Three);
        let e4 = Square::new(File::E, Rank::Four);
        let c5 = Square::new(File::C, Rank::Five);
        let c3 = Square::new(File::C, Rank::Three);
        let own_pieces = BitBoard::from(c5) | BitBoard::from(g3);
        let opponent_pieces = BitBoard::new(0); // No opponent pieces

        let expected = BitBoard::from(d2) | BitBoard::from(c3)
            | BitBoard::from(c3) | BitBoard::from(d2) | BitBoard::from(d6)
            | BitBoard::from(f2) | BitBoard::from(f6) | BitBoard::from(g5);

        let moves = Knight::get_moves(&BitBoard::from(e4), e4, &own_pieces, &opponent_pieces, &Color::White);

        assert_eq!(moves, expected);
    }

}