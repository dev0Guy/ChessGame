pub(crate) mod pawn;
pub(crate) mod common;
pub(crate) mod knight;
pub(crate) mod rock;
pub(crate) mod bishop;
pub(crate) mod queen;
pub(crate) mod king;

use strum_macros::EnumIter;
use rock::Rock;
use bishop::Bishop;
use crate::bitboard::BitBoard;
use crate::pieces::common::{Color, PossibleMoves};
use crate::square::Square;

// TODO: create enum for pieces
#[derive(EnumIter, Clone)]
pub(crate) enum Piece{
    Pawn,
    Knight,
    Rock,
    Bishop,
    Queen,
    King,
}


type CaculateFn = fn(&BitBoard, Square, &BitBoard, &BitBoard, &Color) -> BitBoard;

impl Piece{

    pub fn moves_function(&self) -> CaculateFn {
        match self {
            Piece::Pawn => pawn::Pawn::get_moves,
            Piece::Knight => knight::Knight::get_moves,
            Piece::Bishop => Bishop::get_moves,
            Piece::Rock => Rock::get_moves,
            Piece::Queen => queen::Queen::get_moves,
            Piece::King => king::King::get_moves,
        }
    }

    pub fn capture_function(&self) -> CaculateFn {
        match self {
            Piece::Pawn => pawn::Pawn::get_capture,
            Piece::Knight => knight::Knight::get_capture,
            Piece::Bishop => Bishop::get_capture,
            Piece::Rock => Rock::get_capture,
            Piece::Queen => queen::Queen::get_capture,
            Piece::King => king::King::get_capture,
        }
    }
}

impl From<Piece> for usize{
    fn from(value: Piece) -> Self {
        value as usize
    }
}



