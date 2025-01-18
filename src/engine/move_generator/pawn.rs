use crate::engine::board::board;
use crate::engine::board::location::{Location, Rank};
use crate::engine::board::pieces::{Side};
use crate::engine::move_generator::base::{MoveGenerator, PieceMovementType};

/// A move generator for pawn pieces in chess.
/// This struct provides functionality to generate all valid moves for a pawn,
/// including single steps, double steps (from the starting rank), and diagonal captures.
/// It ensures that moves adhere to the rules of chess, such as blocked paths and valid captures.
pub(crate) struct PawnMoveGen;


impl PawnMoveGen {
    /// Returns the starting rank for pawns based on their side.
    ///
    /// ## Parameters
    /// - `side`: The side of the pawn (`Side::White` or `Side::Black`).
    ///
    /// ## Returns
    /// - The starting rank for the given side.
    fn get_start_rank(side: Side) -> Rank {
        match side {
            Side::White => Rank::Two,
            Side::Black => Rank::Seven,
        }
    }

    fn get_promotion_rank(side: Side) -> Rank {
        match side {
            Side::White => Rank::Eight,
            Side::Black => Rank::One
        }
    }

    /// Returns the move direction for pawns based on their side.
    ///
    /// ## Parameters
    /// - `side`: The side of the pawn (`Side::White` or `Side::Black`).
    ///
    /// ## Returns
    /// - `1` for white pawns (moving upward) and `-1` for black pawns (moving downward).
    fn get_move_direction(side: Side) -> i8 {
        match side {
            Side::White => 1,
            Side::Black => -1,
        }
    }

    /// Checks if a move to the specified location is valid for relocation.
    ///
    /// This function verifies whether the given target location is valid
    /// (i.e., within the bounds of the board and unoccupied). If the location is valid,
    /// it returns the location; otherwise, it returns `None`.
    ///
    /// ## Parameters
    /// - `board`: A reference to the current game board.
    /// - `loc`: A `Result` containing the target location (`Ok(Location)`) or an error message (`Err(String)`).
    ///
    /// ## Returns
    /// - `Some(Location)`: The valid target location if the move is within bounds and the square is unoccupied.
    /// - `None`: If the location is out of bounds or the square is occupied.
    fn check_valid_move(
        board: &board::Board,
        loc: Result<Location, String>,
    ) -> Option<Location>{
        match loc {
            Ok(valid_loc) if board[valid_loc].is_none() => Some(valid_loc),
            _ => None
        }
    }


    /// Checks if a move to the specified location is valid for capturing an enemy piece.
    ///
    /// This function verifies whether the given target location is valid
    /// (i.e., within the bounds of the board and occupied by an opponent's piece). If the location is valid for capture,
    /// it returns the location; otherwise, it returns `None`.
    ///
    /// ## Parameters
    /// - `board`: A reference to the current game board.
    /// - `loc`: A `Result` containing the target location (`Ok(Location)`) or an error message (`Err(String)`).
    /// - `side`: The side of the current piece (`Side::White` or `Side::Black`).
    ///
    /// ## Returns
    /// - `Some(Location)`: The valid target location if the move is within bounds and the square is occupied by an opponent's piece.
    /// - `None`: If the location is out of bounds or the square is unoccupied or occupied by a friendly piece.
    fn check_valid_capture(
        board: &board::Board,
        loc: Result<Location, String>,
        side: Side,
    )-> Option<Location> {
        match loc {
            Ok(valid_loc) => {
                match board[valid_loc] {
                    Some(piece) if piece.side != side => return Some(valid_loc),
                    _ => { None},
                }
            }
            _ => None
        }
    }
}


impl MoveGenerator for PawnMoveGen {

