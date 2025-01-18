use crate::engine::board::board;
use crate::engine::board::location::Location;
use crate::engine::board::pieces::Side;
use crate::engine::move_generator::base::{MoveGenerator, PieceMovementType};

const KING_POSSIBLE_DIRECTIONS: [(i8, i8); 8] = [
    (0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1),
];

/// A move generator for the king piece in chess.
///
/// The king can move one square in any direction: horizontally, vertically, or diagonally.
/// This implementation ensures that the king does not move outside the board boundaries.
pub(crate) struct KingMoveGen;

impl MoveGenerator for KingMoveGen {
    /// Generates all valid moves for a king based on its current location, side, and the board state.
    ///
    /// ## Parameters
    /// - `board`: A reference to the current game board.
    /// - `loc`: The current location of the king.
    /// - `side`: The side of the king (`Side::White` or `Side::Black`).
    ///
    /// ## Returns
    /// - A list of valid moves for the king, represented as `PieceMovementType`.
    fn generate_moves(board: &board::Board, loc: Location, side: Side) -> Vec<PieceMovementType> {
        let mut moves = Vec::new();

        for &(dx, dy) in &KING_POSSIBLE_DIRECTIONS {
            if let Ok(target_loc) = loc.offset(dy, dx) {
                match board[target_loc] {
                    Some(piece) if piece.side != side => {
                        moves.push(PieceMovementType::Capture(target_loc));
                    }
                    None => {
                        moves.push(PieceMovementType::Relocate(target_loc));
                    }
                    _ => {}
                }
            }
        }
        moves
    }
}

impl KingMoveGen{
    fn is_checked(loc: Location, board: &board::Board) -> bool{
        true
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
    fn test_king_moves_empty_board() {
        let mut board = Board::new();
        let loc = Location::new(File::D, Rank::Four);
        board[loc] = Some(Piece::new(PieceType::King, Side::White));

        let moves = KingMoveGen::generate_moves(&board, loc, Side::White);

        let expected_moves = [
            Location::new(File::C, Rank::Three),
            Location::new(File::C, Rank::Four),
            Location::new(File::C, Rank::Five),
            Location::new(File::D, Rank::Three),
            Location::new(File::D, Rank::Five),
            Location::new(File::E, Rank::Three),
            Location::new(File::E, Rank::Four),
            Location::new(File::E, Rank::Five),
        ];

        assert!(expected_moves.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert_eq!(moves.len(), expected_moves.len());
    }

    #[test]
    fn test_king_moves_with_same_color_obstacles() {
        let mut board = Board::new();
        let loc = Location::new(File::D, Rank::Four);
        board[loc] = Some(Piece::new(PieceType::King, Side::White));

        board[Location::new(File::C, Rank::Three)] = Some(Piece::new(PieceType::Pawn, Side::White));
        board[Location::new(File::E, Rank::Five)] = Some(Piece::new(PieceType::Pawn, Side::White));

        let moves = KingMoveGen::generate_moves(&board, loc, Side::White);

        let expected_moves = [
            Location::new(File::C, Rank::Four),
            Location::new(File::C, Rank::Five),
            Location::new(File::D, Rank::Three),
            Location::new(File::D, Rank::Five),
            Location::new(File::E, Rank::Three),
            Location::new(File::E, Rank::Four),
        ];

        assert!(expected_moves.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert!(!moves.contains(&PieceMovementType::Relocate(Location::new(File::C, Rank::Three))));
        assert!(!moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Five))));
        assert_eq!(moves.len(), expected_moves.len());
    }

    #[test]
    fn test_king_moves_with_opposite_color_obstacles() {
        let mut board = Board::new();
        let loc = Location::new(File::D, Rank::Four);
        board[loc] = Some(Piece::new(PieceType::King, Side::White));

        // Add opposite-color obstacles
        board[Location::new(File::C, Rank::Three)] = Some(Piece::new(PieceType::Pawn, Side::Black));
        board[Location::new(File::E, Rank::Five)] = Some(Piece::new(PieceType::Pawn, Side::Black));

        let moves = KingMoveGen::generate_moves(&board, loc, Side::White);

        let expected_moves = [
            Location::new(File::C, Rank::Four),
            Location::new(File::C, Rank::Five),
            Location::new(File::D, Rank::Three),
            Location::new(File::D, Rank::Five),
            Location::new(File::E, Rank::Three),
            Location::new(File::E, Rank::Four),
        ];

        assert!(expected_moves.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert!(moves.contains(&PieceMovementType::Capture(Location::new(File::C, Rank::Three))));
        assert!(moves.contains(&PieceMovementType::Capture(Location::new(File::E, Rank::Five))));
        assert_eq!(moves.len(), expected_moves.len() + 2);
    }

    #[test]
    fn test_king_moves_at_board_edge() {
        let mut board = Board::new();
        let loc = Location::new(File::A, Rank::One);
        board[loc] = Some(Piece::new(PieceType::King, Side::White));

        let moves = KingMoveGen::generate_moves(&board, loc, Side::White);

        let expected_moves = [
            Location::new(File::A, Rank::Two),
            Location::new(File::B, Rank::One),
            Location::new(File::B, Rank::Two),
        ];

        assert!(expected_moves.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert_eq!(moves.len(), expected_moves.len());
    }
}