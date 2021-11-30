use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const STEPS: u32 = 4;
const OFFSET_DOWN: u32 = 0;
const OFFSET_LEFT: u32 = STEPS;
const OFFSET_RIGHT: u32 = 2 * STEPS;
const OFFSET_UP: u32 = 3 * STEPS;

const SPRITE_SHEET: &str = "textures/player.png";
pub const SPRITE_WIDTH: f32 = 14.0;
pub const SPRITE_HEIGHT: f32 = 25.0;

const STEP_DURATION_SECONDS: f32 = 0.15;

pub struct StepTimer(Timer);

#[derive(Bundle)]
pub struct AnimateBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
    step_timer: StepTimer,
}

pub fn add_bundles_to_entity(
    entity: &mut bevy::ecs::system::EntityCommands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load(SPRITE_SHEET);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT),
        STEPS as usize,
        4,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    entity
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .insert(StepTimer(Timer::from_seconds(STEP_DURATION_SECONDS, true)));
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(
        &mut StepTimer,
        &mut TextureAtlasSprite,
        &mut Transform,
        &RigidBodyVelocity,
    )>,
) {
    for (mut timer, mut sprite, mut transform, rigid_body_velocity) in query.iter_mut() {
        // current animation frame
        let mut step_offset = sprite.index % STEPS;
        let mut base_offset = sprite.index - step_offset;

        // progress time
        timer.0.tick(time.delta());

        if timer.0.finished() {
            // take a step
            if rigid_body_velocity.linvel.magnitude() > 0.0 {
                step_offset += 1;
                step_offset %= STEPS;
            } else {
                step_offset = 0;
            }

            // calculate new direction only when taking a step
            if rigid_body_velocity.linvel.x.abs() > rigid_body_velocity.linvel.y.abs() {
                // moving horizontally
                if rigid_body_velocity.linvel.x > 0.0 {
                    base_offset = OFFSET_RIGHT;
                } else {
                    base_offset = OFFSET_LEFT;
                }
            } else {
                // movign vertically
                if rigid_body_velocity.linvel.y > 0.0 {
                    base_offset = OFFSET_UP;
                } else {
                    base_offset = OFFSET_DOWN;
                }
            }
        }

        sprite.index = base_offset + step_offset;

        // account for 1 pixel of padding around the sprite, and an extra pixel under the feet.
        transform.translation.z = crate::z_from_y(transform.translation.y, SPRITE_HEIGHT - 4.0);
    }
}
