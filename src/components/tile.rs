use bevy::prelude::*;

/// Composant représentant une tuile du jeu 2048.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Tile {
    /// Valeur de la tuile (2, 4, 8, ..., 2048).
    pub value: u32,
    /// Indique si la tuile a déjà été fusionnée pendant ce tour (pour éviter les doubles fusions).
    pub merged: bool,
}
