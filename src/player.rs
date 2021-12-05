use crate::map;
use crate::z::ZSync;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const MOUSE_WALKING_SENSITIVITY: f32 = 15.0;
const WALKING_SPEED: f32 = 75.0;

const STEPS: u32 = 4;
const OFFSET_DOWN: u32 = 0;
const OFFSET_LEFT: u32 = STEPS;
const OFFSET_RIGHT: u32 = 2 * STEPS;
const OFFSET_UP: u32 = 3 * STEPS;

const SPRITE_SHEET: &str = "textures/player.png";
pub const SPRITE_WIDTH: f32 = 12.0;
pub const SPRITE_HEIGHT: f32 = 23.0;
pub const SPRITE_SHEET_PADDING: f32 = 1.0;

const STEP_DURATION_SECONDS: f32 = 0.15;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(system.system());
    }
}

#[derive(Default)]
pub struct ClickStart(Vec2);

pub struct StepTimer(Timer);

#[derive(Default)]
pub struct Player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(RigidBodyBundle {
            position: Vec3::new(map::MAP_WIDTH / 2.0, map::MAP_HEIGHT / 2.0, 0.0).into(),
            mass_properties: RigidBodyMassProps {
                flags: RigidBodyMassPropsFlags::ROTATION_LOCKED,
                ..Default::default()
            },
            ..Default::default()
        })
        // This collider can be used to detect collisions with the full height of the player
        .insert_bundle(ColliderBundle {
            collider_type: ColliderType::Sensor,
            shape: ColliderShape::capsule(
                Vec2::new(0.0, (SPRITE_HEIGHT - SPRITE_WIDTH) / 2.0).into(),
                Vec2::new(0.0, (SPRITE_WIDTH - SPRITE_HEIGHT) / 2.0).into(),
                SPRITE_WIDTH / 2.0,
            ),
            ..Default::default()
        })
        .insert(ClickStart::default())
        .insert(ColliderPositionSync::Discrete)
        .insert(Player::default())
        .with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                        asset_server.load(SPRITE_SHEET),
                        Vec2::new(
                            SPRITE_WIDTH + SPRITE_SHEET_PADDING * 2.0,
                            SPRITE_HEIGHT + SPRITE_SHEET_PADDING * 2.0,
                        ),
                        STEPS as usize,
                        4,
                    )),
                    transform: Transform {
                        translation: Vec3::ZERO,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(StepTimer(Timer::from_seconds(STEP_DURATION_SECONDS, true)))
                .insert(ZSync(-SPRITE_HEIGHT / 2.0));
            // This collider is used for collision when walking
            parent.spawn_bundle(ColliderBundle {
                shape: ColliderShape::ball(SPRITE_WIDTH / 2.0),
                position: Vec2::new(0.0, (SPRITE_WIDTH - SPRITE_HEIGHT) / 2.0).into(),
                ..Default::default()
            });
        });
}

pub fn system(
    windows: Res<Windows>,
    time: Res<Time>,
    mouse_buttons: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut ClickStart, &mut RigidBodyVelocity, &Children)>,
    mut children_query: Query<(&mut StepTimer, &mut TextureAtlasSprite)>,
) {
    if let Some(window) = windows.get_primary() {
        for (mut click_start, mut rigid_body_velocity, children) in query.iter_mut() {
            for &child in children.iter() {
                if let Ok((mut timer, mut sprite)) = children_query.get_mut(child) {
                    let mut velocity = Vec2::ZERO;

                    // get relevant keyboard presses
                    let up =
                        keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
                    let down =
                        keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
                    let left =
                        keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
                    let right = keyboard_input.pressed(KeyCode::D)
                        || keyboard_input.pressed(KeyCode::Right);

                    // if keyboard moving horizonally
                    if left ^ right {
                        if left {
                            velocity -= Vec2::X;
                        } else {
                            velocity += Vec2::X;
                        }
                    }

                    // if keyboard moving vertically
                    if up ^ down {
                        if down {
                            velocity -= Vec2::Y;
                        } else {
                            velocity += Vec2::Y;
                        }
                    }

                    // no input from keyboard, try mouse
                    if velocity == Vec2::ZERO {
                        if let Some(pos) = window.cursor_position() {
                            // Update start click position
                            if mouse_buttons.just_pressed(MouseButton::Left) {
                                click_start.0 = pos;
                            }

                            // Calculate velocity as distance from start click position
                            if mouse_buttons.pressed(MouseButton::Left) {
                                velocity = pos - click_start.0;

                                // scale to screen
                                velocity /= (window.height() * window.height()
                                    + window.width() * window.width())
                                .sqrt();

                                // apply sensitivity to create a small circle of control around the first click
                                velocity *= MOUSE_WALKING_SENSITIVITY;
                            }
                        }
                    }

                    // clamp to 1
                    if velocity.length() > 1.0 {
                        velocity /= velocity.length()
                    }

                    // multiply by walking speed
                    if velocity.length() > 0.0 {
                        velocity *= WALKING_SPEED;
                    }

                    // progress time
                    timer.0.tick(time.delta());

                    // current animation frame
                    let mut step_offset = sprite.index % STEPS;
                    let mut base_offset = sprite.index - step_offset;

                    if timer.0.finished() {
                        // take a step
                        if velocity.length() > 0.0 {
                            step_offset += 1;
                            step_offset %= STEPS;
                        } else {
                            step_offset = 0;
                        }
                    }

                    if velocity.x.abs() > velocity.y.abs() {
                        // moving horizontally
                        if velocity.x > 0.0 {
                            base_offset = OFFSET_RIGHT;
                        } else if velocity.x < 0.0 {
                            base_offset = OFFSET_LEFT;
                        }
                    } else {
                        // moving vertically
                        if velocity.y > 0.0 {
                            base_offset = OFFSET_UP;
                        } else if velocity.y < 0.0 {
                            base_offset = OFFSET_DOWN;
                        }
                    }

                    sprite.index = base_offset + step_offset;

                    rigid_body_velocity.linvel = velocity.into();
                }
            }
        }
    }
}
