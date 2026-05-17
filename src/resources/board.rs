use bevy::prelude::*;

/// Taille de la grille (4x4 pour 2048).
pub const BOARD_SIZE: usize = 4;

/// Représentation de la grille de jeu.
/// Contient les entités des tuiles pour un accès rapide.
#[derive(Resource, Default)]
pub struct Board {
    /// Grille 4x4 contenant les Entity des tuiles (Option pour les cases vides).
    pub tiles: [[Option<Entity>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Crée une nouvelle grille vide.
    pub fn new() -> Self {
        Self {
            tiles: [[None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    /// Vérifie si la grille est pleine (plus de mouvements possibles).
    pub fn is_full(&self) -> bool {
        self.tiles.iter().flatten().all(|tile| tile.is_some())
    }
}
