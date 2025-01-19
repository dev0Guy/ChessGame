use strum::IntoEnumIterator;
use crate::game::{BoardBitSet, Position};
use crate::game::position::{File, Rank};

/// Precomputed masks for horizontal (rank) and vertical (file) moves
pub struct SlideMoveMasks {
    /// Horizontal (-)
    horizontal: [[BoardBitSet; 8]; 8],
    /// Vertical (|)
    vertical: [[BoardBitSet; 8]; 8],
    /// Diagonal (/)
    diagonal: [[BoardBitSet; 8]; 8],
    /// Anti-diagonal (\)
    anti_diagonal: [[BoardBitSet; 8]; 8],
}


impl SlideMoveMasks {

    fn get_horizontal_ray_from(pos: &Position) -> impl Iterator<Item = Position>  + '_ {
        File::iter()
            .filter(|f| f != &pos.file)
            .map(|f| Position::new(f, pos.rank))
            .into_iter()
    }

    fn get_vertical_ray_from(pos: &Position) -> impl Iterator<Item = Position> + '_  {
        Rank::iter()
            .filter(|r| r != &pos.rank)
            .map(|r| Position::new(pos.file, r))
            .into_iter()
    }

    fn get_diagonal_ray_from(pos: &Position) -> impl Iterator<Item = Position> + '_ {
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

    fn get_anti_diagonal_ray_from(pos: &Position) -> impl Iterator<Item = Position> + '_ {
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
        let mut diagonal = [[BoardBitSet::empty(); 8]; 8];
        let mut anti_diagonal = [[BoardBitSet::empty(); 8]; 8];

        for rank in Rank::iter() {
            for file in File::iter() {
                let pos = Position::new(file, rank);
                let horizontal_mask = BoardBitSet::place_multiple_at(Self::get_horizontal_ray_from(&pos));
                let vertical_mask = BoardBitSet::place_multiple_at(Self::get_vertical_ray_from(&pos));
                let diagonal_mask = BoardBitSet::place_multiple_at(Self::get_diagonal_ray_from(&pos));
                let anti_diagonal_mask = BoardBitSet::place_multiple_at(Self::get_anti_diagonal_ray_from(&pos));

                horizontal[rank as usize][file as usize] = horizontal_mask;
                vertical[rank as usize][file as usize] = vertical_mask;
                diagonal[rank as usize][file as usize] = diagonal_mask;
                anti_diagonal[rank as usize][file as usize] = anti_diagonal_mask;
            }
        }

        Self { horizontal, vertical, diagonal, anti_diagonal }
    }

    /// Retrieves the precomputed horizontal mask for the given position.
    pub fn horizontal(&self, pos: Position) -> &BoardBitSet {
        &self.horizontal[usize::from(pos.rank)][usize::from(pos.file)]
    }

    /// Retrieves the precomputed vertical mask for the given position.
    pub fn vertical(&self, pos: Position) -> &BoardBitSet {
        &self.vertical[usize::from(pos.rank)][usize::from(pos.file)]
    }

    /// Retrieves the precomputed diagonal mask for the given position
    pub fn diagonal(&self, pos: Position) -> &BoardBitSet {
        &self.diagonal[usize::from(pos.rank)][usize::from(pos.file)]
    }

    /// Retrieves the precomputed anti-diagonal mask for the given position
    pub fn anti_diagonal(&self, pos: Position) -> &BoardBitSet {
        &self.anti_diagonal[usize::from(pos.rank)][usize::from(pos.file)]
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::position::{File, Rank, Position};

    #[test]
    fn test_horizontal_mask() {
        let masks = SlideMoveMasks::generate();
        let pos = Position::new(File::D, Rank::Four);
        let mask = masks.horizontal(pos);

        assert!(mask.is_set(Position::new(File::A, Rank::Four)));
        assert!(mask.is_set(Position::new(File::H, Rank::Four)));
        assert!(mask.is_set(Position::new(File::C, Rank::Four)));
        assert!(!mask.is_set(pos));
    }

    #[test]
    fn test_vertical_mask() {
        let masks = SlideMoveMasks::generate();
        let pos = Position::new(File::D, Rank::Four);
        let mask = masks.vertical(pos);

        assert!(mask.is_set(Position::new(File::D, Rank::One)));
        assert!(mask.is_set(Position::new(File::D, Rank::Eight)));
        assert!(mask.is_set(Position::new(File::D, Rank::Three)));
        assert!(!mask.is_set(pos));
    }

    // pos: https://lichess.org/editor/8/8/8/8/3B4/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_diagonal_mask() {
        let masks = SlideMoveMasks::generate();
        let pos = Position::new(File::D, Rank::Four);
        let mask = masks.diagonal(pos);
        assert!(mask.is_set(Position::new(File::A, Rank::One)));
        assert!(mask.is_set(Position::new(File::B, Rank::Two)));
        assert!(mask.is_set(Position::new(File::C, Rank::Three)));
        assert!(mask.is_set(Position::new(File::E, Rank::Five)));
        assert!(mask.is_set(Position::new(File::F, Rank::Six)));
        assert!(mask.is_set(Position::new(File::G, Rank::Seven)));
        assert!(mask.is_set(Position::new(File::H, Rank::Eight)));
        assert!(!mask.is_set(pos));
    }

    // pos: https://lichess.org/editor/8/8/8/8/3B4/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_anti_diagonal_mask() {
        let masks = SlideMoveMasks::generate();
        let pos = Position::new(File::D, Rank::Four);
        let mask = masks.anti_diagonal(pos);
        assert!(mask.is_set(Position::new(File::C, Rank::Five)));
        assert!(mask.is_set(Position::new(File::E, Rank::Three)));
        assert!(mask.is_set(Position::new(File::B, Rank::Six)));
        assert!(mask.is_set(Position::new(File::F, Rank::Two)));
        assert!(!mask.is_set(pos));
    }
}