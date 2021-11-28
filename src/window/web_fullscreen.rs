use bevy::prelude::*;
use std::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

pub struct WebFullscreenPlugin;

impl Plugin for WebFullscreenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let channel = std::sync::mpsc::channel();
        let resize_sender: Sender<(f32, f32)> = channel.0;
        let resize_receiver: Receiver<(f32, f32)> = channel.1;

        app.insert_resource(Mutex::new(resize_sender))
            .insert_resource(Mutex::new(resize_receiver))
            .add_startup_system(setup_viewport_resize_system.system())
            .add_system(viewport_resize_system.system());
    }
}

fn web_window() -> web_sys::Window {
    web_sys::window().expect("could not get window")
}

fn web_window_size() -> (f32, f32) {
    let window = web_window();
    (
        window.inner_width().unwrap().as_f64().unwrap() as f32,
        window.inner_height().unwrap().as_f64().unwrap() as f32,
    )
}

fn setup_viewport_resize_system(resize_sender: Res<Mutex<Sender<(f32, f32)>>>) {
    let local_sender = resize_sender.lock().unwrap().clone();
    local_sender.send(web_window_size()).unwrap();

    gloo_events::EventListener::new(&web_window(), "resize", move |_event| {
        local_sender.send(web_window_size()).unwrap();
    })
    .forget();
}

fn viewport_resize_system(
    mut windows: ResMut<Windows>,
    resize_receiver: Res<Mutex<Receiver<(f32, f32)>>>,
) {
    if let Some(window) = windows.get_primary_mut() {
        if let Ok((x, y)) = resize_receiver.lock().unwrap().try_recv() {
            window.set_resolution(x, y);
        }
    }
}
