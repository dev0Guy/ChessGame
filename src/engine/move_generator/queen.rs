use crate::engine::board::board;
use crate::engine::board::location::{Location};
use crate::engine::board::pieces::{Side};
use crate::engine::move_generator::base::{MoveGenerator, PieceMovementType};

const QUEEN_POSSIBLE_DIRECTIONS: [(i8, i8); 8] = [
    (0, 1), (0, -1), (1, 0), (-1, 0),
    (1, 1), (-1, -1), (1, -1), (-1, 1),
];

/// A move generator for the queen piece in chess.
///
/// Queens can move any number of squares in any direction: horizontally, vertically, or diagonally.
/// They stop moving when they encounter an obstacle (another piece or the edge of the board).
/// Queens can capture opponent pieces but cannot move through other pieces.
///
/// This implementation leverages sliding move logic for all eight directions.
pub(crate) struct QueenMoveGen;

/// Generates all valid moves for a queen based on its current location, side, and the board state.
///
/// ## Parameters
/// - `board`: A reference to the current game board.
/// - `loc`: The current location of the queen.
/// - `side`: The side of the queen (`Side::White` or `Side::Black`).
///
/// ## Returns
/// - A list of valid moves for the queen, represented as `PieceMovementType`.
impl MoveGenerator for QueenMoveGen {
    fn generate_moves(board: &board::Board, loc: Location, side: Side) -> Vec<PieceMovementType> {
        Self::generate_sliding_moves(board, loc, side, &QUEEN_POSSIBLE_DIRECTIONS)
    }
}


#[warn(unused_variables)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::board::board::Board;
    use crate::engine::board::location::{File, Location, Rank};
    use crate::engine::board::pieces::{Piece, PieceType, Side};
    use crate::engine::move_generator::base::PieceMovementType;

    #[test]
    fn test_queen_moves_empty_board() {
        let mut board = Board::new();
        let loc = Location::new(File::D, Rank::Four);
        board[loc] = Some(Piece::new(PieceType::Queen, Side::White));

        let moves = QueenMoveGen::generate_moves(&board, loc, Side::White);

        let horizontal_movements = [
            Location::new(File::D, Rank::One),
            Location::new(File::D, Rank::Two),
            Location::new(File::D, Rank::Three),
            Location::new(File::D, Rank::Five),
            Location::new(File::D, Rank::Six),
            Location::new(File::D, Rank::Seven),
            Location::new(File::D, Rank::Eight),
        ];
        assert!(horizontal_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));

        let vertical_movements = [
            Location::new(File::A, Rank::Four),
            Location::new(File::B, Rank::Four),
            Location::new(File::C, Rank::Four),
            Location::new(File::E, Rank::Four),
            Location::new(File::F, Rank::Four),
            Location::new(File::G, Rank::Four),
            Location::new(File::H, Rank::Four),
        ];
        assert!(vertical_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));

        let diagonal_movements = [
            Location::new(File::A, Rank::One),
            Location::new(File::B, Rank::Two),
            Location::new(File::C, Rank::Three),
            Location::new(File::E, Rank::Five),
            Location::new(File::F, Rank::Six),
            Location::new(File::G, Rank::Seven),
            Location::new(File::H, Rank::Eight),
            Location::new(File::A, Rank::Seven),
            Location::new(File::B, Rank::Six),
            Location::new(File::C, Rank::Five),
            Location::new(File::E, Rank::Three),
            Location::new(File::F, Rank::Two),
            Location::new(File::G, Rank::One),
        ];
        assert!(diagonal_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
    }

    #[test]
    fn test_queen_moves_with_same_color_obstacles() {
        let mut board = Board::new();
        let loc = Location::new(File::D, Rank::Four);
        board[loc] = Some(Piece::new(PieceType::Queen, Side::Black));

        board[Location::new(File::D, Rank::Six)] = Some(Piece::new(PieceType::Pawn, Side::White));
        board[Location::new(File::B, Rank::Four)] = Some(Piece::new(PieceType::Pawn, Side::White));
        board[Location::new(File::E, Rank::Five)] = Some(Piece::new(PieceType::Pawn, Side::White));

        let moves = QueenMoveGen::generate_moves(&board, loc, Side::White);

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
            Location::new(File::E, Rank::Four),
            Location::new(File::F, Rank::Four),
            Location::new(File::G, Rank::Four),
            Location::new(File::H, Rank::Four),

        ];
        assert!(vertical_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert!(!moves.contains(&PieceMovementType::Relocate(Location::new(File::B, Rank::Four))));

        let diagonal_movements = [
            Location::new(File::A, Rank::One),
            Location::new(File::B, Rank::Two),
            Location::new(File::C, Rank::Three),
            Location::new(File::C, Rank::Five),
            Location::new(File::A, Rank::Seven),
            Location::new(File::B, Rank::Six),
            Location::new(File::E, Rank::Three),
            Location::new(File::F, Rank::Two),
            Location::new(File::G, Rank::One),
        ];

        assert!(diagonal_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert!(!moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Five))));
        assert_eq!(moves.len(), diagonal_movements.len() + horizontal_movements.len() + vertical_movements.len());
    }

    #[test]
    fn test_queen_moves_with_opposite_color_obstacles() {
        let mut board = Board::new();
        let loc = Location::new(File::D, Rank::Four);
        board[loc] = Some(Piece::new(PieceType::Queen, Side::Black));

        board[Location::new(File::D, Rank::Six)] = Some(Piece::new(PieceType::Pawn, Side::White));
        board[Location::new(File::B, Rank::Four)] = Some(Piece::new(PieceType::Pawn, Side::White));
        board[Location::new(File::E, Rank::Five)] = Some(Piece::new(PieceType::Pawn, Side::White));

        let moves = QueenMoveGen::generate_moves(&board, loc, Side::Black);

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
            Location::new(File::E, Rank::Four),
            Location::new(File::F, Rank::Four),
            Location::new(File::G, Rank::Four),
            Location::new(File::H, Rank::Four),

        ];
        assert!(vertical_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert!(!moves.contains(&PieceMovementType::Relocate(Location::new(File::B, Rank::Four))));

        let diagonal_movements = [
            Location::new(File::A, Rank::One),
            Location::new(File::B, Rank::Two),
            Location::new(File::C, Rank::Three),
            Location::new(File::C, Rank::Five),
            Location::new(File::A, Rank::Seven),
            Location::new(File::B, Rank::Six),
            Location::new(File::E, Rank::Three),
            Location::new(File::F, Rank::Two),
            Location::new(File::G, Rank::One),
        ];

        assert!(diagonal_movements.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert!(!moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Five))));

        let capture_loc = [
            Location::new(File::D, Rank::Six),
            Location::new(File::B, Rank::Four),
            Location::new(File::E, Rank::Five)
        ];
        assert!(capture_loc.iter().all(|&loc| moves.contains(&PieceMovementType::Capture(loc))));
    }
}
