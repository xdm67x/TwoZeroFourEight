// src/input/mod.rs
pub mod resources;
mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use resources::Direction;
use systems::handle_input;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Direction>().add_systems(
            Update,
            handle_input.run_if(
                in_state(AppState::InGame)
                    .or(in_state(AppState::Won))
                    .or(in_state(AppState::GameOver)),
            ),
        );
    }
}
