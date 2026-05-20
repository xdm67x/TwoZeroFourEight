// src/board/mod.rs
pub mod resources;
mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use resources::{Board, Score};
use systems::{process_turn, spawn_initial_tiles};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .init_resource::<Score>()
            .add_systems(Startup, spawn_initial_tiles)
            .add_systems(
                Update,
                process_turn.run_if(
                    in_state(AppState::InGame)
                        .or(in_state(AppState::Won))
                        .or(in_state(AppState::GameOver)),
                ),
            );
    }
}
