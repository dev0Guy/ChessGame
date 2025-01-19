use crate::engine::board::pieces::Side;
use crate::game::{Position};
use crate::game::position::{File, Rank};

pub struct HorizontalMovement;
pub struct VerticalMovement;
pub struct DiagonalMovement;
pub struct AntiDiagonalMovement;

pub(crate) trait AttackMoveOptions {
    fn attack_option(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_;
}

pub(crate) trait RegularMoveOptions {
    fn move_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_;
}

pub(crate) fn movement_by_const(consts: Vec<(i8,i8)>,pos: &Position) -> impl Iterator<Item = Position> + '_{
    consts
        .into_iter()
        .map(|(f, r)| (pos.file.offset(f), pos.rank.offset(r)))
        .filter(|(f, r)| f.is_some() && r.is_some())
        .map(|(f, r)| Position::new(f.unwrap(), r.unwrap()))
}

impl RegularMoveOptions for HorizontalMovement {
    fn move_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_ {
        File::iter()
            .filter(|f| f != &pos.file)
            .map(|f| Position::new(f, pos.rank))
            .into_iter()
    }
}

impl RegularMoveOptions for VerticalMovement {
    fn move_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_{
        Rank::iter()
            .filter(|r| r != &pos.rank)
            .map(|r| Position::new(pos.file, r))
            .into_iter()
    }
}

impl RegularMoveOptions for DiagonalMovement {
    fn move_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_ {
        let top_right = (1..)
            .map(|i| (pos.file.offset(i), pos.rank.offset(i)))
            .take_while(|(f, r)| f.is_some() && r.is_some())
            .map(|(f,r)| Position::new(f.unwrap(), r.unwrap()));

        let bottom_left = (1..)
            .map(|i| (pos.file.offset(-i), pos.rank.offset(-i)))
            .take_while(|(f, r)| f.is_some() && r.is_some())
            .map(|(f,r)| Position::new(f.unwrap(), r.unwrap()));

        top_right.chain(bottom_left)
    }
}

impl RegularMoveOptions for AntiDiagonalMovement {
    fn move_options(pos: &Position, side: Side) -> impl Iterator<Item = Position> + '_ {
        let top_left = (1..)
            .map(|i| (pos.file.offset(-i), pos.rank.offset(i)))
            .take_while(|(f, r)| f.is_some() && r.is_some())
            .map(|(f,r)| Position::new(f.unwrap(), r.unwrap()));

        let bottom_right = (1..)
            .map(|i| (pos.file.offset(i), pos.rank.offset(-i)))
            .take_while(|(f, r)| f.is_some() && r.is_some())
            .map(|(f,r)| Position::new(f.unwrap(), r.unwrap()));

        top_left.chain(bottom_right)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::position::{File, Rank};

    #[test]
    fn test_horizontal_movement() {
        let pos = Position::new(File::D, Rank::Four);
        let expected: Vec<_> = File::iter()
            .filter(|&f| f != File::D)
            .map(|f| Position::new(f, Rank::Four))
            .collect();

        let result: Vec<_> = HorizontalMovement::move_options(&pos, Side::White).collect();
        assert_eq!(result, expected, "Horizontal movement failed.");
    }

    #[test]
    fn test_vertical_movement() {
        let pos = Position::new(File::D, Rank::Four);
        let expected: Vec<_> = Rank::iter()
            .filter(|&r| r != Rank::Four)
            .map(|r| Position::new(File::D, r))
            .collect();

        let result: Vec<_> = VerticalMovement::move_options(&pos,Side::White).collect();
        assert_eq!(result, expected, "Vertical movement failed.");
    }

    #[test]
    fn test_diagonal_movement() {
        let pos = Position::new(File::D, Rank::Four);

        let expected: Vec<_> = vec![
            Position::new(File::E, Rank::Five),
            Position::new(File::F, Rank::Six),
            Position::new(File::G, Rank::Seven),
            Position::new(File::H, Rank::Eight),
            Position::new(File::C, Rank::Three),
            Position::new(File::B, Rank::Two),
            Position::new(File::A, Rank::One),
        ];

        let result: Vec<_> = DiagonalMovement::move_options(&pos,Side::White).collect();
        assert_eq!(result, expected, "Diagonal movement failed.");
    }

    #[test]
    fn test_anti_diagonal_movement() {
        let pos = Position::new(File::D, Rank::Four);

        let expected: Vec<_> = vec![
            Position::new(File::C, Rank::Five),
            Position::new(File::B, Rank::Six),
            Position::new(File::A, Rank::Seven),
            Position::new(File::E, Rank::Three),
            Position::new(File::F, Rank::Two),
            Position::new(File::G, Rank::One),
        ];

        let result: Vec<_> = AntiDiagonalMovement::move_options(&pos,Side::White).collect();
        assert_eq!(result, expected, "Anti-diagonal movement failed.");
    }
}


