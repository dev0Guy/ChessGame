use crate::{BitBoard};
use crate::square::{Square, File, Rank};
use super::common::{Color, PossibleMoves};

/// Description
/// Slide for each rank/ file.
/// stop movement when capture other piece or blocked by its own piece(exclusive)
struct Rock;

impl PossibleMoves for Rock {
    fn get_moves(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pics: &BitBoard, color: &Color) -> BitBoard {
        todo!()
    }
}



impl Rock{
    fn get_horizontal_moves(){

    }
}