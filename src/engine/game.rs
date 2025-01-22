use strum::IntoEnumIterator;
use crate::bitboard::BitBoard;
use crate::pieces;
use crate::pieces::common::Color;
use crate::pieces::Piece;
use crate::square::{File, Rank, Square};

pub(crate) struct Game {
    pieces_location: [[BitBoard; 6]; 2],
    pieces_square: [[Vec<Square>; 6]; 2],
    pieces_capture_movement: [[BitBoard; 6]; 2],
    pieces_movement: [[BitBoard; 6]; 2],
}


impl Game {
    pub fn new() -> Self {
        let pieces_location = Self::start_position_mask();
        let pieces_capture_movement = [[BitBoard::empty(); 6]; 2];
        let pieces_movement = [[BitBoard::empty(); 6]; 2];
        let pieces_square = Self::start_position();
        let mut game = Self {
            pieces_location,
            pieces_movement,
            pieces_capture_movement,
            pieces_square
        };
        game.compute_attack_threat_and_move();
        game
    }


    // pub fn move_piece(
    //     &mut self,
    //     color: Color,
    //     piece: usize,
    //     from: Square,
    //     to: Square,
    // ) -> Result<(), String> {
    //     let side_idx = usize::from(color);
    //     let from_bitboard = BitBoard::from(from);
    //     let to_bitboard = BitBoard::from(to);
    //     let legal_moves_for_piece = self.pieces_movement[side_idx][piece] | self.pieces_capture_movement[side_idx][piece];
    //     let is_piece_in_from_location = !(self.pieces_location[side_idx][piece] & from_bitboard).is_empty();
    //     let is_move_is_acceptable = !(legal_moves_for_piece & to_bitboard).is_empty();
    //
    //     if !is_piece_in_from_location {
    //         return Err(format!("No piece of type {} at {:?}", piece, from));
    //     }
    //     if !is_move_is_acceptable{
    //         return Err(format!("Movement is not possible for piece {}", piece));
    //     }
    //     self.compute_own_pieces(side_idx);
    //     Ok(())
    // }
}

impl Game{
    fn start_position_mask() -> [[BitBoard; 6]; 2]{
        // TODO: fix to include all
        [
            [
                BitBoard::new(0xff00), // pawn
                BitBoard::new(0x42), // knight
                BitBoard::new(0x24), // bishop
                BitBoard::new(0x81), // rock
                BitBoard::new(0x8), // queen
                BitBoard::new(0x10), // king
            ],
            [
                BitBoard::new(0xff000000000000),
                BitBoard::new(0x4200000000000000),
                BitBoard::new(0x2400000000000000),
                BitBoard::new(0x8100000000000000),
                BitBoard::new(0x800000000000000),
                BitBoard::new(0x1000000000000000),
            ]
        ]
    }

    fn start_position() -> [[Vec<Square>; 6]; 2]{
        [
            [
                vec![
                    Square::new(File::A, Rank::Two),
                    Square::new(File::B, Rank::Two),
                    Square::new(File::C, Rank::Two),
                    Square::new(File::D, Rank::Two),
                    Square::new(File::E, Rank::Two),
                    Square::new(File::F, Rank::Two),
                    Square::new(File::G, Rank::Two),
                    Square::new(File::H, Rank::Two),
                ],
                vec![
                    Square::new(File::B, Rank::One),
                    Square::new(File::G, Rank::One),
                ],
                vec![
                    Square::new(File::C, Rank::One),
                    Square::new(File::F, Rank::One),
                ],
                vec![
                    Square::new(File::A, Rank::One),
                    Square::new(File::H, Rank::One),
                ],
                vec![
                    Square::new(File::D, Rank::One),
                ],
                vec![
                    Square::new(File::E, Rank::One),
                ]
            ],
            [
                vec![
                    Square::new(File::A, Rank::Seven),
                    Square::new(File::B, Rank::Seven),
                    Square::new(File::C, Rank::Seven),
                    Square::new(File::D, Rank::Seven),
                    Square::new(File::E, Rank::Seven),
                    Square::new(File::F, Rank::Seven),
                    Square::new(File::G, Rank::Seven),
                    Square::new(File::H, Rank::Seven),
                ],
                vec![
                    Square::new(File::B, Rank::Eight),
                    Square::new(File::G, Rank::Eight),
                ],
                vec![
                    Square::new(File::C, Rank::Eight),
                    Square::new(File::F, Rank::Eight),
                ],
                vec![
                    Square::new(File::A, Rank::Eight),
                    Square::new(File::H, Rank::Eight),
                ],
                vec![
                    Square::new(File::D, Rank::Eight),
                ],
                vec![
                    Square::new(File::E, Rank::Eight),
                ]
            ]
        ]
    }

