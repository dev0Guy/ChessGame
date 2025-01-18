use std::ops::{Index, IndexMut};
use strum::IntoEnumIterator;
use crate::engine::board::location::{File, Location, Rank};
use crate::engine::board::pieces::{Piece};
use crate::engine::game::user_actions::MoveAction;

const FILE_NAMES: &'static str = "   A B C D E F G H";


/// A chessboard representation containing a 8x8 grid of optional pieces.
///
/// Each square on the board may either contain a `Piece` or be empty (`None`).
/// The board supports indexing by [`Location`] and provides methods for common operations.
#[derive(Debug)]
pub struct Board([[Option<Piece>; 8]; 8]);


impl Index<Location> for Board {
    type Output = Option<Piece>;

    /// Returns a reference to the piece (or `None`) at the given location.
    ///
    /// ## Parameters
    /// - `index`: The location on the board to access.
    ///
    /// ## Returns
    /// - A reference to the `Option<Piece>` at the specified location.
    fn index(&self, index: Location) -> &Self::Output {
        &self.0[7 - index.rank as usize][index.file as usize]
    }
}

impl IndexMut<Location> for Board {
    /// Returns a mutable reference to the piece (or `None`) at the given location.
    ///
    /// ## Parameters
    /// - `index`: The location on the board to access.
    ///
    /// ## Returns
    /// - A mutable reference to the `Option<Piece>` at the specified location.
    fn index_mut(&mut self, index: Location) -> &mut Self::Output {
        &mut self.0[7 - index.rank as usize][index.file as usize]
    }
}


impl Board {

    /// Creates a new, empty chessboard.
    /// Each square on the board is initialized to `None`.
    pub fn new() -> Self {
        Self([[None; 8]; 8])
    }

    /// Returns an iterator over the rows of the board.
    ///
    /// Each row is represented as an array of 8 `Option<Piece>` elements.
    pub fn iter(&self) -> std::slice::Iter<[Option<Piece>; 8]> {
        self.0.iter()
    }


    /// Executes a move action on the board.
    ///
    /// Moves a piece from the `from` location to the `to` location, clearing
    /// the `from` square after the move.
    ///
    /// ## Parameters
    /// - `move_action`: The move to be executed, containing `from` and `to` locations.
    pub fn action(&mut self, move_action: &MoveAction) {
        self[move_action.to] = self[move_action.from];
        self[move_action.from] = None;
    }

    pub fn to_bitboards(&self) -> [[u64; 6]; 2] {
        let mut bitboards = [[0u64; 6]; 2];

        for rank in Rank::iter() {
            for file in File::iter() {
                let file = file as usize;
                let rank = rank as usize;
                if let Some(piece) = self.0[rank][file] {
                    let bit_index = (rank) * 8 + (file);
                    let color_index = piece.side as usize;
                    let piece_index = piece.piece_type as usize;
                    bitboards[color_index][piece_index] |= 1 << bit_index;
                }
            }
        }

        bitboards
    }
}