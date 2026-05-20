# Plugin Architecture Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor the 2048 Bevy codebase from a monolithic `main.rs` wiring to five focused plugins, each owning its systems, resources, and components, driven by a Bevy `States`-based `AppState`.

**Architecture:** `AppState` (Bevy `States`) replaces the old `GameState` `Resource` and gates system execution. Five plugins (`CameraPlugin`, `InputPlugin`, `BoardPlugin`, `RenderPlugin`, `UIPlugin`) each call `app.add_systems(...)` in their own `Plugin::build`. `main.rs` registers `AppState` then the five plugins — nothing more.

**Tech Stack:** Rust, Bevy 0.18.1, rand 0.8

> **Note on Startup vs OnEnter(InGame):** `setup_camera`, `setup_grid_background`, and `setup_ui` use `Startup` (not `OnEnter(InGame)`) to avoid re-spawning permanent entities on restart. Only tile spawning uses `Startup` directly (restart is handled inside `process_turn`).
>
> **Note on OnExit(InGame):** The spec listed `OnExit(InGame)` for tile cleanup. This plan intentionally omits it: tiles must remain visible when entering `Won`/`GameOver` (overlay shows on top). Cleanup happens in `process_turn`'s `Restart` branch instead, which runs regardless of current state.

---

## File Map

| Action | Path | Responsibility |
|--------|------|----------------|
| Create | `src/states.rs` | `AppState` enum (Bevy States) |
| Create | `src/camera/mod.rs` | `CameraPlugin` + `setup_camera` |
| Create | `src/input/resources.rs` | `Direction` resource |
| Create | `src/input/systems.rs` | `handle_input` |
| Create | `src/input/mod.rs` | `InputPlugin` |
| Create | `src/board/resources.rs` | `Board`, `Score`, `BOARD_SIZE`, `has_moves`, `has_2048` |
| Create | `src/board/systems.rs` | `process_turn`, `spawn_random_tile`, `spawn_initial_tiles` |
| Create | `src/board/mod.rs` | `BoardPlugin` |
| Create | `src/render/systems.rs` | `setup_grid_background` |
| Create | `src/render/mod.rs` | `RenderPlugin` |
| Create | `src/ui/components.rs` | `ScoreText`, `OverlayText` |
| Create | `src/ui/systems.rs` | `setup_ui`, `update_ui` |
| Create | `src/ui/mod.rs` | `UIPlugin` |
| Rewrite | `src/main.rs` | `App` with 5 plugins only |
| Delete | `src/resources/` | Dissolved into `board/` |
| Delete | `src/systems/` | Dissolved into plugin dirs |
| Delete | `src/utils/` | Dissolved into `input/` |

---

## Task 1: Create `src/states.rs`

**Files:**
- Create: `src/states.rs`

- [ ] **Step 1: Create the file**

```rust
// src/states.rs
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    MainMenu,
    #[default]
    InGame,
    Paused,
    Won,
    GameOver,
}
```

> `InGame` is the default so the game boots straight into a match. `MainMenu` and `Paused` are scaffolded for future use but not wired to any systems yet.

- [ ] **Step 2: Verify old code still compiles (regression guard)**

```bash
cargo check
```
Expected: no errors (new file not yet referenced from `main.rs`)

- [ ] **Step 3: Commit**

```bash
git add src/states.rs
git commit -m "feat: add AppState (Bevy States) replacing GameState resource"
```

---

## Task 2: Create `src/board/resources.rs`

**Files:**
- Create: `src/board/resources.rs`

- [ ] **Step 1: Write failing tests first**

