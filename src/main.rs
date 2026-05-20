// src/main.rs
mod board;
mod camera;
mod components;
mod input;
mod render;
mod states;
mod ui;

use bevy::prelude::*;
use camera::CameraPlugin;
use board::BoardPlugin;
use input::InputPlugin;
use render::RenderPlugin;
use states::AppState;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins((CameraPlugin, RenderPlugin, BoardPlugin, InputPlugin, UIPlugin))
        .run();
}
