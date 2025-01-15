use std::io::{self, Write};
use std::iter::Iterator;
use crossterm::style::{style, Color, StyledContent, Stylize};
use regex::Regex;
use crate::engine::board::board::Board;
use crate::engine::board::location::{Location};
use crate::engine::board::pieces::{Piece, PieceType, Side};
use crate::engine::movement::moves;
use super::base::GUI;

pub struct CommandPromptGUI{
    writer: io::Stdout,
    reader: io::Stdin,
}

const FILE_NAMES_ROW: &'static str = "   A B C D E F G H";
const MOVE_REGEX: &'static str = r"^move\s+([a-h][1-8])\s+([a-h][1-8])$";
const SHOW_REGEX: &'static str = r"^show\s+([a-h][1-8])$";


impl CommandPromptGUI{
    pub fn new() -> Self {
        Self{
            reader: io::stdin(),
            writer: io::stdout(),
        }
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

    fn receive_input(&mut self) -> String {
        let mut input = String::new();
        self.reader
            .read_line(&mut input)
            .unwrap();
        input.trim().to_lowercase()
    }

    fn extract_move(regex: Regex, s: &str) -> moves::Action{
        let caps = regex.captures(s).unwrap();
        let from = caps.get(1).unwrap().as_str();
        let to = caps.get(2).unwrap().as_str();
        let from = Location::from(from).unwrap();
        let to = Location::from(to).unwrap();
        moves::Action::Move(moves::MoveAction::new(from, to, moves::MoveType::Normal))
    }

    fn extract_show(regex: Regex, s: &str) -> moves::Action{
        let caps = regex.captures(s).unwrap();
        let from = caps.get(1).unwrap().as_str();
        let from = Location::from(from).unwrap();
        moves::Action::ShowMoveOption(from)
    }

}

impl GUI<moves::Action> for CommandPromptGUI{
    fn render(&mut self, board: &Board) {
        writeln!(self.writer, "{}", FILE_NAMES_ROW).unwrap();
        for (rank, row) in board.iter().enumerate() {
            write!(self.writer, "{}|", 8 - rank).unwrap();
            for cell in row.iter() {
                let styled = Self::styled_symbol(cell);
                write!(self.writer, " {}", styled).unwrap();
            }
            writeln!(self.writer, "|{}", 8 - rank).unwrap();
        }
        writeln!(self.writer, "{}", FILE_NAMES_ROW).unwrap();
    }


    fn wait_and_process_event(&mut self) -> moves::Action {
        let move_regex = Regex::new(MOVE_REGEX).unwrap();
        let show_regex = Regex::new(SHOW_REGEX).unwrap();
        match self.receive_input().as_str() {
            "quit" | "q" => moves::Action::Resign,
            "draw" => moves::Action::OfferDraw,
            "accept" => moves::Action::AcceptDraw,
            s if show_regex.is_match(s) => Self::extract_show(show_regex, s),
            s if move_regex.is_match(s) => Self::extract_move(move_regex, s),
            _ => moves::Action::Error,
        }
    }
}