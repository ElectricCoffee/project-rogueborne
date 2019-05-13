// The code in this project is licenced under the GNU General Public Licence 3.0
mod components;
mod systems;
mod resources;

use tcod::*;
use specs::prelude::*;

pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;

fn main() {
    let root = console::Root::initializer()
        .font("assets/font.png", FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Project Rogueborne")
        .init();

    system::set_fps(20);

    let mut world = specs::World::new();
    let mut dispatcher = specs::DispatcherBuilder::new()
        .with_thread_local(systems::Draw)
        .with_thread_local(systems::Control)
        .build();

    dispatcher.setup(&mut world.res);

    world.create_entity()
        .with(components::Position::new(10, 10))
        .with(components::Drawable { char: '@' })
        .with(components::Controllable)
        .build();

    world.create_entity()
        .with(components::Position::new(15, 15))
        .with(components::Drawable { char: 'T' })
        .build();

    world.add_resource(root);

    loop {
        dispatcher.dispatch(&world.res);

        if world.read_resource::<console::Root>().window_closed() {
            break;
        }

        world.maintain();
    }
}
