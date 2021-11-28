use bevy::prelude::*;

pub mod game_camera;
pub mod map;
pub mod player;
pub mod window;

pub fn z_from_y(y: f32, sprite_height: f32) -> f32 {
    -(y - sprite_height / 2.0) / 1000.0 + 10.0
}

fn main() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_plugin(crate::window::WebFullscreenPlugin);

    app.add_plugin(game_camera::GameCameraPlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(player::PlayerPlugin)
        .run();
}
