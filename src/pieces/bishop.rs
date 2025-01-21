use crate::bitboard::BitBoard;
use crate::pieces::common::{Color, PossibleMoves};
use crate::square::Square;

const DIAGONAL_MASK: [u64; 15] = [
    0x1, 0x102, 0x10204, 0x1020408,
    0x102040810, 0x10204081020, 0x1020408102040,
    0x102040810204080, 0x204081020408000,
    0x408102040800000, 0x810204080000000,
    0x1020408000000000, 0x2040800000000000,
    0x4080000000000000, 0x8000000000000000
];

const ANTI_DIAGONAL_MASK: [u64; 15] =  [
    0x80, 0x8040, 0x804020, 0x80402010, 0x8040201008,
    0x804020100804, 0x80402010080402,
    0x8040201008040201, 0x4020100804020100,
    0x2010080402010000, 0x1008040201000000,
    0x804020100000000, 0x402010000000000,
    0x201000000000000, 0x100000000000000
];

/// Description
/// Slide in diagonal or anti-diagonal
/// stop movement when capture other piece or blocked by its own piece(exclusive)
struct Bishop;


impl PossibleMoves for Bishop {
    fn get_moves(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
       Self::get_diagonal_moves(piece, square, own_pieces, opponent_pieces, color)
        | Self::get_anti_diagonal_moves(piece, square, own_pieces, opponent_pieces, color)
    }
}

impl Bishop {

    /// Computes the diagonal mask for the given square.
    /// # Parameters
    /// - `square`: The [`Square`] for which to calculate the diagonal mask.
    /// # Returns
    /// A [`BitBoard`] containing the mask for the diagonal.
    fn get_diagonal_mask(square: Square) -> BitBoard{
        let rank = usize::from(square.rank());
        let file = usize::from(square.file());
        BitBoard::new(DIAGONAL_MASK[file + rank])
    }

    /// Computes the anti-diagonal mask for the given square.
    /// # Parameters
    /// - `square`: The [`Square`] for which to calculate the anti-diagonal mask.
    /// # Returns
    /// A [`BitBoard`] containing the mask for the anti-diagonal.
    fn get_anti_diagonal_mask(square: Square) -> BitBoard{
        let rank = usize::from(square.rank());
        let file = usize::from(square.file());
        BitBoard::new(DIAGONAL_MASK[7 + (file - rank)])
    }

    /// Computes all possible diagonal moves for a piece located on the given square.
    /// # Parameters
    /// - `piece`: A [`BitBoard`] representing the single position of the piece.
    /// - `square`: The [`Square`] where the piece is located.
    /// - `own_pieces`: A [`BitBoard`] representing the positions of all friendly pieces.
    /// - `opponent_pieces`: A [`BitBoard`] representing the positions of all opponent pieces.
    /// - `color`: The [`Color`] of the piece (`Color::White` or `Color::Black`).
    /// # Returns
    /// A [`BitBoard`] representing all valid diagonal moves for the piece.
    fn get_diagonal_moves(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
        let diagonal_mask = Self::get_diagonal_mask(square);
        let occupied_diagonal = Self::occupied(own_pieces, opponent_pieces) & diagonal_mask;
        let diagonal_up = occupied_diagonal - (*piece * 2);
        let diagonal_down = (occupied_diagonal.reverse() - (*piece * 2)).reverse();
        (diagonal_up ^ diagonal_down) & diagonal_mask
    }

    /// Computes all possible anti-diagonal moves for a piece located on the given square.
    /// # Parameters
    /// - `piece`: A [`BitBoard`] representing the single position of the piece.
    /// - `square`: The [`Square`] where the piece is located.
    /// - `own_pieces`: A [`BitBoard`] representing the positions of all friendly pieces.
    /// - `opponent_pieces`: A [`BitBoard`] representing the positions of all opponent pieces.
    /// - `color`: The [`Color`] of the piece (`Color::White` or `Color::Black`).
    /// # Returns
    /// A [`BitBoard`] representing all valid anti-diagonal moves for the piece.
    fn get_anti_diagonal_moves(piece: &BitBoard, square: Square, own_pieces: &BitBoard, opponent_pieces: &BitBoard, color: &Color) -> BitBoard {
        let anti_diagonal_mask = Self::get_diagonal_mask(square);
        let occupied_anti_diagonal = Self::occupied(own_pieces, opponent_pieces) & anti_diagonal_mask;
        let ant_diagonal_up = occupied_anti_diagonal - (*piece * 2);
        let anti_diagonal_down = (occupied_anti_diagonal.reverse() - (*piece * 2)).reverse();
        (ant_diagonal_up ^ anti_diagonal_down) & anti_diagonal_mask
    }
}