```rust
// at the bottom of src/board/resources.rs (add after the implementations)
#[cfg(test)]
mod tests {
    use super::*;

    fn grid_from(values: [[u32; 4]; 4]) -> [[Option<u32>; 4]; 4] {
        values.map(|row| row.map(|v| if v == 0 { None } else { Some(v) }))
    }

    #[test]
    fn has_moves_empty_board() {
        let grid = [[None; 4]; 4];
        assert!(has_moves(&grid));
    }

    #[test]
    fn has_moves_full_no_merges() {
        let grid = grid_from([
            [2, 4, 2, 4],
            [4, 2, 4, 2],
            [2, 4, 2, 4],
            [4, 2, 4, 2],
        ]);
        assert!(!has_moves(&grid));
    }

    #[test]
    fn has_moves_adjacent_same_value() {
        let grid = grid_from([
            [2, 2, 4, 8],
            [4, 8, 16, 32],
            [8, 16, 32, 64],
            [16, 32, 64, 128],
        ]);
        assert!(has_moves(&grid));
    }

    #[test]
    fn has_2048_not_present() {
        let grid = grid_from([[1024, 0, 0, 0], [0; 4], [0; 4], [0; 4]]);
        assert!(!has_2048(&grid));
    }

    #[test]
    fn has_2048_present() {
        let grid = grid_from([[2048, 0, 0, 0], [0; 4], [0; 4], [0; 4]]);
        assert!(has_2048(&grid));
    }
}
```

- [ ] **Step 2: Create the full file**

```rust
// src/board/resources.rs
use bevy::prelude::*;

pub const BOARD_SIZE: usize = 4;

#[derive(Resource, Default)]
pub struct Board {
    pub tiles: [[Option<Entity>; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn clear(&mut self) {
        self.tiles = [[None; BOARD_SIZE]; BOARD_SIZE];
    }
}

#[derive(Resource, Debug, Default)]
pub struct Score {
    pub value: u32,
}

impl Score {
    pub fn add(&mut self, points: u32) {
        self.value += points;
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }
}

pub fn has_moves(grid: &[[Option<u32>; BOARD_SIZE]; BOARD_SIZE]) -> bool {
    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            if grid[row][col].is_none() {
                return true;
            }
            let current = grid[row][col].unwrap();
            if col + 1 < BOARD_SIZE {
                if let Some(right) = grid[row][col + 1] {
                    if right == current {
                        return true;
                    }
                }
            }
            if row + 1 < BOARD_SIZE {
                if let Some(down) = grid[row + 1][col] {
                    if down == current {
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub fn has_2048(grid: &[[Option<u32>; BOARD_SIZE]; BOARD_SIZE]) -> bool {
    grid.iter()
        .flatten()
        .any(|cell| cell.is_some_and(|v| v >= 2048))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid_from(values: [[u32; 4]; 4]) -> [[Option<u32>; 4]; 4] {
        values.map(|row| row.map(|v| if v == 0 { None } else { Some(v) }))
    }

    #[test]
    fn has_moves_empty_board() {
        let grid = [[None; 4]; 4];
        assert!(has_moves(&grid));
    }

    #[test]
    fn has_moves_full_no_merges() {
        let grid = grid_from([
            [2, 4, 2, 4],
            [4, 2, 4, 2],
            [2, 4, 2, 4],
            [4, 2, 4, 2],
        ]);
        assert!(!has_moves(&grid));
    }

    #[test]
    fn has_moves_adjacent_same_value() {
        let grid = grid_from([
            [2, 2, 4, 8],
            [4, 8, 16, 32],
            [8, 16, 32, 64],
            [16, 32, 64, 128],
        ]);
        assert!(has_moves(&grid));
    }

    #[test]
    fn has_2048_not_present() {
        let grid = grid_from([[1024, 0, 0, 0], [0; 4], [0; 4], [0; 4]]);
        assert!(!has_2048(&grid));
    }

    #[test]
    fn has_2048_present() {
        let grid = grid_from([[2048, 0, 0, 0], [0; 4], [0; 4], [0; 4]]);
        assert!(has_2048(&grid));
    }
}
```

- [ ] **Step 3: Verify old code still compiles**

```bash
cargo check
```
Expected: no errors

- [ ] **Step 4: Commit**

```bash
git add src/board/resources.rs
git commit -m "feat: add board/resources.rs with Board, Score, helpers and unit tests"
```

---

## Task 3: Create `src/camera/mod.rs`

**Files:**
- Create: `src/camera/mod.rs`

- [ ] **Step 1: Create the file**

```rust
// src/camera/mod.rs
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Transform::default().looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb_u8(121, 163, 189)),
            ..default()
        },
    ));
}
```

