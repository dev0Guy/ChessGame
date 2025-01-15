mod engine;

use engine::movement::moves::Move;
use engine::board::board::Board;
use crate::engine::board::location::{File, Location, Rank};
use crate::engine::board::pieces::{Piece, PieceType, Side};
use crate::engine::game::Game;
use crate::engine::movement::moves::MoveType;

fn main() {
    println!("Hello, world!");
    let mut game = Game::new();
    game.start();
//     TODO: On Game run make sure each time it get a board position from client
}
