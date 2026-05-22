// src/states.rs
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    InGame,
    Won,
    GameOver,
}
