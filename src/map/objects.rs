use super::{MAP_HEIGHT, MAP_WIDTH};
use crate::door::Door;
use crate::z::ZSync;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

struct ObjectDetails<'a> {
    count: usize,
    path: &'a str,
    size: Vec2,
    offset: Vec2,
    hitboxes: Vec<(Vec2, ColliderShape)>,
    doors: Vec<(Vec2, ColliderShape)>,
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let objects = vec![
        ObjectDetails {
            count: 10,
            path: "textures/house.png",
            size: Vec2::new(76.0, 55.0),
            offset: Vec2::new(2.0, 0.0),
            hitboxes: vec![(Vec2::new(38.0, 20.0), ColliderShape::cuboid(36.0, 20.0))],
            doors: vec![(Vec2::new(48.0, 16.0), ColliderShape::cuboid(16.0, 16.0))],
        },
        ObjectDetails {
            count: 7,
            path: "textures/teepee.png",
            size: Vec2::new(46.0, 61.0),
            offset: Vec2::new(0.0, 0.0),
            hitboxes: vec![(Vec2::new(23.0, 10.0), ColliderShape::cuboid(23.0, 10.0))],
            doors: vec![],
        },
        ObjectDetails {
            count: 1,
            path: "textures/double_teepee.png",
            size: Vec2::new(92.0, 61.0),
            offset: Vec2::new(0.0, 0.0),
            hitboxes: vec![(Vec2::new(46.0, 10.0), ColliderShape::cuboid(46.0, 10.0))],
            doors: vec![],
        },
        ObjectDetails {
            count: 20,
            path: "textures/oak_tree.png",
            size: Vec2::new(62.0, 111.0),
            offset: Vec2::new(0.0, 0.0),
            hitboxes: vec![(Vec2::new(31.0, 10.0), ColliderShape::ball(10.0))],
            doors: vec![],
        },
    ];

    let mut rng = rand::thread_rng();

    let mut door_count = 0;

    for object in objects.iter() {
        let handle = materials.add(asset_server.load(object.path).into());
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
                                    object.size.x / 2.0 + object.offset.x,
                                    object.size.y / 2.0 + object.offset.y,
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
                        position: (object.offset + *offset + Vec2::new(x, y)).into(),
                        body_type: RigidBodyType::Static,
                        ..Default::default()
                    })
                    .insert_bundle(ColliderBundle {
                        shape: shape.clone(),
                        ..Default::default()
                    })
                    .insert(ColliderPositionSync::Discrete);
            }

            for (offset, shape) in object.doors.iter() {
                commands
                    .spawn_bundle(ColliderBundle {
                        position: (object.offset + *offset + Vec2::new(x, y)).into(),
                        shape: shape.clone(),
                        ..Default::default()
                    })
                    .insert(Door(door_count));

                door_count += 1;
            }
        }
    }
}
