use crate::bitboard::BitBoard;
use crate::engine::game;

mod bitboard;
mod square;
mod pieces;
mod engine;
mod gui;

fn main() {
    let mut game = game::Game::new();
    game.start();
}