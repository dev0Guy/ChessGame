use crate::engine::board::board::Board;
use crate::engine::board::location::{File, Location, Rank};
use crate::engine::board::pieces::{Piece, PieceType, Side};
use crate::engine::game::{get_move_generator, user_actions};
use crate::engine::gui::base::GUI;
use crate::engine::move_generator::king::KingMoveGen;
use std::fmt::Debug;
use strum::IntoEnumIterator;
use crate::engine::game::user_actions::MoveAction;
use crate::engine::move_generator::base::{MoveGenerator};

// TODO: duplicate code on check check and check checkmate and validate move.
// TODO: Change to get all possible action and if none then checkmate else validate user pick is one of them
/// Initial chess positions of the white pieces.
const WHITE_PIECES: [(Location, Piece); 16] = get_location_by_side(Side::White);

/// Initial chess positions of the black pieces.
const BLACK_PIECES: [(Location, Piece); 16] = get_location_by_side(Side::Black);

/// Returns the initial positions of chess pieces for the given player side.
///
/// Where each cell is containing of [`Location`] and a [`Piece`]
/// The location represents the square on the chessboard, and the piece is the corresponding
/// chess piece placed there.
///
/// ## Parameters
/// - `side`: The side for which to generate piece positions (`White` or `Black`).
///
/// ## Returns
/// - An array of tuples representing the initial positions of pieces for the given side.
const fn get_location_by_side(side: Side) -> [(Location, Piece); 16] {
    let pieces_rank = match side {
        Side::White => Rank::One,
        Side::Black => Rank::Eight,
    };
    let pawn_rank = match side {
        Side::White => Rank::Two,
        Side::Black => Rank::Seven,
    };
    [
        (
            Location::new(File::A, pieces_rank),
            Piece::new(PieceType::Rook, side),
        ),
        (
            Location::new(File::B, pieces_rank),
            Piece::new(PieceType::Knight, side),
        ),
        (
            Location::new(File::C, pieces_rank),
            Piece::new(PieceType::Bishop, side),
        ),
        (
            Location::new(File::D, pieces_rank),
            Piece::new(PieceType::Queen, side),
        ),
        (
            Location::new(File::E, pieces_rank),
            Piece::new(PieceType::King, side),
        ),
        (
            Location::new(File::F, pieces_rank),
            Piece::new(PieceType::Bishop, side),
        ),
        (
            Location::new(File::G, pieces_rank),
            Piece::new(PieceType::Knight, side),
        ),
        (
            Location::new(File::H, pieces_rank),
            Piece::new(PieceType::Rook, side),
        ),
        (
            Location::new(File::A, pawn_rank),
            Piece::new(PieceType::Pawn, side),
        ),
        (
            Location::new(File::B, pawn_rank),
            Piece::new(PieceType::Pawn, side),
        ),
        (
            Location::new(File::C, pawn_rank),
            Piece::new(PieceType::Pawn, side),
        ),
        (
            Location::new(File::D, pawn_rank),
            Piece::new(PieceType::Pawn, side),
        ),
        (
            Location::new(File::E, pawn_rank),
            Piece::new(PieceType::Pawn, side),
        ),
        (
            Location::new(File::F, pawn_rank),
            Piece::new(PieceType::Pawn, side),
        ),
        (
            Location::new(File::G, pawn_rank),
            Piece::new(PieceType::Pawn, side),
        ),
        (
            Location::new(File::H, pawn_rank),
            Piece::new(PieceType::Pawn, side),
        ),
    ]
}

/// Represents a chess game.
///
/// The `Game` struct is responsible for managing the chessboard, interacting
/// with the graphical user interface (GUI), and processing user actions.
pub struct Game<D: GUI<user_actions::Action>> {
    /// The chessboard representing the current state of the game.
    board: Board,
    /// The graphical user interface used for rendering the board and handling user input.
    gui: D,
    /// The side of player turn
    active: Side,
    /// Current king position by type
    king_pos: [Location; 2]
}


impl<D: GUI<user_actions::Action>> Game<D> {
    /// Creates a new `Game` instance.
    ///
    /// ## Parameters
    /// - `gui`: A boxed GUI interface  that handles rendering and user interaction.
    ///
    /// ## Returns
    /// - A boxed `Game` instance
    pub fn new(gui: D) -> Self {
        Self {
            board: Board::new(),
            gui,
            active: Side::White,
            king_pos: [Location::new(File::E, Rank::One), Location::new(File::E, Rank::Eight)]
        }
    }

