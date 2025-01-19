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
use crate::engine::board::pieces::{PieceType, Side};
use super::position::Position;

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



#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::position::{File, Rank};
    use crate::engine::board::pieces::Side;
    use std::collections::HashSet;

    #[test]
    fn test_rook_moves() {
        let pos = Position::new(File::D, Rank::Four);
        let expected: HashSet<_> = vec![
            Position::new(File::A, Rank::Four),
            Position::new(File::B, Rank::Four),
            Position::new(File::C, Rank::Four),
            Position::new(File::E, Rank::Four),
            Position::new(File::F, Rank::Four),
            Position::new(File::G, Rank::Four),
            Position::new(File::H, Rank::Four),
            Position::new(File::D, Rank::One),
            Position::new(File::D, Rank::Two),
            Position::new(File::D, Rank::Three),
            Position::new(File::D, Rank::Five),
            Position::new(File::D, Rank::Six),
            Position::new(File::D, Rank::Seven),
            Position::new(File::D, Rank::Eight),
        ]
            .into_iter()
            .collect();

        let result: HashSet<_> = get_piece_moves(PieceType::Rook, &pos, Side::White).into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_bishop_moves() {
        let pos = Position::new(File::D, Rank::Four);
        let expected: HashSet<_> = vec![
            Position::new(File::E, Rank::Five),
            Position::new(File::F, Rank::Six),
            Position::new(File::G, Rank::Seven),
            Position::new(File::H, Rank::Eight),
            Position::new(File::C, Rank::Three),
            Position::new(File::B, Rank::Two),
            Position::new(File::A, Rank::One),
            Position::new(File::C, Rank::Five),
            Position::new(File::B, Rank::Six),
            Position::new(File::A, Rank::Seven),
            Position::new(File::E, Rank::Three),
            Position::new(File::F, Rank::Two),
            Position::new(File::G, Rank::One),
        ]
            .into_iter()
            .collect();

        let result: HashSet<_> = get_piece_moves(PieceType::Bishop, &pos, Side::White).into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_queen_moves() {
        let pos = Position::new(File::D, Rank::Four);
        let expected: HashSet<_> = vec![
            Position::new(File::A, Rank::Four),
            Position::new(File::B, Rank::Four),
            Position::new(File::C, Rank::Four),
            Position::new(File::E, Rank::Four),
            Position::new(File::F, Rank::Four),
            Position::new(File::G, Rank::Four),
            Position::new(File::H, Rank::Four),
            Position::new(File::D, Rank::One),
            Position::new(File::D, Rank::Two),
            Position::new(File::D, Rank::Three),
            Position::new(File::D, Rank::Five),
            Position::new(File::D, Rank::Six),
            Position::new(File::D, Rank::Seven),
            Position::new(File::D, Rank::Eight),
            Position::new(File::E, Rank::Five),
            Position::new(File::F, Rank::Six),
            Position::new(File::G, Rank::Seven),
            Position::new(File::H, Rank::Eight),
            Position::new(File::C, Rank::Three),
            Position::new(File::B, Rank::Two),
            Position::new(File::A, Rank::One),
            Position::new(File::C, Rank::Five),
            Position::new(File::B, Rank::Six),
            Position::new(File::A, Rank::Seven),
            Position::new(File::E, Rank::Three),
            Position::new(File::F, Rank::Two),
            Position::new(File::G, Rank::One),
        ]
            .into_iter()
            .collect();

        let result: HashSet<_> = get_piece_moves(PieceType::Queen, &pos, Side::White).into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_pawn_moves() {
        let pos = Position::new(File::D, Rank::Two);
        let expected: HashSet<_> = vec![
            Position::new(File::D, Rank::Three),
            Position::new(File::D, Rank::Four),
        ]
            .into_iter()
            .collect();

        let result: HashSet<_> = get_piece_moves(PieceType::Pawn, &pos, Side::White).into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_king_moves() {
        let pos = Position::new(File::D, Rank::Four);
        let expected: HashSet<_> = vec![
            Position::new(File::E, Rank::Four),
            Position::new(File::E, Rank::Five),
            Position::new(File::D, Rank::Five),
            Position::new(File::C, Rank::Five),
            Position::new(File::C, Rank::Four),
            Position::new(File::C, Rank::Three),
            Position::new(File::D, Rank::Three),
            Position::new(File::E, Rank::Three),
        ]
            .into_iter()
            .collect();

        let result: HashSet<_> = get_piece_moves(PieceType::King, &pos, Side::White).into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_knight_moves() {
        let pos = Position::new(File::D, Rank::Four);
        let expected: HashSet<_> = vec![
            Position::new(File::E, Rank::Six),
            Position::new(File::F, Rank::Five),
            Position::new(File::F, Rank::Three),
            Position::new(File::E, Rank::Two),
            Position::new(File::C, Rank::Two),
            Position::new(File::B, Rank::Three),
            Position::new(File::B, Rank::Five),
            Position::new(File::C, Rank::Six),
        ]
            .into_iter()
            .collect();

        let result: HashSet<_> = get_piece_moves(PieceType::Knight, &pos, Side::White).into_iter().collect();
        assert_eq!(result, expected);
    }
}