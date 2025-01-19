use std::ops;
use crate::game::position::Position;

/// Representation of a bitboard for efficient chessboard operations.
#[derive(Clone, Copy, Debug, Default)]
pub struct BoardBitSet(u64);

impl BoardBitSet{

    // Return empty `BoardBitSet` all zero
    pub fn empty() -> Self{
        Self(0)
    }

    /// Return new `BoardBitSet`
    pub fn new(v: u64) -> Self{
        Self(v)
    }

    /// Create a `BoardBitSet` with `pos` set to 1.
    pub fn place_at(pos: Position) -> Self{
        Self::new(1 << Position::position_bitboard_index(&pos))
    }

    pub fn place_multiple_at<T: IntoIterator<Item = Position>>(&mut self, positions: T) {
        positions.into_iter().for_each(|pos| {
            self.0 |= 1 << Position::position_bitboard_index(&pos);
        });
    }

    /// Returns `true` if the bit at `pos` is 1, `false` otherwise.
    pub fn is_set(&self, pos: Position) -> bool {
        let index = Position::position_bitboard_index(&pos);
        (self.0 & (1 << index)) != 0
    }
}

impl ops::BitOr for BoardBitSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl ops::BitOrAssign for BoardBitSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl ops::BitAnd for BoardBitSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl ops::BitAndAssign for BoardBitSet {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ops::BitXor for BoardBitSet {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl ops::BitXorAssign for BoardBitSet {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::position::{File, Rank, Position};

    #[test]
    fn test_empty_board() {
        let bitset = BoardBitSet::empty();
        assert_eq!(bitset.0, 0);
    }

    #[test]
    fn test_place_at() {
        let pos = Position::new(File::C, Rank::Three);
        let bitset = BoardBitSet::place_at(pos);

        assert_eq!(bitset.0, 1 << 18);
        assert!(bitset.is_set(pos));
    }

    #[test]
    fn test_is_set() {
        let pos = Position::new(File::D, Rank::Four);
        let mut bitset = BoardBitSet::empty();

        assert!(!bitset.is_set(pos));

        bitset = BoardBitSet::place_at(pos);

        assert!(bitset.is_set(pos));
    }

    #[test]
    fn test_place_multiple_at() {
        let positions = vec![
            Position::new(File::A, Rank::One),
            Position::new(File::B, Rank::Two),
            Position::new(File::C, Rank::Three),
        ];

        let mut bitset = BoardBitSet::empty();
        bitset.place_multiple_at(positions.clone());

        for pos in positions {
            assert!(bitset.is_set(pos));
        }

        let unset_pos = Position::new(File::D, Rank::Four);
        assert!(!bitset.is_set(unset_pos));
    }

    #[test]
    fn test_place_multiple_at_bitmask() {
        let positions = vec![
            Position::new(File::A, Rank::One),
            Position::new(File::H, Rank::Eight),
        ];

        let mut bitset = BoardBitSet::empty();
        bitset.place_multiple_at(positions.clone());

        let expected_bitmask = (1 << 0) | (1 << 63);
        assert_eq!(bitset.0, expected_bitmask);
    }
}