use crate::engine::board::board;
use crate::engine::board::location::Location;
use crate::engine::board::pieces::Side;
use crate::engine::move_generator::base::{MoveGenerator, PieceMovementType};

const ROCK_POSSIBLE_DIRECTIONS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];


/// A move generator for the rook piece in chess.
///
/// Rooks can move any number of squares horizontally or vertically until they encounter an obstacle
/// (another piece or the edge of the board). They can capture opponent pieces but cannot move
/// through other pieces.
///
/// This implementation leverages sliding move logic for horizontal and vertical directions.
pub(crate) struct RookMoveGen;

impl MoveGenerator for RookMoveGen {
    /// Generates all valid moves for a rook based on its current location, side, and the board state.
    ///
    /// ## Parameters
    /// - `board`: A reference to the current game board.
    /// - `loc`: The current location of the rook.
    /// - `side`: The side of the rook (`Side::White` or `Side::Black`).
    ///
    /// ## Returns
    /// - A list of valid moves for the rook, represented as `PieceMovementType`.
    fn generate_moves(board: &board::Board, loc: Location, side: Side) -> Vec<PieceMovementType> {
        Self::generate_sliding_moves(board, loc, side, &ROCK_POSSIBLE_DIRECTIONS)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::board::board::Board;
    use crate::engine::board::location::{Location, File, Rank};
    use crate::engine::board::pieces::{Piece, PieceType, Side};
    use crate::engine::move_generator::base::PieceMovementType;

    #[test]
    fn test_rook_moves_empty_board() {
        let mut board = Board::new();
        let loc = Location::new(File::D, Rank::Four);
        board[loc] = Some(Piece::new(PieceType::Rook, Side::White));

        let moves = RookMoveGen::generate_moves(&board, loc, Side::White);

        let vertical_movements = [
            Location::new(File::D, Rank::One),
            Location::new(File::D, Rank::Two),
            Location::new(File::D, Rank::Three),
            Location::new(File::D, Rank::Five),
            Location::new(File::D, Rank::Six),
            Location::new(File::D, Rank::Seven),
            Location::new(File::D, Rank::Eight),
        ];
        assert!(vertical_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));

        let horizontal_movements = [
            Location::new(File::A, Rank::Four),
            Location::new(File::B, Rank::Four),
            Location::new(File::C, Rank::Four),
            Location::new(File::E, Rank::Four),
            Location::new(File::F, Rank::Four),
            Location::new(File::G, Rank::Four),
            Location::new(File::H, Rank::Four),
        ];
        assert!(horizontal_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));

        assert_eq!(moves.len(), vertical_movements.len() + horizontal_movements.len());
    }

    #[test]
    fn test_rook_moves_with_same_color_obstacles() {
        let mut board = Board::new();
        let loc = Location::new(File::D, Rank::Four);
        board[loc] = Some(Piece::new(PieceType::Rook, Side::White));

        board[Location::new(File::D, Rank::Six)] = Some(Piece::new(PieceType::Pawn, Side::White));
        board[Location::new(File::B, Rank::Four)] = Some(Piece::new(PieceType::Pawn, Side::White));

        let moves = RookMoveGen::generate_moves(&board, loc, Side::White);

        let vertical_movements = [
            Location::new(File::D, Rank::One),
            Location::new(File::D, Rank::Two),
            Location::new(File::D, Rank::Three),
            Location::new(File::D, Rank::Five),
        ];
        assert!(vertical_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert!(!moves.contains(&PieceMovementType::Relocate(Location::new(File::D, Rank::Six))));

        let horizontal_movements = [
            Location::new(File::C, Rank::Four),
        ];
        assert!(horizontal_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert!(!moves.contains(&PieceMovementType::Relocate(Location::new(File::B, Rank::Four))));
    }

    #[test]
    fn test_rook_moves_with_opposite_color_obstacles() {
        let mut board = Board::new();
        let loc = Location::new(File::D, Rank::Four);
        board[loc] = Some(Piece::new(PieceType::Rook, Side::White));

        board[Location::new(File::D, Rank::Six)] = Some(Piece::new(PieceType::Pawn, Side::Black));
        board[Location::new(File::B, Rank::Four)] = Some(Piece::new(PieceType::Pawn, Side::Black));

        let moves = RookMoveGen::generate_moves(&board, loc, Side::White);

        let vertical_movements = [
            Location::new(File::D, Rank::One),
            Location::new(File::D, Rank::Two),
            Location::new(File::D, Rank::Three),
            Location::new(File::D, Rank::Five),
        ];
        assert!(vertical_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert!(moves.contains(&PieceMovementType::Capture(Location::new(File::D, Rank::Six))));
        assert!(!moves.contains(&PieceMovementType::Relocate(Location::new(File::D, Rank::Seven))));

        let horizontal_movements = [
            Location::new(File::C, Rank::Four),
        ];
        assert!(horizontal_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert!(moves.contains(&PieceMovementType::Capture(Location::new(File::B, Rank::Four))));
        assert!(!moves.contains(&PieceMovementType::Relocate(Location::new(File::A, Rank::Four))));
    }
}