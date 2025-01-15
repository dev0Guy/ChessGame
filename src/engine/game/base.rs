use std::fmt::Debug;
use crate::engine::board::board::Board;
use crate::engine::board::pieces::{Piece, PieceType, Side};
use crate::engine::board::location::{File, Location, Rank};
use crate::engine::gui::base::GUI;
use crate::engine::movement::moves;

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

const WHITE_PIECES: [(Location, Piece); 16] = get_location_by_side(Side::White);
const BLACK_PIECES: [(Location, Piece); 16] = get_location_by_side(Side::Black);


pub struct Game{
    board: Board,
    gui: Box<dyn GUI<moves::Action>>
}

impl Game {

    pub fn new(gui: Box<dyn GUI<moves::Action>>) -> Box<Self> {
        Box::new(Self {
            board: Board::new(),
            gui,
        })
    }

    fn reset_board(&mut self){
        self.board = Board::new();
        WHITE_PIECES.into_iter()
            .chain(BLACK_PIECES.into_iter())
            .for_each(|(location, piece)| {
                self.board[location] = Some(piece);
            });
    }


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
