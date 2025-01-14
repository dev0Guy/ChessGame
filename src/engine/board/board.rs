use crossterm::{
    cursor,
    execute,
    style::{style, Color, PrintStyledContent, StyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use std::ops::{Index, IndexMut};
use crate::engine::board::location::Location;
use crate::engine::board::pieces::{Piece, PieceType, Side};
use crate::engine::movement::moves::Move;

const FILE_NAMES: &'static str = "  A B C D E F G H";

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

    pub fn visualize(&self) {
        let mut stdout = stdout();
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )
            .unwrap();
        println!("{}", FILE_NAMES);

        for (rank, row) in self.0.iter().enumerate() {
            print!("{} ", 8 - rank);

            for cell in row.iter() {
                let styled = Self::styled_symbol(cell);
                execute!(stdout, PrintStyledContent(styled)).unwrap();
                print!(" ");
            }

            println!(" {}", 8 - rank);
        }
        println!("{}", FILE_NAMES);
        stdout.flush().unwrap();
    }

    pub fn action(&mut self, move_action: Move) {
        self[move_action.to] = self[move_action.from];
        self[move_action.from] = None;
    }
}