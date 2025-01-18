use crate::engine::board::location::Location;

#[derive(Debug)]
pub struct MoveAction {
    pub from: Location,
    pub to: Location,
}

#[derive(Debug)]
pub enum UserAction<MoveAction> {
    OfferDraw,
    AcceptDraw,
    Resign,
    Move(MoveAction),
    ShowMoveOption(Location),
}

pub type Action = UserAction<MoveAction>;

impl MoveAction {
    pub fn new(from: Location, to: Location) -> Self{
        Self{from, to}
    }
}