- [ ] **Step 2: Verify old code still compiles**

```bash
cargo check
```
Expected: no errors

- [ ] **Step 3: Commit**

```bash
git add src/camera/mod.rs
git commit -m "feat: add CameraPlugin"
```

---

## Task 4: Create `src/input/`

**Files:**
- Create: `src/input/resources.rs`
- Create: `src/input/systems.rs`
- Create: `src/input/mod.rs`

- [ ] **Step 1: Create `src/input/resources.rs`**

```rust
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
```

- [ ] **Step 2: Create `src/input/systems.rs`**

```rust
// src/input/systems.rs
use crate::input::resources::Direction;
use crate::states::AppState;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

pub fn handle_input(
    mut direction: ResMut<Direction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        *direction = Direction::Restart;
        return;
    }

    if *state.get() != AppState::InGame {
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
```

- [ ] **Step 3: Create `src/input/mod.rs`**

```rust
// src/input/mod.rs
pub mod resources;
mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use resources::Direction;
use systems::handle_input;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Direction>().add_systems(
            Update,
            handle_input.run_if(
                in_state(AppState::InGame)
                    .or(in_state(AppState::Won))
                    .or(in_state(AppState::GameOver)),
            ),
        );
    }
}
```

- [ ] **Step 4: Verify old code still compiles**

```bash
cargo check
```
Expected: no errors

- [ ] **Step 5: Commit**

```bash
git add src/input/
git commit -m "feat: add InputPlugin with Direction resource and handle_input system"
```

---

## Task 5: Create `src/render/`

**Files:**
- Create: `src/render/systems.rs`
- Create: `src/render/mod.rs`

- [ ] **Step 1: Create `src/render/systems.rs`**

```rust
// src/render/systems.rs
use crate::board::resources::BOARD_SIZE;
use bevy::prelude::*;

const TILE_SIZE: f32 = 100.0;
const GAP: f32 = 12.0;
const GRID_BG_SIZE: f32 = BOARD_SIZE as f32 * TILE_SIZE + (BOARD_SIZE as f32 + 1.0) * GAP;

pub fn setup_grid_background(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.106, 0.106, 0.118),
            custom_size: Some(Vec2::new(GRID_BG_SIZE, GRID_BG_SIZE)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
```

- [ ] **Step 2: Create `src/render/mod.rs`**

```rust
// src/render/mod.rs
mod systems;

use bevy::prelude::*;
use systems::setup_grid_background;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_grid_background);
    }
}
```

- [ ] **Step 3: Verify old code still compiles**

```bash
cargo check
```
Expected: no errors

- [ ] **Step 4: Commit**

```bash
git add src/render/
git commit -m "feat: add RenderPlugin with setup_grid_background"
```

---

## Task 6: Create `src/ui/`

**Files:**
- Create: `src/ui/components.rs`
- Create: `src/ui/systems.rs`
- Create: `src/ui/mod.rs`

- [ ] **Step 1: Create `src/ui/components.rs`**

```rust
// src/ui/components.rs
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct OverlayText;
```

- [ ] **Step 2: Create `src/ui/systems.rs`**

```rust
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
```

- [ ] **Step 3: Create `src/ui/mod.rs`**

```rust
// src/ui/mod.rs
pub mod components;
mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use systems::{setup_ui, update_ui};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui).add_systems(
            Update,
            update_ui.run_if(
                in_state(AppState::InGame)
                    .or(in_state(AppState::Won))
                    .or(in_state(AppState::GameOver)),
            ),
        );
    }
}
```

- [ ] **Step 4: Verify old code still compiles**

```bash
cargo check
```
Expected: no errors

- [ ] **Step 5: Commit**

```bash
git add src/ui/
git commit -m "feat: add UIPlugin with ScoreText, OverlayText, setup_ui, update_ui"
```

---

## Task 7: Create `src/board/systems.rs` and `src/board/mod.rs`

**Files:**
- Create: `src/board/systems.rs`
- Create: `src/board/mod.rs`

- [ ] **Step 1: Write failing tests for `compact_and_merge`**

