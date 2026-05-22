// src/board/systems.rs
use crate::board::resources::{has_2048, has_moves, Board, Score, BOARD_SIZE};
use crate::components::{Position, Tile};
use crate::input::resources::Direction;
use crate::states::AppState;
use bevy::prelude::*;
use rand::Rng;

const TILE_SIZE: f32 = 100.0;
const GAP: f32 = 12.0;
const GRID_OFFSET: f32 =
    (BOARD_SIZE as f32 * TILE_SIZE + (BOARD_SIZE as f32 - 1.0) * GAP) / 2.0 - TILE_SIZE / 2.0;
const FONT_SIZE: f32 = 40.0;

pub fn spawn_initial_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut board: ResMut<Board>,
) {
    spawn_random_tile(&mut commands, &mut board, &asset_server);
    spawn_random_tile(&mut commands, &mut board, &asset_server);
}

pub fn process_turn(
    mut commands: Commands,
    mut direction: ResMut<Direction>,
    mut board: ResMut<Board>,
    mut score: ResMut<Score>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<(Entity, &Position, &Tile)>,
    asset_server: Res<AssetServer>,
) {
    let dir = *direction;

    if dir == Direction::Restart {
        for (entity, _, _) in query.iter() {
            commands.entity(entity).despawn();
        }
        board.clear();
        score.reset();
        if *state.get() != AppState::InGame {
            next_state.set(AppState::InGame);
        }
        spawn_random_tile(&mut commands, &mut board, &asset_server);
        spawn_random_tile(&mut commands, &mut board, &asset_server);
        *direction = Direction::None;
        return;
    }

    if dir == Direction::None || *state.get() != AppState::InGame {
        return;
    }

    let mut grid: [[Option<u32>; BOARD_SIZE]; BOARD_SIZE] = [[None; BOARD_SIZE]; BOARD_SIZE];
    for (_, pos, tile) in query.iter() {
        grid[pos.row][pos.col] = Some(tile.value);
    }

    let (new_grid, points, moved) = apply_move(&grid, dir);

    if !moved {
        *direction = Direction::None;
        return;
    }

    score.add(points);
    despawn_all_tiles(&mut commands, &board);
    spawn_tiles_from_grid(&mut commands, &new_grid, &mut board, &asset_server);

    if has_2048(&new_grid) {
        next_state.set(AppState::Won);
        *direction = Direction::None;
        return;
    }

    if !has_moves(&new_grid) {
        next_state.set(AppState::GameOver);
        *direction = Direction::None;
        return;
    }

    spawn_random_tile(&mut commands, &mut board, &asset_server);
    *direction = Direction::None;
}

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
                let original: Vec<u32> = (0..BOARD_SIZE).map(|col| grid[row][col].unwrap_or(0)).collect();
                let line: Vec<u32> = (0..BOARD_SIZE).filter_map(|col| grid[row][col]).collect();
                let (merged, pts, changed) = compact_and_merge(line, &original);
                if changed { moved = true; }
                points += pts;
                for (col, &val) in merged.iter().enumerate() {
                    if val > 0 { new_grid[row][col] = Some(val); }
                }
            }
        }
        Direction::Right => {
            for row in 0..BOARD_SIZE {
                let original: Vec<u32> = (0..BOARD_SIZE).rev().map(|col| grid[row][col].unwrap_or(0)).collect();
                let line: Vec<u32> = (0..BOARD_SIZE).rev().filter_map(|col| grid[row][col]).collect();
                let (merged, pts, changed) = compact_and_merge(line, &original);
                if changed { moved = true; }
                points += pts;
                for (i, &val) in merged.iter().enumerate() {
                    if val > 0 { new_grid[row][BOARD_SIZE - 1 - i] = Some(val); }
                }
            }
        }
        Direction::Up => {
            for col in 0..BOARD_SIZE {
                let original: Vec<u32> = (0..BOARD_SIZE).map(|row| grid[row][col].unwrap_or(0)).collect();
                let line: Vec<u32> = (0..BOARD_SIZE).filter_map(|row| grid[row][col]).collect();
                let (merged, pts, changed) = compact_and_merge(line, &original);
                if changed { moved = true; }
                points += pts;
                for (row, &val) in merged.iter().enumerate() {
                    if val > 0 { new_grid[row][col] = Some(val); }
                }
            }
        }
        Direction::Down => {
            for col in 0..BOARD_SIZE {
                let original: Vec<u32> = (0..BOARD_SIZE).rev().map(|row| grid[row][col].unwrap_or(0)).collect();
                let line: Vec<u32> = (0..BOARD_SIZE).rev().filter_map(|row| grid[row][col]).collect();
                let (merged, pts, changed) = compact_and_merge(line, &original);
                if changed { moved = true; }
                points += pts;
                for (i, &val) in merged.iter().enumerate() {
                    if val > 0 { new_grid[BOARD_SIZE - 1 - i][col] = Some(val); }
                }
            }
        }
        _ => {}
    }

    (new_grid, points, moved)
}

fn compact_and_merge(line: Vec<u32>, original: &[u32]) -> (Vec<u32>, u32, bool) {
    let mut compacted: Vec<u32> = line.into_iter().filter(|&v| v != 0).collect();
    let mut points = 0u32;
    let mut i = 0;
    while i < compacted.len() {
        if i + 1 < compacted.len() && compacted[i] == compacted[i + 1] {
            compacted[i] *= 2;
            points += compacted[i];
            compacted.remove(i + 1);
        }
        i += 1;
    }
    while compacted.len() < BOARD_SIZE { compacted.push(0); }

    let changed = compacted != original;
    (compacted, points, changed)
}

