use crate::engine::board::board::Board;
use crate::engine::board::location::{File, Location, Rank};
use crate::engine::board::pieces::{Piece, PieceType, Side};
use crate::engine::game::{get_move_generator, user_actions};
use crate::engine::gui::base::GUI;
use crate::engine::move_generator::king::KingMoveGen;



use crate::engine::move_generator::base::{MoveGenerator, PieceMovementType};
use std::fmt::Debug;

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
pub struct Game {
    /// The chessboard representing the current state of the game.
    board: Board,
    /// The graphical user interface used for rendering the board and handling user input.
    gui: Box<dyn GUI<user_actions::Action>>,
    /// The side of player turn
    active: Side,
    /// Current king position by type
    king_pos: [Location; 2]
}

impl Game {
    /// Creates a new `Game` instance.
    ///
    /// ## Parameters
    /// - `gui`: A boxed GUI interface  that handles rendering and user interaction.
    ///
    /// ## Returns
    /// - A boxed `Game` instance
    pub fn new(gui: Box<dyn GUI<user_actions::Action>>) -> Self {
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

    /// Checks if the king at the specified location is in check.
    ///
    /// This function determines if the king at the given location is under attack
    /// by any opponent piece. It works by generating all possible moves for the king
    /// and checking if any of these moves result in a capture of the king.
    ///
    /// # Arguments
    ///
    /// * `board` - A reference to the current game board.
    /// * `king_loc` - The location of the king to be checked.
    ///
    /// # Returns
    ///
    /// * `true` if the king is in check, i.e., under attack by an opponent piece.
    /// * `false` otherwise.
    fn is_checked(board: &Board, king_loc: Location) -> bool {
        match board[king_loc] {
            Some(piece) if piece.piece_type == PieceType::King => {
                KingMoveGen::generate_moves(board, king_loc, piece.side)
                    .iter()
                    .any(|movement| {
                        matches!(movement, PieceMovementType::Capture(target) if *target == king_loc)
                    })
            }
            _ => false,
        }
    }

    /// Validates if a given move is legal in the current game state.
    ///
    /// This function checks whether the move specified by the `MoveAction` is valid:
    /// - The selected piece must belong to the active side.
    /// - The target location must be within the possible moves for the selected piece.
    ///
    /// ## Parameters
    /// - `action`: A reference to a `MoveAction` containing the `from` and `to` locations
    ///   of the piece being moved.
    ///
    /// ## Returns
    /// - `true` if the move is valid.
    /// - `false` if the move is invalid (e.g., no piece at the `from` location, the piece
    ///   does not belong to the active side, or the target location is not valid).
    fn validate_move(&self, action: &user_actions::MoveAction) -> bool {
        match self.board[action.from] {
            Some(selected_piece) if selected_piece.side != self.active => false,
            Some(piece) => {
                let king_pos = self.king_pos[piece.side as usize];
                self.get_moves_by_type(piece.piece_type, action.from, self.active)
                    .contains(&action.to) && !Self::is_checked(&self.board, king_pos)
            }
            None => false,
        }
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
        loop {
            let user_action: user_actions::Action = self.gui.wait_and_process_event();
            match user_action {
                user_actions::Action::OfferDraw => todo!(),
                user_actions::Action::Resign => todo!(),
                user_actions::Action::AcceptDraw => todo!(),
                user_actions::Action::ShowMoveOption(x) => {
                    match self.board[x] {
                        Some(piece) => {
                            let show_values = self.get_moves_by_type(
                                piece.piece_type,
                                x,
                                piece.side
                            );
                            self.gui.render(&self.board, self.active, show_values);
                        }
                        _ => continue,
                    }
                }
                user_actions::Action::Move(move_action) if self.validate_move(&move_action) => {
                    self.board.action(move_action);
                    self.switch_active_side();
                    self.gui.render(&self.board, self.active, vec![]);
                }
                action => {
                    println!("{:?} is in correct", action);
                }
            };
        }
    }
}




