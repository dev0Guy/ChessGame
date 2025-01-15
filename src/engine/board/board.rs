use std::io;
use std::ops::{Index, IndexMut};
use crossterm::style::{style, Color, StyledContent, Stylize};
use crate::engine::board::location::Location;
use crate::engine::board::pieces::{Piece, PieceType, Side};
use crate::engine::movement::moves::Move;

const FILE_NAMES: &'static str = "   A B C D E F G H";

#[derive(Debug)]
pub struct Board([[Option<Piece>; 8]; 8]);

impl Index<Location> for Board {
    type Output = Option<Piece>;

    fn index(&self, index: Location) -> &Self::Output {
        &self.0[index.rank as usize][index.file as usize]
    }
}

impl IndexMut<Location> for Board {
    fn index_mut(&mut self, index: Location) -> &mut Self::Output {
        &mut self.0[index.rank as usize][index.file as usize]
    }
}


impl Board {
    pub fn new() -> Self {
        Self([[None; 8]; 8])
    }

    fn styled_symbol(cell: &Option<Piece>) -> StyledContent<&'static str> {
        match cell {
            Some(Piece { piece_type: PieceType::King, side: Side::White }) => style("♔").with(Color::White),
            Some(Piece { piece_type: PieceType::King, side: Side::Black }) => style("♚").with(Color::DarkGrey),
            Some(Piece { piece_type: PieceType::Queen, side: Side::White }) => style("♕").with(Color::White),
            Some(Piece { piece_type: PieceType::Queen, side: Side::Black }) => style("♛").with(Color::DarkGrey),
            Some(Piece { piece_type: PieceType::Rook, side: Side::White }) => style("♖").with(Color::White),
            Some(Piece { piece_type: PieceType::Rook, side: Side::Black }) => style("♜").with(Color::DarkGrey),
            Some(Piece { piece_type: PieceType::Bishop, side: Side::White }) => style("♗").with(Color::White),
            Some(Piece { piece_type: PieceType::Bishop, side: Side::Black }) => style("♝").with(Color::DarkGrey),
            Some(Piece { piece_type: PieceType::Knight, side: Side::White }) => style("♘").with(Color::White),
            Some(Piece { piece_type: PieceType::Knight, side: Side::Black }) => style("♞").with(Color::DarkGrey),
            Some(Piece { piece_type: PieceType::Pawn, side: Side::White }) => style("♙").with(Color::White),
            Some(Piece { piece_type: PieceType::Pawn, side: Side::Black }) => style("♟").with(Color::DarkGrey),
            None => style(".").with(Color::Grey),
        }
    }

    pub fn visualize(&self, writer: &mut impl io::Write) {
        writeln!(writer, "{}", FILE_NAMES).unwrap();
        for (rank, row) in self.0.iter().enumerate() {
            write!(writer, "{}|", 8 - rank).unwrap();
            for cell in row.iter() {
                let styled = Self::styled_symbol(cell);
                write!(writer, " {}", styled).unwrap();
            }
            writeln!(writer, "|{}", 8 - rank).unwrap();
        }
        writeln!(writer, "{}", FILE_NAMES).unwrap();
    }

    pub fn action(&mut self, move_action: Move) {
        self[move_action.to] = self[move_action.from];
        self[move_action.from] = None;
    }
}