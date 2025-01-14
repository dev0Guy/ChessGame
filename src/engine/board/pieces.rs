#[derive(Debug, Clone, Copy)]
pub enum Side{
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType{
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece{
    pub piece_type: PieceType,
    pub side: Side,
}

impl Piece {
    pub(crate) const fn new(piece_type: PieceType, side: Side) -> Self {
        Self{piece_type, side}
    }
}