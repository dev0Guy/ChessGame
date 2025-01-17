use crate::engine::board::board::Board;
use crate::engine::board::location::{File, Location, Rank};
use crate::engine::board::pieces::{Piece, PieceType, Side};
use crate::engine::game::user_actions;
use crate::engine::gui::base::GUI;
use crate::engine::move_generator::pawn::PawnMoveGen;
use crate::engine::move_generator::base::{MoveGenerator};
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
        }
    }

    /// Resets the chessboard to its initial state.
    ///
    /// This method clears the board and places all the pieces in their starting positions
    /// for both white and black sides.
    fn reset_board(&mut self) {
        self.board = Board::new();
        WHITE_PIECES
            .into_iter()
            .chain(BLACK_PIECES.into_iter())
            .for_each(|(location, piece)| {
                self.board[location] = Some(piece);
            });
    }

    fn switch_active_side(&mut self) {
        self.active = match self.active {
            Side::White => Side::Black,
            Side::Black => Side::White,
        };
    }

    fn validate_move(&self, action: &user_actions::MoveAction) -> bool {
        match self.board[action.from] {
            None => false,
            Some(selected_piece) if selected_piece.side != self.active => false,
            Some(piece) => {
                match piece.piece_type {
                    PieceType::Pawn => {}
                    PieceType::Rook => {}
                    PieceType::Knight => {}
                    PieceType::Bishop => {}
                    PieceType::Queen => {}
                    PieceType::King => {}
                }
                // TODO: validate check option and move for piece
                true
            }
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
        self.gui.render(&self.board, self.active);
        loop {
            let user_action: user_actions::Action = self.gui.wait_and_process_event();
            match user_action {
                user_actions::Action::OfferDraw => {
                    // TODO: ADD
                }
                user_actions::Action::Resign => {
                    // TODO: ADD
                }
                user_actions::Action::AcceptDraw => {
                    // TODO: ADD
                }
                user_actions::Action::ShowMoveOption(x) if self.board[x].is_some() => {
                    let values = match self.board[x].unwrap().piece_type {
                        PieceType::Pawn => { PawnMoveGen::generate_moves(&self.board, x, self.active) },
                        PieceType::Rook => { todo!()},
                        PieceType::Knight => {todo!()},
                        PieceType::Bishop => {todo!()},
                        PieceType::Queen => {todo!()},
                        PieceType::King => {todo!()},
                    };
                    println!("Called Show on {:?}", values)
                }
                user_actions::Action::Move(move_action) if self.validate_move(&move_action) => {
                    self.board.action(move_action);
                    self.switch_active_side();
                    self.gui.render(&self.board, self.active);
                }
                action => {
                    // TODO: send to gui action is not available, enter a new one
                }
            };
        }
    }
}
