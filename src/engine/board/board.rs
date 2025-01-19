use std::ops::{Index, IndexMut};
use strum::IntoEnumIterator;
use crate::engine::board::location::{File, Location, Rank};
use crate::engine::board::pieces::{Piece, PieceType, Side};
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

        for (rank, row) in self.0.iter().enumerate() {
            for (file, piece) in row.iter().enumerate() {
                if let Some(piece) = piece {
                    let bit_index = rank * 8 + file;
                    bitboards[piece.side as usize][piece.piece_type as usize] |= 1 << bit_index;
                }
            }
        }

        bitboards
    }

    pub fn from_bitboards(bitboards: [[u64; 6]; 2]) -> Self {
        let mut board = [[None; 8]; 8];

        for (side_index, pieces) in bitboards.iter().enumerate() {
            for (piece_type_index, &bitboard) in pieces.iter().enumerate() {
                let mut bitboard = bitboard;
                while bitboard != 0 {
                    let bit_index = bitboard.trailing_zeros() as usize;
                    let rank = bit_index / 8;
                    let file = bit_index % 8;

                    board[rank][file] = Some(
                        Piece::new(
                            PieceType::from_repr(piece_type_index).unwrap(),
                            Side::from_repr(side_index).unwrap()
                        )
                    );
                    // Clear the least significant bit
                    bitboard &= bitboard - 1;
                }
            }
        }

        Self(board)
    }
}