use strum::IntoEnumIterator;
use crate::bitboard::BitBoard;
use crate::gui::cmd::CommandPromptGUI;
use crate::pieces;
use crate::pieces::common::Color;
use crate::pieces::Piece;
use crate::square::{File, Rank, Square};

pub(crate) struct Game {
    gui: CommandPromptGUI,
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
        let gui = CommandPromptGUI::new();
        let mut game = Self {
            gui,
            pieces_location,
            pieces_movement,
            pieces_capture_movement,
            pieces_square
        };
        game.compute_attack_threat_and_move();
        game
    }

    pub fn start(&mut self){
        let board_position = self.get_all_position();
        self.gui.render(&board_position, Color::White);
        self.gui.wait_and_process_event();
    }


    /// Check if move is possible
    /// Firstly check if move inside the move list. If do try move and make sure doesn't cause checkmate
    pub fn check_move(&self, from: Square, to: Square, piece: Piece, color: Color) -> Result<(), String> {
        let piece_idx = usize::from(piece.clone());
        let side_idx = usize::from(color);
        let from_bitboard = BitBoard::from(from);
        let to_bitboard = BitBoard::from(to);
        let legal_moves_for_piece = self.pieces_movement[side_idx][piece_idx] | self.pieces_capture_movement[side_idx][piece_idx];
        let is_piece_in_from_location = !(self.pieces_location[side_idx][piece_idx] & from_bitboard).is_empty();
        let is_piece_in_to_location = !(self.pieces_location[side_idx][piece_idx] & to_bitboard).is_empty();
        if !is_piece_in_from_location | !is_piece_in_to_location{
            return Err(format!("No piece of type {:?} from {:?}, to {:?}", piece, from, to));
        }
        let is_inside_legal_moves = !(legal_moves_for_piece & to_bitboard).is_empty();
        if !is_inside_legal_moves{
            return Err(format!("Move({:?}, {:?} -> {:?}) not inside the legal move set", piece, from, to));
        }
        Ok(())
    }

    fn update_state(&mut self, from: Square, to: Square, piece: Piece, side: Color){
        let side_idx = usize::from(side);
        let piece_idx = usize::from(piece);
        self.pieces_location[side_idx][piece_idx] ^= BitBoard::from(from);
        self.pieces_location[side_idx][piece_idx] |= BitBoard::from(to);
        self.pieces_square[side_idx][piece_idx] = self.pieces_square[piece_idx][piece_idx]
            .iter()
            .map(|x| if *x == from { to } else { *x })
            .collect();
        self.compute_attack_threat_and_move();
    }

}

impl Game{
    fn start_position_mask() -> [[BitBoard; 6]; 2]{
        let mut start_position = [[BitBoard::empty(); 6]; 2];
            let white_side = usize::from(Color::White);
            let black_side = usize::from(Color::Black);
            for piece in Piece::iter(){
                start_position[white_side][usize::from(piece)] |= match piece {
                    Piece::Pawn => BitBoard::new(0xff00),
                    Piece::Knight => BitBoard::new(0xff00),
                    Piece::Rock => BitBoard::new(0x81),
                    Piece::Bishop => BitBoard::new(0x24),
                    Piece::Queen => BitBoard::new(0x8),
                    Piece::King => BitBoard::new(0x10)
                };
                start_position[black_side][usize::from(piece)] |= match piece {
                    Piece::Pawn => BitBoard::new(0xff000000000000),
                    Piece::Knight => BitBoard::new(0x4200000000000000),
                    Piece::Rock => BitBoard::new(0x8100000000000000),
                    Piece::Bishop => BitBoard::new(0x2400000000000000),
                    Piece::Queen => BitBoard::new(0x800000000000000),
                    Piece::King => BitBoard::new(0x1000000000000000)
                };
            }
        start_position
    }

    fn start_position() -> [[Vec<Square>; 6]; 2]{
        let mut start_position: [[Vec<Square>; 6]; 2] =
            std::array::from_fn(|_| std::array::from_fn(|_| Vec::new()));
        for piece in Piece::iter(){
            for side in Color::iter(){
                let rank = match (side, piece) {
                    (Color::White, Piece::Pawn) => Rank::Two,
                    (Color::Black, Piece::Pawn) => Rank::Seven,
                    (Color::White, _) => Rank::One,
                    (Color::Black, _) => Rank::Eight,
                };
                start_position[usize::from(side)][usize::from(piece)] = match piece {
                    Piece::Pawn => vec![
                        Square::new(File::A, rank),
                        Square::new(File::B, rank),
                        Square::new(File::C, rank),
                        Square::new(File::D, rank),
                        Square::new(File::E, rank),
                        Square::new(File::F, rank),
                        Square::new(File::G, rank),
                        Square::new(File::H, rank),
                    ],
                    Piece::Knight => vec![
                        Square::new(File::B, rank),
                        Square::new(File::G, rank),
                    ],
                    Piece::Rock => vec![
                        Square::new(File::A, rank),
                        Square::new(File::H, rank),
                    ],
                    Piece::Bishop => vec![
                        Square::new(File::C, rank),
                        Square::new(File::F, rank),
                    ],
                    Piece::Queen => vec![
                        Square::new(File::D, rank),
                    ],
                    Piece::King => vec![
                        Square::new(File::E, rank),
                    ]
                };
            }

        }
        start_position
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

    fn get_all_position(&self) -> [Option<Piece>; 64]{
        let mut board = [None; 64];
        for side in Color::iter(){
            for piece in Piece::iter(){
                for square in &self.pieces_square[usize::from(side)][usize::from(piece)]{
                    let idx = usize::from(*square);
                    board[idx] = Some(piece);
                }
            }
        }
        board
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
    fn test_king_checked() {
        let mut piece_capture = [BitBoard::empty(); 6];
        piece_capture[usize::from(Piece::Rock)] = BitBoard::new(0xff);
        let king = BitBoard::new(0x8);
        let result = Game::is_checked(&king, &piece_capture);
        assert!(result);
    }

    #[test]
    fn test_king_not_checked() {
        let mut piece_capture = [BitBoard::empty(); 6];
        piece_capture[usize::from(Piece::Rock)] = BitBoard::new(0xff);
        piece_capture[usize::from(Piece::Bishop)] = BitBoard::new(0x8040201008040201);
        let king = BitBoard::new(0x800000000000000);
        let result = Game::is_checked(&king, &piece_capture);
        assert!(!result);
    }

    #[test]
    fn test_update_state_move_piece_success() {
        let mut game = Game::new();

        let from = Square::new(File::A, Rank::Two);
        let to = Square::new(File::A, Rank::Three);
        let piece = Piece::Pawn;
        let side = Color::White;

        game.update_state(from, to, piece, side);
        let expected = BitBoard::from(to);
        let res = expected & game.pieces_location[0][0];
        assert!(!res.is_empty());
    }

}


