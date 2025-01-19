use strum::IntoEnumIterator;
use crate::game::{BoardBitSet, Position};
use crate::game::position::{File, Rank};

/// Precomputed masks for horizontal (rank) and vertical (file) moves
pub struct MoveMasks {
    horizontal: [[BoardBitSet; 8]; 8],
    vertical: [[BoardBitSet; 8]; 8],
}


impl MoveMasks {
    /// Generates precomputed move masks for each square on the board.
    ///
    /// For each square, a horizontal mask includes all other squares on the same rank (row),
    /// and a vertical mask includes all other squares on the same file (column).
    ///
    /// # Returns
    /// A `MoveMasks` instance containing precomputed masks for horizontal and vertical moves.
    pub fn generate() -> Self {
        let mut horizontal = [[BoardBitSet::empty(); 8]; 8];
        let mut vertical = [[BoardBitSet::empty(); 8]; 8];

        for rank in Rank::iter() {
            for file in File::iter() {
                let pos = Position::new(file, rank);

                let mut horizontal_mask = BoardBitSet::empty();
                for f in File::iter() {
                    if f != file {
                        horizontal_mask |= BoardBitSet::place_at(Position::new(f, rank));
                    }
                }

                let mut vertical_mask = BoardBitSet::empty();
                for r in Rank::iter() {
                    if r != rank {
                        vertical_mask |= BoardBitSet::place_at(Position::new(file, r));
                    }
                }

                horizontal[rank as usize][file as usize] = horizontal_mask;
                vertical[rank as usize][file as usize] = vertical_mask;
            }
        }

        Self { horizontal, vertical }
    }

    /// Retrieves the precomputed horizontal mask for the given position.
    ///
    /// The horizontal mask includes all squares on the same rank (row) as the given position,
    /// excluding the square itself.
    ///
    /// # Parameters
    /// - `pos`: The position on the board for which the horizontal mask is requested.
    ///
    /// # Returns
    /// A `BoardBitSet` representing all valid horizontal moves from the given position.
    pub fn horizontal(&self, pos: Position) -> &BoardBitSet {
        &self.horizontal[usize::from(pos.rank)][usize::from(pos.file)]
    }

    /// Retrieves the precomputed vertical mask for the given position.
    ///
    /// The vertical mask includes all squares on the same file (column) as the given position,
    /// excluding the square itself.
    ///
    /// # Parameters
    /// - `pos`: The position on the board for which the vertical mask is requested.
    ///
    /// # Returns
    /// A `BoardBitSet` representing all valid vertical moves from the given position.
    pub fn vertical(&self, pos: Position) -> &BoardBitSet {
        &self.vertical[usize::from(pos.rank)][usize::from(pos.file)]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::position::{File, Rank, Position};

    #[test]
    fn test_horizontal_mask() {
        let masks = MoveMasks::generate();
        let pos = Position::new(File::D, Rank::Four);
        let mask = masks.horizontal(pos);

        assert!(mask.is_set(Position::new(File::A, Rank::Four)));
        assert!(mask.is_set(Position::new(File::H, Rank::Four)));
        assert!(mask.is_set(Position::new(File::C, Rank::Four)));
        assert!(!mask.is_set(pos));
    }

    #[test]
    fn test_vertical_mask() {
        let masks = MoveMasks::generate();
        let pos = Position::new(File::D, Rank::Four);
        let mask = masks.vertical(pos);

        assert!(mask.is_set(Position::new(File::D, Rank::One)));
        assert!(mask.is_set(Position::new(File::D, Rank::Eight)));
        assert!(mask.is_set(Position::new(File::D, Rank::Three)));
        assert!(!mask.is_set(pos));
    }
}