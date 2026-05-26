// src/ui/systems.rs
use crate::states::AppState;
use crate::ui::components::{OverlayPanel, OverlayText, ScoreText};
use bevy::prelude::*;

const GRID_SIZE: f32 = 800.0;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("GoogleSans-Regular.ttf");

    commands.spawn((
        Text2d::new("2048"),
        TextFont {
            font: font.clone(),
            font_size: 52.0,
            ..default()
        },
        TextColor(Color::srgb(0.929, 0.761, 0.180)),
        Transform::from_xyz(0.0, 305.0, 10.0),
    ));

    commands.spawn((
        Sprite {
            color: Color::srgba(0.0, 0.0, 0.0, 0.78),
            custom_size: Some(Vec2::splat(GRID_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 99.0),
        OverlayPanel,
        Visibility::Hidden,
    ));

    commands.spawn((
        Text2d::new(""),
        TextFont {
            font: font.clone(),
            font_size: 36.0,
            ..default()
        },
        TextColor(Color::srgb(0.976, 0.965, 0.945)),
        Transform::from_xyz(0.0, 0.0, 100.0),
        OverlayText,
        Visibility::Hidden,
    ));
}

pub fn update_ui(
    state: Res<State<AppState>>,
    mut overlay_query: Query<
        (&mut Text2d, &mut Visibility),
        (With<OverlayText>, Without<ScoreText>, Without<OverlayPanel>),
    >,
    mut panel_query: Query<
        &mut Visibility,
        (With<OverlayPanel>, Without<OverlayText>, Without<ScoreText>),
    >,
) {
    let show_overlay = matches!(state.get(), AppState::Won | AppState::GameOver);

    if let Some(mut panel_vis) = panel_query.iter_mut().next() {
        *panel_vis = if show_overlay {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    if let Some((mut overlay_text, mut visibility)) = overlay_query.iter_mut().next() {
        match state.get() {
            AppState::InGame => {
                *visibility = Visibility::Hidden;
            }
            AppState::Won => {
                *visibility = Visibility::Visible;
                overlay_text.0 = "You Win!\nR to restart".to_owned();
            }
            AppState::GameOver => {
                *visibility = Visibility::Visible;
                overlay_text.0 = "Game Over\nR to restart".to_owned();
            }
        }
    }
}
