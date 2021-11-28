use bevy::prelude::*;

#[derive(Default)]
pub struct Control {
    pub mode: ControlMode,
    pub click_start_location: Vec2,
}

#[derive(PartialEq)]
pub enum ControlMode {
    None,
    Keyboard,
    Mouse,
}

impl Default for ControlMode {
    // TODO: derive once https://github.com/rust-lang/rust/issues/87517
    fn default() -> Self {
        Self::None
    }
}
