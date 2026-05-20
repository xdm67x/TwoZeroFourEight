// src/input/systems.rs
use crate::input::resources::Direction;
use crate::states::AppState;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

pub fn handle_input(
    mut direction: ResMut<Direction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        *direction = Direction::Restart;
        return;
    }

    if *state.get() != AppState::InGame {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowLeft)
        || keyboard_input.just_pressed(KeyCode::KeyA)
        || keyboard_input.just_pressed(KeyCode::KeyQ)
    {
        *direction = Direction::Left;
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight)
        || keyboard_input.just_pressed(KeyCode::KeyD)
    {
        *direction = Direction::Right;
    } else if keyboard_input.just_pressed(KeyCode::ArrowUp)
        || keyboard_input.just_pressed(KeyCode::KeyW)
        || keyboard_input.just_pressed(KeyCode::KeyZ)
    {
        *direction = Direction::Up;
    } else if keyboard_input.just_pressed(KeyCode::ArrowDown)
        || keyboard_input.just_pressed(KeyCode::KeyS)
    {
        *direction = Direction::Down;
    }
}
