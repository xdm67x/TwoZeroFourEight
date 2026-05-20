// src/input/resources.rs
use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
    Restart,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::None
    }
}
