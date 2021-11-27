use bevy::prelude::*;

const SPRITE_INDEX_DOWN: u32 = 0;
const SPRITE_INDEX_LEFT: u32 = 4;
const SPRITE_INDEX_RIGHT: u32 = 8;
const SPRITE_INDEX_UP: u32 = 12;

#[derive(Default)]
struct Player {
    velocity: (i32, i32),
    base_sprite_offset: u32,
    step_sprite_offset: u32,
}

impl Player {
    // Returns the offset in the sprite sheet
    fn sprite_offset(&self) -> u32 {
        self.base_sprite_offset + self.step_sprite_offset
    }

    fn is_moving(&self) -> bool {
        self.velocity != (0, 0)
    }

    // Takes one step through the sprite sheet
    fn step(&mut self) {
        self.step_sprite_offset = if self.is_moving() {
            (self.step_sprite_offset + 1) % 4
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
        player.velocity = (0, 0);

        // moving horizonally
        if keyboard_input.pressed(KeyCode::A) ^ keyboard_input.pressed(KeyCode::D) {
            if keyboard_input.pressed(KeyCode::A) {
                player.base_sprite_offset = SPRITE_INDEX_LEFT;
                player.velocity.0 = -1;
            } else {
                player.base_sprite_offset = SPRITE_INDEX_RIGHT;
                player.velocity.0 = 1;
            }
        }

        // moving vertically
        if keyboard_input.pressed(KeyCode::S) ^ keyboard_input.pressed(KeyCode::W) {
            if keyboard_input.pressed(KeyCode::S) {
                player.base_sprite_offset = SPRITE_INDEX_DOWN;
                player.velocity.1 = -1;
            } else {
                player.base_sprite_offset = SPRITE_INDEX_UP;
                player.velocity.1 = 1;
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
        &mut Player,
    )>,
) {
    for (mut timer, mut sprite, _texture_atlas_handle, mut player) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            player.step();
            sprite.index = player.sprite_offset();
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 23.0), 16, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(4.0)),
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert(Player::default());
}
