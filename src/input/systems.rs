// src/input/systems.rs
use crate::input::resources::Direction;
use crate::states::AppState;
use bevy::ecs::message::MessageReader;
use bevy::input::keyboard::KeyCode;
use bevy::input::touch::{TouchInput, TouchPhase};
use bevy::prelude::*;

const SWIPE_THRESHOLD: f32 = 30.0;

fn swipe_direction(start: Vec2, end: Vec2) -> Option<Direction> {
    let delta = end - start;
    if delta.length_squared() < SWIPE_THRESHOLD * SWIPE_THRESHOLD {
        return None;
    }
    if delta.x.abs() > delta.y.abs() {
        Some(if delta.x > 0.0 { Direction::Right } else { Direction::Left })
    } else {
        // Touch screen Y increases downward: negative delta.y = finger moved up
        Some(if delta.y < 0.0 { Direction::Up } else { Direction::Down })
    }
}

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

pub fn handle_touch_input(
    mut direction: ResMut<Direction>,
    mut touch_events: MessageReader<TouchInput>,
    mut tracked: Local<Option<(u64, Vec2)>>,
) {
    for event in touch_events.read() {
        match event.phase {
            TouchPhase::Started => {
                if tracked.is_none() {
                    *tracked = Some((event.id, event.position));
                }
            }
            TouchPhase::Ended => {
                if let Some((id, start_pos)) = *tracked {
                    if event.id == id {
                        *tracked = None;
                        if let Some(dir) = swipe_direction(start_pos, event.position) {
                            *direction = dir;
                        }
                    }
                }
            }
            TouchPhase::Canceled => {
                if tracked.map_or(false, |(id, _)| id == event.id) {
                    *tracked = None;
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::Vec2;

    #[test]
    fn swipe_left() {
        let dir = swipe_direction(Vec2::new(200.0, 100.0), Vec2::new(150.0, 102.0));
        assert_eq!(dir, Some(Direction::Left));
    }

    #[test]
    fn swipe_right() {
        let dir = swipe_direction(Vec2::new(100.0, 100.0), Vec2::new(160.0, 98.0));
        assert_eq!(dir, Some(Direction::Right));
    }

    #[test]
    fn swipe_up() {
        // Screen Y increases downward: swipe up → delta.y < 0
        let dir = swipe_direction(Vec2::new(100.0, 200.0), Vec2::new(102.0, 140.0));
        assert_eq!(dir, Some(Direction::Up));
    }

    #[test]
    fn swipe_down() {
        // Screen Y increases downward: swipe down → delta.y > 0
        let dir = swipe_direction(Vec2::new(100.0, 100.0), Vec2::new(98.0, 160.0));
        assert_eq!(dir, Some(Direction::Down));
    }

    #[test]
    fn tap_too_short_ignored() {
        let dir = swipe_direction(Vec2::new(100.0, 100.0), Vec2::new(110.0, 100.0));
        assert_eq!(dir, None);
    }

    #[test]
    fn diagonal_resolves_to_dominant_axis() {
        // dx=60 > dy=20 → horizontal
        let dir = swipe_direction(Vec2::new(100.0, 100.0), Vec2::new(160.0, 120.0));
        assert_eq!(dir, Some(Direction::Right));
    }
}
