use crate::engine::board::board;
use crate::engine::board::location::Location;
use crate::engine::board::pieces::Side;
use crate::engine::move_generator::base::{MoveGenerator, PieceMovementType};



const KNIGHT_POSSIBLE_MOVES: [(i8, i8); 8] = [(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2), ];
/// A move generator for the knight piece in chess.
///
/// Knights move in an "L" shape: two squares in one direction and one square perpendicular to it.
/// Knights can jump over other pieces, so their moves are not blocked by obstacles.
pub(crate) struct KnightMoveGen;

impl KnightMoveGen {

}

impl MoveGenerator for KnightMoveGen {
    /// Generates all valid moves for a knight based on its current location, side, and the board state.
    ///
    /// ## Parameters
    /// - `board`: A reference to the current game board.
    /// - `loc`: The current location of the knight.
    /// - `side`: The side of the knight (`Side::White` or `Side::Black`).
    ///
    /// ## Returns
    /// - A list of valid moves for the knight, represented as `PieceMovementType`.
    fn generate_moves(board: &board::Board, loc: Location, side: Side) -> Vec<PieceMovementType> {
        let mut moves = Vec::new();

        for &(dx, dy) in &KNIGHT_POSSIBLE_MOVES {
            if let Ok(target_loc) = loc.offset(dy, dx) {
                match board[target_loc] {
                    Some(piece) if piece.side != side => {
                        // Capture opponent piece
                        moves.push(PieceMovementType::Capture(target_loc));
                    }
                    None => {
                        // Move to empty square
                        moves.push(PieceMovementType::Relocate(target_loc));
                    }
                    _ => {
                        // Friendly piece - do nothing
                    }
                }
            }
        }

        moves
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
    fn test_knight_moves_empty_board() {
        let mut board = Board::new();
        let loc = Location::new(File::D, Rank::Four);
        board[loc] = Some(Piece::new(PieceType::Knight, Side::White));

        let moves = KnightMoveGen::generate_moves(&board, loc, Side::White);

        let expected_moves = [
            Location::new(File::B, Rank::Three),
            Location::new(File::B, Rank::Five),
            Location::new(File::C, Rank::Two),
            Location::new(File::C, Rank::Six),
            Location::new(File::E, Rank::Two),
            Location::new(File::E, Rank::Six),
            Location::new(File::F, Rank::Three),
            Location::new(File::F, Rank::Five),
        ];

        assert!(expected_moves.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert_eq!(moves.len(), expected_moves.len());
    }

    #[test]
    fn test_knight_moves_with_same_color_obstacles() {
        let mut board = Board::new();
        let loc = Location::new(File::D, Rank::Four);
        board[loc] = Some(Piece::new(PieceType::Knight, Side::White));

        board[Location::new(File::B, Rank::Three)] = Some(Piece::new(PieceType::Pawn, Side::White));
        board[Location::new(File::E, Rank::Six)] = Some(Piece::new(PieceType::Pawn, Side::White));

        let moves = KnightMoveGen::generate_moves(&board, loc, Side::White);

        let expected_moves = [
            Location::new(File::B, Rank::Five),
            Location::new(File::C, Rank::Two),
            Location::new(File::C, Rank::Six),
            Location::new(File::E, Rank::Two),
            Location::new(File::F, Rank::Three),
            Location::new(File::F, Rank::Five),
        ];

        assert!(expected_moves.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert!(!moves.contains(&PieceMovementType::Relocate(Location::new(File::B, Rank::Three))));
        assert!(!moves.contains(&PieceMovementType::Relocate(Location::new(File::E, Rank::Six))));
        assert_eq!(moves.len(), expected_moves.len());
    }

    #[test]
    fn test_knight_moves_with_opposite_color_obstacles() {
        let mut board = Board::new();
        let loc = Location::new(File::D, Rank::Four);
        board[loc] = Some(Piece::new(PieceType::Knight, Side::White));

        // Add opposite-color obstacles
        board[Location::new(File::B, Rank::Three)] = Some(Piece::new(PieceType::Pawn, Side::Black));
        board[Location::new(File::E, Rank::Six)] = Some(Piece::new(PieceType::Pawn, Side::Black));

        let moves = KnightMoveGen::generate_moves(&board, loc, Side::White);

        let expected_moves = [
            Location::new(File::B, Rank::Five),
            Location::new(File::C, Rank::Two),
            Location::new(File::C, Rank::Six),
            Location::new(File::E, Rank::Two),
            Location::new(File::F, Rank::Three),
            Location::new(File::F, Rank::Five),
        ];

        assert!(expected_moves.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert!(moves.contains(&PieceMovementType::Capture(Location::new(File::B, Rank::Three))));
        assert!(moves.contains(&PieceMovementType::Capture(Location::new(File::E, Rank::Six))));
        assert_eq!(moves.len(), expected_moves.len() + 2);
    }

    #[test]
    fn test_knight_moves_at_board_edge() {
        let mut board = Board::new();
        let loc = Location::new(File::A, Rank::One);
        board[loc] = Some(Piece::new(PieceType::Knight, Side::White));

        let moves = KnightMoveGen::generate_moves(&board, loc, Side::White);

        let expected_moves = [
            Location::new(File::B, Rank::Three),
            Location::new(File::C, Rank::Two),
        ];

        assert!(expected_moves.iter().all(|&loc| moves.contains(&PieceMovementType::Relocate(loc))));
        assert_eq!(moves.len(), expected_moves.len());
    }
}