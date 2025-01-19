use crate::engine::board::pieces::Side;
use super::common::{DiagonalMovement, AntiDiagonalMovement, AttackMoveOptions, RegularMoveOptions};
use crate::game::Position;

pub struct BishopMoves;

impl RegularMoveOptions for BishopMoves {
    // Return x-ray of all possible movement (on empty board both)
    fn move_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_ {
        DiagonalMovement::move_options(pos, side)
            .chain(AntiDiagonalMovement::move_options(pos, side))
    }
}

impl AttackMoveOptions for BishopMoves{
    fn attack_options(pos: &Position, side: Side) -> impl Iterator<Item=Position> + '_ {
        Self::move_options(pos, side)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::position::{File, Rank};

    #[test]
    fn test_bishop_movement() {
        let pos = Position::new(File::D, Rank::Four);

        let expected: Vec<_> = vec![
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

        let result: Vec<_> = BishopMoves::move_options(&pos, Side::White).collect();
        assert_eq!(result, expected);
    }
}