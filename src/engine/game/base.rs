use std::fmt::Debug;
use crate::engine::board::board::Board;
use crate::engine::board::pieces::{Piece, PieceType, Side};
use crate::engine::board::location::{File, Location, Rank};
use crate::engine::gui::base::GUI;
use crate::engine::movement::moves;


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
const fn get_location_by_side(side: Side) -> [(Location, Piece); 16]{
    let pieces_rank = match side {
        Side::White => Rank::One,
        Side::Black => Rank::Eight
    };
    let pawn_rank = match side {
        Side::White => Rank::Two,
        Side::Black => Rank::Seven
    };
    [
        (Location::new(File::A, pieces_rank), Piece::new(PieceType::Rook, side)),
        (Location::new(File::B, pieces_rank), Piece::new(PieceType::Knight, side)),
        (Location::new(File::C, pieces_rank), Piece::new(PieceType::Bishop, side)),
        (Location::new(File::D, pieces_rank), Piece::new(PieceType::Queen, side)),
        (Location::new(File::E, pieces_rank), Piece::new(PieceType::King, side)),
        (Location::new(File::F, pieces_rank), Piece::new(PieceType::Bishop, side)),
        (Location::new(File::G, pieces_rank), Piece::new(PieceType::Knight, side)),
        (Location::new(File::H, pieces_rank), Piece::new(PieceType::Rook, side)),
        (Location::new(File::A, pawn_rank), Piece::new(PieceType::Pawn, side)),
        (Location::new(File::B, pawn_rank), Piece::new(PieceType::Pawn, side)),
        (Location::new(File::C, pawn_rank), Piece::new(PieceType::Pawn, side)),
        (Location::new(File::D, pawn_rank), Piece::new(PieceType::Pawn, side)),
        (Location::new(File::E, pawn_rank), Piece::new(PieceType::Pawn, side)),
        (Location::new(File::F, pawn_rank), Piece::new(PieceType::Pawn, side)),
        (Location::new(File::G, pawn_rank), Piece::new(PieceType::Pawn, side)),
        (Location::new(File::H, pawn_rank), Piece::new(PieceType::Pawn, side)),
    ]
}

/// Represents a chess game.
///
/// The `Game` struct is responsible for managing the chessboard, interacting
/// with the graphical user interface (GUI), and processing user actions.
pub struct Game{
    /// The chessboard representing the current state of the game.
    board: Board,
    /// The graphical user interface used for rendering the board and handling user input.
    gui: Box<dyn GUI<moves::Action>>
}

impl Game {

    /// Creates a new `Game` instance.
    ///
    /// ## Parameters
    /// - `gui`: A boxed GUI interface  that handles rendering and user interaction.
    ///
    /// ## Returns
    /// - A boxed `Game` instance
    pub fn new(gui: Box<dyn GUI<moves::Action>>) -> Self {
        Self {
            board: Board::new(),
            gui,
        }
    }

    /// Resets the chessboard to its initial state.
    ///
    /// This method clears the board and places all the pieces in their starting positions
    /// for both white and black sides.
    fn reset_board(&mut self){
        self.board = Board::new();
        WHITE_PIECES.into_iter()
            .chain(BLACK_PIECES.into_iter())
            .for_each(|(location, piece)| {
                self.board[location] = Some(piece);
            });
    }

    /// Starts the chess game.
    ///
    /// This method resets the board to its initial state and enters the main game loop,
    /// rendering the board and processing user actions.
    ///
    /// The game continues until a termination condition (e.g., resignation or draw) is met.
    pub fn start(&mut self){
        self.reset_board();
        loop{
            self.gui.render(&self.board);
            let user_action: moves::Action = self.gui.wait_and_process_event();
            match user_action {
                moves::Action::OfferDraw => {}
                moves::Action::Resign => {}
                moves::Action::AcceptDraw => {}
                moves::Action::Move(move_action) => {
                    // Pass action to Move Gen to validate
                    self.board.action(move_action);
                }
                moves::Action::Error => {
                    println!("Error");
                }
                moves::Action::ShowMoveOption(x) => {
                    println!("Called Show on {:?}", x)
                }
            }
        }
    }
}
