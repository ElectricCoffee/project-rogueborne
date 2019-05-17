use tcod::{
    console::*,
    colors::{self, Color},
};

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

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub char: char,
    pub is_blocked: bool,
    pub blocks_sight: bool,
    pub is_explored: bool,
    pub foreground_color: Option<Color>,
    pub background_color: Option<Color>,

}

impl Tile {
    pub fn wall() -> Tile {
        Tile { 
            char: '#', 
            is_blocked: true, 
            blocks_sight: true, 
            is_explored: false,
            foreground_color: Some(colors::GREY),
            background_color: Some(colors::DARK_GREY),
        }
    }

    pub fn dirt() -> Tile {
        Tile { 
            char: '"', 
            is_blocked: false, 
            blocks_sight: false, 
            is_explored: false,
            foreground_color: Some(colors::BRASS),
            background_color: None,
        }
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

    pub fn get_tile(&self, x: usize, y: usize) -> Tile {
        let width = self.width as usize;
        let i = y * width + x;
        self.map[i]
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        let width = self.width as usize;
        let i = y * width + x;
        &mut self.map[i]
    }
}