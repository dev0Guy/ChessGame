use strum::IntoEnumIterator;
use crate::engine::board::board;
use crate::engine::board::board::Board;
use crate::engine::board::location::{File, Location, Rank};
use crate::engine::board::pieces::{Piece, PieceType, Side};
use crate::engine::game::get_move_generator;
use crate::engine::game::threat::ThreadBoard;
use crate::engine::move_generator::base::{MoveGenerator, PieceMovementType};

const KING_POSSIBLE_DIRECTIONS: [(i8, i8); 8] = [
    (0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1),
];

const KINGS_START_POSITION: [Location; 2] = [Location::new(File::E, Rank::One), Location::new(File::E, Rank::Eight)];

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

        Self::king_side_castling(board, &loc).map(|castling_move| moves.push(castling_move));
        Self::queen_side_castling(board, &loc).map(|castling_move| moves.push(castling_move));
        moves
    }
}

impl KingMoveGen {

    fn king_side_castling(
        board: &Board,
        loc: &Location
    ) -> Option<PieceMovementType>{
        if let piece = board[*loc] {
            match piece {
                Some(Piece{ piece_type: PieceType::King, side, has_moved: false}) => {
                    let rook_loc = Location::new(File::H, loc.rank);
                    if matches!(board[rook_loc], Some(Piece{ piece_type: PieceType::Rook, side, has_moved: false}))
                        && board[Location::new(File::F, loc.rank)].is_none()
                        && board[Location::new(File::G, loc.rank)].is_none()
                    {
                        let king_target = Location::new(File::G, loc.rank);
                        let rook_target = Location::new(File::F, loc.rank);
                        return Some(PieceMovementType::Castle(king_target, rook_target));
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn queen_side_castling(board: &Board, loc: &Location) -> Option<PieceMovementType>{
        if let piece = board[*loc] {
            match piece {
                Some(Piece{ piece_type: PieceType::King, has_moved: false, ..}) => {
                    let rook_loc = Location::new(File::A, loc.rank);
                    if matches!(board[rook_loc], Some(Piece{ piece_type: PieceType::Rook, side, has_moved: false}))
                        && board[Location::new(File::B, loc.rank)].is_none()
                        && board[Location::new(File::C, loc.rank)].is_none()
                        && board[Location::new(File::D, loc.rank)].is_none()
                    {
                        let king_target = Location::new(File::C, loc.rank);
                        let rook_target = Location::new(File::D, loc.rank);
                        return Some(PieceMovementType::Castle(king_target, rook_target));
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Finds all the locations of pieces that are attacking the given location.
    ///
    /// This function determines which pieces on the board are currently threatening
    /// the piece at the specified location (`k_loc`). It iterates through all possible
    /// piece types and uses their respective move generators to calculate potential
    /// attack moves. If a move matches the criteria for a valid attack, the location
    /// of the attacking piece is included in the result.
    ///
    /// ## Parameters
    /// - `k_loc`: A reference to the location being checked for attacks.
    /// - `board`: A reference to the current game board.
    ///
    /// ## Returns
    /// - A `Vec<Location>` containing the locations of all pieces that are attacking
    ///   the piece at `k_loc`.
    ///
    /// ## Behavior
    /// - If there is no piece at `k_loc`, an empty vector is returned.
    /// - For each piece type, the function checks all valid capture moves and confirms
    ///   whether the attacking piece matches the expected type.
    /// - The resulting vector contains only the locations of valid attacking pieces.
    pub(crate) fn get_checked_pieces_location(k_loc: &Location, board: &board::Board) -> Vec<Location>{
        let mut attack_locations = vec![];
        if let Some(piece) = board[*k_loc] {
            attack_locations = PieceType::iter()
                .flat_map(|piece_type| {
                    get_move_generator(piece_type)(board, *k_loc, piece.side)
                        .into_iter()
                        .filter_map(move |movement| {
                            match movement {
                                PieceMovementType::Capture(attack_loc) => {
                                    match board[attack_loc] {
                                        Some(piece) if piece.piece_type == piece_type =>  Some(attack_loc),
                                        _ => None
                                    }
                                }
                                _ => None
                            }
                        })
                })
                .collect::<Vec<Location>>();
        }
        attack_locations
    }

    /// Checks if the king is in check.
    ///
    /// ## Parameters
    /// - `board`: A reference to the current game board.
    /// - `king_loc`: The location of the king.
    /// - `side`: The side of the king (`Side::White` or `Side::Black`).
    ///
    /// ## Returns
    /// - `true` if the king is in check.
    /// - `false` otherwise.
    pub(crate) fn is_checked(k_loc: &Location, board: &board::Board) -> bool {
        !Self::get_checked_pieces_location(k_loc, board).is_empty()
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

    #[test]
    fn test_king_not_in_check_empty_board() {
        let mut board = Board::new();
        let king_loc = Location::new(File::D, Rank::Four);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));

        let in_check = KingMoveGen::is_checked(&king_loc, &board);

        assert!(!in_check, "King should not be in check on an empty board.");
    }

    #[test]
    fn test_king_in_check_by_rook() {
        let mut board = Board::new();
        let king_loc = Location::new(File::D, Rank::Four);
        let attacker_loc = Location::new(File::D, Rank::Eight);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));

        board[attacker_loc] = Some(Piece::new(PieceType::Rook, Side::Black));

        let in_check = KingMoveGen::is_checked(&king_loc, &board);

        assert!(in_check, "King should be in check by a rook.");
    }

    #[test]
    fn test_king_in_check_by_knight() {
        let mut board = Board::new();
        let king_loc = Location::new(File::E, Rank::Four);
        let attacker_loc = Location::new(File::G, Rank::Five);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));

        board[attacker_loc] = Some(Piece::new(PieceType::Knight, Side::Black));

        let in_check = KingMoveGen::is_checked(&king_loc, &board);

        assert!(in_check, "King should be in check by a knight.");
    }

    #[test]
    fn test_king_not_in_check_blocked_by_friendly_piece() {
        let mut board = Board::new();
        let king_loc = Location::new(File::D, Rank::Four);
        let attacker_loc = Location::new(File::D, Rank::Eight);
        let blocker_loc = Location::new(File::D, Rank::Six);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        board[attacker_loc] = Some(Piece::new(PieceType::Rook, Side::Black));
        board[blocker_loc] = Some(Piece::new(PieceType::Pawn, Side::White));

        let in_check = KingMoveGen::is_checked(&king_loc, &board);

        assert!(!in_check, "King should not be in check if the attack is blocked by a friendly piece.");
    }

    #[test]
    fn test_king_in_check_by_multiple_attackers() {
        let mut board = Board::new();
        let king_loc = Location::new(File::D, Rank::Four);
        let attacker_1_loc = Location::new(File::D, Rank::Eight);
        let attacker_2_loc = Location::new(File::H, Rank::Four);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));

        board[attacker_1_loc] = Some(Piece::new(PieceType::Rook, Side::Black));
        board[attacker_2_loc] = Some(Piece::new(PieceType::Rook, Side::Black));

        let in_check = KingMoveGen::is_checked(&king_loc, &board);

        assert!(in_check, "King should be in check by multiple attackers.");
    }

    /// board: https://lichess.org/editor/8/8/8/8/4K3/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_check_pieces_no_attacking_pieces() {
        let mut board = Board::new();
        let king_loc = Location::new(File::E, Rank::Four);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));

        let attack_locations = KingMoveGen::get_checked_pieces_location(&king_loc, &board);

        assert!(attack_locations.is_empty(), "No pieces should be attacking the king.");
    }

