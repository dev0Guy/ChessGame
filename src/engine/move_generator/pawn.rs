use crate::engine::board::board;
use crate::engine::board::location::{Location, Rank};
use crate::engine::board::pieces::Side;
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

    /// Adds a valid relocation move to the list if the target location is empty.
    ///
    /// ## Parameters
    /// - `board`: The current game board.
    /// - `loc`: The target location as a `Result` (contains the location or an error message).
    /// - `moves`: The list of moves to which the valid move will be added.
    ///
    /// ## Notes
    /// - The move is only added if the location is valid and unoccupied.
    fn add_move_if_valid(
        board: &board::Board,
        loc: Result<Location, String>,
        moves: &mut Vec<PieceMovementType>,
    ) {
        if let Ok(valid_loc) = loc {
            if board[valid_loc].is_none() {
                moves.push(PieceMovementType::Relocate(valid_loc));
            }
        }
    }

    /// Adds a valid capture move to the list if the target location contains an enemy piece.
    ///
    /// ## Parameters
    /// - `board`: The current game board.
    /// - `loc`: The target location as a `Result` (contains the location or an error message).
    /// - `side`: The side of the current pawn (`Side::White` or `Side::Black`).
    /// - `moves`: The list of moves to which the valid capture will be added.
    fn add_capture_if_valid(
        board: &board::Board,
        loc: Result<Location, String>,
        side: Side,
        moves: &mut Vec<PieceMovementType>,
    ) {
        if let Ok(valid_loc) = loc {
            if let Some(piece) = board[valid_loc] {
                if piece.side != side {
                    moves.push(PieceMovementType::Capture(valid_loc));
                }
            }
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
        let is_in_start_rank = loc.rank == Self::get_start_rank(side);
        let single_step_loc = loc.offset(direction, 0);
        let double_step_loc = loc.offset(2 * direction, 0);

        if let Ok(single_loc) = single_step_loc {
            Self::add_move_if_valid(board, Ok(single_loc), &mut moves);
            let is_frivolous_cell_empty = board[single_loc].is_none();
            if is_in_start_rank && is_frivolous_cell_empty{
                Self::add_move_if_valid(board, double_step_loc, &mut moves);
            }
        }

        for &dx in &[-1, 1] {
            Self::add_capture_if_valid(board, loc.offset(direction, dx), side, &mut moves);
        }
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

}