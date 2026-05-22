// src/render/systems.rs
use crate::board::resources::BOARD_SIZE;
use bevy::prelude::*;

const TILE_SIZE: f32 = 100.0;
const GAP: f32 = 12.0;
const GRID_BG_SIZE: f32 = BOARD_SIZE as f32 * TILE_SIZE + (BOARD_SIZE as f32 + 1.0) * GAP;
const GRID_OFFSET: f32 =
    (BOARD_SIZE as f32 * TILE_SIZE + (BOARD_SIZE as f32 - 1.0) * GAP) / 2.0 - TILE_SIZE / 2.0;

pub fn setup_grid_background(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.106, 0.106, 0.118),
            custom_size: Some(Vec2::splat(GRID_BG_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let x = col as f32 * (TILE_SIZE + GAP) - GRID_OFFSET;
            let y = -(row as f32 * (TILE_SIZE + GAP) - GRID_OFFSET);
            commands.spawn((
                Sprite {
                    color: Color::srgb(0.165, 0.165, 0.180),
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.5),
            ));
        }
    }
}
