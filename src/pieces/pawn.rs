use crate::square::{Rank, File};
use crate::bitboard::BitBoard;
use super::common::PossibleMoves;


/// Description
/// - (White) pawn can make a single step forward (UP) for each rank expect 8 rank and only if free.
/// - (White) pawn can make a double step forward (UP) to only  rank 4 and only if free.
/// - (White) pawn can capture (LEFT) for all file expect rank 8 to all file expect H and if not empty with enemy piece
/// - (White) pawn can capture (RIGHT) for all file expect rank 8 to all file expect A and if not empty with enemy piece
/// - (White) pawn promotion: one square forward or a diagonal capture results in it landing on rank 8
/// - (Black) pawn can make a single step forward (DOWN) for each rank expect 1 rank and only if free.
/// - (Black) pawn can make a double step forward (DOWN) to only  rank 5 and only if free.
/// - (Black) pawn can capture (LEFT) for all file expect rank 1 to all file expect H and if not empty with enemy piece
/// - (Black) pawn can capture (RIGHT) for all file expect rank 1 to all file expect A and if not empty with enemy piece
/// - (BLACK) pawn promotion: one square forward or a diagonal capture results in it landing on rank 1
struct Pawn;


impl PossibleMoves for Pawn {
    fn get_moves(own_pieces: &BitBoard, opponent_pics: &BitBoard) -> BitBoard {

        todo!()
    }
}



