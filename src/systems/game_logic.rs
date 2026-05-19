//! Système principal : algorithme 2048 (mouvement + fusion) et gestion du tour.
//!
//! Ce module contient toute la logique métier du 2048 :
//! 1. Construction d'une grille de valeurs à partir des entités ECS
//! 2. Application de l'algo de compaction/fusion pour la direction choisie
//! 3. Despawn/Respawn des entités pour refléter le nouvel état
//! 4. Détection de victoire (2048) et de défaite (plus de mouvements)
//! 5. Apparition d'une nouvelle tuile après chaque mouvement valide

use crate::components::{Position, Tile};
use crate::resources::board::{has_2048, has_moves, Board, BOARD_SIZE};
use crate::resources::game_state::GameState;
use crate::resources::score::Score;
use crate::utils::directions::Direction;
use bevy::prelude::*;
use rand::Rng;

/// --- Point d'entrée : traite un tour complet ---
///
/// Ce système est appelé à chaque frame dans la chaîne Update.
/// Il ne fait rien sauf si :
/// - `direction` n'est pas `None` (le joueur a appuyé sur une flèche)
/// - `game_state` est `Playing` (la partie n'est pas finie)
///
/// Si `direction` est `Restart`, il nettoie tout et relance une partie.
pub fn process_turn(
    mut commands: Commands,
    mut direction: ResMut<Direction>,
    mut board: ResMut<Board>,
    mut score: ResMut<Score>,
    mut game_state: ResMut<GameState>,
    query: Query<(Entity, &Position, &Tile)>,
    asset_server: Res<AssetServer>,
) {
    let dir = *direction;

    // --- Cas spécial : redémarrage ---
    if dir == Direction::Restart {
        // Despawn toutes les tuiles existantes
        for (entity, _, _) in query.iter() {
            commands.entity(entity).despawn();
        }
        board.clear();
        score.reset();
        *game_state = GameState::Playing;
        *direction = Direction::None;
        // Spawn 2 tuiles initiales
        spawn_random_tile(&mut commands, &mut board, &asset_server);
        spawn_random_tile(&mut commands, &mut board, &asset_server);
        return;
    }

    // Rien à faire si pas de direction ou jeu terminé
    if dir == Direction::None || *game_state != GameState::Playing {
        return;
    }

    // ─── Étape 1 : Construire la grille des valeurs ───
    let mut grid: [[Option<u32>; BOARD_SIZE]; BOARD_SIZE] = [[None; BOARD_SIZE]; BOARD_SIZE];
    for (_, pos, tile) in query.iter() {
        grid[pos.row][pos.col] = Some(tile.value);
    }

    // ─── Étape 2 : Appliquer l'algorithme 2048 ───
    let (new_grid, points, moved) = apply_move(&grid, dir);

    // Si rien n'a bougé, on annule le tour
    if !moved {
        *direction = Direction::None;
        return;
    }

    // ─── Étape 3 : Mettre à jour le score ───
    score.add(points);

    // ─── Étape 4 : Despawn toutes les anciennes entités ───
    despawn_all_tiles(&mut commands, &board);

    // ─── Étape 5 : Respwan les tuiles depuis la nouvelle grille ───
    spawn_tiles_from_grid(&mut commands, &new_grid, &mut board, &asset_server);

    // ─── Étape 6 : Vérifier victoire / défaite ───
    if has_2048(&new_grid) {
        *game_state = GameState::Won;
        *direction = Direction::None;
        return;
    }

    if !has_moves(&new_grid) {
        *game_state = GameState::GameOver;
        *direction = Direction::None;
        return;
    }

    // ─── Étape 7 : Spawn une nouvelle tuile aléatoire ───
    spawn_random_tile(&mut commands, &mut board, &asset_server);

    // ─── Étape 8 : Réinitialiser la direction ───
    *direction = Direction::None;
}

// ============================================================================
//  ALGORITHME 2048
// ============================================================================

