use crate::engine::board::board;
use crate::engine::board::location::Location;
use crate::engine::board::pieces::{Side};

#[derive(Debug, PartialEq)]
pub(crate) enum PieceMovementType {
    Relocate(Location),
    Capture(Location),
    Promotion(Location),
    Castle(Location, Location)
}

impl PieceMovementType {
    /// Extracts the `Location` from the `PieceMovementType`.
    pub fn location(&self) -> Option<Location> {
        match self {
            PieceMovementType::Relocate(val)
            | PieceMovementType::Capture(val)
            | PieceMovementType::Promotion(val) => Some(val.clone()),
            PieceMovementType::Castle(loc1, loc2) => None
        }
    }
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

    fn generate_moves_bitboard(board: &board::Board, loc: Location, side: Side) -> u64 {
        Self::generate_moves(board, loc, side)
            .into_iter()
            .filter_map(|movement| movement.location()) // Extract valid locations
            .fold(0u64, |bitboard, loc| bitboard | (1 << loc.to_bit_index()))
    }

    fn generate_moves_in_direction_bitboard(
        board: &board::Board,
        loc: Location,
        side: Side,
        dx: i8,
        dy: i8,
    ) -> u64 {
        let mut moves_bitboard = 0u64;
        let mut current_loc = loc;

        while let Ok(next_loc) = current_loc.offset(dy, dx) {
            let bit_index = next_loc.to_bit_index();
            match board[next_loc] {
                Some(piece) if piece.side != side => {
                    moves_bitboard |= 1 << bit_index; // Capture
                    break;
                }
                Some(_) => break, // Blocked by same-side piece
                None => {
                    moves_bitboard |= 1 << bit_index; // Empty square
                    current_loc = next_loc;
                }
            }
        }

        moves_bitboard
    }

    fn generate_sliding_moves_bitboard(
        board: &board::Board,
        loc: Location,
        side: Side,
        directions: &[(i8, i8)],
    ) -> u64 {
        directions.iter().fold(0u64, |bitboard, &(dx, dy)| {
            bitboard | Self::generate_moves_in_direction_bitboard(board, loc, side, dx, dy)
        })
    }


    // fn moves_to_bitboard(moves: Vec<PieceMovementType>) -> u64 {
    //     moves
    //         .into_iter()
    //         .fold(0u64, |bitboard, movement| {
    //             bitboard | (1 << movement.location().to_bitboard_index())
    //         })
    // }
}