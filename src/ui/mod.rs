// src/ui/mod.rs
pub mod components;
mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use systems::{setup_ui, update_ui};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui).add_systems(
            Update,
            update_ui.run_if(
                in_state(AppState::InGame)
                    .or(in_state(AppState::Won))
                    .or(in_state(AppState::GameOver)),
            ),
        );
    }
}
