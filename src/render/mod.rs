// src/render/mod.rs
mod systems;

use bevy::prelude::*;
use systems::setup_grid_background;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid_background);
    }
}
