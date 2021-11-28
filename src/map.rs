use bevy::prelude::*;
use rand::Rng;

pub const MAP_WIDTH: usize = 32;
pub const MAP_HEIGHT: usize = 32;
pub const SPRITE_SIZE: f32 = 32.0;

const OBJECTS: &[ObjectDetails] = &[
    ObjectDetails {
        count: 10,
        sprite_path: "textures/house.png",
        sprite_height: 55.0,
    },
    ObjectDetails {
        count: 7,
        sprite_path: "textures/teepee.png",
        sprite_height: 61.0,
    },
    ObjectDetails {
        count: 1,
        sprite_path: "textures/double_teepee.png",
        sprite_height: 61.0,
    },
    ObjectDetails {
        count: 20,
        sprite_path: "textures/oak_tree.png",
        sprite_height: 111.0,
    },
];

struct ObjectDetails<'a> {
    count: usize,
    sprite_path: &'a str,
    sprite_height: f32,
}

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

    for object in OBJECTS.iter() {
        let handle = materials.add(asset_server.load(object.sprite_path).into());
        for _ in 0..object.count {
            let x = rng.gen_range(0..MAP_WIDTH) as f32 * SPRITE_SIZE;
            let y = rng.gen_range(0..MAP_HEIGHT) as f32 * SPRITE_SIZE;

            commands.spawn().insert_bundle(SpriteBundle {
                material: handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, crate::z_from_y(y, object.sprite_height)),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
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
