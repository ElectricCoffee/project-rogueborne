// The code in this project is licenced under the GNU General Public Licence 3.0
use specs::prelude::*;
use tcod::*;
use tcod::input::*;

use crate::{
    components::*,
};

pub struct Draw;
impl<'a> System<'a> for Draw {
    type SystemData = (
        WriteExpect<'a, console::Root>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Drawable>
    );

    fn run(&mut self, (mut root, pos, drw): Self::SystemData) {
        use specs::Join;

        root.clear();

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