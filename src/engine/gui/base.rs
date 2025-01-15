use crate::engine::board::board::Board;

pub trait GUI<T>{
    fn render(&mut self, game: &Board);

    fn wait_and_process_event(&mut self) -> T;
}