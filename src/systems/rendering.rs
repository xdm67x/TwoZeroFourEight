//! Système de rendu : dessine le fond de la grille et gère l'affichage visuel.
//!
//! Ce module est responsable de :
//! - Afficher un fond sombre derrière la grille de jeu
//! - (Futur) Ajouter des animations, ombres, etc.

use crate::resources::board::BOARD_SIZE;
use bevy::prelude::*;

/// Taille d'une tuile en pixels (doit correspondre à celle dans game_logic.rs).
const TILE_SIZE: f32 = 100.0;
/// Espacement entre les tuiles.
const GAP: f32 = 12.0;

/// Taille totale du fond de la grille.
const GRID_BG_SIZE: f32 = BOARD_SIZE as f32 * TILE_SIZE + (BOARD_SIZE as f32 + 1.0) * GAP;

/// Système exécuté au Startup : dessine le fond de la grille.
///
/// Le fond est un grand rectangle sombre derrière toutes les tuiles.
/// Il est placé à z=0 pour que les tuiles (z=1) apparaissent devant.
pub fn setup_grid_background(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.106, 0.106, 0.118), // #1b1b1e
            custom_size: Some(Vec2::new(GRID_BG_SIZE, GRID_BG_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
