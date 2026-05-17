use crate::components::{Position, Tile};
use crate::utils::directions::Direction;
use bevy::prelude::*;

/// Système pour déplacer les tuiles dans une direction donnée.
pub fn move_tiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Position, &mut Tile)>,
    direction: Res<Direction>,
) {
    // TODO: Implémenter la logique de mouvement
    // 1. Trier les tuiles selon la direction
    // 2. Déplacer chaque tuile vers le bord
    // 3. Mettre à jour les positions dans la grille
}
