use strum::IntoEnumIterator;
use crate::bitboard::BitBoard;
use crate::gui::cmd::CommandPromptGUI;
use crate::pieces::common::{Color};
use crate::pieces::Piece;
use crate::square::{File, Rank, Square};

#[derive(Debug)]
pub enum GameResult {
    Checkmate(Color),
    Draw,
}

pub(crate) struct Game {
    gui: CommandPromptGUI,
    pieces_location: [[BitBoard; 6]; 2],
    pieces_square: [[Vec<Square>; 6]; 2],
    pieces_capture_movement: [[BitBoard; 6]; 2],
    pieces_movement: [[BitBoard; 6]; 2],
    turn: Color
}

// TODO: add castle action
impl Game {
    /// Validates whether a move from one square to another is legal based on the current game state.
    ///
    /// # Arguments
    /// - `from`: The starting `Square` where the piece currently resides.
    /// - `to`: The destination `Square` where the piece is intended to move.
    ///
    /// # Returns
    /// - `Ok(Piece)`: If the move is valid, returns the `Piece` being moved.
    /// - `Err(String)`: If the move is invalid, returns an error message explaining why.
    fn validate_move(&self, from: Square, to: Square) -> Result<Piece, String>{
        let [_, bit_to] = [BitBoard::from(from), BitBoard::from(to)];
        let piece = self.get_piece_by_location(self.turn, from);
        match piece {
            None =>  Err(format!("Piece doesn't exist in square {:?}", from)),
            Some(piece) => {
                let (legal_movement, legal_capture) = self.compute_attack_threat_and_move_to_given(from, piece, self.turn);
                let is_inside_legal_moves = !((legal_movement | legal_capture) & bit_to).is_empty();
                if !is_inside_legal_moves{
                    println!("legal_movement {:?}", legal_movement);
                    println!("legal_capture {:?}", legal_capture);
                    println!("bit_to {:?}", bit_to);
                    Err(format!("{:?} in square {:?} is not inside legal moves.", piece, from))
                } else {
                    Ok(piece)
                }
            }
        }
    }

    /// Creates a new instance of the `Game` struct and initializes the game state.
    ///
    /// # Returns
    /// - A fully initialized `Game` instance with the starting positions of pieces, movement masks, and other game data.
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

    /// Starts the main game loop, handling rendering, user input, and game state updates.
    pub fn start(&mut self){
        let mut board_position = self.get_all_position();
        loop{
            self.gui.render(&board_position, self.turn);
            if let Some(result) = self.game_result() {
                println!("Game result: {:?}", result);
                break;
            }
            if let Some((from, to)) = self.gui.wait_and_process_event() {
                match self.validate_move(from, to) {
                    Err(err) =>  println!("{}", err),
                    Ok(piece) => {
                        match self.try_update_state(from, to, piece, self.turn) {
                            Err(err) => println!("{}", err),
                            Ok(()) => {
                                board_position[usize::from(from)] = None;
                                board_position[usize::from(to)] = Some((piece, self.turn));
                                self.turn = self.turn.opposite();
                            }
                        }
                    }
                }
            }
        }
    }

