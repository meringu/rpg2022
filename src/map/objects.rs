use super::{MAP_HEIGHT, MAP_WIDTH};
use crate::z::ZSync;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

struct ObjectDetails<'a> {
    count: usize,
    sprite_path: &'a str,
    height: f32,
    width: f32,
    hitboxes: Vec<(Vec2, ColliderShape)>,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let objects = vec![
        ObjectDetails {
            count: 10,
            sprite_path: "textures/house.png",
            height: 55.0,
            width: 74.0,
            hitboxes: vec![(Vec2::new(37.0, 20.0), ColliderShape::cuboid(37.0, 20.0))],
        },
        ObjectDetails {
            count: 7,
            sprite_path: "textures/teepee.png",
            height: 61.0,
            width: 46.0,
            hitboxes: vec![(Vec2::new(23.0, 10.0), ColliderShape::cuboid(23.0, 10.0))],
        },
        ObjectDetails {
            count: 1,
            sprite_path: "textures/double_teepee.png",
            height: 61.0,
            width: 92.0,
            hitboxes: vec![(Vec2::new(46.0, 10.0), ColliderShape::cuboid(46.0, 10.0))],
        },
        ObjectDetails {
            count: 20,
            sprite_path: "textures/oak_tree.png",
            height: 111.0,
            width: 62.0,
            hitboxes: vec![(Vec2::new(31.0, 10.0), ColliderShape::ball(10.0))],
        },
    ];

    let mut rng = rand::thread_rng();

    for object in objects.iter() {
        let handle = materials.add(asset_server.load(object.sprite_path).into());
        for _ in 0..object.count {
            let x = rng.gen_range(0..MAP_WIDTH as u32) as f32;
            let y = rng.gen_range(0..MAP_HEIGHT as u32) as f32;

            commands
                .spawn()
                .insert(Transform {
                    translation: Vec3::new(x, y, 0.0),
                    ..Default::default()
                })
                .insert(GlobalTransform::default())
                .with_children(|parent| {
                    parent
                        .spawn_bundle(SpriteBundle {
                            material: handle.clone(),
                            transform: Transform {
                                translation: Vec3::new(
                                    object.width / 2.0,
                                    object.height / 2.0,
                                    0.0,
                                ),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(ZSync::default());
                });

            for (offset, shape) in object.hitboxes.iter() {
                commands
                    .spawn_bundle(RigidBodyBundle {
                        position: Vec2::new(x + offset.x, y + offset.y).into(),
                        body_type: RigidBodyType::Static,
                        ..Default::default()
                    })
                    .insert_bundle(ColliderBundle {
                        shape: shape.clone(),
                        material: ColliderMaterial {
                            restitution: 0.7,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(ColliderPositionSync::Discrete);
            }
        }
    }
}
