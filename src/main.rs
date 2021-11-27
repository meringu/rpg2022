use bevy::prelude::*;

mod player;

fn main() {
    let mut app = App::build();

    #[cfg(target_arch = "wasm32")]
    app.insert_resource(WindowDescriptor {
        canvas: Some("#rpg-canvas".to_string()),
        ..Default::default()
    });

    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_plugin(player::PlayerPlugin);

    app.run();
}
