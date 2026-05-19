use bevy::prelude::*;

/// Configure la caméra 2D avec un fond sombre et un centrage correct.
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Transform::default().looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb_u8(121, 163, 189)), // rgb(121, 163, 189)
            ..default()
        },
    ));
}
