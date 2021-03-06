// The code in this project is licenced under the GNU General Public Licence 3.0
mod components;
mod systems;
mod resources;

use tcod::*;
use specs::prelude::*;

pub const SCREEN_WIDTH: u32 = 80;
pub const SCREEN_HEIGHT: u32 = 50;
pub const MAP_HEIGHT: u32 = SCREEN_HEIGHT - 5;
pub const MAP_WIDTH: u32 = SCREEN_WIDTH;

fn main() {
    let root = console::Root::initializer()
        .font("assets/font.png", FontLayout::AsciiInRow)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Project Rogueborne")
        .init();

    system::set_fps(20);

    let mut world = specs::World::new();

    world.add_resource(resources::GameState::Game);
    world.add_resource(resources::Map::new(MAP_WIDTH, MAP_HEIGHT));

    let mut game_dispatcher = specs::DispatcherBuilder::new()
        .with_thread_local(systems::Draw)
        .with_thread_local(systems::Control)
        .build();

    game_dispatcher.setup(&mut world.res);

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
        use resources::GameState::*;

        match *world.read_resource::<resources::GameState>() {
            MainMenu => {}
            MapGen => {}
            Game => game_dispatcher.dispatch(&world.res),
            Pause => {}
            Quit => {}
        }

        if world.read_resource::<console::Root>().window_closed() {
            break;
        }

        world.maintain();
    }
}
