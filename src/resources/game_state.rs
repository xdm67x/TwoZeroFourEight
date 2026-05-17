use bevy::prelude::*;

/// État global du jeu.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// Le jeu est en cours.
    Playing,
    /// Le joueur a atteint 2048 et gagné.
    Won,
    /// Plus de mouvements possibles, partie terminée.
    GameOver,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Playing
    }
}
