use crate::engine::board::board;
use crate::engine::board::location::Location;
use crate::engine::board::pieces::{PieceType, Side};
use crate::engine::move_generator::base::{MoveGenerator, PieceMovementType};
use crate::engine::move_generator::bishop::BishopMoveGen;
use crate::engine::move_generator::king::KingMoveGen;
use crate::engine::move_generator::knight::KnightMoveGen;
use crate::engine::move_generator::pawn::PawnMoveGen;
use crate::engine::move_generator::queen::QueenMoveGen;
use crate::engine::move_generator::rock::RookMoveGen;

// TODO: Organize modules to export only what needed
pub mod base;
pub mod user_actions;
pub(crate) mod threat;
type MoveGeneratorFn = fn(&board::Board, Location, Side) -> Vec<PieceMovementType>;

pub(crate) fn get_move_generator(piece_type: PieceType) -> MoveGeneratorFn {
    match piece_type {
        PieceType::Pawn => PawnMoveGen::generate_moves,
        PieceType::Rook => RookMoveGen::generate_moves,
        PieceType::Knight => KnightMoveGen::generate_moves,
        PieceType::Bishop => BishopMoveGen::generate_moves,
        PieceType::Queen => QueenMoveGen::generate_moves,
        PieceType::King => KingMoveGen::generate_moves,
    }
}
