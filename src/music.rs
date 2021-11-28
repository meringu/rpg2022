use bevy::prelude::*;
use bevy_kira_audio::Audio;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_looped(asset_server.load("sound/theme.ogg"));
}
