use bevy::prelude::*;

/// Direction de mouvement des tuiles.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None, // Aucune direction (état par défaut)
}

impl Default for Direction {
    fn default() -> Self {
        Direction::None
    }
}
