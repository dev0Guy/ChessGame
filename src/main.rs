mod engine;
use engine::game::base::Game;
use engine::gui::{cmd::CommandPromptGUI, base::GUI};
use engine::game::user_actions;

// TODO:
// [V] Document every thing
// [ ] Create Tests
// [ ] Validation on difference actions
//     [ ] Paw-sun
//     [ ] Slide by direction
//     [ ] Single step by direction
//     [ ] Knight move_generator as combination of single step
//     [ ] Pawn eat on diagonal
//     [ ] Pawn jump on start two square
//     [ ] Validate cannot pass existing in the way piece accept Knight
//     [ ] Validate is check
//     [ ] Validate can't move piece who block check
//     [ ] Validate on check only moves pieces that block check
//     [ ] Promotion of pawn to one of option
fn main() {
    let gui: Box<dyn GUI<user_actions::Action>> = Box::new(CommandPromptGUI::new());
    let mut game: Game = Game::new(gui);
    game.start();
}