    /// board: https://lichess.org/editor/4r3/8/8/8/4K3/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_check_pieces_attacking_rook() {
        let mut board = Board::new();
        let king_loc = Location::new(File::E, Rank::Four);
        let attacker_loc = Location::new(File::E, Rank::Eight);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        board[attacker_loc] = Some(Piece::new(PieceType::Rook, Side::Black));

        let attack_locations = KingMoveGen::get_checked_pieces_location(&king_loc, &board);

        assert_eq!(attack_locations, vec![attacker_loc], "The rook should be threatening the king.");
    }

    /// board: https://lichess.org/editor/8/8/8/6n1/4K3/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_check_pieces_attacking_knight() {
        let mut board = Board::new();
        let king_loc = Location::new(File::E, Rank::Four);
        let attacker_loc = Location::new(File::G, Rank::Five);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        board[attacker_loc] = Some(Piece::new(PieceType::Knight, Side::Black));

        let attack_locations = KingMoveGen::get_checked_pieces_location(&king_loc, &board);

        assert_eq!(attack_locations, vec![attacker_loc], "The knight should be threatening the king.");
    }

    /// board: https://lichess.org/editor/4r3/8/8/8/r3K3/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_check_pieces_multiple_attacking_pieces() {
        let mut board = Board::new();
        let king_loc = Location::new(File::E, Rank::Four);
        let attacker1_loc = Location::new(File::E, Rank::Eight);
        let attacker2_loc = Location::new(File::A, Rank::Four);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        board[attacker1_loc] = Some(Piece::new(PieceType::Rook, Side::Black));
        board[attacker2_loc] = Some(Piece::new(PieceType::Rook, Side::Black));

        let attack_locations = KingMoveGen::get_checked_pieces_location(&king_loc, &board);

        assert_eq!(
            attack_locations,
            vec![attacker1_loc, attacker2_loc],
            "Both rooks should be threatening the king."
        );
    }

    /// board: https://lichess.org/editor/4r3/8/4P3/8/4K3/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_check_pieces_blocked_attack() {
        let mut board = Board::new();
        let king_loc = Location::new(File::E, Rank::Four);
        let attacker_loc = Location::new(File::E, Rank::Eight);
        let blocker_loc = Location::new(File::E, Rank::Six);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        board[attacker_loc] = Some(Piece::new(PieceType::Rook, Side::Black));
        board[blocker_loc] = Some(Piece::new(PieceType::Pawn, Side::White));

        let attack_locations = KingMoveGen::get_checked_pieces_location(&king_loc, &board);

        assert!(attack_locations.is_empty(), "The attack should be blocked by a friendly piece.");
    }

    /// board: https://lichess.org/editor/8/8/8/8/8/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_check_pieces_no_piece_at_location() {
        let mut board = Board::new();
        let empty_loc = Location::new(File::E, Rank::Four);

        board[empty_loc] = None;

        let attack_locations = KingMoveGen::get_checked_pieces_location(&empty_loc, &board);

        assert!(attack_locations.is_empty(), "No attacks should be detected if there is no piece.");
    }

    /// board: https://lichess.org/editor/8/8/8/8/8/8/8/4K2R_w_KAha_-_0_1?color=white
    #[test]
    fn test_king_side_castling() {
        let mut board = Board::new();
        let king_loc = Location::new(File::E, Rank::One);
        let rook_loc = Location::new(File::H, Rank::One);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        board[rook_loc] = Some(Piece::new(PieceType::Rook, Side::White));

        let moves = KingMoveGen::generate_moves(&board, king_loc, Side::White);

        let expected_castling_move = PieceMovementType::Castle(
            Location::new(File::G, Rank::One),
            Location::new(File::F, Rank::One),
        );

        assert!(moves.contains(&expected_castling_move), "Kingside castling should be a valid move.");
    }

    /// board: https://lichess.org/editor/8/8/8/8/8/8/8/R3K3_w_HQha_-_0_1?color=white
    #[test]
    fn test_queen_side_castling() {
        let mut board = Board::new();
        let king_loc = Location::new(File::E, Rank::One);
        let rook_loc = Location::new(File::A, Rank::One);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        board[rook_loc] = Some(Piece::new(PieceType::Rook, Side::White));

        let moves = KingMoveGen::generate_moves(&board, king_loc, Side::White);

        let expected_castling_move = PieceMovementType::Castle(
            Location::new(File::C, Rank::One),
            Location::new(File::D, Rank::One),
        );

        assert!(moves.contains(&expected_castling_move), "Queenside castling should be a valid move.");
    }

    /// board: https://lichess.org/editor/8/8/8/8/8/8/8/4KB1R_w_KAha_-_0_1?color=white
    #[test]
    fn test_castling_blocked_by_piece() {
        let mut board = Board::new();
        let king_loc = Location::new(File::E, Rank::One);
        let rook_loc = Location::new(File::H, Rank::One);
        let blocking_piece_loc = Location::new(File::F, Rank::One);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        board[rook_loc] = Some(Piece::new(PieceType::Rook, Side::White));

        board[blocking_piece_loc] = Some(Piece::new(PieceType::Bishop, Side::White));

        let moves = KingMoveGen::generate_moves(&board, king_loc, Side::White);

        let invalid_castling_move = PieceMovementType::Castle(
            Location::new(File::G, Rank::One),
            Location::new(File::F, Rank::One),
        );

        assert!(!moves.contains(&invalid_castling_move), "Kingside castling should not be allowed if there is a blocking piece.");
    }

    /// board: https://lichess.org/editor/4k2r/8/8/8/8/8/8/8_w_HAka_-_0_1?color=white
    #[test]
    fn test_black_king_side_castling() {
        let mut board = Board::new();
        let king_loc = Location::new(File::E, Rank::Eight);
        let rook_loc = Location::new(File::H, Rank::Eight);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::Black));
        board[rook_loc] = Some(Piece::new(PieceType::Rook, Side::Black));

        let moves = KingMoveGen::generate_moves(&board, king_loc, Side::Black);

        let expected_castling_move = PieceMovementType::Castle(
            Location::new(File::G, Rank::Eight),
            Location::new(File::F, Rank::Eight),
        );

        assert!(moves.contains(&expected_castling_move), "Black kingside castling should be a valid move.");
    }

    #[test]
    fn test_castling_king_has_moved() {
        let mut board = Board::new();
        let king_loc = Location::new(File::E, Rank::One);
        let rook_loc = Location::new(File::H, Rank::One);

        board[king_loc] = Some(Piece {
            piece_type: PieceType::King,
            side: Side::White,
            has_moved: true,
        });
        board[rook_loc] = Some(Piece::new(PieceType::Rook, Side::White));

        let moves = KingMoveGen::generate_moves(&board, king_loc, Side::White);

        let invalid_castling_move = PieceMovementType::Castle(
            Location::new(File::G, Rank::One),
            Location::new(File::F, Rank::One),
        );

        assert!(!moves.contains(&invalid_castling_move), "Castling should not be allowed if the king has moved.");
    }

    #[test]
    fn test_castling_rook_has_moved() {
        let mut board = Board::new();
        let king_loc = Location::new(File::E, Rank::One);
        let rook_loc = Location::new(File::H, Rank::One);

        board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        board[rook_loc] = Some(Piece {
            piece_type: PieceType::Rook,
            side: Side::White,
            has_moved: true,
        });

        let moves = KingMoveGen::generate_moves(&board, king_loc, Side::White);

        let invalid_castling_move = PieceMovementType::Castle(
            Location::new(File::G, Rank::One),
            Location::new(File::F, Rank::One),
        );

        assert!(!moves.contains(&invalid_castling_move), "Castling should not be allowed if the rook has moved.");
    }
}