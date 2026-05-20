// src/states.rs
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    InGame,
    Paused,
    Won,
    GameOver,
}
