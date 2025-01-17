use crate::engine::board::board;
use crate::engine::board::location::Location;
use crate::engine::board::pieces::Side;
use crate::engine::move_generator::base::{MoveGenerator, PieceMovementType};

const BISHOP_POSSIBLE_DIRECTIONS: [(i8, i8); 4] = [(1, 1), (-1, -1), (1, -1), (-1, 1)];

/// A move generator for the bishop piece in chess.
///
/// Bishops can move diagonally any number of squares until they encounter an obstacle
/// (another piece or the edge of the board). They can capture opponent pieces but cannot move
/// through other pieces.
///
/// This implementation leverages sliding move logic for diagonal directions.
pub(crate) struct BishopMoveGen;

impl MoveGenerator for BishopMoveGen {
    /// Generates all valid moves for a bishop based on its current location, side, and the board state.
    ///
    /// ## Parameters
    /// - `board`: A reference to the current game board.
    /// - `loc`: The current location of the bishop.
    /// - `side`: The side of the bishop (`Side::White` or `Side::Black`).
    ///
    /// ## Returns
    /// - A list of valid moves for the bishop, represented as `PieceMovementType`.
    fn generate_moves(board: &board::Board, loc: Location, side: Side) -> Vec<PieceMovementType> {
        Self::generate_sliding_moves(board, loc, side, &BISHOP_POSSIBLE_DIRECTIONS)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::board::board::Board;
    use crate::engine::board::location::{File, Location, Rank};
    use crate::engine::board::pieces::{Piece, PieceType, Side};
    use crate::engine::move_generator::base::PieceMovementType;

    #[test]
    fn test_bishop_moves_empty_board() {
        let mut board = Board::new();
        let loc = Location::new(File::C, Rank::Three);
        board[loc] = Some(Piece::new(PieceType::Bishop, Side::White));

        let moves = BishopMoveGen::generate_moves(&board, loc, Side::White);

        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::A, Rank::One))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::B, Rank::Two))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::D, Rank::Four))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Five))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::F, Rank::Six))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::G, Rank::Seven))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::H, Rank::Eight))));

        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::A, Rank::Five))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::B, Rank::Four))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::D, Rank::Two))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::One))));
        assert_eq!(moves.len(), 11);
    }

    #[test]
    fn test_bishop_at_edge_of_board() {
        let mut board = Board::new();
        let loc = Location::new(File::A, Rank::One);
        board[loc] = Some(Piece::new(PieceType::Bishop, Side::White));

        let moves = BishopMoveGen::generate_moves(&board, loc, Side::White);

        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::B, Rank::Two))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::C, Rank::Three))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::D, Rank::Four))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Five))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::F, Rank::Six))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::G, Rank::Seven))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::H, Rank::Eight))));
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_bishop_moves_with_obstacles_no_capture() {
        let mut board = Board::new();
        let loc = Location::new(File::C, Rank::Three);
        board[loc] = Some(Piece::new(PieceType::Bishop, Side::White));

        board[Location::new(File::B, Rank::Two)] = Some(Piece::new(PieceType::Pawn, Side::White));

        let moves = BishopMoveGen::generate_moves(&board, loc, Side::White);

        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::D, Rank::Four))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Five))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::F, Rank::Six))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::G, Rank::Seven))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::H, Rank::Eight))));

        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::A, Rank::Five))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::B, Rank::Four))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::D, Rank::Two))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::One))));
        assert_eq!(moves.len(), 9);
    }

    #[test]
    fn test_bishop_moves_with_obstacles_with_capture() {
        let mut board = Board::new();
        let loc = Location::new(File::C, Rank::Three);
        board[loc] = Some(Piece::new(PieceType::Bishop, Side::White));

        board[Location::new(File::B, Rank::Two)] = Some(Piece::new(PieceType::Pawn, Side::Black));

        let moves = BishopMoveGen::generate_moves(&board, loc, Side::White);

        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::D, Rank::Four))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Five))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::F, Rank::Six))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::G, Rank::Seven))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::H, Rank::Eight))));

        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::A, Rank::Five))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::B, Rank::Four))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::D, Rank::Two))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::One))));
        assert!(moves.contains(&PieceMovementType::Capture(Location::new(File::B, Rank::Two))));

        assert_eq!(moves.len(), 10);
    }

    #[test]
    fn test_bishop_closed_by_all_sides() {
        let mut board = Board::new();
        let loc = Location::new(File::B, Rank::Two);
        board[loc] = Some(Piece::new(PieceType::Bishop, Side::White));
        // Add blocking pieces
        board[Location::new(File::A, Rank::One)] = Some(Piece::new(PieceType::Pawn, Side::White));
        board[Location::new(File::A, Rank::Three)] = Some(Piece::new(PieceType::Pawn, Side::White));
        board[Location::new(File::C, Rank::Three)] = Some(Piece::new(PieceType::Pawn, Side::White));
        board[Location::new(File::C, Rank::One)] = Some(Piece::new(PieceType::Pawn, Side::White));

        let moves = BishopMoveGen::generate_moves(&board, loc, Side::White);

        assert_eq!(moves.len(), 0);
    }

}