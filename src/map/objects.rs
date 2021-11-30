use super::{MAP_HEIGHT, MAP_WIDTH};
use bevy::prelude::*;
use rand::Rng;

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

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for object in OBJECTS.iter() {
        let handle = materials.add(asset_server.load(object.sprite_path).into());
        for _ in 0..object.count {
            let x = rng.gen_range(0..MAP_WIDTH as u32) as f32;
            let y = rng.gen_range(0..MAP_HEIGHT as u32) as f32;

            commands.spawn_bundle(SpriteBundle {
                material: handle.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, crate::z_from_y(y, object.sprite_height)),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
