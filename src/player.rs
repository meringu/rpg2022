use crate::game_camera::GameCamera;
use bevy::prelude::*;

const SPRITE_WIDTH: f32 = 16.0;
const SPRITE_HEIGHT: f32 = 23.0;
const SPRITE_ZOOM: f32 = 4.0;

const SPRITE_FRAMES: u32 = 4;
const SPRITE_INDEX_DOWN: u32 = 0;
const SPRITE_INDEX_LEFT: u32 = SPRITE_FRAMES;
const SPRITE_INDEX_RIGHT: u32 = 2 * SPRITE_FRAMES;
const SPRITE_INDEX_UP: u32 = 3 * SPRITE_FRAMES;

const WALKING_SPEED: f32 = 150.0;
const STEP_DURATION_SECONDS: f32 = 0.15;

#[derive(Default)]
struct Player {
    position: Vec3,
    direction: Vec3,
    base_sprite_offset: u32,
    step_sprite_offset: u32,
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
            .add_system(animate_sprite_system.system());
    }
}

fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Player>) {
    for mut player in query.iter_mut() {
        player.direction = Vec3::ZERO;

        // moving horizonally
        if keyboard_input.pressed(KeyCode::A) ^ keyboard_input.pressed(KeyCode::D) {
            if keyboard_input.pressed(KeyCode::A) {
                player.base_sprite_offset = SPRITE_INDEX_LEFT;
                player.direction -= Vec3::X;
            } else {
                player.base_sprite_offset = SPRITE_INDEX_RIGHT;
                player.direction += Vec3::X;
            }
        }

        // moving vertically
        if keyboard_input.pressed(KeyCode::S) ^ keyboard_input.pressed(KeyCode::W) {
            if keyboard_input.pressed(KeyCode::S) {
                player.base_sprite_offset = SPRITE_INDEX_DOWN;
                player.direction -= Vec3::Y;
            } else {
                player.base_sprite_offset = SPRITE_INDEX_UP;
                player.direction += Vec3::Y;
            }
        }

        if !player.is_moving() {
            player.step_sprite_offset = 0;
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
    mut game_camera_query: Query<&mut GameCamera>,
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
        player.position += delta;

        // move camera to follow player
        for mut camera in game_camera_query.iter_mut() {
            let horizonal_bound = crate::WINDOW_WIDTH / 2.0 - SPRITE_WIDTH / 2.0 * SPRITE_ZOOM;

            if player.position.x - camera.position.x - horizonal_bound > 0.0 {
                camera.position.x = player.position.x - horizonal_bound
            }

            if player.position.x - camera.position.x + horizonal_bound < 0.0 {
                camera.position.x = player.position.x + horizonal_bound
            }

            let vertical_bound = crate::WINDOW_HEIGHT / 2.0 - SPRITE_HEIGHT / 2.0 * SPRITE_ZOOM;

            if player.position.y - camera.position.y - vertical_bound > 0.0 {
                camera.position.y = player.position.y - vertical_bound
            }

            if player.position.y - camera.position.y + vertical_bound < 0.0 {
                camera.position.y = player.position.y + vertical_bound
            }

            transform.translation = player.position - camera.position;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/player.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT),
        16,
        1,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(SPRITE_ZOOM)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(STEP_DURATION_SECONDS, true))
        .insert(Player::default());
}
