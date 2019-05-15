use tcod::console::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    MainMenu,
    MapGen,
    Game,
    Pause,
    Quit,
}

impl Default for GameState {
    fn default() -> GameState {
        GameState::Game
    }
}