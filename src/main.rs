use crate::bitboard::BitBoard;

mod bitboard;
mod board;
mod square;
mod pieces;
mod engine;

// TODO: when building the game should get attack vector and decrease from king movements
fn main() {
    let x = bitboard::BitBoard::new(0xff);
    let y = square::Rank::Eight;
    println!("{:?}", BitBoard::from(y));

    println!("Hello, world!");
}