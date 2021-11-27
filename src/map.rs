use bevy::prelude::*;
use rand::Rng;

pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;
pub const SPRITE_SIZE: f32 = 32.0;

const SPRITE_GRASS: &str = "textures/tiles/grass.png";
const SPRITE_TUFT: &str = "textures/tiles/tuft.png";

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let grass_handle = materials.add(asset_server.load(SPRITE_GRASS).into());
    let tuft_handle = materials.add(asset_server.load(SPRITE_TUFT).into());

    let mut rng = rand::thread_rng();

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let translation = Vec3::new(
                x as f32 - MAP_WIDTH as f32 / 2.0,
                y as f32 - MAP_HEIGHT as f32 / 2.0,
                0.0,
            ) * SPRITE_SIZE
                * crate::SPRITE_ZOOM
                + (Vec3::X + Vec3::Y) * crate::SPRITE_ZOOM * SPRITE_SIZE / 2.0;
            let transform = Transform {
                translation,
                scale: Vec3::splat(crate::SPRITE_ZOOM),
                ..Default::default()
            };

            commands.spawn().insert_bundle(SpriteBundle {
                material: grass_handle.clone(),
                transform,
                sprite: Sprite::new(Vec2::splat(SPRITE_SIZE)),
                ..Default::default()
            });

            if rng.gen_range(0..5) == 0 {
                commands.spawn().insert_bundle(SpriteBundle {
                    material: tuft_handle.clone(),
                    transform,
                    sprite: Sprite::new(Vec2::splat(SPRITE_SIZE)),
                    ..Default::default()
                });
            }
        }
    }
}
