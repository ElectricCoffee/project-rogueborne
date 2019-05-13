// The code in this project is licenced under the GNU General Public Licence 3.0
use specs::prelude::*;
use tcod::*;

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
        root.wait_for_keypress(false);
    }
}