/// Applique le mouvement dans la direction donnée.
/// Retourne `(nouvelle_grille, points_gagnés, quelque_chose_a_bougé)`.
fn apply_move(
    grid: &[[Option<u32>; BOARD_SIZE]; BOARD_SIZE],
    direction: Direction,
) -> ([[Option<u32>; BOARD_SIZE]; BOARD_SIZE], u32, bool) {
    let mut new_grid = [[None; BOARD_SIZE]; BOARD_SIZE];
    let mut points = 0u32;
    let mut moved = false;

    match direction {
        Direction::Left => {
            for row in 0..BOARD_SIZE {
                let line: Vec<u32> = (0..BOARD_SIZE).filter_map(|col| grid[row][col]).collect();
                let (merged, pts, changed) = compact_and_merge(line);
                if changed {
                    moved = true;
                }
                points += pts;
                for (col, &val) in merged.iter().enumerate() {
                    if val > 0 {
                        new_grid[row][col] = Some(val);
                    }
                }
            }
        }
        Direction::Right => {
            for row in 0..BOARD_SIZE {
                // On inverse l'ordre de lecture pour traiter de droite à gauche
                let line: Vec<u32> = (0..BOARD_SIZE)
                    .rev()
                    .filter_map(|col| grid[row][col])
                    .collect();
                let (merged, pts, changed) = compact_and_merge(line);
                if changed {
                    moved = true;
                }
                points += pts;
                // On réinverse pour écrire dans la grille
                for (i, &val) in merged.iter().enumerate() {
                    if val > 0 {
                        new_grid[row][BOARD_SIZE - 1 - i] = Some(val);
                    }
                }
            }
        }
        Direction::Up => {
            for col in 0..BOARD_SIZE {
                let line: Vec<u32> = (0..BOARD_SIZE).filter_map(|row| grid[row][col]).collect();
                let (merged, pts, changed) = compact_and_merge(line);
                if changed {
                    moved = true;
                }
                points += pts;
                for (row, &val) in merged.iter().enumerate() {
                    if val > 0 {
                        new_grid[row][col] = Some(val);
                    }
                }
            }
        }
        Direction::Down => {
            for col in 0..BOARD_SIZE {
                let line: Vec<u32> = (0..BOARD_SIZE)
                    .rev()
                    .filter_map(|row| grid[row][col])
                    .collect();
                let (merged, pts, changed) = compact_and_merge(line);
                if changed {
                    moved = true;
                }
                points += pts;
                for (i, &val) in merged.iter().enumerate() {
                    if val > 0 {
                        new_grid[BOARD_SIZE - 1 - i][col] = Some(val);
                    }
                }
            }
        }
        _ => {}
    }

    (new_grid, points, moved)
}

/// Le cœur de l'algorithme : compaction + fusion sur une ligne.
///
/// Algorithme pour une ligne (ex: [2, 0, 2, 4] → Gauche) :
/// 1. Compacter (supprimer les zéros) → [2, 2, 4]
/// 2. Fusionner les paires adjacentes identiques → [4, 4]  (2+2 fusionné)
/// 3. Remplir avec des zéros jusqu'à 4 éléments → [4, 4, 0, 0]
///
/// Retourne `(ligne_résultat, points_gagnés, quelque_chose_a_changé)`.
fn compact_and_merge(line: Vec<u32>) -> (Vec<u32>, u32, bool) {
    // Sauvegarde de l'état initial pour détecter les changements
    let original = {
        let mut v = line.clone();
        while v.len() < BOARD_SIZE {
            v.push(0);
        }
        v
    };

    // 1. Compaction : supprimer les zéros
    let mut compacted: Vec<u32> = line.into_iter().filter(|&v| v != 0).collect();

    // 2. Fusion : parcourir et fusionner les paires identiques
    let mut points = 0u32;
    let mut i = 0;
    while i < compacted.len() {
        if i + 1 < compacted.len() && compacted[i] == compacted[i + 1] {
            // Fusion !
            compacted[i] *= 2;
            points += compacted[i];
            compacted.remove(i + 1); // Supprime la tuile absorbée
        }
        i += 1;
    }

    // 3. Remplir avec des zéros
    while compacted.len() < BOARD_SIZE {
        compacted.push(0);
    }

    let changed = compacted != original;
    (compacted, points, changed)
}

// ============================================================================
//  GESTION DES ENTITÉS (SPAWN / DESPAWN)
// ============================================================================

/// Désalloue toutes les tuiles actuellement sur le plateau.
fn despawn_all_tiles(commands: &mut Commands, board: &Board) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if let Some(entity) = board.tiles[row][col] {
                // despawn() en Bevy 0.18 détruit l'entité ET ses enfants
                commands.entity(entity).despawn();
            }
        }
    }
}

/// Recrée toutes les tuiles à partir d'une grille de valeurs.
/// Chaque tuile est une entité avec :
/// - Un composant [`Tile`] (valeur)
/// - Un composant [`Position`] (ligne, colonne)
/// - Un [`Sprite`] de couleur adaptée à la valeur
/// - Un [`Transform`] positionné dans l'espace 2D
/// - Un enfant [`Text2d`] affichant le chiffre
fn spawn_tiles_from_grid(
    commands: &mut Commands,
    grid: &[[Option<u32>; BOARD_SIZE]; BOARD_SIZE],
    board: &mut ResMut<Board>,
    asset_server: &Res<AssetServer>,
) {
    let font_handle = asset_server.load("GoogleSans-Regular.ttf");

    board.clear();

    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if let Some(value) = grid[row][col] {
                let entity = spawn_single_tile(commands, row, col, value, &font_handle);
                board.tiles[row][col] = Some(entity);
            }
        }
    }
}

