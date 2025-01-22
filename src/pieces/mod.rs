pub(crate) mod pawn;
pub(crate) mod common;
pub(crate) mod knight;
pub(crate) mod rock;
pub(crate) mod bishop;
pub(crate) mod queen;
pub(crate) mod king;


use rock::Rock;
use bishop::Bishop;
use crate::pieces::common::PossibleMoves;

pub fn get_piece_moves(v: usize){
    // match v {
    //     0 => pawn::Pawn::get_moves,
    //     1 => knight::Knight::get_moves,
    //     2 => Bishop::get_moves,
    //     3 => Rock::get_moves,
    //     4 => queen::Queen::get_moves),
    //     5 => king::King::get_moves),
    //     _ => unreachable!(),
    // }
}