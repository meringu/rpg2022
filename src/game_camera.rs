use crate::map;
use crate::player;
use bevy::prelude::*;

const CAMERA_PADDING: f32 = 32.0;

#[derive(Default)]
pub struct GameCamera;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system_to_stage(CoreStage::PostUpdate, follow_player.system());
    }
}

#[allow(clippy::type_complexity)]
fn follow_player(
    mut query_set: QuerySet<(
        Query<(&Transform, &player::Player)>,
        Query<(&GameCamera, &mut Transform)>,
    )>,
) {
    let mut pos = Vec3::ZERO;

    for (transform, _player) in query_set.q0().iter() {
        pos = transform.translation;
    }

    for (_camera, mut cam) in query_set.q1_mut().iter_mut() {
        // move camera if player out of bounds
        let horizonal_bound = crate::WINDOW_WIDTH / 2.0
            - (player::SPRITE_WIDTH / 2.0 + CAMERA_PADDING) * crate::SPRITE_ZOOM;

        if pos.x - cam.translation.x - horizonal_bound > 0.0 {
            cam.translation.x = pos.x - horizonal_bound
        }

        if pos.x - cam.translation.x + horizonal_bound < 0.0 {
            cam.translation.x = pos.x + horizonal_bound
        }

        let vertical_bound = crate::WINDOW_HEIGHT / 2.0
            - (player::SPRITE_HEIGHT / 2.0 + CAMERA_PADDING) * crate::SPRITE_ZOOM;

        if pos.y - cam.translation.y - vertical_bound > 0.0 {
            cam.translation.y = pos.y - vertical_bound
        }

        if pos.y - cam.translation.y + vertical_bound < 0.0 {
            cam.translation.y = pos.y + vertical_bound
        }

        // move camera if map out of bounds
        let map_half_width = map::MAP_WIDTH as f32 / 2.0 * map::SPRITE_SIZE * crate::SPRITE_ZOOM;
        if -map_half_width > cam.translation.x - crate::WINDOW_WIDTH / 2.0 {
            cam.translation.x = -map_half_width + crate::WINDOW_WIDTH / 2.0;
        }
        if map_half_width < cam.translation.x + crate::WINDOW_WIDTH / 2.0 {
            cam.translation.x = map_half_width - crate::WINDOW_WIDTH / 2.0;
        }

        let map_half_height = map::MAP_HEIGHT as f32 / 2.0 * map::SPRITE_SIZE * crate::SPRITE_ZOOM;
        if -map_half_height > cam.translation.y - crate::WINDOW_HEIGHT / 2.0 {
            cam.translation.y = -map_half_height + crate::WINDOW_HEIGHT / 2.0;
        }
        if map_half_height < cam.translation.y + crate::WINDOW_HEIGHT / 2.0 {
            cam.translation.y = map_half_height - crate::WINDOW_HEIGHT / 2.0;
        }
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameCamera::default());
}
