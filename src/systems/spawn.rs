use crate::components::{Position, Tile};
use crate::resources::board::{Board, BOARD_SIZE};
use bevy::prelude::*;
use bevy::sprite::Sprite;
use rand::Rng;

/// Système pour spawner une nouvelle tuile à une position aléatoire vide.
pub fn spawn_random_tile(mut commands: Commands, mut board: ResMut<Board>) {
    // Trouver toutes les positions vides
    let mut empty_positions = Vec::new();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board.tiles[row][col].is_none() {
                empty_positions.push((row, col));
            }
        }
    }

    // Si des positions vides existent
    if !empty_positions.is_empty() {
        // Choisir une position aléatoire
        let (row, col) = empty_positions[rand::thread_rng().gen_range(0..empty_positions.len())];

        // Valeur aléatoire (90% de chance d'avoir un 2)
        let value = if rand::random::<f32>() < 0.9 { 2 } else { 4 };

        // Créer la tuile
        let entity = commands
            .spawn((
                Tile {
                    value,
                    merged: false,
                },
                Position { row, col },
                Sprite {
                    color: get_tile_color(value),
                    custom_size: Some(Vec2::new(100.0, 100.0)),
                    ..default()
                },
                Transform::from_xyz(
                    col as f32 * 110.0 - 150.0, // Centrer la grille
                    row as f32 * 110.0 - 150.0,
                    0.0,
                ),
            ))
            .id();

        // Mettre à jour la grille
        board.tiles[row][col] = Some(entity);
    }
}

/// Retourne une couleur selon la valeur de la tuile
fn get_tile_color(value: u32) -> Color {
    match value {
        2 => Color::srgba(0.9, 0.85, 0.75, 1.0),
        4 => Color::srgba(0.9, 0.8, 0.7, 1.0),
        8 => Color::srgba(0.95, 0.6, 0.4, 1.0),
        16 => Color::srgba(0.95, 0.5, 0.3, 1.0),
        32 => Color::srgba(0.95, 0.4, 0.3, 1.0),
        64 => Color::srgba(0.95, 0.3, 0.2, 1.0),
        128 => Color::srgba(0.9, 0.8, 0.3, 1.0),
        256 => Color::srgba(0.9, 0.75, 0.3, 1.0),
        512 => Color::srgba(0.9, 0.7, 0.3, 1.0),
        1024 => Color::srgba(0.9, 0.65, 0.3, 1.0),
        2048 => Color::srgba(0.9, 0.6, 0.3, 1.0),
        _ => Color::srgba(0.3, 0.3, 0.3, 1.0),
    }
}