    fn combine(board: &[BitBoard; 6]) -> BitBoard{
        (Piece::iter())
            .into_iter()
            .map(|i| board[usize::from(i)])
            .fold(BitBoard::empty(), |acc, board| {
                acc | board
            })
    }

    fn compute_attack_threat_and_move(&mut self){
        for side in Color::iter(){
            let opponent_side = side.opposite();
            let side_index = usize::from(side);
            let opponent_index = usize::from(opponent_side);
            for piece in Piece::iter(){
                let piece_idx = usize::from(piece.clone());
                let own_pieces =  &Self::combine(&self.pieces_location[side_index]);
                let opponent_pieces = &Self::combine(&self.pieces_location[opponent_index]);
                for square in &self.pieces_square[side_index][piece_idx]{
                    let square = square.clone();
                    self.pieces_movement[side_index][piece_idx] |= piece.moves_function()(
                        &self.pieces_location[side_index][piece_idx],
                        square,
                        own_pieces,
                        opponent_pieces,
                        &side
                    );
                    self.pieces_capture_movement[side_index][piece_idx] |=  piece.capture_function()(
                        &self.pieces_location[side_index][piece_idx],
                        square,
                        own_pieces,
                        opponent_pieces,
                        &side
                    );
                }
            }
        };
    }

    fn is_checked(opponent_king: &BitBoard, pieces_capture: &[BitBoard; 6]) -> bool{
        let attack = Self::combine(&pieces_capture);
        !(attack & *opponent_king).is_empty()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bitboard::BitBoard;

    #[test]
    fn test_game_initialization_white() {
        let game = Game::new();
        // for white
        let rocks = BitBoard::new(0x81);
        let pawns = BitBoard::new(0xff00);
        let knight = BitBoard::new(0x42);
        let bishops = BitBoard::new(0x24);
        let king = BitBoard::new(0x10);
        let queen = BitBoard::new(0x8);
        assert_eq!(game.pieces_location[0][0], pawns);
        assert_eq!(game.pieces_location[0][1], knight);
        assert_eq!(game.pieces_location[0][2], bishops);
        assert_eq!(game.pieces_location[0][3], rocks);
        assert_eq!(game.pieces_location[0][4], queen);
        assert_eq!(game.pieces_location[0][5], king);
        // movement
        assert_eq!(game.pieces_movement[0][0], BitBoard::new(0xffff0000));
        assert_eq!(game.pieces_movement[0][1], BitBoard::new(0xa50000));
        assert_eq!(game.pieces_movement[0][2], BitBoard::empty());
        assert_eq!(game.pieces_movement[0][3], BitBoard::empty());
        assert_eq!(game.pieces_movement[0][4], BitBoard::empty());
        assert_eq!(game.pieces_movement[0][5], BitBoard::empty());
        // capture
        assert_eq!(game.pieces_capture_movement[0][0], BitBoard::empty());
        assert_eq!(game.pieces_capture_movement[0][1], BitBoard::new(0xa50000));
        assert_eq!(game.pieces_capture_movement[0][2], BitBoard::empty());
        assert_eq!(game.pieces_capture_movement[0][3], BitBoard::empty());
        assert_eq!(game.pieces_capture_movement[0][4], BitBoard::empty());
        assert_eq!(game.pieces_capture_movement[0][5], BitBoard::empty());
    }

    #[test]
    fn test_king_check_rock() {

        let mut piece_capture = [BitBoard::empty(); 6];
        piece_capture[usize::from(Piece::Rock)] = BitBoard::new(0x80);
        let king = BitBoard::new(0x8);

        let result = Game::is_checked(&king, &piece_capture);
        println!("{}", result)
    }
}


