use bevy::prelude::*;
use rand::Rng;

pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;
pub const SPRITE_SIZE: f32 = 32.0;

const NUM_OAK_TREES: usize = 20;
const SPRITE_OAK_TREE: &str = "textures/oak_tree.png";
const OAK_TREE_HEIGHT: f32 = 111.0;

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
    let mut rng = rand::thread_rng();

    let oak_tree_handle = materials.add(asset_server.load(SPRITE_OAK_TREE).into());
    for _ in 0..NUM_OAK_TREES {
        let x = rng.gen_range(-(MAP_WIDTH as f32)..MAP_WIDTH as f32) / 2.0 * SPRITE_SIZE;
        let y = rng.gen_range(-(MAP_HEIGHT as f32)..MAP_HEIGHT as f32) / 2.0 * SPRITE_SIZE;

        commands.spawn().insert_bundle(SpriteBundle {
            material: oak_tree_handle.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, crate::z_from_y(y, OAK_TREE_HEIGHT)),
                ..Default::default()
            },
            ..Default::default()
        });
    }

    let grass_handle = materials.add(asset_server.load(SPRITE_GRASS).into());
    let tuft_handle = materials.add(asset_server.load(SPRITE_TUFT).into());

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let transform = Transform {
                translation: Vec3::new(
                    (x as f32 + 0.5) * SPRITE_SIZE,
                    (y as f32 + 0.5) * SPRITE_SIZE,
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
