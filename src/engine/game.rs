use std::fmt::format;
use std::ops::BitAndAssign;
use strum::IntoEnumIterator;
use crate::bitboard::BitBoard;
use crate::gui::cmd::CommandPromptGUI;
use crate::pieces::common::{Color, PossibleMoves};
use crate::pieces::Piece;
use crate::square::{File, Rank, Square};

pub(crate) struct Game {
    gui: CommandPromptGUI,
    pieces_location: [[BitBoard; 6]; 2],
    pieces_square: [[Vec<Square>; 6]; 2],
    pieces_capture_movement: [[BitBoard; 6]; 2],
    pieces_movement: [[BitBoard; 6]; 2],
    turn: Color
}


impl Clone for Game{
    fn clone(&self) -> Self {
        Self{
            turn: self.turn,
            pieces_square: self.pieces_square.clone(),
            pieces_movement: self.pieces_movement.clone(),
            pieces_location: self.pieces_location.clone(),
            pieces_capture_movement: self.pieces_capture_movement.clone(),
            gui: CommandPromptGUI::new()
        }
    }
}

impl Game {

    fn validate_move(&self, from: Square, to: Square) -> Result<Piece, String>{
        let [_, bit_to] = [BitBoard::from(from), BitBoard::from(to)];
        let piece = self.get_piece_by_location(self.turn, from);
        match piece {
            None =>  Err(format!("Piece doesn't exist in square {:?}", from)),
            Some(piece) => {
                let (legal_movement, legal_capture) = self.compute_attack_threat_and_move_to_given(from, piece, self.turn);
                let is_inside_legal_moves = !((legal_movement | legal_capture) & bit_to).is_empty();
                if !is_inside_legal_moves{
                    Err(format!("{:?} in square {:?} is not inside legal moves.", piece, from))
                } else {
                    Ok(piece)
                }
            }
        }
    }


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
            pieces_square,
            turn: Color::White
        };
        game.compute_attack_threat_and_move();
        game
    }

    pub fn start(&mut self){
        let mut board_position = self.get_all_position();
        loop{
            self.gui.render(&board_position, self.turn);
            match self.is_game_over() {
                Some(val) => {
                    println!("Game Over!, {:?}", val);
                    break;
                }
                None => {}
            };
            let action = self.gui.wait_and_process_event();
            let action_res = match action {
                None => continue,
                Some((from, to)) => {
                    match self.validate_move(from, to) {
                        Err(val) => Err(val),
                        Ok(piece) => {
                            let status = self.try_update_state(from, to, piece, self.turn);
                            match status {
                                Err(val) => Err(val),
                                Ok(()) => {
                                    board_position[usize::from(from)] = None;
                                    board_position[usize::from(to)] = Some((piece, self.turn));
                                    self.turn = self.turn.opposite();
                                    Ok(())
                                }
                            }
                        }
                    }
                }
            };
            match action_res {
                Err(str) => println!("{}", str),
                Ok(_) => {}
            }
        }
    }


    /// Check if move is possible
    /// Firstly check if move inside the move list. If do try move and make sure doesn't cause checkmate
    pub fn check_move(&self, from: Square, to: Square, color: Color) -> Result<Piece, String> {
        let piece = self.get_piece_by_location(color, from);
        let side_idx = usize::from(color);
        let from_bitboard = BitBoard::from(from);
        let to_bitboard = BitBoard::from(to);
        match piece {
            None =>  Err(format!("Piece doesn't exist in square {:?}", from)),
            Some(piece) => {
                let piece_idx = usize::from(piece);
                let own_pieces = self.pieces_location[side_idx][piece_idx];
                let (legal_movement, legal_capture) = self.compute_attack_threat_and_move_to_given(from, piece, color);
                let is_inside_legal_moves = !((legal_movement | legal_capture) & to_bitboard).is_empty();
                if (own_pieces & from_bitboard).is_empty(){
                    return Err(format!("No piece of type {:?} in square {:?}", piece, from));
                }
                if !is_inside_legal_moves{
                    return Err(format!("Move({:?}, {:?} -> {:?}) not inside the legal move set", piece, from, to));
                }
                Ok(piece)
            }
        }
    }

    fn try_update_state(&mut self, from: Square, to: Square, piece: Piece, side: Color) -> Result<(), String> {
        let opponent_side = side.opposite();
        let side_idx = usize::from(side);
        let opponent_side_idx = usize::from(opponent_side);
        let piece_idx = usize::from(piece);
        let opponent_location = self.get_piece_by_location(opponent_side, to);
        let game = self.clone();
        // TODO: check if board is in check
        // update position mask
        self.pieces_location[side_idx][piece_idx] ^= BitBoard::from(from);
        self.pieces_location[side_idx][piece_idx] |= BitBoard::from(to);
        match opponent_location {
            None => {}
            Some(piece) => {
                let opponent_piece_idx = usize::from(piece);
                self.pieces_location[opponent_side_idx][opponent_piece_idx] &= !BitBoard::from(to);
                self.pieces_square[opponent_side_idx][opponent_piece_idx].retain(|&x| x != to);
            }
        }
        // change square
        self.pieces_square[side_idx][piece_idx] = self.pieces_square[side_idx][piece_idx]
            .iter()
            .map(|x| if *x == from { to } else { *x })
            .collect();
        // get new attacks
        self.compute_attack_threat_and_move();
        if self.is_checked(){
            self.set_from(game);
            return Err(format!("After move king is still on check {:?}", from));
        }
        Ok(())
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
                    Piece::Knight => BitBoard::new(0x42),
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

    fn compute_attack_threat_and_move_to_given(&self, square: Square, piece: Piece, color: Color) -> (BitBoard, BitBoard){
        let side_index = usize::from(color);
        let piece_idx = usize::from(piece);
        let opponent_index = usize::from(color.opposite());
        let own_pieces =  &Self::combine(&self.pieces_location[side_index]);
        let opponent_pieces = &Self::combine(&self.pieces_location[opponent_index]);
        let piece_bit = &self.pieces_location[side_index][piece_idx] & (&BitBoard::from(square));
        let movement = piece.moves_function()(
            &piece_bit,
            square,
            own_pieces,
            opponent_pieces,
            &color
        );
        let capture = piece.capture_function()(
            &piece_bit,
            square,
            own_pieces,
            opponent_pieces,
            &color
        );
        (movement, capture)
    }

    fn compute_attack_threat_and_move(&mut self){
        self.pieces_movement.iter_mut()
            .for_each(|piece_move| piece_move.iter_mut()
                .for_each(|board| board.clear()));
        self.pieces_capture_movement.iter_mut()
            .for_each(|piece_move| piece_move.iter_mut()
                .for_each(|board| board.clear()));
        for side in Color::iter(){
            let opponent_side = side.opposite();
            let side_index = usize::from(side);
            for piece in Piece::iter(){
                let piece_idx = usize::from(piece.clone());
                for square in &self.pieces_square[side_index][piece_idx]{
                    let (movement, capture) = self.compute_attack_threat_and_move_to_given(square.clone(), piece, side);
                    self.pieces_movement[side_index][piece_idx] |= movement;
                    self.pieces_capture_movement[side_index][piece_idx] |= capture;
                }
            }
        };
    }

    fn is_checked(&self) -> bool{
        let attack = Self::combine(&self.pieces_capture_movement[usize::from(self.turn.opposite())]);
        let king_pos = self.pieces_location[usize::from(self.turn)][usize::from(Piece::King)];
        !(attack & king_pos).is_empty()
    }

    fn get_all_position(&self) -> [Option<(Piece, Color)>; 64]{
        let mut board = [None; 64];
        for side in Color::iter(){
            for piece in Piece::iter(){
                for square in &self.pieces_square[usize::from(side)][usize::from(piece)]{
                    let idx = usize::from(*square);
                    board[idx] = Some((piece, side));
                }
            }
        }
        board
    }

    fn get_piece_by_location(&self, color: Color, square: Square) -> Option<Piece> {
        Piece::iter()
            .find(|piece| self.pieces_square[usize::from(color)][usize::from(*piece)].contains(&square))
    }

    fn set_from(&mut self, other: Game){
        self.pieces_square = other.pieces_square;
        self.pieces_location = other.pieces_location;
        self.pieces_movement = other.pieces_movement;
        self.pieces_capture_movement = other.pieces_capture_movement;
    }

    pub fn is_game_over(&self) -> Option<GameResult> {
        let side_idx = usize::from(self.turn);
        let opponent_idx = usize::from(self.turn.opposite());
        let has_no_moves = !self.has_legal_moves();
        if has_no_moves{ return Some(GameResult::Draw) }
        let king_position = self.pieces_location[side_idx][usize::from(Piece::King)];
        let king_movement = self.pieces_movement[side_idx][usize::from(Piece::King)];
        let possible_moves = !king_position & !king_movement & Self::combine(&self.pieces_movement[side_idx]) | Self::combine(&self.pieces_capture_movement[side_idx]);
        let capture_moves = Self::combine(&self.pieces_movement[opponent_idx]) | Self::combine(&self.pieces_capture_movement[opponent_idx]);
        let orig_attacking = self.get_attacking_pieces();
        let is_king_has_way_to_escape = !(king_movement & !(capture_moves)).is_empty();
        let attacking = orig_attacking.iter()
            .filter(|(_, board)| board == &(board & (&!possible_moves)))
            .map(|(_, board)| board)
            .collect::<Vec<&BitBoard>>();
        let is_not_check_mate = orig_attacking.is_empty() || attacking.is_empty() || is_king_has_way_to_escape;
        match is_not_check_mate {
            true => None,
            false => Some(GameResult::Checkmate(self.turn.opposite()))
        }
    }

    fn has_legal_moves(&self) -> bool {
        let side_idx = usize::from(self.turn);
        self.pieces_movement[side_idx]
            .iter()
            .chain(self.pieces_capture_movement[side_idx].iter())
            .any(|bitboard| !bitboard.is_empty())
    }

    fn get_attacking_pieces(&self) -> Vec<(Piece, BitBoard)>{
        let side_idx = usize::from(self.turn);
        let opponent_side = self.turn.opposite();
        let opponent_side_idx = usize::from(opponent_side);
        let king_position = self.pieces_location[side_idx][usize::from(Piece::King)];
        let mut attacking: Vec<(Piece, BitBoard)> = Vec::new();
        for piece in Piece::iter(){
            let attacking_board = self.pieces_capture_movement[opponent_side_idx][usize::from(piece)] & king_position;
            if !attacking_board.is_empty(){
                attacking.push((piece, attacking_board));
            }
        }
        attacking

    }

}



#[derive(Debug)]
pub enum GameResult {
    Checkmate(Color),
    Draw,
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

    // #[test]
    // fn test_update_state_move_piece_success() {
    //     let mut game = Game::new();
    //
    //     let from = Square::new(File::A, Rank::Two);
    //     let to = Square::new(File::A, Rank::Three);
    //     let piece = Piece::Pawn;
    //     let side = Color::White;
    //
    //     game.try_update_state(from, to, piece, side);
    //     let expected = BitBoard::from(to);
    //     let res = expected & game.pieces_location[0][0];
    //     assert!(!res.is_empty());
    // }

}


