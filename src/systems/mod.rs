//! Module contenant tous les systèmes du jeu.
//!
//! Les systèmes contiennent la logique de jeu (mouvement, fusion, spawn, etc.).
mod camera;
mod input;
mod merge;
mod movement;
mod spawn;

use crate::resources::board::Board;
use crate::utils::directions::Direction;
use bevy::prelude::*;

pub use camera::setup_camera;
pub use input::handle_input;
pub use merge::merge_tiles;
pub use movement::move_tiles;
pub use spawn::spawn_random_tile;

/// Système de démarrage : initialise la grille et spawn les premières tuiles.
pub fn setup_game(mut commands: Commands) {
    // Initialiser la grille et la direction
    commands.insert_resource(Board::new());
    commands.insert_resource(Direction::None);
}

/// Système pour spawner une nouvelle tuile après un mouvement
pub fn spawn_new_tile_after_move(
    mut commands: Commands,
    mut board: ResMut<Board>,
    last_direction: Res<Direction>,
) {
    // Spawner une nouvelle tuile seulement si un mouvement a eu lieu
    if *last_direction != Direction::None {
        spawn::spawn_random_tile(commands, board);
    }
}