fn despawn_all_tiles(commands: &mut Commands, board: &Board) {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if let Some(entity) = board.tiles[row][col] {
                commands.entity(entity).despawn();
            }
        }
    }
}

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

pub fn spawn_random_tile(
    commands: &mut Commands,
    board: &mut ResMut<Board>,
    asset_server: &Res<AssetServer>,
) {
    let mut empty_positions = Vec::new();
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if board.tiles[row][col].is_none() {
                empty_positions.push((row, col));
            }
        }
    }
    if empty_positions.is_empty() { return; }

    let idx = rand::thread_rng().gen_range(0..empty_positions.len());
    let (row, col) = empty_positions[idx];
    let value = if rand::random::<f32>() < 0.9 { 2 } else { 4 };

    let font_handle = asset_server.load("GoogleSans-Regular.ttf");
    let entity = spawn_single_tile(commands, row, col, value, &font_handle);
    board.tiles[row][col] = Some(entity);
}

fn spawn_single_tile(
    commands: &mut Commands,
    row: usize,
    col: usize,
    value: u32,
    font_handle: &Handle<Font>,
) -> Entity {
    let (bg_color, text_color) = tile_colors(value);
    let x = col as f32 * (TILE_SIZE + GAP) - GRID_OFFSET;
    let y = -(row as f32 * (TILE_SIZE + GAP) - GRID_OFFSET);

    commands
        .spawn((
            Tile { value, merged: false },
            Position { row, col },
            Sprite {
                color: bg_color,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            Transform::from_xyz(x, y, 1.0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text2d::new(value.to_string()),
                TextFont { font: font_handle.clone(), font_size: FONT_SIZE, ..default() },
                TextColor(text_color),
                Transform::from_xyz(0.0, -4.0, 0.1),
            ));
        })
        .id()
}

fn tile_colors(value: u32) -> (Color, Color) {
    let text_dark = Color::srgb(0.467, 0.431, 0.396);
    let text_light = Color::srgb(0.976, 0.965, 0.945);
    match value {
        2    => (Color::srgb(0.933, 0.894, 0.855), text_dark),
        4    => (Color::srgb(0.929, 0.878, 0.784), text_dark),
        8    => (Color::srgb(0.949, 0.694, 0.475), text_light),
        16   => (Color::srgb(0.961, 0.584, 0.388), text_light),
        32   => (Color::srgb(0.965, 0.486, 0.373), text_light),
        64   => (Color::srgb(0.965, 0.369, 0.231), text_light),
        128  => (Color::srgb(0.929, 0.812, 0.447), text_light),
        256  => (Color::srgb(0.929, 0.800, 0.380), text_light),
        512  => (Color::srgb(0.929, 0.784, 0.314), text_light),
        1024 => (Color::srgb(0.929, 0.773, 0.247), text_light),
        2048 => (Color::srgb(0.929, 0.761, 0.180), text_light),
        _    => (Color::srgb(0.235, 0.227, 0.196), text_light),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_merge_empty_line() {
        let original = vec![0, 0, 0, 0];
        let (result, pts, changed) = compact_and_merge(vec![], &original);
        assert_eq!(result, vec![0, 0, 0, 0]);
        assert_eq!(pts, 0);
        assert!(!changed);
    }

    #[test]
    fn compact_merge_no_change() {
        let original = vec![2, 4, 8, 16];
        let (result, pts, changed) = compact_and_merge(vec![2, 4, 8, 16], &original);
        assert_eq!(result, vec![2, 4, 8, 16]);
        assert_eq!(pts, 0);
        assert!(!changed);
    }

    #[test]
    fn compact_merge_simple_pair() {
        let original = vec![2, 2, 0, 0];
        let (result, pts, changed) = compact_and_merge(vec![2, 2], &original);
        assert_eq!(result, vec![4, 0, 0, 0]);
        assert_eq!(pts, 4);
        assert!(changed);
    }

    #[test]
    fn compact_merge_with_gaps() {
        let original = vec![2, 2, 4, 0];
        let (result, pts, changed) = compact_and_merge(vec![2, 2, 4], &original);
        assert_eq!(result, vec![4, 4, 0, 0]);
        assert_eq!(pts, 4);
        assert!(changed);
    }

    #[test]
    fn compact_merge_no_double_merge() {
        let original = vec![2, 2, 2, 2];
        let (result, pts, changed) = compact_and_merge(vec![2, 2, 2, 2], &original);
        assert_eq!(result, vec![4, 4, 0, 0]);
        assert_eq!(pts, 8);
        assert!(changed);
    }

    #[test]
    fn compact_merge_slide_without_merge() {
        // [None, 2, None, 4] → original [0, 2, 0, 4], résultat attendu [2, 4, 0, 0]
        let original = vec![0, 2, 0, 4];
        let (result, pts, changed) = compact_and_merge(vec![2, 4], &original);
        assert_eq!(result, vec![2, 4, 0, 0]);
        assert_eq!(pts, 0);
        assert!(changed); // les tuiles ont bougé même sans merge
    }
}
