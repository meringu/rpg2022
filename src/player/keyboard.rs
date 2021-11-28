use super::{Control, ControlMode, FrameBase, Velocity};
use bevy::prelude::*;

pub struct KeyboardPlugin;

impl Plugin for KeyboardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(input.system());
    }
}

fn input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut FrameBase, &mut Control)>,
) {
    for (mut velocity, mut frame, mut control) in query.iter_mut() {
        // calculated direction
        let mut dir = Vec3::ZERO;

        // if moving horizonally
        if keyboard_input.pressed(KeyCode::A) ^ keyboard_input.pressed(KeyCode::D) {
            if keyboard_input.pressed(KeyCode::A) {
                frame.set_left();
                dir -= Vec3::X;
            } else {
                frame.set_right();
                dir += Vec3::X;
            }
        }

        // if moving vertically
        if keyboard_input.pressed(KeyCode::S) ^ keyboard_input.pressed(KeyCode::W) {
            if keyboard_input.pressed(KeyCode::S) {
                frame.set_down();
                dir -= Vec3::Y;
            } else {
                frame.set_up();
                dir += Vec3::Y;
            }
        }

        // If moving switch to keyboard mode or release control
        if dir != Vec3::ZERO {
            control.mode = ControlMode::Keyboard;
        }

        // If we're in keyboard mode, update player direction
        if control.mode == ControlMode::Keyboard {
            if dir == Vec3::ZERO {
                control.mode = ControlMode::None;
            } else {
                // cap diagonal at walk speed
                dir /= dir.length();
            }
            velocity.0 = dir;
        }
    }
}
