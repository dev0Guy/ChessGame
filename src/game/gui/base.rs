use crate::engine::board::pieces::Side;
use crate::game::bitboard::BitBoard;
use crate::game::Position;

pub trait GUI<T>{
    fn render(&mut self, board: &BitBoard, side: Side, pos: Option<Position>);

    fn wait_and_process_event(&mut self) -> T;
}