These will live inside `src/board/systems.rs`. Write them before the implementation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_merge_empty_line() {
        let (result, pts, changed) = compact_and_merge(vec![]);
        assert_eq!(result, vec![0, 0, 0, 0]);
        assert_eq!(pts, 0);
        assert!(!changed);
    }

    #[test]
    fn compact_merge_no_change() {
        let (result, pts, changed) = compact_and_merge(vec![2, 4, 8, 16]);
        assert_eq!(result, vec![2, 4, 8, 16]);
        assert_eq!(pts, 0);
        assert!(!changed);
    }

    #[test]
    fn compact_merge_simple_pair() {
        let (result, pts, changed) = compact_and_merge(vec![2, 2]);
        assert_eq!(result, vec![4, 0, 0, 0]);
        assert_eq!(pts, 4);
        assert!(changed);
    }

    #[test]
    fn compact_merge_with_gaps() {
        let (result, pts, changed) = compact_and_merge(vec![2, 2, 4]);
        assert_eq!(result, vec![4, 4, 0, 0]);
        assert_eq!(pts, 4);
        assert!(changed);
    }

    #[test]
    fn compact_merge_no_double_merge() {
        // [2, 2, 2, 2] should become [4, 4, 0, 0], not [8, 0, 0, 0]
        let (result, pts, changed) = compact_and_merge(vec![2, 2, 2, 2]);
        assert_eq!(result, vec![4, 4, 0, 0]);
        assert_eq!(pts, 8);
        assert!(changed);
    }
}
```

- [ ] **Step 2: Create `src/board/systems.rs`**

```rust
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
                let line: Vec<u32> = (0..BOARD_SIZE).filter_map(|col| grid[row][col]).collect();
                let (merged, pts, changed) = compact_and_merge(line);
                if changed { moved = true; }
                points += pts;
                for (col, &val) in merged.iter().enumerate() {
                    if val > 0 { new_grid[row][col] = Some(val); }
                }
            }
        }
        Direction::Right => {
            for row in 0..BOARD_SIZE {
                let line: Vec<u32> = (0..BOARD_SIZE).rev().filter_map(|col| grid[row][col]).collect();
                let (merged, pts, changed) = compact_and_merge(line);
                if changed { moved = true; }
                points += pts;
                for (i, &val) in merged.iter().enumerate() {
                    if val > 0 { new_grid[row][BOARD_SIZE - 1 - i] = Some(val); }
                }
            }
        }
        Direction::Up => {
            for col in 0..BOARD_SIZE {
                let line: Vec<u32> = (0..BOARD_SIZE).filter_map(|row| grid[row][col]).collect();
                let (merged, pts, changed) = compact_and_merge(line);
                if changed { moved = true; }
                points += pts;
                for (row, &val) in merged.iter().enumerate() {
                    if val > 0 { new_grid[row][col] = Some(val); }
                }
            }
        }
        Direction::Down => {
            for col in 0..BOARD_SIZE {
                let line: Vec<u32> = (0..BOARD_SIZE).rev().filter_map(|row| grid[row][col]).collect();
                let (merged, pts, changed) = compact_and_merge(line);
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

fn compact_and_merge(line: Vec<u32>) -> (Vec<u32>, u32, bool) {
    let original = {
        let mut v = line.clone();
        while v.len() < BOARD_SIZE { v.push(0); }
        v
    };

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
        let (result, pts, changed) = compact_and_merge(vec![]);
        assert_eq!(result, vec![0, 0, 0, 0]);
        assert_eq!(pts, 0);
        assert!(!changed);
    }

    #[test]
    fn compact_merge_no_change() {
        let (result, pts, changed) = compact_and_merge(vec![2, 4, 8, 16]);
        assert_eq!(result, vec![2, 4, 8, 16]);
        assert_eq!(pts, 0);
        assert!(!changed);
    }

    #[test]
    fn compact_merge_simple_pair() {
        let (result, pts, changed) = compact_and_merge(vec![2, 2]);
        assert_eq!(result, vec![4, 0, 0, 0]);
        assert_eq!(pts, 4);
        assert!(changed);
    }

    #[test]
    fn compact_merge_with_gaps() {
        let (result, pts, changed) = compact_and_merge(vec![2, 2, 4]);
        assert_eq!(result, vec![4, 4, 0, 0]);
        assert_eq!(pts, 4);
        assert!(changed);
    }

    #[test]
    fn compact_merge_no_double_merge() {
        let (result, pts, changed) = compact_and_merge(vec![2, 2, 2, 2]);
        assert_eq!(result, vec![4, 4, 0, 0]);
        assert_eq!(pts, 8);
        assert!(changed);
    }
}
```

- [ ] **Step 3: Create `src/board/mod.rs`**

```rust
// src/board/mod.rs
pub mod resources;
mod systems;

use crate::states::AppState;
use bevy::prelude::*;
use resources::{Board, Score};
use systems::{process_turn, spawn_initial_tiles};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .init_resource::<Score>()
            .add_systems(Startup, spawn_initial_tiles)
            .add_systems(
                Update,
                process_turn.run_if(
                    in_state(AppState::InGame)
                        .or(in_state(AppState::Won))
                        .or(in_state(AppState::GameOver)),
                ),
            );
    }
}
```

- [ ] **Step 4: Verify old code still compiles**

```bash
cargo check
```
Expected: no errors

- [ ] **Step 5: Commit**

```bash
git add src/board/
git commit -m "feat: add BoardPlugin with process_turn, spawn logic, and unit tests"
```

---

## Task 8: Rewrite `src/main.rs`

**Files:**
- Modify: `src/main.rs`

- [ ] **Step 1: Replace the entire file**

```rust
// src/main.rs
mod board;
mod camera;
mod components;
mod input;
mod render;
mod states;
mod ui;

