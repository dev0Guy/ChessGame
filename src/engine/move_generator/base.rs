use crate::engine::board::board;
use crate::engine::board::location::Location;
use crate::engine::board::pieces::{Piece, Side};

#[derive(Debug, PartialEq)]
pub(crate) enum PieceMovementType {
    Relocate(Location),
    Capture(Location),
    // TODO: Maybe Check
}


pub(crate) trait MoveGenerator {
    fn generate_moves(board: &board::Board, loc: Location, side: Side) -> Vec<PieceMovementType>;
}