use crate::game::{BoardBitSet, Position};

pub struct HorizontalMovement;
pub struct VerticalMovement;
pub struct DiagonalMovement;
pub struct AntiDiagonalMovement;

pub(crate) trait Movement{
    fn compute<T: Iterator<Item=Position>>(&self, pos: Position)-> T;
}

impl Movement for HorizontalMovement {
    fn compute<T: Iterator<Item=Position>>(&self, pos: Position) -> T {
        todo!()
    }
}

impl Movement for VerticalMovement {
    fn compute<T: Iterator<Item=Position>>(&self, pos: Position) -> T {
        todo!()
    }
}

impl Movement for DiagonalMovement {
    fn compute<T: Iterator<Item=Position>>(&self, pos: Position) -> T {
        todo!()
    }
}

impl Movement for AntiDiagonalMovement {
    fn compute<T: Iterator<Item=Position>>(&self, pos: Position) -> T {
        todo!()
    }
}



