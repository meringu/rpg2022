use super::{Control, ControlMode, FrameBase, Velocity};
use bevy::prelude::*;

const MOUSE_WALKING_SENSITIVITY: f32 = 15.0;

pub fn input(
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<(&mut Velocity, &mut FrameBase, &mut Control)>,
) {
    println!("foo");
    if let Some(window) = windows.get_primary() {
        for (mut velocity, mut frame, mut control) in query.iter_mut() {
            // just clicked
            if buttons.just_pressed(MouseButton::Left) {
                // update first click with current position
                control.click_start_location = match window.cursor_position() {
                    Some(v) => v,
                    None => return,
                };

                control.mode = ControlMode::Mouse
            }

            // if we release while moving with the mouse
            if control.mode == ControlMode::Mouse && buttons.just_released(MouseButton::Left) {
                control.mode = ControlMode::None;
                velocity.0 = Vec3::ZERO;
            }

            // Calculate movement
            if control.mode == ControlMode::Mouse {
                if let Some(pos) = window.cursor_position() {
                    let mut dir = pos - control.click_start_location;

                    // scale to screen
                    dir /= (window.height() * window.height() + window.width() * window.width())
                        .sqrt();

                    // apply sensitivity to create a small circle of control around the first click
                    dir *= MOUSE_WALKING_SENSITIVITY;

                    // clamp to 1
                    if dir.length() > 1.0 {
                        dir /= dir.length()
                    }

                    velocity.0 = Vec3::new(dir.x, dir.y, 0.0);
                }

                // calculate sprite direction. dont' update if not moving
                if velocity.0.x.abs() > velocity.0.y.abs() {
                    // moving horizontally
                    if velocity.0.x > 0.0 {
                        frame.set_right();
                    } else {
                        frame.set_left();
                    }
                } else {
                    // movign vertically
                    if velocity.0.y > 0.0 {
                        frame.set_up();
                    } else {
                        frame.set_down();
                    }
                }
            }
        }
    }
}
