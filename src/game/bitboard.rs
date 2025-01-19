use super::bitset::BoardBitSet;
use crate::engine::board::pieces::{Piece, PieceType, Side};

/// A representation of the chessboard.
pub struct Board {
    /// One bitboard for each piece kind
    piece_sets: [BoardBitSet; 6],
    /// One bitboard for each color
    color_sets: [BoardBitSet; 2],
    /// A bitboard representing all occupied squares
    occupied_set: BoardBitSet,
    /// active side
    active_color: Side,
}


impl Board {
    /// Creates a new chessboard with default values.
    pub fn new() -> Self {
        Self {
            piece_sets: Default::default(),
            color_sets: Default::default(),
            occupied_set: BoardBitSet::empty(),
            active_color: Side::White,
        }
    }
}

impl From<Side> for usize {
    fn from(color: Side) -> Self {
        match color {
            Side::White => 0,
            Side::Black => 1,
        }
    }
}

impl From<PieceType> for usize {
    fn from(piece: PieceType) -> Self {
        match piece {
            PieceType::King => 0,
            PieceType::Queen => 1,
            PieceType::Rook => 2,
            PieceType::Bishop => 3,
            PieceType::Knight => 4,
            PieceType::Pawn => 5,
        }
    }
}