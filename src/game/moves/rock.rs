use crate::engine::board::pieces::Side;
use super::common::{HorizontalMovement, VerticalMovement, AttackMoveOptions, RegularMoveOptions};
use crate::game::Position;

pub struct RockMoves;

impl RegularMoveOptions for RockMoves {
    // Return x-ray of all possible movement (on empty board)
    fn move_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_ {
        HorizontalMovement::move_options(pos, side)
            .chain(VerticalMovement::move_options(pos,side))
    }
}

impl AttackMoveOptions for RockMoves {
    // Return x-ray of all possible movement (on empty board)
    fn attack_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_ {
        Self::move_options(pos, side)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::position::{File, Rank};

    // pos: https://lichess.org/editor/8/8/8/8/3R4/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_rook_movement() {
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
        ];

        let result: Vec<_> = RockMoves::move_options(&pos, Side::White).collect();
        assert_eq!(result, expected);
    }
}