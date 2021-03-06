// The code in this project is licenced under the GNU General Public Licence 3.0
pub mod map;

use specs::prelude::*;
use tcod::*;
use tcod::input::*;

use crate::{
    components::*,
    resources::Map,
};

/// Draws the contents of the game onto the screen.
pub struct Draw;
impl<'a> System<'a> for Draw {
    type SystemData = (
        Read<'a, Map>,
        WriteExpect<'a, console::Root>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Drawable>
    );

    fn run(&mut self, (map, mut root, pos, drw): Self::SystemData) {
        use specs::Join;

        root.clear();

        for y in 0 .. crate::MAP_HEIGHT {
            for x in 0 .. crate::MAP_WIDTH {
                let tile = map.get_tile(x as usize, y as usize);
                
                if let Some(color) = tile.background_color {
                    root.set_char_background(x as i32, y as i32, color, BackgroundFlag::Set);
                }

                root.put_char(x as i32, y as i32, tile.char, BackgroundFlag::None);

                if let Some(color) = tile.foreground_color {
                    root.set_char_foreground(x as i32, y as i32, color);
                }
            }
        }

        for (pos, drw) in (&pos, &drw).join() {
            root.put_char(pos.x, pos.y, drw.char, BackgroundFlag::None);
        }

        root.flush();
    }
}

pub struct Control;
impl<'a> System<'a> for Control {
    type SystemData = (
        WriteExpect<'a, console::Root>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Controllable>,
    );

    fn run(&mut self, (mut root, mut pos, _control): Self::SystemData) {
        use KeyCode::{Up, Down, Left, Right, Home, End, PageUp, PageDown};
        use specs::Join;
        
        for (pos, _) in (&mut pos, &_control).join() {
            match root.wait_for_keypress(false) {
                // cardinals
                Key { code: Up   , .. } => pos.y -= 1,
                Key { code: Down , .. } => pos.y += 1,
                Key { code: Left , .. } => pos.x -= 1,
                Key { code: Right, .. } => pos.x += 1,
                // diagonals
                Key { code: Home    , .. } => { pos.x -= 1; pos.y -= 1 }
                Key { code: PageUp  , .. } => { pos.x += 1; pos.y -= 1 }
                Key { code: End     , .. } => { pos.x -= 1; pos.y += 1 }
                Key { code: PageDown, .. } => { pos.x += 1; pos.y += 1 }
                _ => Default::default(),
            }
        }
    }
}