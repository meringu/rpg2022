#[cfg(target_arch = "wasm32")]
mod web_fullscreen;

#[cfg(target_arch = "wasm32")]
pub use web_fullscreen::*;
