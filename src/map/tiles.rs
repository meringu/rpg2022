use super::{MAP_HEIGHT, MAP_WIDTH, SPRITE_SIZE};
use bevy::prelude::*;
use rand::Rng;

const SPRITE_GRASS: &str = "textures/tiles/grass.png";
const SPRITE_TUFT: &str = "textures/tiles/tuft.png";

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let grass_handle = materials.add(asset_server.load(SPRITE_GRASS).into());
    let tuft_handle = materials.add(asset_server.load(SPRITE_TUFT).into());

    for x in 0..(MAP_WIDTH / SPRITE_SIZE) as u32 {
        for y in 0..(MAP_HEIGHT / SPRITE_SIZE) as u32 {
            let transform = Transform {
                translation: Vec3::new(
                    (x as f32 + 0.5) * SPRITE_SIZE as f32,
                    (y as f32 + 0.5) * SPRITE_SIZE as f32,
                    0.0,
                ),
                ..Default::default()
            };

            commands.spawn().insert_bundle(SpriteBundle {
                material: grass_handle.clone(),
                transform,
                ..Default::default()
            });

            if rng.gen_range(0..5) == 0 {
                commands.spawn().insert_bundle(SpriteBundle {
                    material: tuft_handle.clone(),
                    transform,
                    ..Default::default()
                });
            }
        }
    }
}
