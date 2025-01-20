use crate::bitboard;
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
    /// - `own_pieces`: A `BitBoard` representing the locations of the player's own pieces of a given type.
    /// - `opponent_pieces`: A `BitBoard` representing the locations of the opponent's pieces.
    ///
    /// # Returns
    /// - A `BitBoard` where each set bit represents a square the piece can legally move to.
    /// - The method ensures that:
    ///   - Moves that would place the piece on a square occupied by a friendly piece are excluded.
    ///   - Captures (landing on an opponent piece's square) are included.
    ///   - For pieces with complex movement (e.g., sliding pieces), valid moves account for blockers.
    fn get_moves(own_pieces: &bitboard::BitBoard, opponent_pics: &bitboard::BitBoard) -> bitboard::BitBoard;
}