    /// Resets the chessboard to its initial state.
    ///
    /// This method clears the board and places all the pieces in their starting positions
    /// for both white and black sides.
    fn reset_board(&mut self) {
        self.board = Board::new();
        self.king_pos = [Location::new(File::E, Rank::One), Location::new(File::E, Rank::Eight)];
        WHITE_PIECES
            .into_iter()
            .chain(BLACK_PIECES.into_iter())
            .for_each(|(location, piece)| {
                self.board[location] = Some(piece);
            });
    }

    /// Toggles the active side in the game.
    ///
    /// This function switches the active player from `Side::White` to `Side::Black`
    /// or from `Side::Black` to `Side::White`. It is typically called at the end of
    /// a turn to alternate the active side.
    fn switch_active_side(&mut self) {
        self.active = match self.active {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };
    }

    /// Generates all valid moves for a specific piece type at a given location.
    ///
    /// This function determines the possible moves for a piece of the specified type
    /// (`p_type`) at the provided `loc` and for the given `side`. It uses the appropriate
    /// move generator for each piece type.
    ///
    /// ## Parameters
    /// - `p_type`: The type of the piece (`PieceType`) to generate moves for.
    /// - `loc`: The current location of the piece on the board.
    /// - `side`: The side (`Side::White` or `Side::Black`) of the piece.
    ///
    /// ## Returns
    /// - A `Vec<Location>` containing all valid locations the piece can move to.
    fn get_moves_by_type(&self, p_type: PieceType, loc: Location, side: Side) -> Vec<Location>{
        get_move_generator(p_type)(&self.board, loc, side)
            .into_iter()
            .map(|x| { x.location() })
            .collect::<Vec<Location>>()
    }


    /// Validates if a given move is legal in the current game state.
    ///
    /// This function checks whether the move specified by the `MoveAction` is valid:
    /// - The selected piece must belong to the active side.
    ///
    /// ## Parameters
    /// - `action`: A reference to a `MoveAction` containing the `from` and `to` locations
    ///   of the piece being moved.
    ///
    /// ## Returns
    /// - `false` if the move is invalid (e.g., no piece at the `from` location, the piece
    ///   does not belong to the active side, or the target location is not valid).
    fn validate_move(&self, action: &MoveAction) -> bool {
        match self.board[action.from] {
            Some(piece) if piece.side == self.active => {
                self.get_moves_by_type(piece.piece_type, action.from, piece.side)
                    .contains(&action.to)
            }
            _ => false
        }
    }

    /// Attempts to execute a move and validates its legality.
    ///
    /// This function simulates the execution of a move while ensuring it does not leave
    /// the current player's king in check. If the move is valid, the board state is updated,
    /// the king's position is adjusted (if the king moves), and the active side is switched.
    /// If the move is invalid (leaves the king in check), the board is reverted to its
    /// original state, and an error is returned.
    ///
    /// ## Parameters
    /// - `action`: A reference to the `MoveAction` struct representing the move to be attempted.
    ///
    /// ## Returns
    /// - `Ok(())` if the move is valid and successfully executed.
    /// - `Err(String)` if the move is invalid, with an error message indicating the reason.
    ///
    /// ## Behavior
    /// - The function saves the original state of the board and the king's position before simulating the move.
    /// - If the move involves the king, its position is temporarily updated for validation.
    /// - The function checks if the move leaves the king in check using `KingMoveGen::is_checked`.
    /// - If the move is invalid, the board state is reverted, and the function returns an error.
    /// - If the move is valid, the board state is updated, and the active side is switched.
    fn try_move(
        &mut self,
        action: &MoveAction,
    ) -> Result<(), String>{
        let mut king_loc = self.king_pos[self.active as usize];
        let from_state = self.board[action.from].clone();
        let to_state = self.board[action.to].clone();
        match from_state {
            Some(piece) if piece.piece_type == PieceType::King => {
                king_loc = action.to;
            },
            _ => {},
        };
        self.board.action(action);
        if KingMoveGen::is_checked(&king_loc, &self.board){
            self.board[action.from] = from_state;
            self.board[action.to] = to_state;
            return Err(format!("{:?} is in correct", action));
        }
        self.king_pos[self.active as usize] = action.to;
        self.switch_active_side();
        Ok(())
    }

