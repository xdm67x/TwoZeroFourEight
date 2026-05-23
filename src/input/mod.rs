// src/input/mod.rs
pub mod resources;
mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use resources::Direction;
use systems::{handle_input, handle_touch_input};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Direction>()
            .add_systems(
                Update,
                handle_input.in_set(InputSet).run_if(
                    in_state(AppState::InGame)
                        .or(in_state(AppState::Won))
                        .or(in_state(AppState::GameOver)),
                ),
            )
            .add_systems(
                Update,
                handle_touch_input
                    .in_set(InputSet)
                    .after(handle_input)
                    .run_if(
                        in_state(AppState::InGame)
                            .or(in_state(AppState::Won))
                            .or(in_state(AppState::GameOver)),
                    ),
            );
    }
}
