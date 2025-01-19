use crate::engine::board::pieces::Side;
use crate::game::moves::common::{HorizontalMovement, AttackMoveOptions, VerticalMovement, DiagonalMovement, AntiDiagonalMovement, RegularMoveOptions};
use crate::game::Position;

pub struct QueenMoves;

impl RegularMoveOptions for QueenMoves {
    // Return x-ray of all possible movement (on empty board) diagonal, horizontal and vertical
    fn move_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_ {
        HorizontalMovement::move_options(pos, side)
            .chain(VerticalMovement::move_options(pos, side))
            .chain(DiagonalMovement::move_options(pos, side))
            .chain(AntiDiagonalMovement::move_options(pos, side))
    }
}

impl AttackMoveOptions for QueenMoves{
    fn attack_option(pos: &Position, side: Side) -> impl Iterator<Item=Position> + '_ {
        Self::move_options(pos, side)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::position::{File, Rank};

    // board: https://lichess.org/editor/8/8/8/8/3Q4/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_queen_move_options() {
        let pos = Position::new(File::D, Rank::Four);

        let expected: Vec<_> = vec![
            Position::new(File::A, Rank::Four),
            Position::new(File::B, Rank::Four),
            Position::new(File::C, Rank::Four),
            Position::new(File::E, Rank::Four),
            Position::new(File::F, Rank::Four),
            Position::new(File::G, Rank::Four),
            Position::new(File::H, Rank::Four),
            Position::new(File::D, Rank::One),
            Position::new(File::D, Rank::Two),
            Position::new(File::D, Rank::Three),
            Position::new(File::D, Rank::Five),
            Position::new(File::D, Rank::Six),
            Position::new(File::D, Rank::Seven),
            Position::new(File::D, Rank::Eight),
            Position::new(File::E, Rank::Five),
            Position::new(File::F, Rank::Six),
            Position::new(File::G, Rank::Seven),
            Position::new(File::H, Rank::Eight),
            Position::new(File::C, Rank::Three),
            Position::new(File::B, Rank::Two),
            Position::new(File::A, Rank::One),
            Position::new(File::C, Rank::Five),
            Position::new(File::B, Rank::Six),
            Position::new(File::A, Rank::Seven),
            Position::new(File::E, Rank::Three),
            Position::new(File::F, Rank::Two),
            Position::new(File::G, Rank::One),
        ];

        let result: Vec<_> = QueenMoves::move_options(&pos, Side::White).collect();
        assert_eq!(result, expected);
    }

    // board: https://lichess.org/editor/8/8/8/8/3Q4/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_queen_attack_options() {
        let pos = Position::new(File::D, Rank::Four);

        let expected: Vec<_> = vec![
            Position::new(File::A, Rank::Four),
            Position::new(File::B, Rank::Four),
            Position::new(File::C, Rank::Four),
            Position::new(File::E, Rank::Four),
            Position::new(File::F, Rank::Four),
            Position::new(File::G, Rank::Four),
            Position::new(File::H, Rank::Four),
            Position::new(File::D, Rank::One),
            Position::new(File::D, Rank::Two),
            Position::new(File::D, Rank::Three),
            Position::new(File::D, Rank::Five),
            Position::new(File::D, Rank::Six),
            Position::new(File::D, Rank::Seven),
            Position::new(File::D, Rank::Eight),
            Position::new(File::E, Rank::Five),
            Position::new(File::F, Rank::Six),
            Position::new(File::G, Rank::Seven),
            Position::new(File::H, Rank::Eight),
            Position::new(File::C, Rank::Three),
            Position::new(File::B, Rank::Two),
            Position::new(File::A, Rank::One),
            Position::new(File::C, Rank::Five),
            Position::new(File::B, Rank::Six),
            Position::new(File::A, Rank::Seven),
            Position::new(File::E, Rank::Three),
            Position::new(File::F, Rank::Two),
            Position::new(File::G, Rank::One),
        ];

        let result: Vec<_> = QueenMoves::attack_option(&pos, Side::White).collect();
        assert_eq!(result, expected);
    }
}