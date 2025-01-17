use crate::engine::board::board;
use crate::engine::board::location::Location;
use crate::engine::board::pieces::Side;
use crate::engine::move_generator::base::{MoveGenerator, PieceMovementType};

const ROCK_POSSIBLE_DIRECTIONS: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub(crate) struct RookMoveGen;

impl MoveGenerator for RookMoveGen {
    fn generate_moves(board: &board::Board, loc: Location, side: Side) -> Vec<PieceMovementType> {
        Self::generate_sliding_moves(board, loc, side, &ROCK_POSSIBLE_DIRECTIONS)
    }
}