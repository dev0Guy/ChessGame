use crate::engine::board::pieces::Side;
use super::common::{RegularMoveOptions, AttackMoveOptions, movement_by_const};
use crate::game::Position;
use crate::game::position::Rank;

pub struct PawnMoves;


impl RegularMoveOptions for PawnMoves {
    fn move_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_ {
        let mut moves = match side {
            Side::White => vec![(0, 1)],
            Side::Black => vec![(0, -1)]
        };

        if (side == Side::White && pos.rank == Rank::Two) || (side == Side::Black && pos.rank == Rank::Seven) {
            moves.push(match side {
                Side::White => (0, 2),
                Side::Black => (0, -2),
            });
        }
        movement_by_const(moves, pos)
    }
}

impl AttackMoveOptions for PawnMoves {
    fn attack_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_ {
        let diagonals = match side {
            Side::White => vec![(1, 1), (-1, 1)],
            Side::Black => vec![(1, -1), (-1, -1)],
        };

        movement_by_const(diagonals, pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::board::pieces::Side;
    use crate::game::position::{File, Rank};
    use std::collections::HashSet;

    #[test]
    fn test_white_pawn_moves_initial() {
        let pos = Position::new(File::D, Rank::Two);

        let expected: HashSet<_> = vec![
            Position::new(File::D, Rank::Three),
            Position::new(File::D, Rank::Four),
        ]
            .into_iter()
            .collect();

        let result: HashSet<_> = PawnMoves::move_options(&pos, Side::White).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_black_pawn_moves_initial() {
        let pos = Position::new(File::D, Rank::Seven);

        let expected: HashSet<_> = vec![
            Position::new(File::D, Rank::Six),
            Position::new(File::D, Rank::Five),
        ]
            .into_iter()
            .collect();

        let result: HashSet<_> = PawnMoves::move_options(&pos, Side::Black).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_white_pawn_moves_non_initial() {
        let pos = Position::new(File::D, Rank::Three);

        let expected: HashSet<_> = vec![Position::new(File::D, Rank::Four)]
            .into_iter()
            .collect();

        let result: HashSet<_> = PawnMoves::move_options(&pos, Side::White).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_black_pawn_moves_non_initial() {
        let pos = Position::new(File::D, Rank::Six);

        let expected: HashSet<_> = vec![Position::new(File::D, Rank::Five)]
            .into_iter()
            .collect();

        let result: HashSet<_> = PawnMoves::move_options(&pos, Side::Black).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_white_pawn_attack() {
        let pos = Position::new(File::D, Rank::Two);

        let expected: HashSet<_> = vec![
            Position::new(File::C, Rank::Three),
            Position::new(File::E, Rank::Three),
        ]
            .into_iter()
            .collect();

        let result: HashSet<_> = PawnMoves::attack_options(&pos, Side::White).collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_black_pawn_attack() {
        let pos = Position::new(File::D, Rank::Seven);

        let expected: HashSet<_> = vec![
            Position::new(File::C, Rank::Six),
            Position::new(File::E, Rank::Six),
        ]
            .into_iter()
            .collect();

        let result: HashSet<_> = PawnMoves::attack_options(&pos, Side::Black).collect();
        assert_eq!(result, expected);
    }
}