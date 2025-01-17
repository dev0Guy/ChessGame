use crate::engine::board::board;
use crate::engine::board::location::Location;
use crate::engine::board::pieces::Side;
use crate::engine::move_generator::base::{MoveGenerator, PieceMovementType};

const QUEEN_POSSIBLE_DIRECTIONS: [(i8, i8); 8] = [
    (0, 1), (0, -1), (1, 0), (-1, 0),
    (1, 1), (-1, -1), (1, -1), (-1, 1),
];
pub(crate) struct QueenMoveGen;

impl MoveGenerator for QueenMoveGen {
    fn generate_moves(board: &board::Board, loc: Location, side: Side) -> Vec<PieceMovementType> {
        Self::generate_sliding_moves(board, loc, side, &QUEEN_POSSIBLE_DIRECTIONS)
    }
}