    fn check_is_check_and_rollback(&mut self, action: &MoveAction) -> bool {
        let mut king_loc = self.king_pos[self.active as usize];
        let from_state = self.board[action.from].clone();
        let to_state = self.board[action.to].clone();
        match from_state {
            Some(piece) if piece.piece_type == PieceType::King => {
                king_loc = action.to;
            },
            _ => {},
        };
        self.board.action(action);
        let is_checked = KingMoveGen::is_checked(&king_loc, &self.board);
        self.board[action.from] = from_state;
        self.board[action.to] = to_state;
        is_checked
    }

    /// Checks if the game is over due to checkmate.
    ///
    /// ## Returns
    /// - `true` if the active player is in checkmate.
    /// - `false` otherwise.
    fn is_checkmate(&mut self) -> bool {
        let board_cell_location = File::iter()
                .flat_map(|file| {
                    Rank::iter()
                        .filter_map(move |rank| {
                        Some((file, rank))
                    })
                });
        let my_piece_legal_moves_location = board_cell_location
            .filter_map(|(file, rank)| {
                let original_loc = Location::new(file, rank);
                match self.board[original_loc] {
                    Some(p) if p.side == self.active=> {
                        let actions = self.get_moves_by_type(p.piece_type, original_loc, self.active)
                            .into_iter()
                            .map(|next_loc|{
                                MoveAction { from: original_loc, to: next_loc }
                            })
                            .collect::<Vec<MoveAction>>();
                        Some(actions)
                    }
                    _ => {None}
                }
            })
            .flatten()
            .collect::<Vec<MoveAction>>();
        for action in my_piece_legal_moves_location.iter() {
            if !self.check_is_check_and_rollback(action) {
                return false;
            }
        }
        true

    }

