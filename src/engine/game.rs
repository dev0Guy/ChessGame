use crate::bitboard::BitBoard;
use crate::pieces;
use crate::pieces::common::Color;
use crate::square::{File, Rank, Square};

struct Game {
    pieces_location: [[BitBoard; 6]; 2],
    pieces_square: [[Vec<Square>; 6]; 2],
    pieces_capture_movement: [[BitBoard; 6]; 2],
    pieces_movement: [[BitBoard; 6]; 2],
}


impl Game {
    pub fn new() -> Self {
        let pieces_location = Self::start_position_mask();
        let pieces_threat_attack = [[BitBoard::empty(); 6]; 2];
        let pieces_movement = [[BitBoard::empty(); 6]; 2];
        let pieces_square = Self::start_position();
        let game = Self {
            pieces_location,
            pieces_movement,
            pieces_capture_movement: pieces_threat_attack,
            pieces_square
        };
        // TODO: calculate the threat attack and piece movement
        game
    }
}



impl Game{
    // fn compute_attack_threat_and_move(&mut self) -> [[BitBoard; 6]; 2]{
    //     for side in 0..2{
    //         let opponent_side = 1 - side;
    //         let side_color = match side {
    //             0 => Color::White,
    //             1 => Color::Black,
    //             _ => unreachable!(),
    //         };
    //         for piece in 0..6{
    //             self.pieces_square &= 0;
    //                 for square in self.pieces_square[side][piece]{
    //                 self.pieces_capture_movement[side][piece] = pieces::get_piece(piece).get_moves(
    //                     &self.pieces_location[side][piece],
    //                     square,
    //                     &self.compute_own_pieces(side, piece),
    //                     &self.compute_own_pieces(opponent_side, piece),
    //                     &side_color
    //                 );
    //                 self.pieces_movement[side][piece] = pieces::get_piece(piece).get_moves(
    //                     &self.pieces_location[side][piece],
    //                     square,
    //                     &self.compute_own_pieces(side, piece),
    //                     &self.compute_own_pieces(opponent_side, piece),
    //                     &side_color
    //                 );
    //             }
    //         }
    //     }
    // }

    fn compute_own_pieces(&self, side: usize, piece: usize) -> BitBoard{
        (0..6)
            .into_iter()
            .filter(|(&i)| i != piece)
            .map(|i| self.pieces_location[side][i])
            .fold(BitBoard::empty(), |acc, board| {
                acc | board
            })
    }

}

impl Game{
    fn start_position_mask() -> [[BitBoard; 6]; 2]{
        [
            [
                BitBoard::new(0x0000000000000001), // pawn
                BitBoard::new(0x0000000000000002), // knight
                BitBoard::new(0x0000000000000004), // bishop
                BitBoard::new(0x0000000000000008), // rock
                BitBoard::new(0x0000000000000010), // queen
                BitBoard::new(0x0000000000000020), // king
            ],
            [
                BitBoard::new(0x0000000000000040),
                BitBoard::new(0x0000000000000080),
                BitBoard::new(0x0000000000000100),
                BitBoard::new(0x0000000000000200),
                BitBoard::new(0x0000000000000400),
                BitBoard::new(0x0000000000000800),
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
}
