use strum::IntoEnumIterator;
use super::bitset::BoardBitSet;
use crate::engine::board::pieces::{PieceType, Side};
use crate::game::position::{File, Rank, Position};
use super::moves;

pub(crate) struct BitBoard {
    /// for each side and piece save movement for each cell
    piece_movement: [[[BoardBitSet; 64] ;6] ;2],
    /// for each side and piece save capture movement for each cell
    piece_capture_movement: [[[BoardBitSet; 64] ;6] ;2],
    /// piece location
    piece_location: [[BoardBitSet; 6] ;2]
}


impl BitBoard{
    pub(crate) fn new()-> Self{
        let mut piece_movement: [[[BoardBitSet; 64]; 6]; 2] = [[[BoardBitSet::empty(); 64]; 6]; 2];
        let mut piece_capture_movement: [[[BoardBitSet; 64]; 6]; 2] = [[[BoardBitSet::empty(); 64]; 6]; 2];
        let piece_location = [initialize_piece_locations(Side::White), initialize_piece_locations(Side::Black)];
        for side in [Side::White, Side::Black]{
            for piece in PieceType::iter(){
                piece_movement[usize::from(side)][usize::from(piece)] = compute_for_all_board(
                    piece,
                    side,
                    moves::get_piece_moves
                );
                piece_capture_movement[usize::from(side)][usize::from(piece)] = compute_for_all_board(
                    piece,
                    side,
                    moves::get_piece_capture_movement
                );
            }
        }
        Self{piece_movement, piece_capture_movement, piece_location}
    }



}

fn initialize_piece_locations(side: Side) -> [BoardBitSet; 6] {

    let mut piece_location = [BoardBitSet::empty(); 6];

    let back_rank = match side {
        Side::White => Rank::One,
        Side::Black => Rank::Eight,
    };

    let pawn_rank = match side {
        Side::White => Rank::Two,
        Side::Black => Rank::Seven,
    };

    piece_location[0] = BoardBitSet::place_at(Position::new(File::E, back_rank));

    piece_location[1] = BoardBitSet::place_at(Position::new(File::D, back_rank));

    piece_location[2] = BoardBitSet::place_at(Position::new(File::A, back_rank))
        | BoardBitSet::place_at(Position::new(File::H, back_rank));

    piece_location[3] = BoardBitSet::place_at(Position::new(File::C, back_rank))
        | BoardBitSet::place_at(Position::new(File::F, back_rank));

    piece_location[4] = BoardBitSet::place_at(Position::new(File::B, back_rank))
        | BoardBitSet::place_at(Position::new(File::G, back_rank));

    for file in File::iter() {
        piece_location[5] |= BoardBitSet::place_at(Position::new(file, pawn_rank));
    }

    piece_location
}


fn compute_for_all_board(
    p: PieceType,
    s: Side,
    func: fn(PieceType, &Position, Side) -> Vec<Position>,
) -> [BoardBitSet; 64]{
    let mut bitsets = [BoardBitSet::empty(); 64];
    for file in File::iter(){
        for rank in Rank::iter(){
            let [file_idx, rank_idx]= [usize::from(file), usize::from(rank)];
            let pos = Position::new(file, rank);
            bitsets[pos.position_bitboard_index()] = BoardBitSet::place_multiple_at(func(p, &pos, s));
        }
    }
    bitsets
}


impl From<Side> for usize {
    fn from(color: Side) -> Self {
        match color {
            Side::White => 0,
            Side::Black => 1,
        }
    }
}

impl From<PieceType> for usize {
    fn from(piece: PieceType) -> Self {
        match piece {
            PieceType::King => 0,
            PieceType::Queen => 1,
            PieceType::Rook => 2,
            PieceType::Bishop => 3,
            PieceType::Knight => 4,
            PieceType::Pawn => 5,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::board::pieces::{PieceType, Side};
    use crate::game::position::{File, Rank, Position};

    #[test]
    fn test_bitboard_initialization() {
        let bitboard = BitBoard::new();

        for side in [Side::White, Side::Black] {
            for piece in PieceType::iter() {
                let movement = &bitboard.piece_movement[usize::from(side)][usize::from(piece)];
                let capture_movement = &bitboard.piece_capture_movement[usize::from(side)][usize::from(piece)];
                assert_eq!(movement.len(), 64, "Invalid movement length for side {:?} piece {:?}", side, piece);
                assert_eq!(capture_movement.len(), 64, "Invalid capture movement length for side {:?} piece {:?}", side, piece);
            }
        }
    }

    #[test]
    fn test_compute_for_all_board() {
        let func = |_: PieceType, pos: &Position, _: Side| -> Vec<Position> {
            vec![Position::new(pos.file, pos.rank)]
        };

        let bitsets = compute_for_all_board(PieceType::King, Side::White, func);

        for file in File::iter() {
            for rank in Rank::iter() {
                let pos = Position::new(file, rank);
                let index = pos.position_bitboard_index();
                assert!(bitsets[index].is_set(pos), "Bitset does not contain position {:?}", pos);
            }
        }
    }

    #[test]
    fn test_from_side_and_piece() {
        assert_eq!(usize::from(Side::White), 0);
        assert_eq!(usize::from(Side::Black), 1);

        assert_eq!(usize::from(PieceType::King), 0);
        assert_eq!(usize::from(PieceType::Queen), 1);
        assert_eq!(usize::from(PieceType::Rook), 2);
        assert_eq!(usize::from(PieceType::Bishop), 3);
        assert_eq!(usize::from(PieceType::Knight), 4);
        assert_eq!(usize::from(PieceType::Pawn), 5);
    }

    #[test]
    fn test_initialize_piece_locations() {
        let white_locations = initialize_piece_locations(Side::White);
        let black_locations = initialize_piece_locations(Side::Black);

        assert!(white_locations[0].is_set(Position::new(File::E, Rank::One)));
        assert!(white_locations[1].is_set(Position::new(File::D, Rank::One)));
        assert!(white_locations[2].is_set(Position::new(File::A, Rank::One)));
        assert!(white_locations[2].is_set(Position::new(File::H, Rank::One)));
        assert!(white_locations[5].is_set(Position::new(File::A, Rank::Two)));
        assert!(white_locations[5].is_set(Position::new(File::H, Rank::Two)));

        assert!(black_locations[0].is_set(Position::new(File::E, Rank::Eight)));
        assert!(black_locations[1].is_set(Position::new(File::D, Rank::Eight)));
        assert!(black_locations[2].is_set(Position::new(File::A, Rank::Eight)));
        assert!(black_locations[2].is_set(Position::new(File::H, Rank::Eight)));
        assert!(black_locations[5].is_set(Position::new(File::A, Rank::Seven)));
        assert!(black_locations[5].is_set(Position::new(File::H, Rank::Seven)));
    }

    #[test]
    fn test_bitboard_new() {
        let bitboard = BitBoard::new();

        assert!(bitboard.piece_location[0][0].is_set(Position::new(File::E, Rank::One)));
        assert!(bitboard.piece_location[1][0].is_set(Position::new(File::E, Rank::Eight)));

        assert!(bitboard.piece_location[0][5].is_set(Position::new(File::A, Rank::Two)));
    }
}

