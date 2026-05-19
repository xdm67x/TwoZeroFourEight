//! Jeu 2048 implémenté avec Bevy et compilé vers WASM.
//!
//! ## Architecture
//!
//! Le jeu suit le pattern ECS (Entity Component System) de Bevy :
//!
//! **Composants** → [`components`] : données attachées aux entités
//! - [`Tile`] : valeur et état d'une tuile (2, 4, 8...)
//! - [`Position`] : ligne/colonne sur la grille 4×4
//!
//! **Ressources** → [`resources`] : données globales partagées
//! - [`Board`] : la grille 4×4 qui référence les entités
//! - [`GameState`] : état de la partie (Playing / Won / GameOver)
//! - [`Score`] : score du joueur
//!
//! **Systèmes** → [`systems`] : la logique du jeu
//! - [`handle_input`] : capture les touches (flèches, R)
//! - [`process_turn`] : applique l'algorithme 2048 (mouvement + fusion)
//! - [`update_ui`] : met à jour le score et les overlays
//!
//! ## Chaîne de systèmes (ordre d'exécution)
//!
//! ```
//! Startup : setup_camera → setup_grid_background → setup_ui → spawn × 2
//! Update  : handle_input → process_turn → update_ui
//! ```

mod components;
mod resources;
mod systems;
mod utils;

use crate::resources::{board::Board, game_state::GameState, score::Score};
use crate::systems::game_logic::process_turn;
use crate::systems::rendering::setup_grid_background;
use crate::systems::ui::{setup_ui, update_ui};
use crate::systems::{handle_input, setup_camera};
use crate::utils::directions::Direction;
use bevy::prelude::*;

fn main() {
    App::new()
        // ── Plugins ──────────────────────────────────────────────
        .add_plugins(DefaultPlugins)
        // ── Ressources globales ──────────────────────────────────
        // init_resource appelle Default::default() pour créer la ressource
        .init_resource::<GameState>()
        .init_resource::<Score>()
        .init_resource::<Board>()
        .init_resource::<Direction>()
        // ── Systèmes de démarrage (Startup) ──────────────────────
        // Ils s'exécutent une seule fois au lancement.
        // On spawn 2 tuiles pour commencer la partie (règle du 2048).
        .add_systems(Startup, (setup_camera, setup_grid_background, setup_ui))
        .add_systems(
            Startup,
            // On utilise run_if pour ne spawner qu'une fois, mais on a besoin de 2 tuiles.
            // On va plutôt les spawner après setup via un système séparé.
            spawn_initial_tiles,
        )
        // ── Systèmes de mise à jour (Update) ─────────────────────
        // .chain() garantit l'ordre : input → logique → UI
        .add_systems(Update, (handle_input, process_turn, update_ui).chain())
        .run();
}

/// Spawn les 2 premières tuiles au démarrage.
///
/// On utilise un système séparé pour avoir accès aux ressources
/// après leur initialisation par les systèmes Startup précédents.
fn spawn_initial_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut board: ResMut<Board>,
) {
    use crate::systems::game_logic::spawn_random_tile;
    spawn_random_tile(&mut commands, &mut board, &asset_server);
    spawn_random_tile(&mut commands, &mut board, &asset_server);
}
