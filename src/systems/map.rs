// The code in this project is licenced under the GNU General Public Licence 3.0
use crate::resources::{
    GameState,
    Map,
};
use specs::prelude::*;
use rand::prelude::*;

pub struct GenerateMap;
impl<'a> System<'a> for GenerateMap {
    type SystemData = (Write<'a, Map>);

    fn run(&mut self, mut map: Self::SystemData) {

    }
}

pub struct PlaceItems;
impl<'a> System<'a> for PlaceItems {
    type SystemData = (
        Entities<'a>,
        Write<'a, Map>,
        Write<'a, GameState>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut map, mut state) = data;

        *state = GameState::Game;
    }
}