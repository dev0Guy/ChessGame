mod engine;
use engine::game::base::Game;
use engine::gui::{cmd::CommandPromptGUI, base::GUI};
use crate::engine::movement::moves;

fn main() {
    let gui: Box<dyn GUI<moves::Action>> = Box::new(CommandPromptGUI::new());
    let mut game: Game = Game::new(gui);
    game.start();
}
