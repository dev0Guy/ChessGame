use crate::engine::board::location::Location;
use crate::engine::board::pieces::{Piece};


#[derive(Default, Debug)]
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

#[derive(Debug)]
pub struct MoveAction {
    pub from: Location,
    pub to: Location,
    pub move_type: MoveType
}

#[derive(Debug)]
pub enum UserAction<MoveAction> {
    OfferDraw,
    AcceptDraw,
    Resign,
    Move(MoveAction),
    ShowMoveOption(Location),
    Error
}

pub type Action = UserAction<MoveAction>;

impl MoveAction {
    pub fn new(from: Location, to: Location, move_type: MoveType) -> Self{
        Self{from, to, move_type}
    }
}