    /// Attempts to update the game state based on a move, validating that the move does not leave the king in check.
    /// # Arguments
    ///
    /// - `from`: The `Square` where the piece is currently located.
    /// - `to`: The `Square` where the piece is intended to move.
    /// - `piece`: The `Piece` being moved (e.g., pawn, knight, rook).
    /// - `side`: The `Color` of the player making the move (e.g., `Color::White` or `Color::Black`).
    /// # Returns
    ///
    /// - `Ok(())`: If the state is successfully updated and the move is valid.
    /// - `Err(String)`: If the move leaves the player's king in check, an error is returned with a descriptive message.
    fn try_update_state(&mut self, from: Square, to: Square, piece: Piece, side: Color) -> Result<(), String> {
        let opponent_side = side.opposite();
        let side_idx = usize::from(side);
        let opponent_side_idx = usize::from(opponent_side);
        let piece_idx = usize::from(piece);
        let opponent_location = self.get_piece_by_location(opponent_side, to);
        let game = self.clone();
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
    /// Generates the starting position bitboards for all pieces on the chessboard.
    /// # Returns
    ///
    /// A 2D array of `BitBoard`:
    /// - `[[BitBoard; 6]; 2]`
    /// - The outer array corresponds to the two sides: White and Black.
    /// - The inner array corresponds to the six piece types: Pawn, Knight, Rook, Bishop, Queen, and King.
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

    /// Generates the starting positions of all pieces on the chessboard as a nested array of vectors.
    ///
    /// # Returns
    /// - `[[Vec<Square>; 6]; 2]`
    ///   - The outer array corresponds to the two sides: White and Black.
    ///   - The inner array corresponds to the six piece types: Pawn, Knight, Rook, Bishop, Queen, and King.
    ///   - Each vector contains the `Square` positions for the respective pieces.
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

    /// Combines multiple `BitBoard` values into a single `BitBoard` by performing a bitwise OR operation.
    ///
    /// # Arguments
    /// - `board`: A reference to an array of `BitBoard` values (one for each piece type).
    ///   - The array has a fixed size of 6, corresponding to the six piece types (Pawn, Knight, Rook, Bishop, Queen, King).
    ///
    /// # Returns
    /// - A single `BitBoard` representing the combined positions of all pieces in the input array.
    fn combine(board: &[BitBoard; 6]) -> BitBoard{
        (Piece::iter())
            .into_iter()
            .map(|i| board[usize::from(i)])
            .fold(BitBoard::empty(), |acc, board| {
                acc | board
            })
    }

    /// Computes the attack threat and legal moves for a given piece on a specific square.
    ///
    /// # Arguments
    /// - `square`: The `Square` where the piece is currently located.
    /// - `piece`: The `Piece` being evaluated (e.g., Pawn, Knight, Rook, etc.).
    /// - `color`: The `Color` of the piece (e.g., `Color::White` or `Color::Black`).
    ///
    /// # Returns
    /// - A tuple `(BitBoard, BitBoard)`:
    ///   - The first `BitBoard` represents the legal movement options for the piece.
    ///   - The second `BitBoard` represents the attack (capture) options for the piece.
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

    /// Computes and updates the attack threats and legal moves for all pieces on the board.
    fn compute_attack_threat_and_move(&mut self){
        self.pieces_movement.iter_mut()
            .for_each(|piece_move| piece_move.iter_mut()
                .for_each(|board| board.clear()));
        self.pieces_capture_movement.iter_mut()
            .for_each(|piece_move| piece_move.iter_mut()
                .for_each(|board| board.clear()));
        for side in Color::iter(){
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

    /// Determines if the current player's king is in check.
    fn is_checked(&self) -> bool{
        let attack = Self::combine(&self.pieces_capture_movement[usize::from(self.turn.opposite())]);
        let king_pos = self.pieces_location[usize::from(self.turn)][usize::from(Piece::King)];
        !(attack & king_pos).is_empty()
    }

    /// Retrieves the current positions of all pieces on the board as a flat array.
    ///
    /// # Returns
    /// - `[Option<(Piece, Color)>; 64]`
    ///   - An array where each index corresponds to a square on the chessboard (0 for A1, 63 for H8).
    ///   - Each element is either `Some((Piece, Color))` if a piece occupies the square, or `None` if the square is empty.
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

    /// Retrieves the piece located at a specific square for a given color.
    ///
    /// # Arguments
    /// - `color`: The `Color` of the player (`Color::White` or `Color::Black`).
    /// - `square`: The `Square` to query for a piece.
    ///
    /// # Returns
    /// - `Some(Piece)`: If a piece of the specified color occupies the given square, returns the piece type (e.g., Pawn, Knight, etc.).
    /// - `None`: If no piece of the specified color is present on the given square.
    fn get_piece_by_location(&self, color: Color, square: Square) -> Option<Piece> {
        Piece::iter()
            .find(|piece| self.pieces_square[usize::from(color)][usize::from(*piece)].contains(&square))
    }

    /// Copies the state of another `Game` instance into the current instance.
    ///
    /// # Arguments
    /// - `other`: The `Game` instance from which the state will be copied.
    fn set_from(&mut self, other: Game){
        self.pieces_square = other.pieces_square;
        self.pieces_location = other.pieces_location;
        self.pieces_movement = other.pieces_movement;
        self.pieces_capture_movement = other.pieces_capture_movement;
    }

    /// Determines the current result of the game, if any.
    ///
    /// # Returns
    /// - `Some(GameResult)`:
    ///   - `GameResult::Draw`: If the current player has no legal moves but the game is not in checkmate.
    ///   - `GameResult::Checkmate(Color)`: If the current player is in checkmate, returns the color of the player who lost.
    /// - `None`: If the game is still ongoing and no result has been determined.
    fn game_result(&self) -> Option<GameResult> {
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
            false => Some(GameResult::Checkmate(self.turn))
        }
    }

    /// Determines if the current player has any legal moves available.
    fn has_legal_moves(&self) -> bool {
        let side_idx = usize::from(self.turn);
        self.pieces_movement[side_idx]
            .iter()
            .chain(self.pieces_capture_movement[side_idx].iter())
            .any(|bitboard| !bitboard.is_empty())
    }

    /// Identifies the opponent's pieces that are currently attacking the player's king.
    ///
    /// # Returns
    /// - `Vec<(Piece, BitBoard)>`:
    ///   - A vector where each element represents an opponent piece that is attacking the king.
    ///   - Each tuple consists of:
    ///     - `Piece`: The type of the attacking piece (e.g., Pawn, Knight, Rook).
    ///     - `BitBoard`: The bitboard representing the attacking piece's position.
    fn get_attacking_pieces(&self) -> Vec<(Piece, BitBoard)>{
        let side_idx = usize::from(self.turn);
        let opponent_side = self.turn.opposite();
        let opponent_side_idx = usize::from(opponent_side);
        let king_position = self.pieces_location[side_idx][usize::from(Piece::King)];
        let mut attacking: Vec<(Piece, BitBoard)> = Vec::new();
        for piece in Piece::iter(){
            let piece_idx = usize::from(piece);
            let attacking_board = self.pieces_location[opponent_side_idx][piece_idx] | self.pieces_capture_movement[opponent_side_idx][piece_idx];
            if !attacking_board.is_empty(){
                attacking.push((piece, attacking_board));
            }
        }
        attacking

    }
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