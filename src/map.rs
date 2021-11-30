mod objects;
mod tiles;

use bevy::prelude::*;

const SPRITE_SIZE: f32 = 32.0;
pub const MAP_WIDTH: f32 = 32.0 * SPRITE_SIZE;
pub const MAP_HEIGHT: f32 = 32.0 * SPRITE_SIZE;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(objects::setup.system())
            .add_startup_system(tiles::setup.system());
    }
}
