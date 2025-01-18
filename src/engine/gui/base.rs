use crate::engine::board::board::Board;
use crate::engine::board::location::Location;
use crate::engine::board::pieces::Side;

/// A trait representing the graphical user interface (GUI) for any board game.
///
/// This trait defines the basic operations required for rendering the game state
/// and handling user interactions. Implementers of this trait can create custom GUIs
/// for different platforms or rendering backends.
///
/// ## Associated Type
/// - `T`: Represents the type of action or event that the GUI processes and returns.
///
pub trait GUI<T>{
    /// Renders the current game state on the GUI.
    ///
    /// This method takes a reference to the [`Board`] and displays it in the GUI.
    ///
    /// ## Parameters
    /// - `game`: A reference to the current game state, represented by the [`Board`].
    /// - `active_side`: current side turn.
    fn render(&mut self, game: &Board, active_side: Side, show_loc: Vec<Location>);

    fn wait_and_process_event(&mut self) -> T;

}