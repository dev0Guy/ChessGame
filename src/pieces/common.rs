use strum_macros::EnumIter;
use crate::bitboard::BitBoard;
use crate::square::Square;

/// Represents the color of a chess piece or player.
#[derive(Clone, Copy, Debug, EnumIter)]
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
pub(crate) trait PossibleMoves{
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
    fn get_moves(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard;

    /// Computes all possible capture moves for a piece located on the given square.
    ///
    /// # Parameters
    /// - `piece`: A [`BitBoard`] representing the single position of the piece.
    /// - `square`: The [`Square`] where the piece is located.
    /// - `own_pieces`: A [`BitBoard`] representing the positions of all friendly pieces.
    /// - `opponent_pieces`: A [`BitBoard`] representing the positions of all opponent pieces.
    /// - `color`: The [`Color`] of the piece (`Color::White` or `Color::Black`).
    ///
    /// # Returns
    /// A [`BitBoard`] representing all valid capture moves for the piece.
    fn get_capture(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard{
        Self::get_moves(piece, square, own_pieces, opponent_pieces, color)
    }

    /// Computes a `BitBoard` representing all occupied squares on the chessboard.
    #[inline]
    fn occupied(own_pieces: &BitBoard, opponent_pieces: &BitBoard) -> BitBoard {
        own_pieces | opponent_pieces
    }

    /// Computes a `BitBoard` representing all empty squares on the chessboard.
    #[inline]
    fn empty(own_pieces: &BitBoard, opponent_pieces: &BitBoard) -> BitBoard {
        !(Self::occupied(own_pieces, opponent_pieces))
    }

}


impl Color{
    pub fn opposite(&self) -> Color{
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}


impl From<Color> for usize{
    fn from(value: Color) -> Self {
        value as usize
    }
}