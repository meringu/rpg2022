mod animate;
mod input;

use crate::map;
pub use animate::{SPRITE_HEIGHT, SPRITE_WIDTH};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(animate::animate.system())
            .add_system(input::input.system());
    }
}

#[derive(Default)]
pub struct Player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(RigidBodyBundle {
            position: init_position().into(),
            mass_properties: RigidBodyMassProps {
                flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::capsule(
                Vec2::new(SPRITE_WIDTH / 2.0, SPRITE_WIDTH / 2.0).into(),
                Vec2::new(
                    SPRITE_WIDTH / 2.0,
                    SPRITE_WIDTH / 2.0,
                    // Should be: (SPRITE_HEIGHT - SPRITE_WIDTH / 2.0) for projectile colision detection
                )
                .into(),
                SPRITE_WIDTH / 2.0,
            ),
            material: ColliderMaterial {
                restitution: 0.7,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(input::InputBundle::default())
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::from(Color::rgb(1.5, 0.0, 0.0)))
        .insert(Player::default())
        .with_children(|parent| animate::add_to_parent(parent, asset_server, texture_atlases));
}

pub fn init_position() -> Vec3 {
    let y = map::MAP_HEIGHT / 2.0;
    Vec3::new(map::MAP_WIDTH / 2.0, y, 0.0)
}
