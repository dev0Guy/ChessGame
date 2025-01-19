use crate::engine::board::pieces::{PieceType, Side};
use crate::game::Position;

mod common;
pub mod rock;
pub mod bishop;
mod queen;
mod knight;
mod king;
mod pawn;

use rock::RockMoves;
use bishop::BishopMoves;
use queen::QueenMoves;
use knight::KnightMoves;
use pawn::PawnMoves;
use king::KingMoves;
use common::{AttackMoveOptions, RegularMoveOptions};

pub fn get_piece_moves(
    piece_type: PieceType,
    pos: &Position,
    side: Side,
) -> Vec<Position> {
    match piece_type {
        PieceType::Rook => RockMoves::move_options(pos, side).collect(),
        PieceType::Bishop => BishopMoves::move_options(pos, side).collect(),
        PieceType::Queen => QueenMoves::move_options(pos, side).collect(),
        PieceType::Knight => KnightMoves::move_options(pos, side).collect(),
        PieceType::King => KingMoves::move_options(pos, side).collect(),
        PieceType::Pawn => PawnMoves::move_options(pos, side).collect(),
    }
}

pub fn get_piece_attack(
    piece_type: PieceType,
    pos: &Position,
    side: Side,
) -> Vec<Position> {
    match piece_type {
        PieceType::Rook => RockMoves::attack_options(pos, side).collect(),
        PieceType::Bishop => BishopMoves::attack_options(pos, side).collect(),
        PieceType::Queen => QueenMoves::attack_options(pos, side).collect(),
        PieceType::Knight => KnightMoves::attack_options(pos, side).collect(),
        PieceType::King => KingMoves::attack_options(pos, side).collect(),
        PieceType::Pawn => PawnMoves::attack_options(pos, side).collect(),
    }
}
