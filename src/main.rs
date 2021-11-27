use bevy::prelude::*;

use rpg::*;

fn main() {
    let mut app = App::build();

    app.insert_resource(WindowDescriptor {
        #[cfg(target_arch = "wasm32")]
        canvas: Some("#rpg-canvas".to_string()),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        resizable: false,
        ..Default::default()
    });

    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_plugin(game_camera::GameCameraPlugin);
    app.add_plugin(map::MapPlugin);
    app.add_plugin(player::PlayerPlugin);

    app.run();
}
