// src/ui/systems.rs
use crate::board::resources::Score;
use crate::states::AppState;
use crate::ui::components::{OverlayPanel, OverlayText, ScoreText};
use bevy::prelude::*;

const GRID_SIZE: f32 = 460.0;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("GoogleSans-Regular.ttf");

    commands.spawn((
        Text2d::new("2048"),
        TextFont { font: font.clone(), font_size: 52.0, ..default() },
        TextColor(Color::srgb(0.929, 0.761, 0.180)),
        Transform::from_xyz(0.0, 305.0, 10.0),
    ));

    commands.spawn((
        Text2d::new("Score: 0"),
        TextFont { font: font.clone(), font_size: 28.0, ..default() },
        TextColor(Color::srgb(0.976, 0.965, 0.945)),
        Transform::from_xyz(0.0, 255.0, 10.0),
        ScoreText,
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
        TextFont { font: font.clone(), font_size: 36.0, ..default() },
        TextColor(Color::srgb(0.976, 0.965, 0.945)),
        Transform::from_xyz(0.0, 0.0, 100.0),
        OverlayText,
        Visibility::Hidden,
    ));
}

pub fn update_ui(
    score: Res<Score>,
    state: Res<State<AppState>>,
    mut score_query: Query<&mut Text2d, (With<ScoreText>, Without<OverlayText>)>,
    mut overlay_query: Query<
        (&mut Text2d, &mut Visibility),
        (With<OverlayText>, Without<ScoreText>, Without<OverlayPanel>),
    >,
    mut panel_query: Query<
        &mut Visibility,
        (With<OverlayPanel>, Without<OverlayText>, Without<ScoreText>),
    >,
) {
    if let Some(mut score_text) = score_query.iter_mut().next() {
        score_text.0 = format!("Score: {}", score.value);
    }

    let show_overlay = matches!(state.get(), AppState::Won | AppState::GameOver);

    if let Some(mut panel_vis) = panel_query.iter_mut().next() {
        *panel_vis = if show_overlay { Visibility::Visible } else { Visibility::Hidden };
    }

    if let Some((mut overlay_text, mut visibility)) = overlay_query.iter_mut().next() {
        match state.get() {
            AppState::InGame => {
                *visibility = Visibility::Hidden;
            }
            AppState::Won => {
                *visibility = Visibility::Visible;
                overlay_text.0 = format!(
                    "You Win!\nScore: {}\n\nR to restart",
                    score.value
                );
            }
            AppState::GameOver => {
                *visibility = Visibility::Visible;
                overlay_text.0 =
                    format!("Game Over\nScore: {}\n\nR to restart", score.value);
            }
        }
    }
}
