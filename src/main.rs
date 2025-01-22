use crate::bitboard::BitBoard;
use crate::engine::game;

mod bitboard;
mod board;
mod square;
mod pieces;
mod engine;

// TODO: when building the game should get attack vector and decrease from king movements
fn main() {
    let game = game::Game::new();
    // println!("{:?}", BitBoard::new(0x10204081020));
    // println!("{:?}", BitBoard::from(Bishop::get_diagonal_mask(Square::new(File::D, Rank::Two))));
}