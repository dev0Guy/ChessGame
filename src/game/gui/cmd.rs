use std::io;
use std::io::Write;
use crossterm::style::{style, Color, StyledContent, Stylize};
use regex::Regex;
use crate::engine::board::pieces::{Piece, PieceType, Side};
use crate::game::bitboard::BitBoard;
use crate::game::{actions, Position};
use crate::game::position::{File, Rank};
use super::base::GUI;

pub struct CommandPromptGUI{
    writer: io::Stdout,
    reader: io::Stdin,
}

const FILE_NAMES_ROW: &'static str = "   A B C D E F G H";
const MOVE_REGEX: &'static str = r"^move\s+([a-h][1-8])\s+([a-h][1-8])$";
const SHOW_REGEX: &'static str = r"^show\s+([a-h][1-8])$";

impl GUI<actions::Action> for CommandPromptGUI{
    fn render(&mut self, board: &BitBoard, side: Side, pos: Option<Position>){
        writeln!(self.writer, "{}", FILE_NAMES_ROW).unwrap();
        for (rank, row) in board.iter().enumerate() {
            write!(self.writer, "{}|", 8 - rank).unwrap();
            for (file, cell) in row.iter().enumerate() {
                let mut styled = Self::styled_symbol(cell);
                let file = File::from(file);
                let rank = Rank::from(7 - rank);
                write!(self.writer, " {}", styled).unwrap();
            }
            writeln!(self.writer, "|{}", 8 - rank).unwrap();
        }
        writeln!(self.writer, "{}", FILE_NAMES_ROW).unwrap();
        write!(self.writer, "{:?} Turn:", side).unwrap();
        self.writer.flush().unwrap();
    }

    fn wait_and_process_event(&mut self) -> actions::Action {
        let move_regex = Regex::new(MOVE_REGEX).unwrap();
        let show_regex = Regex::new(SHOW_REGEX).unwrap();
        loop {
            let binding = self.receive_input();
            let user_action = binding.as_str();
            match user_action {
                "help" | "h" => {
                    self.show_help_information();
                    continue;
                },
                "quit" | "q" => return actions::Action::Resign,
                "draw" => return actions::Action::OfferDraw,
                "accept" => return actions::Action::AcceptDraw,
                // s if show_regex.is_match(s) => return Self::extract_show(show_regex, s),
                s if move_regex.is_match(s) => return Self::extract_move(move_regex, s),
                _ => {
                    writeln!(self.writer, "Invalid command, {}", &user_action).unwrap();
                    self.show_help_information();
                    continue;
                }
            }
        }

    }
}


impl CommandPromptGUI{
    pub fn new() -> Self {
        Self{
            reader: io::stdin(),
            writer: io::stdout(),
        }
    }

    fn styled_symbol(cell: &Option<Piece>) -> StyledContent<&'static str> {
        match cell {
            Some(Piece { piece_type: PieceType::King, side: Side::White , ..}) => style("♔").with(Color::White),
            Some(Piece { piece_type: PieceType::King, side: Side::Black , .. }) => style("♚").with(Color::DarkGrey),
            Some(Piece { piece_type: PieceType::Queen, side: Side::White , .. }) => style("♕").with(Color::White),
            Some(Piece { piece_type: PieceType::Queen, side: Side::Black , .. }) => style("♛").with(Color::DarkGrey),
            Some(Piece { piece_type: PieceType::Rook, side: Side::White , ..}) => style("♖").with(Color::White),
            Some(Piece { piece_type: PieceType::Rook, side: Side::Black , ..}) => style("♜").with(Color::DarkGrey),
            Some(Piece { piece_type: PieceType::Bishop, side: Side::White , .. }) => style("♗").with(Color::White),
            Some(Piece { piece_type: PieceType::Bishop, side: Side::Black , .. }) => style("♝").with(Color::DarkGrey),
            Some(Piece { piece_type: PieceType::Knight, side: Side::White , .. }) => style("♘").with(Color::White),
            Some(Piece { piece_type: PieceType::Knight, side: Side::Black , .. }) => style("♞").with(Color::DarkGrey),
            Some(Piece { piece_type: PieceType::Pawn, side: Side::White , .. }) => style("♙").with(Color::White),
            Some(Piece { piece_type: PieceType::Pawn, side: Side::Black , .. }) => style("♟").with(Color::DarkGrey),
            None => style("□").with(Color::Grey),
        }
    }

    // fn receive_input(&mut self) -> String {
    //     let mut input = String::new();
    //     self.reader
    //         .read_line(&mut input)
    //         .unwrap();
    //     input.trim().to_lowercase()
    // }
    //
    fn extract_move(regex: Regex, s: &str) -> actions::Action{
        let caps = regex.captures(s).unwrap();
        let from = caps.get(1).unwrap().as_str();
        let to = caps.get(2).unwrap().as_str();
        actions::Action::Move(actions::MoveAction::new(from, to))
    }
    //
    // fn extract_show(regex: Regex, s: &str) -> user_actions::Action{
    //     todo!()
    //     // let caps = regex.captures(s).unwrap();
    //     // let from = caps.get(1).unwrap().as_str();
    //     // let from = Location::from(from).unwrap();
    //     // user_actions::Action::ShowMoveOption(from)
    // }

    fn show_help_information(&mut self){
        writeln!(self.writer ,"=====================================").unwrap();
        writeln!(self.writer ,"       Available commands:").unwrap();
        writeln!(self.writer ,"       help, quit, draw, accept").unwrap();
        writeln!(self.writer ,"       move <from> <to>").unwrap();
        writeln!(self.writer ,"       show <from>").unwrap();
        writeln!(self.writer ,"=====================================").unwrap();
    }

}