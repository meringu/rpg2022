use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const MOUSE_WALKING_SENSITIVITY: f32 = 15.0;
const WALKING_SPEED: f32 = 75.0;

#[derive(Default)]
pub struct ClickStart(Vec2);

#[derive(Bundle, Default)]
pub struct InputBundle {
    pub click_start: ClickStart,
}

pub fn input(
    windows: Res<Windows>,
    mouse_buttons: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut ClickStart, &mut RigidBodyVelocity)>,
) {
    if let Some(window) = windows.get_primary() {
        for (mut click_start, mut rigid_body_velocity) in query.iter_mut() {
            let mut v = Vec2::ZERO;

            let u = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
            let d = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
            let l = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
            let r = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

            // if keyboard moving horizonally
            if l ^ r {
                if l {
                    v -= Vec2::X;
                } else {
                    v += Vec2::X;
                }
            }

            // if keyboard moving vertically
            if u ^ d {
                if d {
                    v -= Vec2::Y;
                } else {
                    v += Vec2::Y;
                }
            }

            // no input from keyboard, try mouse
            if v == Vec2::ZERO {
                if let Some(pos) = window.cursor_position() {
                    // Update start click position
                    if mouse_buttons.just_pressed(MouseButton::Left) {
                        click_start.0 = pos;
                    }

                    // Calculate velocity as distance from start click position
                    if mouse_buttons.pressed(MouseButton::Left) {
                        v = pos - click_start.0;

                        // scale to screen
                        v /= (window.height() * window.height() + window.width() * window.width())
                            .sqrt();

                        // apply sensitivity to create a small circle of control around the first click
                        v *= MOUSE_WALKING_SENSITIVITY;
                    }
                }
            }

            // clamp to 1
            if v.length() > 1.0 {
                v /= v.length()
            }

            // multiply by walking speed
            if v.length() > 0.0 {
                v *= WALKING_SPEED;
            }

            rigid_body_velocity.linvel = v.into();
        }
    }
}
