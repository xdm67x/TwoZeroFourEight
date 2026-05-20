// src/ui/systems.rs
use crate::board::resources::Score;
use crate::states::AppState;
use crate::ui::components::{OverlayText, ScoreText};
use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("GoogleSans-Regular.ttf");

    commands.spawn((
        Text2d::new("2048"),
        TextFont { font: font.clone(), font_size: 52.0, ..default() },
        TextColor(Color::srgb(0.929, 0.761, 0.180)),
        Transform::from_xyz(0.0, 260.0, 10.0),
    ));

    commands.spawn((
        Text2d::new("Score: 0"),
        TextFont { font: font.clone(), font_size: 28.0, ..default() },
        TextColor(Color::srgb(0.976, 0.965, 0.945)),
        Transform::from_xyz(0.0, 220.0, 10.0),
        ScoreText,
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
        (With<OverlayText>, Without<ScoreText>),
    >,
) {
    if let Some(mut score_text) = score_query.iter_mut().next() {
        score_text.0 = format!("Score: {}", score.value);
    }

    if let Some((mut overlay_text, mut visibility)) = overlay_query.iter_mut().next() {
        match state.get() {
            AppState::InGame => {
                *visibility = Visibility::Hidden;
            }
            AppState::Won => {
                *visibility = Visibility::Visible;
                overlay_text.0 = format!(
                    "🎉  You Win!  🎉\nScore: {}\n\nPress R to restart",
                    score.value
                );
            }
            AppState::GameOver => {
                *visibility = Visibility::Visible;
                overlay_text.0 =
                    format!("Game Over\nScore: {}\n\nPress R to restart", score.value);
            }
            _ => {}
        }
    }
}
