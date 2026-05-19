use bevy::prelude::*;

/// Taille de la grille (4×4 pour 2048).
pub const BOARD_SIZE: usize = 4;

/// Représentation de la grille de jeu.
/// Contient les entités des tuiles pour un accès rapide.
#[derive(Resource, Default)]
pub struct Board {
    /// Grille 4×4 contenant les Entity des tuiles (Option pour les cases vides).
    pub tiles: [[Option<Entity>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Vide complètement la grille (met toutes les cases à None).
    pub fn clear(&mut self) {
        self.tiles = [[None; BOARD_SIZE]; BOARD_SIZE];
    }
}

/// Vérifie s'il reste au moins un mouvement possible dans la grille des valeurs.
/// Un mouvement est possible si deux cases adjacentes (horizontalement ou verticalement)
/// contiennent la même valeur, OU s'il y a une case vide.
pub fn has_moves(grid: &[[Option<u32>; BOARD_SIZE]; BOARD_SIZE]) -> bool {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            // Case vide = mouvement possible
            if grid[row][col].is_none() {
                return true;
            }
            let current = grid[row][col].unwrap();
            // Voisin de droite
            if col + 1 < BOARD_SIZE {
                if let Some(right) = grid[row][col + 1] {
                    if right == current {
                        return true;
                    }
                }
            }
            // Voisin du bas
            if row + 1 < BOARD_SIZE {
                if let Some(down) = grid[row + 1][col] {
                    if down == current {
                        return true;
                    }
                }
            }
        }
    }
    false
}

/// Vérifie si la grille contient une tuile de valeur 2048 (condition de victoire).
pub fn has_2048(grid: &[[Option<u32>; BOARD_SIZE]; BOARD_SIZE]) -> bool {
    grid.iter()
        .flatten()
        .any(|cell| cell.is_some_and(|v| v >= 2048))
}
