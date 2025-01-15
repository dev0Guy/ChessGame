mod engine;
use engine::game::base::Game;
use engine::gui::{cmd::CommandPromptGUI, base::GUI};
use crate::engine::movement::moves;

fn main() {
    println!("Hello, world!");
    let gui: Box<dyn GUI<moves::Action>> = Box::new(CommandPromptGUI::new());
    let mut game = Game::new(gui);
    game.start();
}
