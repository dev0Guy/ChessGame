use crate::engine::board::pieces::Side;
use super::common::{RegularMoveOptions, AttackMoveOptions, movement_by_const};
use crate::game::Position;

pub struct KingMoves;

const MOVEMENT_OPTION: [(i8, i8); 8] = [
    (1, 0), (-1, 0), (0, 1), (0, -1),
    (1, 1), (1, -1), (-1, 1), (-1, -1),
];

impl RegularMoveOptions for KingMoves {
    fn move_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_ {
        movement_by_const(MOVEMENT_OPTION.to_vec(), pos)
    }
}

impl AttackMoveOptions for KingMoves {
    fn attack_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_ {
        Self::move_options(pos, side)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::position::{File, Rank};
    use std::collections::HashSet;

    // board: https://lichess.org/editor/8/8/8/8/3K4/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_king_moves_all_options() {
        let pos = Position::new(File::D, Rank::Four);

        let expected: HashSet<_> = vec![
            Position::new(File::E, Rank::Four),
            Position::new(File::E, Rank::Five),
            Position::new(File::D, Rank::Five),
            Position::new(File::C, Rank::Five),
            Position::new(File::C, Rank::Four),
            Position::new(File::C, Rank::Three),
            Position::new(File::D, Rank::Three),
            Position::new(File::E, Rank::Three),
        ]
            .into_iter()
            .collect();

        let result: HashSet<_> = KingMoves::move_options(&pos, Side::White).collect();
        assert_eq!(result, expected);
    }

    // board: https://lichess.org/editor/7K/8/8/8/8/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_king_moves_limited_options() {
        let pos = Position::new(File::H, Rank::Eight);

        let expected: HashSet<_> = vec![
            Position::new(File::G, Rank::Eight),
            Position::new(File::G, Rank::Seven),
            Position::new(File::H, Rank::Seven),
        ]
            .into_iter()
            .collect();

        let result: HashSet<_> = KingMoves::move_options(&pos, Side::White).collect();
        assert_eq!(result, expected);
    }
}