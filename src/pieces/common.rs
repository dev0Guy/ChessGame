use crate::bitboard::BitBoard;

/// Represents the color of a chess piece or player.
pub enum Color {
    White,
    Black,
}

/// A trait for calculating possible moves for a specific piece type in chess.
///
/// This trait defines a method for determining the valid moves for a given piece type, considering
/// the positions of friendly and opponent pieces on the chessboard. The method operates on bitboards
/// representing the current state of the game.
///
/// # Purpose
/// The trait allows different chess pieces to implement their specific move generation logic, taking into
/// account movement rules, captures, and restrictions (like friendly piece blocking).
pub trait PossibleMoves{
    /// Calculates the possible moves for a piece type given the current board state.
    ///
    /// # Parameters
    /// - `piece`: A `BitBoard` representing the location of the piece being evaluated. Only the bits where this
    ///   specific piece exists will be set.
    /// - `own_pieces`: A `BitBoard` representing the locations of all friendly pieces on the board.
    /// - `opponent_pieces`: A `BitBoard` representing the locations of all opponent pieces on the board.
    /// - `color`: The `Color` of the piece being evaluated (`White` or `Black`).
    ///
    /// # Returns
    /// - A `BitBoard` where each set bit represents a square the piece can legally move to.
    /// - The method ensures that:
    ///   - Moves that would place the piece on a square occupied by a friendly piece are excluded.
    ///   - Captures (landing on an opponent piece's square) are included.
    ///   - For pieces with complex movement (e.g., sliding pieces), valid moves account for blockers.
    fn get_moves(piece: &BitBoard,own_pieces: &BitBoard, opponent_pics: &BitBoard, color: Color) -> BitBoard;


}

impl Color {
    /// Returns the opposite color.
    pub fn opposite(&self) -> Color{
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}