use bevy::prelude::*;
use camera::CameraPlugin;
use board::BoardPlugin;
use input::InputPlugin;
use render::RenderPlugin;
use states::AppState;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins((CameraPlugin, RenderPlugin, BoardPlugin, InputPlugin, UIPlugin))
        .run();
}
```

- [ ] **Step 2: Verify the new code compiles**

```bash
cargo check
```
Expected: no errors (old `resources/`, `systems/`, `utils/` modules are no longer referenced and will just be ignored)

- [ ] **Step 3: Run unit tests**

```bash
cargo test
```
Expected: all tests pass — `compact_merge_*` (5 tests) + `has_moves_*` (3 tests) + `has_2048_*` (2 tests)

- [ ] **Step 4: Commit**

```bash
git add src/main.rs
git commit -m "refactor: rewrite main.rs to use 5 plugins and AppState"
```

---

## Task 9: Delete old modules and final verification

**Files:**
- Delete: `src/resources/` (board.rs, game_state.rs, score.rs, mod.rs)
- Delete: `src/systems/` (camera.rs, game_logic.rs, input.rs, rendering.rs, ui.rs, mod.rs)
- Delete: `src/utils/` (directions.rs, mod.rs)

- [ ] **Step 1: Delete old directories**

```bash
rm -rf src/resources src/systems src/utils
```

- [ ] **Step 2: Final compilation and tests**

```bash
cargo check && cargo test
```
Expected:
```
   Compiling TwoZeroFourEight ...
    Finished ...
running 10 tests
test board::resources::tests::has_2048_not_present ... ok
test board::resources::tests::has_2048_present ... ok
test board::resources::tests::has_moves_adjacent_same_value ... ok
test board::resources::tests::has_moves_empty_board ... ok
test board::resources::tests::has_moves_full_no_merges ... ok
test board::systems::tests::compact_merge_empty_line ... ok
test board::systems::tests::compact_merge_no_change ... ok
test board::systems::tests::compact_merge_no_double_merge ... ok
test board::systems::tests::compact_merge_simple_pair ... ok
test board::systems::tests::compact_merge_with_gaps ... ok
test result: ok. 10 passed
```

- [ ] **Step 3: Run the game to verify visually**

```bash
cargo run
```
Expected: game window opens, 2 tiles appear on the grid, arrow keys move tiles, R restarts, reaching 2048 shows "You Win!", no moves shows "Game Over".

- [ ] **Step 4: Final commit**

```bash
git add -A
git commit -m "refactor: remove dissolved modules (resources/, systems/, utils/)"
```
