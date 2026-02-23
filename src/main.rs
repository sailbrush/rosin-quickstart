#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rosin::prelude::*;

use quickstart::{State, main_view};

fn main() {
    env_logger::init();

    let window = WindowDesc::new(callback!(main_view))
        .title("Rosin Quickstart")
        .size(400, 300)
        .min_size(250, 150);

    AppLauncher::new(window)
        .run(State::default(), TranslationMap::default())
        .expect("Failed to launch");
}
