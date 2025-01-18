use std::ops::{Index, IndexMut};
use crate::engine::board::board::Board;
use crate::engine::board::location::Location;
use crate::engine::board::pieces::Piece;

/// Represents a chessboard where each cell indicates whether a specific side
/// can attack that location.
///
/// The `ThreadBoard` is used to track attack able squares for a given side.
/// Each cell is represented as a boolean:
/// - `true`: The square is under attack.
/// - `false`: The square is not under attack.
///
/// The board is internally stored as an 8x8 array of booleans.
#[derive(Debug, Clone)]
pub struct ThreadBoard([[bool; 8]; 8]);


impl Index<Location> for ThreadBoard {
    type Output = bool;

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

impl IndexMut<Location> for ThreadBoard {
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

impl ThreadBoard {
    /// Creates a new, empty chessboard.
    /// Each square on the board is initialized to `false`.
    pub fn new() -> Self {
        Self([[false; 8]; 8])
    }

    pub fn iter(&self) -> std::slice::Iter<[bool; 8]> { self.0.iter() }

    pub fn reset(&mut self) {
        self.0 = [[false; 8]; 8];
    }
}

