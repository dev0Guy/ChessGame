mod bitboard;
mod board;

fn main() {
    let x = bitboard::BitBoard::new(0xff);
    let y = board::Rank::One;
    println!("{:?}", y.as_bitboard());

    println!("Hello, world!");
}