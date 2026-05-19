//! Système UI : affiche le score, le titre, et les overlays Game Over / You Win.
//!
//! Architecture Bevy :
//! - Des entités "statiques" sont spawnées au Startup (titre, label score).
//! - Un système Update (`update_ui`) met à jour le texte du score et
//!   affiche/masque les overlays selon l'état du jeu.

use crate::resources::game_state::GameState;
use crate::resources::score::Score;
use bevy::prelude::*;

/// Tag pour identifier facilement l'entité du texte du score.
#[derive(Component)]
pub struct ScoreText;

/// Tag pour l'entité overlay (Game Over / You Win).
#[derive(Component)]
pub struct OverlayText;

// ============================================================================
//  SETUP INITIAL (Startup)
// ============================================================================

/// Crée tous les éléments UI au lancement du jeu.
///
/// Spawn :
/// - Le titre "2048" en haut
/// - Le label "Score: 0"
/// - L'overlay (invisible au début) qui servira pour Game Over / You Win
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("GoogleSans-Regular.ttf");

    // --- Titre ---
    commands.spawn((
        Text2d::new("2048"),
        TextFont {
            font: font.clone(),
            font_size: 52.0,
            ..default()
        },
        TextColor(Color::srgb(0.929, 0.761, 0.180)), // Doré
        Transform::from_xyz(0.0, 260.0, 10.0),
    ));

    // --- Score ---
    commands.spawn((
        Text2d::new("Score: 0"),
        TextFont {
            font: font.clone(),
            font_size: 28.0,
            ..default()
        },
        TextColor(Color::srgb(0.976, 0.965, 0.945)), // Blanc cassé
        Transform::from_xyz(0.0, 220.0, 10.0),
        ScoreText, // Tag pour le retrouver dans update_ui
    ));

    // --- Overlay (caché par défaut) ---
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
        Visibility::Hidden, // Invisible tant que le jeu n'est pas fini
    ));
}

// ============================================================================
//  MISE À JOUR (Update)
// ============================================================================

/// Met à jour l'affichage du score et des overlays à chaque frame.
pub fn update_ui(
    score: Res<Score>,
    game_state: Res<GameState>,
    mut score_query: Query<&mut Text2d, (With<ScoreText>, Without<OverlayText>)>,
    mut overlay_query: Query<
        (&mut Text2d, &mut Visibility),
        (With<OverlayText>, Without<ScoreText>),
    >,
) {
    // --- Mise à jour du score ---
    // Bevy 0.18 : on utilise iter_mut().next() car get_single_mut() n'existe plus
    if let Some(mut score_text) = score_query.iter_mut().next() {
        score_text.0 = format!("Score: {}", score.value);
    }

    // --- Mise à jour de l'overlay ---
    if let Some((mut overlay_text, mut visibility)) = overlay_query.iter_mut().next() {
        match *game_state {
            GameState::Playing => {
                *visibility = Visibility::Hidden;
            }
            GameState::Won => {
                *visibility = Visibility::Visible;
                overlay_text.0 = format!(
                    "🎉  You Win!  🎉\nScore: {}\n\nPress R to restart",
                    score.value
                );
            }
            GameState::GameOver => {
                *visibility = Visibility::Visible;
                overlay_text.0 = format!("Game Over\nScore: {}\n\nPress R to restart", score.value);
            }
        }
    }
}
