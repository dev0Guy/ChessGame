use crate::bitboard::BitBoard;

mod bitboard;
mod board;
mod square;
mod pieces;

fn main() {
    let x = bitboard::BitBoard::new(0xff);
    let y = square::Rank::Eight;
    println!("{:?}", BitBoard::from(y));

    println!("Hello, world!");
}