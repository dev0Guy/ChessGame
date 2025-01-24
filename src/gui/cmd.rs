use std::io::{Write, self};
use crossterm::style::{self, style, StyledContent, Stylize};
use regex::Regex;
use crate::pieces::common::Color;
use crate::pieces::Piece;
use crate::square::{Square};

pub struct CommandPromptGUI{
    writer: io::Stdout,
    reader: io::Stdin,
}

const FILE_NAMES_ROW: &'static str = "   A B C D E F G H";
const MOVE_REGEX: &'static str = r"^move\s+([a-h][1-8])\s+([a-h][1-8])$";
const SHOW_REGEX: &'static str = r"^show\s+([a-h][1-8])$";

impl CommandPromptGUI{
    pub fn render(&mut self, board: &[(Option<Piece>); 64], side: Color) {
        writeln!(self.writer, "{}", FILE_NAMES_ROW).unwrap();
        for (idx, piece) in board.iter().enumerate() {
            let [rank_idx, file_idx] = [idx / 8, idx % 8];
            let styled = Self::styled_symbol(piece, side);
            if file_idx == 0 {
                write!(self.writer, "{}|", 8 - rank_idx).unwrap();
            }
            write!(self.writer, " {}", styled).unwrap();
            if file_idx == 7{
                writeln!(self.writer, "|{}", 8- rank_idx).unwrap();
            }
        }
        writeln!(self.writer, "{}", FILE_NAMES_ROW).unwrap();
        write!(self.writer, "{:?} Turn:", side).unwrap();
        self.writer.flush().unwrap();
    }

    pub fn wait_and_process_event(&mut self) -> Option<(Square, Square)> {
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
                "quit" | "q" => panic!(),
                "draw" =>  panic!(),
                "accept" =>  panic!(),
                // s if show_regex.is_match(s) => return Self::extract_show(show_regex, s),
                s if move_regex.is_match(s) => return Some(Self::extract_move(move_regex, s)),
                _ => {
                    writeln!(self.writer, "Invalid command, {}", &user_action).unwrap();
                    self.show_help_information();
                    continue;
                }
            }
        }

    }
}



impl CommandPromptGUI {
    pub fn new() -> Self {
        Self {
            reader: io::stdin(),
            writer: io::stdout(),
        }
    }

    fn receive_input(&mut self) -> String {
        let mut input = String::new();
        self.reader
            .read_line(&mut input)
            .unwrap();
        input.trim().to_lowercase()
    }

    fn extract_move(regex: Regex, s: &str) -> (Square, Square) {
        let caps = regex.captures(s).unwrap();
        let from = caps.get(1).unwrap().as_str().to_string();
        let to = caps.get(2).unwrap().as_str().to_string();
        let from = Square::try_from(from).unwrap();
        let to = Square::try_from(to).unwrap();
        (from, to)
    }

    fn styled_symbol(piece: &Option<Piece>, color: Color) -> StyledContent<&'static str> {
        match (piece, color) {
            (Some(Piece::King), Color::White) => style::style("♔").with(style::Color::White),
            (Some(Piece::King), Color::Black) => style::style("♚").with(style::Color::DarkGrey),
            (Some(Piece::Queen), Color::White) => style::style("♕").with(style::Color::White),
            (Some(Piece::Queen), Color::Black) => style::style("♛").with(style::Color::DarkGrey),
            (Some(Piece::Rock), Color::White) => style::style("♖").with(style::Color::White),
            (Some(Piece::Rock), Color::Black) => style::style("♜").with(style::Color::DarkGrey),
            (Some(Piece::Bishop), Color::White) => style::style("♗").with(style::Color::White),
            (Some(Piece::Bishop), Color::Black) => style::style("♝").with(style::Color::DarkGrey),
            (Some(Piece::Knight), Color::White) => style::style("♘").with(style::Color::White),
            (Some(Piece::Knight), Color::Black) => style::style("♞").with(style::Color::DarkGrey),
            (Some(Piece::Pawn), Color::White) => style::style("♙").with(style::Color::White),
            (Some(Piece::Pawn), Color::Black) => style::style("♟").with(style::Color::DarkGrey),
            _ => style("□").with(style::Color::Grey),
        }
    }

    fn show_help_information(&mut self) {
        writeln!(self.writer, "=====================================").unwrap();
        writeln!(self.writer, "       Available commands:").unwrap();
        writeln!(self.writer, "       help, quit, draw, accept").unwrap();
        writeln!(self.writer, "       move <from> <to>").unwrap();
        writeln!(self.writer, "       show <from>").unwrap();
        writeln!(self.writer, "=====================================").unwrap();
    }
}