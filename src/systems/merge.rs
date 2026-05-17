use crate::components::{Position, Tile};
use crate::resources::score::Score;
use bevy::prelude::*;

/// Système pour fusionner les tuiles adjacentes de même valeur.
pub fn merge_tiles(
    mut commands: Commands,
    mut query: Query<(Entity, &Position, &mut Tile)>,
    mut score: ResMut<Score>,
) {
    // TODO: Implémenter la logique de fusion
    // 1. Parcourir la grille dans l'ordre du mouvement
    // 2. Fusionner les tuiles adjacentes de même valeur
    // 3. Mettre à jour le score
    // 4. Marquer les tuiles fusionnées pour éviter les doubles fusions
}
