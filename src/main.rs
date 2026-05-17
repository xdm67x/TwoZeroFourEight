//! Jeu 2048 implémenté avec Bevy et compilé vers WASM.

mod components;
mod resources;
mod systems;
mod utils;

use crate::resources::{board::Board, game_state::GameState, score::Score};
use crate::systems::{
    handle_input, merge_tiles, move_tiles, setup_camera, setup_game, spawn_random_tile,
};
use crate::utils::directions::Direction;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GameState>()
        .init_resource::<Score>()
        .init_resource::<Board>()
        .init_resource::<Direction>()
        .add_systems(Startup, (setup_camera, setup_game, spawn_random_tile))
        .add_systems(
            Update,
            (handle_input, move_tiles, merge_tiles).chain(), // Exécute les systèmes dans l'ordre
        )
        .run();
}
