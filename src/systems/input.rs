//! Système de capture des entrées clavier.
//!
//! Utilise [`just_pressed`] plutôt que [`pressed`] pour garantir
//! qu'une seule pression = un seul mouvement (pas de répétition).

use crate::resources::game_state::GameState;
use crate::utils::directions::Direction;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

/// Capte les touches au clavier et définit la [`Direction`] pour le tour.
///
/// Fonctionnement :
/// - Quand le jeu est en cours (`Playing`) : flèches ou ZQSD/WASD pour bouger
/// - Quand le jeu est fini (`Won` ou `GameOver`) : touche R pour recommencer
/// - Sinon, ne fait rien
pub fn handle_input(
    mut direction: ResMut<Direction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<GameState>,
) {
    // --- Touche R : redémarrer (disponible dans tous les états) ---
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        *direction = Direction::Restart;
        return;
    }

    // --- En cours de partie : touches directionnelles ---
    if *game_state != GameState::Playing {
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
