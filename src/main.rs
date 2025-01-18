mod engine;
use engine::game::base::Game;
use engine::gui::{cmd::CommandPromptGUI, base::GUI};
use engine::game::user_actions;

// TODO:
// [V] Document every thing
// [V] Create Tests
// [ ] Show in console all of the possible moves
// [ ] Add King
// [ ] Add knight moves
// [ ] Pin Piece
// [ ] Castle
// [ ] Add validation action for each piece
// [ ] Add memory for each piece on each location all possible moves
// [ ] Combine precalculate with validate function


fn main() {
    let gui: Box<dyn GUI<user_actions::Action>> = Box::new(CommandPromptGUI::new());
    let mut game: Game = Game::new(gui);
    game.start();
}
