mod engine;
use engine::game::base::Game;
use engine::gui::{cmd::CommandPromptGUI, base::GUI};
use engine::game::user_actions;

// TODO:
// [V] Document every thing
// [V] Create Tests
// [V] Show in console all of the possible moves
// [V] Add King
// [V] Add knight moves
// [V] Pin Piece
// [ ] Castle
// [V] Add validation action for each piece
// [ ] Add memory for each piece on each location all possible moves
// [ ] Combine precalculate with validate function


fn main() {
    let gui= CommandPromptGUI::new();
    let mut game = Game::new(gui);
    game.start();
}
