use bevy::prelude::*;

/// Composant représentant la position d'une tuile sur la grille.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// Ligne (0 = haut, 3 = bas).
    pub row: usize,
    /// Colonne (0 = gauche, 3 = droite).
    pub col: usize,
}
