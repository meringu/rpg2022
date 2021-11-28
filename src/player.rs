use bevy::prelude::*;

const SPRITE_SHEET: &str = "textures/player.png";
pub const SPRITE_WIDTH: f32 = 12.0;
pub const SPRITE_HEIGHT: f32 = 23.0;

const SPRITE_FRAMES: u32 = 4;
const SPRITE_INDEX_DOWN: u32 = 0;
const SPRITE_INDEX_LEFT: u32 = SPRITE_FRAMES;
const SPRITE_INDEX_RIGHT: u32 = 2 * SPRITE_FRAMES;
const SPRITE_INDEX_UP: u32 = 3 * SPRITE_FRAMES;

const WALKING_SPEED: f32 = 75.0;
const MOUSE_WALKING_SENSITIVITY: f32 = 15.0;
const STEP_DURATION_SECONDS: f32 = 0.15;

#[derive(Default)]
pub struct Player {
    direction: Vec3,
    base_sprite_offset: u32,
    step_sprite_offset: u32,
    control_mode: ControlMode,

    // location of the first press that is moving the player
    first_click_location: Vec2,
}

#[derive(PartialEq, Debug)]
enum ControlMode {
    None,
    Keyboard,
    Mouse,
}

impl Default for ControlMode {
    // TODO: derive once https://github.com/rust-lang/rust/issues/87517
    fn default() -> Self {
        Self::None
    }
}

impl Player {
    // Returns the offset in the sprite sheet
    fn sprite_offset(&self) -> u32 {
        self.base_sprite_offset + self.step_sprite_offset
    }

    fn is_moving(&self) -> bool {
        self.direction != Vec3::ZERO
    }

    // Takes one step through the sprite sheet
    fn step(&mut self) {
        self.step_sprite_offset = if self.is_moving() {
            (self.step_sprite_offset + 1) % SPRITE_FRAMES
        } else {
            0
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(keyboard_input_system.system())
            .add_system(mouse_input_system.system())
            .add_system(animate_sprite_system.system());
    }
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Player>) {
    for mut player in query.iter_mut() {
        // calculated direction
        let mut dir = Vec3::ZERO;

        // if moving horizonally
        if keyboard_input.pressed(KeyCode::A) ^ keyboard_input.pressed(KeyCode::D) {
            if keyboard_input.pressed(KeyCode::A) {
                player.base_sprite_offset = SPRITE_INDEX_LEFT;
                dir -= Vec3::X;
            } else {
                player.base_sprite_offset = SPRITE_INDEX_RIGHT;
                dir += Vec3::X;
            }
        }

        // if moving vertically
        if keyboard_input.pressed(KeyCode::S) ^ keyboard_input.pressed(KeyCode::W) {
            if keyboard_input.pressed(KeyCode::S) {
                player.base_sprite_offset = SPRITE_INDEX_DOWN;
                dir -= Vec3::Y;
            } else {
                player.base_sprite_offset = SPRITE_INDEX_UP;
                dir += Vec3::Y;
            }
        }

        // If moving switch to keyboard mode or release control
        if dir != Vec3::ZERO {
            player.control_mode = ControlMode::Keyboard;
        }

        // If we're in keyboard mode, update player direction
        if player.control_mode == ControlMode::Keyboard {
            if dir == Vec3::ZERO {
                player.control_mode = ControlMode::None;
            }
            player.direction = dir;
        }
    }
}

fn mouse_input_system(
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<&mut Player>,
) {
    if let Some(window) = windows.get_primary() {
        for mut player in query.iter_mut() {
            // just clicked
            if buttons.just_pressed(MouseButton::Left) {
                // update first click with current position
                player.first_click_location = match window.cursor_position() {
                    Some(v) => v,
                    None => return,
                };

                player.control_mode = ControlMode::Mouse
            }

            // if we release while moving with the mouse
            if player.control_mode == ControlMode::Mouse && buttons.just_released(MouseButton::Left)
            {
                player.control_mode = ControlMode::None;
                player.direction = Vec3::ZERO;
            }

            // Calculate movement
            if player.control_mode == ControlMode::Mouse {
                if let Some(pos) = window.cursor_position() {
                    let mut velocity = pos - player.first_click_location;

                    // scale to screen
                    velocity /= (window.height() * window.height()
                        + window.width() * window.width())
                    .sqrt();

                    // apply sensitivity to create a small circle of control around the first click
                    velocity *= MOUSE_WALKING_SENSITIVITY;

                    // clamp to 1
                    if velocity.length() > 1.0 {
                        velocity /= velocity.length()
                    }

                    player.direction = Vec3::new(velocity.x, velocity.y, 0.0);
                }

                // calculate sprite direction. dont' update if not moving
                if player.direction.x.abs() > player.direction.y.abs() {
                    // moving horizontally
                    player.base_sprite_offset = if player.direction.x > 0.0 {
                        SPRITE_INDEX_RIGHT
                    } else {
                        SPRITE_INDEX_LEFT
                    }
                } else {
                    // movign vertically
                    player.base_sprite_offset = if player.direction.y > 0.0 {
                        SPRITE_INDEX_UP
                    } else {
                        SPRITE_INDEX_DOWN
                    }
                }
            }
        }
    }
}

fn animate_sprite_system(
    time: Res<Time>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &mut Transform,
        &mut Player,
    )>,
) {
    for (mut timer, mut sprite, _texture_atlas_handle, mut transform, mut player) in
        query.iter_mut()
    {
        // update sprite frame
        timer.tick(time.delta());
        if timer.finished() {
            player.step();
            sprite.index = player.sprite_offset();
        }

        // move player
        let delta =
            player.direction * WALKING_SPEED * time.delta().as_nanos() as f32 / 1_000_000_000.0;
        transform.translation += delta;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(SPRITE_SHEET);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT),
        16,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let transform = Transform {
        translation: Vec3::Z * 100.0,
        ..Default::default()
    };

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .insert(Timer::from_seconds(STEP_DURATION_SECONDS, true))
        .insert(Player::default());
}