    /// Starts the chess game.
    ///
    /// This method resets the board to its initial state and enters the main game loop,
    /// rendering the board and processing user actions.
    ///
    /// The game continues until a termination condition (e.g., resignation or draw) is met.
    pub fn start(&mut self) {
        self.reset_board();
        self.gui.render(&self.board, self.active, vec![]);
        while !self.is_checkmate() {
            let user_action: user_actions::Action = self.gui.wait_and_process_event();
            match user_action {
                user_actions::Action::OfferDraw => todo!(),
                user_actions::Action::Resign => todo!(),
                user_actions::Action::AcceptDraw => todo!(),
                user_actions::Action::ShowMoveOption(x) => {
                    if let Some(piece) = self.board[x] {
                        let show_values = self.get_moves_by_type(piece.piece_type, x, piece.side);
                        self.gui.render(&self.board, self.active, show_values);
                    }
                }
                user_actions::Action::Move(move_action) if self.validate_move(&move_action)=> {
                    let _ = self.try_move(&move_action);
                    self.gui.render(&self.board, self.active, vec![]);
                }
                user_actions::Action::Move(move_action)  => {
                    println!("{:?} is in correct", move_action);
                },

            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::board::location::{File, Location, Rank};
    use crate::engine::board::pieces::{Piece, PieceType, Side};
    use crate::engine::game::user_actions::MoveAction;
    use crate::engine::gui::cmd::CommandPromptGUI;

    fn create_cmd_game() -> Game<CommandPromptGUI>{
        let gui=  CommandPromptGUI::new();
        Game::new(gui)
    }

    /// from: https://lichess.org/editor/8/8/8/8/8/8/4P3/8_w_HAha_-_0_1?color=white
    /// to: https://lichess.org/editor/8/8/8/8/4P3/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_validate_move_active_player_piece() {
        let mut game = create_cmd_game();
        let start_loc = Location::new(File::E, Rank::Two);

        game.board[start_loc] = Some(Piece::new(PieceType::Pawn, Side::White));
        game.active = Side::White;

        let action = MoveAction { from: start_loc, to: Location::new(File::E, Rank::Four) };

        let is_valid = game.validate_move(&action);

        assert!(is_valid, "The move should be valid for the active player's piece.");
    }

    /// from: https://lichess.org/editor/8/8/8/8/8/8/4p3/8_w_HAha_-_0_1?color=white
    /// to: https://lichess.org/editor/8/8/8/8/8/8/8/4p3_w_HAha_-_0_1?color=white
    #[test]
    fn test_validate_move_opponent_piece() {
        let mut game = create_cmd_game();
        let start_loc = Location::new(File::E, Rank::Two);

        game.board[start_loc] = Some(Piece::new(PieceType::Pawn, Side::Black));
        game.active = Side::White;

        let action = MoveAction { from: start_loc, to: Location::new(File::E, Rank::One) };

        let is_valid = game.validate_move(&action);

        assert!(!is_valid, "The move should be invalid when attempting to move the opponent's piece.");
    }

    /// from: https://lichess.org/editor/8/8/8/8/8/8/8/8_w_HAha_-_0_1?color=white
    /// to: https://lichess.org/editor/8/8/8/8/8/8/8/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_validate_move_empty_square() {
        let mut game = create_cmd_game();
        let start_loc = Location::new(File::E, Rank::Two);

        game.board[start_loc] = None;
        game.active = Side::White;

        let action = MoveAction { from: start_loc, to: Location::new(File::E, Rank::Four) };

        let is_valid = game.validate_move(&action);

        assert!(!is_valid, "The move should be invalid when the starting square is empty.");
    }

    /// from: https://lichess.org/editor/4k3/8/8/8/8/8/4P3/4K3_w_HAha_-_0_1?color=white
    /// to: https://lichess.org/editor/4k3/8/8/8/4P3/8/8/4K3_w_HAha_-_0_1?color=white
    #[test]
    fn test_execute_valid_move() {
        let mut game = create_cmd_game();
        let start_loc = Location::new(File::E, Rank::Two);
        let end_loc = Location::new(File::E, Rank::Four);

        game.board[start_loc] = Some(Piece::new(PieceType::Pawn, Side::White));
        game.active = Side::White;

        let action = MoveAction { from: start_loc, to: end_loc };

        let result = game.try_move(&action);

        assert!(matches!(result, Ok(_)), "The move should be valid and executed.");
        println!("{:?}",game.board[start_loc]);
        println!("{:?}",game.board[end_loc]);
        assert!(game.board[end_loc].is_some(), "The piece should have moved to the target location.");
        assert!(game.board[start_loc].is_none(), "The starting location should be empty.");
        assert_eq!(game.active, Side::Black, "The active side should switch after a valid move.");
    }

    /// from: https://lichess.org/editor/4r3/8/8/8/8/8/4R3/4K3_w_HAha_-_0_1?color=white
    /// to: https://lichess.org/editor/4r3/8/8/8/8/8/5R2/4K3_w_HAha_-_0_1?color=white
    #[test]
    fn test_execute_move_invalid_due_to_check() {
        let mut game = create_cmd_game();
        let start_loc = Location::new(File::E, Rank::Two);
        let end_loc = Location::new(File::F, Rank::Two);
        let king_loc = Location::new(File::E, Rank::One);
        let opponent_rook_loc = Location::new(File::E, Rank::Eight);

        game.board[start_loc] = Some(Piece::new(PieceType::Rook, Side::White));
        game.board[king_loc] = Some(Piece::new(PieceType::King, Side::White));

        game.board[opponent_rook_loc] = Some(Piece::new(PieceType::Rook, Side::Black));
        game.king_pos[Side::White as usize] = king_loc;
        game.active = Side::White;

        let action = MoveAction { from: start_loc, to: end_loc };

        let result = game.try_move(&action);

        assert!(matches!(result, Err(_)), "The move should be invalid because it leaves the king in check.");
        assert!(game.board[start_loc].is_some(), "The starting location should remain occupied after the move is reverted.");
        assert!(game.board[end_loc].is_none(), "The target location should remain empty after the move is reverted.");
        assert_eq!(game.active, Side::White, "The active side should not switch after an invalid move.");
    }

    /// from: https://lichess.org/editor/8/8/8/8/8/8/8/4K3_w_HAha_-_0_1?color=white
    /// to: https://lichess.org/editor/8/8/8/8/8/8/4K3/8_w_HAha_-_0_1?color=white
    #[test]
    fn test_execute_move_updates_king_position() {
        let mut game = create_cmd_game();
        let start_loc = Location::new(File::E, Rank::One);
        let end_loc = Location::new(File::E, Rank::Two);

        game.board[start_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.king_pos[Side::White as usize] = start_loc;
        game.active = Side::White;

        let action = MoveAction { from: start_loc, to: end_loc };

        let result = game.try_move(&action);

        assert!(matches!(result, Ok(_)), "The king's move should be valid and executed.");
        assert!(game.board[end_loc].is_some(), "The king should have moved to the target location.");
        assert!(game.board[start_loc].is_none(), "The starting location should be empty.");
        assert_eq!(game.king_pos[Side::White as usize], end_loc, "The king's position should be updated.");
        assert_eq!(game.active, Side::Black, "The active side should switch after a valid move.");
    }

    /// from: https://lichess.org/editor/5r2/8/8/8/8/8/8/4K3_w_HAha_-_0_1?color=white
    /// to: https://lichess.org/editor/5r2/8/8/8/8/8/8/5K2_w_HAha_-_0_1?color=white
    #[test]
    fn test_execute_move_king_to_check_position() {
        let mut game = create_cmd_game();
        let start_loc = Location::new(File::E, Rank::One);
        let end_loc = Location::new(File::F, Rank::One);
        let other_side_rook_loc = Location::new(File::F, Rank::Eight);

        game.board[start_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.board[other_side_rook_loc] = Some(Piece::new(PieceType::Rook, Side::Black));
        game.king_pos[Side::White as usize] = start_loc;
        game.active = Side::White;

        let action = MoveAction { from: start_loc, to: end_loc };

        let result = game.try_move(&action);

        assert!(matches!(result, Err(_)), "The king's move should be valid and executed.");
        assert!(game.board[end_loc].is_none(), "The king should haven't moved to the target location.");
        assert!(game.board[start_loc].is_some(), "King should stayed in the starting location.");
        assert_eq!(game.king_pos[Side::White as usize], start_loc, "The king's position should not be updated.");
        assert_eq!(game.active, Side::White, "The active side should haven't switch after a invalid move.");
    }

    /// from: https://lichess.org/editor/4R3/8/8/8/8/8/8/5K2_w_HAha_-_0_1?color=white
    /// to: https://lichess.org/editor/5R2/8/8/8/8/8/8/5K2_w_HAha_-_0_1?color=white
    #[test]
    fn test_execute_move_king_to_none_check_position() {
        let mut game = create_cmd_game();
        let start_loc = Location::new(File::E, Rank::One);
        let end_loc = Location::new(File::F, Rank::One);
        let other_side_rook_loc = Location::new(File::F, Rank::Eight);

        game.board[start_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.board[other_side_rook_loc] = Some(Piece::new(PieceType::Rook, Side::White));
        game.king_pos[Side::White as usize] = start_loc;
        game.active = Side::White;

        let action = MoveAction { from: start_loc, to: end_loc };

        let result = game.try_move(&action);

        assert!(matches!(result, Ok(_)), "The king's move should be valid and executed.");
        assert!(game.board[end_loc].is_some(), "The king should have moved to the target location.");
        assert!(game.board[start_loc].is_none(), "King shouldn't stayed in the starting location.");
        assert_eq!(game.king_pos[Side::White as usize], end_loc, "The king's position should be updated.");
        assert_eq!(game.active, Side::Black, "The active side should haven't switch after a invalid move.");
    }

    /// from: https://lichess.org/editor/5r2/8/8/8/8/8/5B2/5K2_w_HAha_-_0_1?color=white
    /// to: https://lichess.org/editor/5r2/8/8/2B5/8/8/8/5K2_w_HAha_-_0_1?color=white
    #[test]
    fn test_execute_move_pined_piece() {
        let mut game = create_cmd_game();
        let start_loc = Location::new(File::E, Rank::One);
        let block_loc = Location::new(File::E, Rank::Two);
        let end_loc = Location::new(File::C, Rank::Five);
        let other_side_rook_loc = Location::new(File::E, Rank::Eight);

        game.board[block_loc] = Some(Piece::new(PieceType::Bishop, Side::White));
        game.board[start_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.board[other_side_rook_loc] = Some(Piece::new(PieceType::Rook, Side::Black));
        game.king_pos[Side::White as usize] = start_loc;
        game.active = Side::White;

        let action = MoveAction { from: block_loc, to: end_loc };

        let result = game.try_move(&action);

        assert!(matches!(result, Err(_)), "The bishop move should be valid and executed, its pined.");
        assert!(game.board[end_loc].is_none(), "The king should haven't moved to the target location.");
        assert!(game.board[start_loc].is_some(), "King should stayed in the starting location.");
        assert_eq!(game.king_pos[Side::White as usize], start_loc, "The king's position not should be updated.");
        assert_eq!(game.active, Side::White, "The active side should haven't switch after a invalid move.");
    }

    // board: https://lichess.org/editor/4r3/8/8/8/8/8/8/4K3_w_HAha_-_0_1?color=white
    #[test]
    fn test_checkmate_single_rock_false() {
        let mut game = create_cmd_game();

        // Place the white king
        let king_loc = Location::new(File::E, Rank::One);
        game.board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.king_pos[Side::White as usize] = king_loc;

        game.board[Location::new(File::E, Rank::Eight)] = Some(Piece::new(PieceType::Rook, Side::Black));

        game.active = Side::White;

        let result = game.is_checkmate();

        assert!(
            !result,
            "The king is not in checkmate but was incorrectly detected as checkmate."
        );
    }

    // board: https://lichess.org/editor/4r3/8/8/8/8/8/8/4K3_w_HAha_-_0_1?color=white
    #[test]
    fn test_checkmate_two_rock_false() {
        let mut game = create_cmd_game();

        // Place the white king
        let king_loc = Location::new(File::E, Rank::One);
        game.board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.king_pos[Side::White as usize] = king_loc;

        game.board[Location::new(File::E, Rank::Eight)] = Some(Piece::new(PieceType::Rook, Side::Black));
        game.board[Location::new(File::A, Rank::One)] = Some(Piece::new(PieceType::Rook, Side::Black));

        game.active = Side::White;

        let result = game.is_checkmate();

        assert!(
            !result,
            "The king is not in checkmate but was incorrectly detected as checkmate."
        );
    }

    /// board: https://lichess.org/editor/4r3/8/8/8/8/8/8/r3K3_w_HAha_-_0_1?color=white
    #[test]
    fn test_checkmate_false() {
        let mut game = create_cmd_game();

        let king_loc = Location::new(File::E, Rank::One);
        game.board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.king_pos[Side::White as usize] = king_loc;

        game.board[Location::new(File::E, Rank::Eight)] = Some(Piece::new(PieceType::Rook, Side::Black));
        game.board[Location::new(File::A, Rank::One)] = Some(Piece::new(PieceType::Rook, Side::Black));

        game.active = Side::White;

        let result = game.is_checkmate();

        assert!(!result, "The king is in checkmate but was not detected.");
    }

    /// board: https://lichess.org/editor/8/8/8/8/8/8/8/Kq6_w_HAha_-_0_1?color=white
    #[test]
    fn test_checkmate_one_piece_queen() {
        let mut game = create_cmd_game();

        let king_loc = Location::new(File::A, Rank::One);
        game.board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.king_pos[Side::White as usize] = king_loc;

        game.board[Location::new(File::B, Rank::One)] = Some(Piece::new(PieceType::Rook, Side::Black));

        game.active = Side::White;

        let result = game.is_checkmate();

        assert!(!result, "The king is in checkmate but was not detected.");
    }

    /// board: https://lichess.org/editor/8/8/8/8/8/8/7r/r3K3_w_HAha_-_0_1?color=white
    #[test]
    fn test_checkmate_two_rocks() {
        let mut game = create_cmd_game();

        let king_loc = Location::new(File::E, Rank::Four);
        game.board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.king_pos[Side::White as usize] = king_loc;

        game.board[Location::new(File::A, Rank::One)] = Some(Piece::new(PieceType::Rook, Side::Black));
        game.board[Location::new(File::B, Rank::Two)] = Some(Piece::new(PieceType::Rook, Side::Black));

        game.active = Side::White;

        let result = game.is_checkmate();

        assert!(!result, "The king is in checkmate but was not detected.");
    }

    /// board:https://lichess.org/editor/8/8/b7/8/8/8/2pp4/3K3r_w_HAha_-_0_1?color=white
    #[test]
    fn test_checkmate_bishop_blocked() {
        let mut game = create_cmd_game();

        let king_loc = Location::new(File::D, Rank::One);
        game.board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.king_pos[Side::White as usize] = king_loc;

        game.board[Location::new(File::C, Rank::Two)] = Some(Piece::new(PieceType::Pawn, Side::White));
        game.board[Location::new(File::D, Rank::Two)] = Some(Piece::new(PieceType::Pawn, Side::Black));
        game.board[Location::new(File::A, Rank::Six)] = Some(Piece::new(PieceType::Bishop, Side::Black));
        game.board[Location::new(File::H, Rank::One)] = Some(Piece::new(PieceType::Rook, Side::Black));

        game.active = Side::White;

        let result = game.is_checkmate();

        assert!(!result, "The king is in checkmate but was not detected.");
    }

    /// board: https://lichess.org/editor/8/8/b7/8/8/8/2pp4/3K3r_w_HAha_-_0_1?color=white
    #[test]
    fn test_not_checkmate_one_action_left() {
        let mut game = create_cmd_game();

        let king_loc = Location::new(File::D, Rank::One);
        game.board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.king_pos[Side::White as usize] = king_loc;

        game.board[Location::new(File::D, Rank::Two)] = Some(Piece::new(PieceType::Pawn, Side::Black));
        game.board[Location::new(File::A, Rank::Six)] = Some(Piece::new(PieceType::Bishop, Side::Black));
        game.board[Location::new(File::H, Rank::One)] = Some(Piece::new(PieceType::Rook, Side::Black));

        game.active = Side::White;

        let result = game.is_checkmate();

        assert!(!result, "The king is not checkmate but was incorrectly detected as checkmate.");
    }

    /// board: https://lichess.org/editor/8/8/8/8/8/4n3/2PPP3/2nKP2r_w_HAha_-_0_1?color=white
    #[test]
    fn test_not_check_mate_by_pawn_eat() {
        let mut game = create_cmd_game();

        let king_loc = Location::new(File::D, Rank::One);
        game.board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.king_pos[Side::White as usize] = king_loc;

        game.board[Location::new(File::C, Rank::Two)] = Some(Piece::new(PieceType::Pawn, Side::White));
        game.board[Location::new(File::D, Rank::Two)] = Some(Piece::new(PieceType::Pawn, Side::White));
        game.board[Location::new(File::E, Rank::Two)] = Some(Piece::new(PieceType::Pawn, Side::White));
        game.board[Location::new(File::E, Rank::One)] = Some(Piece::new(PieceType::Pawn, Side::White));
        game.board[Location::new(File::H, Rank::One)] = Some(Piece::new(PieceType::Rook, Side::Black));
        game.board[Location::new(File::E, Rank::Three)] = Some(Piece::new(PieceType::Knight, Side::Black));
        game.board[Location::new(File::C, Rank::One)] = Some(Piece::new(PieceType::Knight, Side::Black));

        game.active = Side::White;

        let result = game.is_checkmate();

        assert!(!result, "The king is not checkmate but was incorrectly detected as checkmate.");
    }

    /// board: https://lichess.org/editor/8/8/8/8/8/4n3/2PPP3/2rKP2r_w_HAha_-_0_1?color=white
    #[test]
    fn test_not_check_mate_by_king_eat() {
        let mut game = create_cmd_game();

        let king_loc = Location::new(File::D, Rank::One);
        game.board[king_loc] = Some(Piece::new(PieceType::King, Side::White));
        game.king_pos[Side::White as usize] = king_loc;

        game.board[Location::new(File::C, Rank::Two)] = Some(Piece::new(PieceType::Pawn, Side::White));
        game.board[Location::new(File::D, Rank::Two)] = Some(Piece::new(PieceType::Pawn, Side::White));
        game.board[Location::new(File::E, Rank::Two)] = Some(Piece::new(PieceType::Pawn, Side::White));
        game.board[Location::new(File::E, Rank::One)] = Some(Piece::new(PieceType::Pawn, Side::White));
        game.board[Location::new(File::H, Rank::One)] = Some(Piece::new(PieceType::Rook, Side::Black));
        game.board[Location::new(File::E, Rank::Three)] = Some(Piece::new(PieceType::Knight, Side::Black));
        game.board[Location::new(File::C, Rank::One)] = Some(Piece::new(PieceType::Rook, Side::Black));

        game.active = Side::White;

        let result = game.is_checkmate();

        assert!(!result, "The king is not checkmate but was incorrectly detected as checkmate.");
    }


}