    /// Generates all valid moves for a pawn based on its current location, side, and the board state.
    ///
    /// ## Parameters
    /// - `board`: The current state of the game board.
    /// - `loc`: The current location of the pawn.
    /// - `side`: The side of the pawn (`Side::White` or `Side::Black`).
    ///
    /// ## Returns
    /// - A list of valid moves for the pawn, represented as `PieceMovementType`.
    ///
    /// ## Move Types
    /// - **Single step**: Moves one square forward if the square is empty.
    /// - **Double step**: Moves two squares forward if both the single-step and double-step squares are empty, and the pawn is in its starting rank.
    /// - **Capture**: Captures an enemy piece diagonally if one exists.
    fn generate_moves(
        board: &board::Board,
        loc: Location,
        side: Side,
    ) -> Vec<PieceMovementType> {
        let mut moves = Vec::new();
        let direction = Self::get_move_direction(side);
        let promotion_rank = Self::get_promotion_rank(side);
        let is_in_start_rank = loc.rank == Self::get_start_rank(side);
        let single_step_loc = loc.offset(direction, 0);
        let double_step_loc = loc.offset(2 * direction, 0);

        if let Some(single_loc) = Self::check_valid_move(board, single_step_loc){
            moves.push(PieceMovementType::Relocate(single_loc));
            if is_in_start_rank && board[single_loc].is_none(){
                if let Some(double_loc) = Self::check_valid_move(board, double_step_loc){
                    moves.push(PieceMovementType::Relocate(double_loc));
                }
            }
        }

        for &dx in &[-1, 1] {
            let capture_loc = loc.offset(direction, dx);
            if let Some(capture_loc) = Self::check_valid_capture(board, capture_loc, side) {
                moves.push(PieceMovementType::Capture(capture_loc));
            }
        }

        moves.iter_mut().for_each(|mv| {
            if let PieceMovementType::Relocate(target) | PieceMovementType::Capture(target) = mv {
                if target.rank == promotion_rank {
                    *mv = PieceMovementType::Promotion(*target);
                }
            }
        });

        moves
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
    fn test_pawn_moves_white_start() {
        let mut board = Board::new();
        let loc = Location::new(File::E, Rank::Two);
        let pawn = Piece::new(PieceType::Pawn, Side::White);
        board[loc] = Some(pawn);

        let moves = PawnMoveGen::generate_moves(&board, loc, Side::White);

        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Three))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Four))));
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_pawn_moves_black_start() {
        let mut board = Board::new();
        let loc = Location::new(File::E, Rank::Seven);
        let pawn = Piece::new(PieceType::Pawn, Side::Black);
        board[loc] = Some(pawn);

        let moves = PawnMoveGen::generate_moves(&board, loc, Side::Black);

        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Six))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Five))));
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_pawn_blocked_moves() {
        let mut board = Board::new();
        let loc = Location::new(File::E, Rank::Two);
        let pawn = Piece::new(PieceType::Pawn, Side::White);
        board[loc] = Some(pawn);

        let blocking_loc = Location::new(File::E, Rank::Three);
        board[blocking_loc] = Some(Piece::new(PieceType::Pawn, Side::Black));

        let moves = PawnMoveGen::generate_moves(&board, loc, Side::White);

        assert!(moves.is_empty());
    }

    #[test]
    fn test_pawn_capture_moves() {
        let mut board = Board::new();
        let loc = Location::new(File::E, Rank::Two);
        let pawn = Piece::new(PieceType::Pawn, Side::White);
        board[loc] = Some(pawn);

        let left_capture_loc = Location::new(File::D, Rank::Three);
        let right_capture_loc = Location::new(File::F, Rank::Three);
        board[left_capture_loc] = Some(Piece::new(PieceType::Pawn, Side::Black));
        board[right_capture_loc] = Some(Piece::new(PieceType::Pawn, Side::Black));

        let moves = PawnMoveGen::generate_moves(&board, loc, Side::White);

        assert!(moves.contains(&PieceMovementType::Capture(left_capture_loc)));
        assert!(moves.contains(&PieceMovementType::Capture(right_capture_loc)));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Three))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Four))));
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn test_pawn_no_capture_same_side() {
        let mut board = Board::new();
        let loc = Location::new(File::E, Rank::Two);
        let pawn = Piece::new(PieceType::Pawn, Side::White);
        board[loc] = Some(pawn);

        let left_block_loc = Location::new(File::D, Rank::Three);
        let right_block_loc = Location::new(File::F, Rank::Three);
        board[left_block_loc] = Some(Piece::new(PieceType::Pawn, Side::White));
        board[right_block_loc] = Some(Piece::new(PieceType::Pawn, Side::White));

        let moves = PawnMoveGen::generate_moves(&board, loc, Side::White);

        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Three))));
        assert!(moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Four))));
        assert!(!moves.contains(&PieceMovementType::Capture(left_block_loc)));
        assert!(!moves.contains(&PieceMovementType::Capture(right_block_loc)));
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_pawn_promotion_white() {
        let mut board = Board::new();
        let loc = Location::new(File::E, Rank::Seven);
        let pawn = Piece::new(PieceType::Pawn, Side::White);
        board[loc] = Some(pawn);

        let moves = PawnMoveGen::generate_moves(&board, loc, Side::White);

        assert!(moves.contains(&PieceMovementType::Promotion(Location::new(File::E, Rank::Eight))));
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn test_pawn_promotion_black() {
        let mut board = Board::new();
        let loc = Location::new(File::E, Rank::Two);
        let pawn = Piece::new(PieceType::Pawn, Side::Black);
        board[loc] = Some(pawn);

        let moves = PawnMoveGen::generate_moves(&board, loc, Side::Black);

        assert!(moves.contains(&PieceMovementType::Promotion(Location::new(File::E, Rank::One))));
        assert_eq!(moves.len(), 1);
    }

    #[test]
    fn test_pawn_promotion_with_capture() {
        let mut board = Board::new();
        let loc = Location::new(File::E, Rank::Seven);
        let pawn = Piece::new(PieceType::Pawn, Side::White);
        board[loc] = Some(pawn);

        let left_capture_loc = Location::new(File::D, Rank::Eight);
        let right_capture_loc = Location::new(File::F, Rank::Eight);
        board[left_capture_loc] = Some(Piece::new(PieceType::Pawn, Side::Black));
        board[right_capture_loc] = Some(Piece::new(PieceType::Pawn, Side::Black));

        let moves = PawnMoveGen::generate_moves(&board, loc, Side::White);

        assert!(moves.contains(&PieceMovementType::Promotion(Location::new(File::E, Rank::Eight))));
        assert!(moves.contains(&PieceMovementType::Promotion(left_capture_loc)));
        assert!(moves.contains(&PieceMovementType::Promotion(right_capture_loc)));
        assert_eq!(moves.len(), 3);
    }
}