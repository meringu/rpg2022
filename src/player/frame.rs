pub const STEPS: u32 = 4;
const OFFSET_DOWN: u32 = 0;
const OFFSET_LEFT: u32 = STEPS;
const OFFSET_RIGHT: u32 = 2 * STEPS;
const OFFSET_UP: u32 = 3 * STEPS;

#[derive(Default)]
pub struct FrameBase {
    offset: u32,
}

#[derive(Default)]
pub struct FrameStep {
    offset: u32,
}

impl FrameBase {
    pub fn set_left(&mut self) {
        self.offset = OFFSET_LEFT;
    }

    pub fn set_right(&mut self) {
        self.offset = OFFSET_RIGHT;
    }

    pub fn set_up(&mut self) {
        self.offset = OFFSET_UP;
    }

    pub fn set_down(&mut self) {
        self.offset = OFFSET_DOWN;
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }
}

impl FrameStep {
    pub fn stop(&mut self) {
        self.offset = 0;
    }

    pub fn step(&mut self) {
        self.offset = (self.offset + 1) % STEPS;
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }
}