/// Crée une tuile individuelle (Sprite + Texte enfant) et retourne son [`Entity`].
fn spawn_single_tile(
    commands: &mut Commands,
    row: usize,
    col: usize,
    value: u32,
    font_handle: &Handle<Font>,
) -> Entity {
    let (bg_color, text_color) = tile_colors(value);

    // Position dans le monde 2D.
    // La grille fait 4×4, chaque tuile fait TILE_SIZE px avec un GAP entre elles.
    // On centre la grille sur l'origine (0,0).
    let x = col as f32 * (TILE_SIZE + GAP) - GRID_OFFSET;
    let y = -(row as f32 * (TILE_SIZE + GAP) - GRID_OFFSET);

    commands
        .spawn((
            Tile {
                value,
                merged: false,
            },
            Position { row, col },
            Sprite {
                color: bg_color,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            Transform::from_xyz(x, y, 1.0), // z=1 pour être au-dessus du fond
        ))
        .with_children(|parent| {
            // Texte enfant : le chiffre au centre de la tuile
            parent.spawn((
                Text2d::new(value.to_string()),
                TextFont {
                    font: font_handle.clone(),
                    font_size: FONT_SIZE,
                    ..default()
                },
                TextColor(text_color),
                // Légèrement descendu pour centrage optique
                Transform::from_xyz(0.0, -4.0, 0.1),
            ));
        })
        .id()
}

// ============================================================================
//  SPAWN DE NOUVELLE TUILE ALÉATOIRE
// ============================================================================

/// Fait apparaître une nouvelle tuile (valeur 2 ou 4) à une position vide aléatoire.
/// Ne fait rien si la grille est pleine.
pub fn spawn_random_tile(
    commands: &mut Commands,
    board: &mut ResMut<Board>,
    asset_server: &Res<AssetServer>,
) {
    // Collecter toutes les positions vides
    let mut empty_positions = Vec::new();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board.tiles[row][col].is_none() {
                empty_positions.push((row, col));
            }
        }
    }

    if empty_positions.is_empty() {
        return;
    }

    // Choisir une position au hasard
    let idx = rand::thread_rng().gen_range(0..empty_positions.len());
    let (row, col) = empty_positions[idx];

    // 90% de chance d'avoir un 2, 10% un 4
    let value = if rand::random::<f32>() < 0.9 { 2 } else { 4 };

    let font_handle = asset_server.load("GoogleSans-Regular.ttf");
    let entity = spawn_single_tile(commands, row, col, value, &font_handle);
    board.tiles[row][col] = Some(entity);
}

// ============================================================================
//  CONSTANTES VISUELLES
// ============================================================================

/// Taille d'une tuile en pixels.
const TILE_SIZE: f32 = 100.0;

/// Espacement entre les tuiles.
const GAP: f32 = 12.0;

/// Décalage pour centrer la grille 4×4 + gaps.
/// Calcul : (4 * TILE_SIZE + 3 * GAP) / 2 = (400 + 36) / 2 = 218
const GRID_OFFSET: f32 =
    (BOARD_SIZE as f32 * TILE_SIZE + (BOARD_SIZE as f32 - 1.0) * GAP) / 2.0 - TILE_SIZE / 2.0;

/// Taille de la police pour le chiffre sur la tuile.
const FONT_SIZE: f32 = 40.0;

/// Palette de couleurs inspirée du 2048 original.
/// Retourne `(couleur_fond, couleur_texte)`.
fn tile_colors(value: u32) -> (Color, Color) {
    let text_dark = Color::srgb(0.467, 0.431, 0.396); // #776e65
    let text_light = Color::srgb(0.976, 0.965, 0.945); // #f9f6f2

    match value {
        2 => (Color::srgb(0.933, 0.894, 0.855), text_dark),
        4 => (Color::srgb(0.929, 0.878, 0.784), text_dark),
        8 => (Color::srgb(0.949, 0.694, 0.475), text_light),
        16 => (Color::srgb(0.961, 0.584, 0.388), text_light),
        32 => (Color::srgb(0.965, 0.486, 0.373), text_light),
        64 => (Color::srgb(0.965, 0.369, 0.231), text_light),
        128 => (Color::srgb(0.929, 0.812, 0.447), text_light),
        256 => (Color::srgb(0.929, 0.800, 0.380), text_light),
        512 => (Color::srgb(0.929, 0.784, 0.314), text_light),
        1024 => (Color::srgb(0.929, 0.773, 0.247), text_light),
        2048 => (Color::srgb(0.929, 0.761, 0.180), text_light),
        _ => (Color::srgb(0.235, 0.227, 0.196), text_light), // 4096+
    }
}
