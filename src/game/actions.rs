use crate::game::Position;

#[derive(Debug)]
pub struct MoveAction {
    pub from: Position,
    pub to: Position,
}

#[derive(Debug)]
pub enum UserAction<MoveAction> {
    OfferDraw,
    AcceptDraw,
    Resign,
    Move(MoveAction),
    ShowMoveOption(Position),
}

pub type Action = UserAction<MoveAction>;

impl MoveAction {
    pub fn new(from: Position, to: Position) -> Self{
        Self{from, to}
    }
}
