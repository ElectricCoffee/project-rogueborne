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
        use specs::Join;
        
        for (pos, _) in (&mut pos, &_control).join() {
            match root.wait_for_keypress(false) {
                Key { code: KeyCode::Up   , .. } => pos.y -= 1,
                Key { code: KeyCode::Down , .. } => pos.y += 1,
                Key { code: KeyCode::Left , .. } => pos.x -= 1,
                Key { code: KeyCode::Right, .. } => pos.x += 1,
                _                                => Default::default(),
            }
        }
    }
}