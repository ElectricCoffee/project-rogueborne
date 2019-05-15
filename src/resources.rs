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

    pub fn dirt() -> Tile {
        Tile { char: '"', is_blocked: false, blocks_sight: false, is_explored: false }
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

    pub fn iter(&self) -> std::slice::Iter<Tile> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Tile> {
        self.map.iter_mut()
    }
}