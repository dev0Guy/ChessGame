use crate::engine::board::pieces::Side;
use crate::game::moves::common::{movement_by_const, AttackMoveOptions, RegularMoveOptions};
use crate::game::Position;

pub struct KnightMoves;

const MOVEMENT_OPTION: [(i8, i8); 8] = [
    (2, 1), (2, -1), (-2, 1), (-2, -1),
    (1, 2), (1, -2), (-1, 2), (-1, -2),
];

impl RegularMoveOptions for KnightMoves {
    // Return x-ray of all possible movement (on empty board both)
    fn move_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_ {
        movement_by_const(MOVEMENT_OPTION.to_vec(), pos)
    }
}

impl AttackMoveOptions for KnightMoves {
    fn attack_options(pos: &Position, side: Side) -> impl Iterator<Item=Position> + '_ {
        Self::move_options(pos, side)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::position::{File, Rank};

    // board: https://lichess.org/editor/8/8/8/8/3N4/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_knight_moves_all_board() {
        let pos = Position::new(File::D, Rank::Four);

        let expected: Vec<_> = vec![
            Position::new(File::F, Rank::Five),
            Position::new(File::F, Rank::Three),
            Position::new(File::B, Rank::Five),
            Position::new(File::B, Rank::Three),
            Position::new(File::E, Rank::Six),
            Position::new(File::E, Rank::Two),
            Position::new(File::C, Rank::Six),
            Position::new(File::C, Rank::Two),
        ];

        let result: Vec<_> = KnightMoves::move_options(&pos, Side::White).collect();
        assert_eq!(result, expected);
    }

    //board: https://lichess.org/editor/7N/8/8/8/8/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_knight_moves_at_edge() {
        let pos = Position::new(File::H, Rank::Eight);

        let expected: Vec<_> = vec![
            Position::new(File::F, Rank::Seven),
            Position::new(File::G, Rank::Six),
        ];

        let result: Vec<_> = KnightMoves::move_options(&pos, Side::White).collect();
        assert_eq!(result, expected);
    }
}

