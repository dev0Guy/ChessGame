#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side{
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType{
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

/// Represents a chess piece.
///
/// A `Piece` consists of two main components:
/// - The type of the piece, represented by [`PieceType`].
/// - The side the piece belongs to, represented by [`Side`].
#[derive(Debug, Clone, Copy)]
pub struct Piece{
    /// The type of the chess piece (e.g., King, Queen, Rook, etc.).
    pub piece_type: PieceType,
    /// The side the chess piece belongs to (e.g., White or Black).
    pub side: Side,
}

impl Piece {
    /// Creates a new `Piece` with the specified type and 'Side'.
    pub(crate) const fn new(piece_type: PieceType, side: Side) -> Self {
        Self{piece_type, side}
    }
}