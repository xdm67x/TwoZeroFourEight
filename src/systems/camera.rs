use bevy::prelude::*;

/// Système pour configurer la caméra
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
