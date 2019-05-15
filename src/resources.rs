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

#[derive(Debug, Clone)]
pub struct Tile {
    pub char: char,
    pub is_blocked: bool,
    pub blocks_sight: bool,
    pub is_explored: bool,
}

impl Tile {
    pub fn wall() -> Tile {
        Tile { char: '#', is_blocked: true, blocks_sight: true, is_explored: false }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Map {
    width: u32,
    height: u32,
    map: Vec<Tile>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        Map {
            width,
            height,
            map: vec![Tile::wall(); (width * height) as usize],
        }
    }
}