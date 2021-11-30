pub mod game_camera;
pub mod map;
pub mod music;
pub mod player;
pub mod window;
pub mod z;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn z_from_y(y: f32, sprite_height: f32) -> f32 {
    -(y - sprite_height / 2.0) / 1000.0 + 10.0
}

fn main() {
    let mut app = App::build();
    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin)
        .add_plugin(crate::window::WebFullscreenPlugin);

    app.add_plugin(bevy_kira_audio::AudioPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_plugin(music::MusicPlugin)
        .add_plugin(game_camera::GameCameraPlugin)
        .add_plugin(map::MapPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(z::ZPlugin)
        .add_startup_system(setup.system())
        .add_system(display_events.system())
        .run();
}

fn setup(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::ZERO.into();
}

fn display_events(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut contact_events: EventReader<ContactEvent>,
) {
    for intersection_event in intersection_events.iter() {
        println!("Received intersection event: {:?}", intersection_event);
    }

    for contact_event in contact_events.iter() {
        println!("Received contact event: {:?}", contact_event);
    }
}
