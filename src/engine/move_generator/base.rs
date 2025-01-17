use crate::engine::board::board;
use crate::engine::board::location::Location;
use crate::engine::board::pieces::{Piece, Side};

#[derive(Debug, PartialEq)]
pub(crate) enum PieceMovementType {
    Relocate(Location),
    Capture(Location),
    Promotion(Location),
}


pub(crate) trait MoveGenerator {
    fn generate_moves(board: &board::Board, loc: Location, side: Side) -> Vec<PieceMovementType>;

    fn generate_moves_in_direction(
        board: &board::Board,
        loc: Location,
        side: Side,
        dx: i8,
        dy: i8,
    ) -> Vec<PieceMovementType> {
        let mut moves = Vec::new();
        let mut current_loc = loc;
        while let Ok(next_loc) = current_loc.offset(dy, dx) {
            match board[next_loc] {
                Some(piece) if piece.side != side => {
                    moves.push(PieceMovementType::Capture(next_loc));
                    break;
                }
                Some(_) => break,
                None => {
                    moves.push(PieceMovementType::Relocate(next_loc));
                    current_loc = next_loc;
                }
            }
        }
        moves
    }

    fn generate_sliding_moves(
        board: &board::Board,
        loc: Location,
        side: Side,
        directions: &[(i8, i8)],
    ) -> Vec<PieceMovementType> {
        directions
            .iter()
            .flat_map(|&(dx, dy)| Self::generate_moves_in_direction(board, loc, side, dx, dy))
            .collect()
    }
}