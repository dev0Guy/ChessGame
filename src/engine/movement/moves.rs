use crate::engine::board::location::Location;
use crate::engine::board::pieces::{Piece};


#[derive(Default)]
pub enum MoveType{
    #[default]
    Normal,
    Attack,
    DoublePawnPush,
    EnPassant,
    PawnPromotion(Piece),
    Check(Piece),
    Castling
}

pub struct Move{
    pub from: Location,
    pub to: Location,
    pub move_type: MoveType
}

impl Move{
    pub fn new(from: Location, to: Location, move_type: MoveType) -> Self{
        Self{from, to, move_type}
    }
}
