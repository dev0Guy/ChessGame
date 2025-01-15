use std::ops::{Index, IndexMut};
use crate::engine::board::location::Location;
use crate::engine::board::pieces::{Piece};
use crate::engine::movement::moves::MoveAction;

const FILE_NAMES: &'static str = "   A B C D E F G H";

#[derive(Debug)]
pub struct Board([[Option<Piece>; 8]; 8]);

impl Index<Location> for Board {
    type Output = Option<Piece>;

    fn index(&self, index: Location) -> &Self::Output {
        &self.0[7 - index.rank as usize][index.file as usize]
    }
}

impl IndexMut<Location> for Board {
    fn index_mut(&mut self, index: Location) -> &mut Self::Output {
        &mut self.0[7 - index.rank as usize][index.file as usize]
    }
}


impl Board {
    pub fn new() -> Self {
        Self([[None; 8]; 8])
    }

    pub fn iter(&self) -> std::slice::Iter<[Option<Piece>; 8]> {
        self.0.iter()
    }

    pub fn action(&mut self, move_action: MoveAction) {
        self[move_action.to] = self[move_action.from];
        self[move_action.from] = None;
    }
}