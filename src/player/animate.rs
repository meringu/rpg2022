use super::{FrameBase, FrameStep, Velocity, SPRITE_HEIGHT};
use bevy::prelude::*;

const WALKING_SPEED: f32 = 75.0;

#[allow(clippy::type_complexity)]
pub fn animate(
    time: Res<Time>,
    mut query: Query<(
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &mut Transform,
        &Velocity,
        &mut FrameStep,
        &FrameBase,
    )>,
) {
    for (
        mut timer,
        mut sprite,
        _texture_atlas_handle,
        mut transform,
        velocity,
        mut frame_step,
        frame_base,
    ) in query.iter_mut()
    {
        // update sprite frame
        timer.tick(time.delta());
        if timer.finished() {
            if velocity.0 == Vec3::ZERO {
                frame_step.stop();
            } else {
                frame_step.step();
            };

            sprite.index = frame_step.offset() + frame_base.offset();
        }

        // move player
        let delta = velocity.0 * WALKING_SPEED * time.delta().as_nanos() as f32 / 1_000_000_000.0;
        transform.translation += delta;
        // 1 pixel of padding around the sprite, and an extra pixel under the feet.
        transform.translation.z = crate::z_from_y(transform.translation.y, SPRITE_HEIGHT - 4.0);
    }
}
