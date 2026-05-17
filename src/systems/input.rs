use crate::utils::directions::Direction;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

/// Système pour capturer les inputs clavier (flèches directionnelles).
pub fn handle_input(mut direction: ResMut<Direction>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        *direction = Direction::Left;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        *direction = Direction::Right;
    } else if keyboard_input.pressed(KeyCode::ArrowUp) {
        *direction = Direction::Up;
    } else if keyboard_input.pressed(KeyCode::ArrowDown) {
        *direction = Direction::Down;
    }
}
