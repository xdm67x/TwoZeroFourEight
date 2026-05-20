// src/render/systems.rs
use crate::board::resources::BOARD_SIZE;
use bevy::prelude::*;

const TILE_SIZE: f32 = 100.0;
const GAP: f32 = 12.0;
const GRID_BG_SIZE: f32 = BOARD_SIZE as f32 * TILE_SIZE + (BOARD_SIZE as f32 + 1.0) * GAP;

pub fn setup_grid_background(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.106, 0.106, 0.118),
            custom_size: Some(Vec2::new(GRID_BG_SIZE, GRID_BG_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
