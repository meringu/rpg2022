use crate::map;
use crate::player;
use bevy::prelude::*;

const CAMERA_PADDING: f32 = 32.0;
const DIAGONAL_PIXELS: f32 = 400.0;

#[derive(Default)]
struct GameCamera;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system_to_stage(CoreStage::PostUpdate, position_camera.system());
    }
}

fn calculate_zoom(window: &Window) -> f32 {
    let mut scale = (window.height() * window.height() + window.width() * window.width()).sqrt()
        / DIAGONAL_PIXELS;

    // Scale up in multiples of 1 unless the window is very small
    if scale > 2.0 {
        scale = scale.floor();
    } else if scale > 1.5 {
        scale = 1.5;
    } else if scale > 1.0 {
        scale = 1.0;
    }

    scale
}

#[allow(clippy::type_complexity)]
fn position_camera(
    windows: Res<Windows>,
    mut query_set: QuerySet<(
        Query<(&Transform, &player::Player)>,
        Query<(&GameCamera, &mut Transform)>,
    )>,
) {
    if let Some(window) = windows.get_primary() {
        let scale = calculate_zoom(window);

        // player position
        let mut pos = Vec3::ZERO;
        for (transform, _player) in query_set.q0().iter() {
            pos = transform.translation;
        }

        for (_camera, mut cam) in query_set.q1_mut().iter_mut() {
            cam.scale = Vec3::splat(1.0 / scale);
            cam.scale.z = 1.0;

            // move camera if player out of bounds
            let horizonal_bound =
                window.width() / 2.0 - (player::SPRITE_WIDTH / 2.0 + CAMERA_PADDING) * scale;

            // clip right
            if (pos.x - cam.translation.x) * scale > horizonal_bound {
                cam.translation.x = pos.x - horizonal_bound / scale;
            }

            // clip left
            if (pos.x - cam.translation.x) * scale < -horizonal_bound {
                cam.translation.x = pos.x + horizonal_bound / scale;
            }

            let vertical_bound =
                window.height() / 2.0 - (player::SPRITE_HEIGHT / 2.0 + CAMERA_PADDING) * scale;

            // clip top
            if (pos.y - cam.translation.y) * scale > vertical_bound {
                cam.translation.y = pos.y - vertical_bound / scale;
            }

            // clip bottom
            if (pos.y - cam.translation.y) * scale < -vertical_bound {
                cam.translation.y = pos.y + vertical_bound / scale;
            }

            // move camera if map out of bounds
            let map_width = map::MAP_WIDTH as f32 * map::SPRITE_SIZE;
            let window_half_width = window.width() / scale / 2.0;
            // clip right
            if cam.translation.x + window_half_width > map_width {
                cam.translation.x = map_width - window_half_width;
            }
            // clip left
            if cam.translation.x < window_half_width {
                cam.translation.x = window_half_width;
            }

            let map_height = map::MAP_HEIGHT as f32 * map::SPRITE_SIZE;
            let window_half_height = window.height() / scale / 2.0;
            // clip top
            if cam.translation.y + window_half_height > map_height {
                cam.translation.y = map_height - window_half_height;
            }
            // clip bottom
            if cam.translation.y < window_half_height {
                cam.translation.y = window_half_height;
            }
        }
    }
}

fn setup(mut commands: Commands) {
    let mut cam = OrthographicCameraBundle::new_2d();
    let player_pos = player::init_position();
    cam.transform.translation.x = player_pos.x;
    cam.transform.translation.y = player_pos.y;

    commands.spawn_bundle(cam).insert(GameCamera::default());
}
