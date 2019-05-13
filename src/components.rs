// The code in this project is licenced under the GNU General Public Licence 3.0
use specs_derive::*;
use specs::prelude::*;

#[derive(Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {x, y}
    }
}

#[derive(Debug, PartialEq, Component)]
#[storage(VecStorage)]
pub struct Drawable {
    pub char: char,
}

#[derive(Default, PartialEq, Debug, Component)]
#[storage(NullStorage)]
pub struct Controllable;