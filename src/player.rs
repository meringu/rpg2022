mod animate;
mod control;
mod frame;
mod keyboard;
mod mouse;
mod velocity;

use crate::map;
use bevy::prelude::*;
use control::{Control, ControlMode};
use frame::{FrameBase, FrameStep};
use keyboard::KeyboardPlugin;
use velocity::Velocity;

const SPRITE_SHEET: &str = "textures/player.png";
pub const SPRITE_WIDTH: f32 = 14.0;
pub const SPRITE_HEIGHT: f32 = 25.0;

const STEP_DURATION_SECONDS: f32 = 0.15;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(KeyboardPlugin)
            .add_startup_system(setup.system())
            .add_system(animate::animate.system())
            .add_system(mouse::input.system());
    }
}

#[derive(Default)]
pub struct Player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(SPRITE_SHEET);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT),
        frame::STEPS as usize,
        4,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let transform = Transform {
        translation: init_position(),
        ..Default::default()
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .insert(Timer::from_seconds(STEP_DURATION_SECONDS, true))
        .insert(Player::default())
        .insert(Velocity::default())
        .insert(FrameBase::default())
        .insert(FrameStep::default())
        .insert(Control::default());
}

pub fn init_position() -> Vec3 {
    let y = map::MAP_HEIGHT as f32 / 2.0 * map::SPRITE_SIZE;
    Vec3::new(
        map::MAP_WIDTH as f32 / 2.0 * map::SPRITE_SIZE,
        y,
        crate::z_from_y(y, SPRITE_HEIGHT),
    )
}
