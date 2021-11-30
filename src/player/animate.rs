use crate::z::ZSync;
use bevy::prelude::*;

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

#[derive(Default)]
pub struct LastPos(Option<Vec3>);

#[derive(Bundle)]
pub struct AnimateBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
    step_timer: StepTimer,
}

pub fn add_to_parent(
    parent: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    parent
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlases.add(TextureAtlas::from_grid(
                asset_server.load(SPRITE_SHEET),
                Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT),
                STEPS as usize,
                4,
            )),
            transform: Transform {
                translation: Vec3::new(SPRITE_WIDTH / 2.0, SPRITE_HEIGHT / 2.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(StepTimer(Timer::from_seconds(STEP_DURATION_SECONDS, true)))
        .insert(LastPos::default())
        .insert(ZSync::default());
}

pub fn animate(
    time: Res<Time>,
    mut query: Query<(
        &mut StepTimer,
        &mut TextureAtlasSprite,
        &mut LastPos,
        &GlobalTransform,
    )>,
) {
    for (mut timer, mut sprite, mut last_pos, global) in query.iter_mut() {
        // progress time
        timer.0.tick(time.delta());

        if let Some(pos) = last_pos.0 {
            let velocity = global.translation - pos;

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

                // reduce jitter by calculating the new direction only when taking a step
                if velocity.x.abs() > velocity.y.abs() {
                    // moving horizontally
                    if velocity.x > 0.0 {
                        base_offset = OFFSET_RIGHT;
                    } else {
                        base_offset = OFFSET_LEFT;
                    }
                } else {
                    // moving vertically
                    if velocity.y > 0.0 {
                        base_offset = OFFSET_UP;
                    } else {
                        base_offset = OFFSET_DOWN;
                    }
                }
            }

            sprite.index = base_offset + step_offset;
        } else {
            // player just spawned, reset frame
            sprite.index = 0;
        }

        last_pos.0 = Some(global.translation);
    }
}
