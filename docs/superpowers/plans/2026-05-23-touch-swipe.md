# Touch Swipe Support Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add mobile swipe gesture detection so players can move tiles on touchscreen devices.

**Architecture:** Extract swipe direction logic into a pure function `swipe_direction`, tested in isolation. A Bevy system `handle_touch_input` reads `TouchInput` events, uses that function, and writes to the existing `Direction` resource — identical integration point to keyboard input.

**Tech Stack:** Bevy 0.18, `bevy::input::touch::{TouchInput, TouchPhase}`, existing `Direction` resource.

---

## File Map

| File | Action | Purpose |
|------|--------|---------|
| `src/input/systems.rs` | Modify | Add `swipe_direction` pure fn + `handle_touch_input` system |
| `src/input/mod.rs` | Modify | Register `handle_touch_input` in `InputPlugin` |

---

### Task 1: Add `swipe_direction` pure function with tests

**Files:**
- Modify: `src/input/systems.rs`

- [ ] **Step 1: Write failing tests at bottom of `src/input/systems.rs`**

Append the following to `src/input/systems.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::math::Vec2;

    #[test]
    fn swipe_left() {
        let dir = swipe_direction(Vec2::new(200.0, 100.0), Vec2::new(150.0, 102.0));
        assert_eq!(dir, Some(Direction::Left));
    }

    #[test]
    fn swipe_right() {
        let dir = swipe_direction(Vec2::new(100.0, 100.0), Vec2::new(160.0, 98.0));
        assert_eq!(dir, Some(Direction::Right));
    }

    #[test]
    fn swipe_up() {
        // Screen Y increases downward: swipe up → delta.y < 0
        let dir = swipe_direction(Vec2::new(100.0, 200.0), Vec2::new(102.0, 140.0));
        assert_eq!(dir, Some(Direction::Up));
    }

    #[test]
    fn swipe_down() {
        // Screen Y increases downward: swipe down → delta.y > 0
        let dir = swipe_direction(Vec2::new(100.0, 100.0), Vec2::new(98.0, 160.0));
        assert_eq!(dir, Some(Direction::Down));
    }

    #[test]
    fn tap_too_short_ignored() {
        let dir = swipe_direction(Vec2::new(100.0, 100.0), Vec2::new(110.0, 100.0));
        assert_eq!(dir, None);
    }

    #[test]
    fn diagonal_resolves_to_dominant_axis() {
        // dx=60 > dy=20 → horizontal
        let dir = swipe_direction(Vec2::new(100.0, 100.0), Vec2::new(160.0, 120.0));
        assert_eq!(dir, Some(Direction::Right));
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

```bash
cargo test swipe
```

Expected: compile error — `swipe_direction` not found yet.

- [ ] **Step 3: Implement `swipe_direction` in `src/input/systems.rs`**

Add this function above the existing `handle_input` function:

```rust
const SWIPE_THRESHOLD: f32 = 30.0;

fn swipe_direction(start: Vec2, end: Vec2) -> Option<Direction> {
    let delta = end - start;
    if delta.length() < SWIPE_THRESHOLD {
        return None;
    }
    if delta.x.abs() > delta.y.abs() {
        Some(if delta.x > 0.0 { Direction::Right } else { Direction::Left })
    } else {
        // Touch screen Y increases downward: negative delta.y = finger moved up
        Some(if delta.y < 0.0 { Direction::Up } else { Direction::Down })
    }
}
```

Also add `use bevy::math::Vec2;` to the imports at the top of the file (it is already pulled in transitively via `bevy::prelude::*` — no import needed if `bevy::prelude::*` is already used; otherwise add it explicitly).

- [ ] **Step 4: Run tests to verify they pass**

```bash
cargo test swipe
```

Expected output:
```
test input::systems::tests::swipe_left ... ok
test input::systems::tests::swipe_right ... ok
test input::systems::tests::swipe_up ... ok
test input::systems::tests::swipe_down ... ok
test input::systems::tests::tap_too_short_ignored ... ok
test input::systems::tests::diagonal_resolves_to_dominant_axis ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/input/systems.rs
git commit -m "feat: add swipe_direction pure function with tests"
```

---

### Task 2: Add `handle_touch_input` system

**Files:**
- Modify: `src/input/systems.rs`
- Modify: `src/input/mod.rs`

- [ ] **Step 1: Add `handle_touch_input` system to `src/input/systems.rs`**

Add the following imports at the top of `src/input/systems.rs` (alongside the existing `use bevy::input::keyboard::KeyCode;`):

```rust
use bevy::input::touch::{TouchInput, TouchPhase};
```

Add this system function after `handle_input`:

```rust
pub fn handle_touch_input(
    mut direction: ResMut<Direction>,
    mut touch_events: EventReader<TouchInput>,
    mut tracked: Local<Option<(u64, Vec2)>>,
) {
    for event in touch_events.read() {
        match event.phase {
            TouchPhase::Started => {
                // Only track the first active finger; ignore subsequent touches
                if tracked.is_none() {
                    *tracked = Some((event.id, event.position));
                }
            }
            TouchPhase::Ended => {
                if let Some((id, start_pos)) = *tracked {
                    if event.id == id {
                        *tracked = None;
                        if let Some(dir) = swipe_direction(start_pos, event.position) {
                            *direction = dir;
                        }
                    }
                }
            }
            TouchPhase::Cancelled => {
                if tracked.map_or(false, |(id, _)| id == event.id) {
                    *tracked = None;
                }
            }
            _ => {}
        }
    }
}
```

Note: `Local<T>` is Bevy's per-system local state — persists across frames, owned by this system only. The `(u64, Vec2)` pair tracks the active finger id + start position so multi-touch is handled correctly (only the first finger drives the swipe).

- [ ] **Step 2: Register the system in `src/input/mod.rs`**

Update the `use` imports to also expose `handle_touch_input`:

```rust
use systems::{handle_input, handle_touch_input};
```

Replace the `add_systems` block inside `Plugin for InputPlugin`:

```rust
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Direction>()
            .add_systems(
                Update,
                handle_input
                    .in_set(InputSet)
                    .run_if(
                        in_state(AppState::InGame)
                            .or(in_state(AppState::Won))
                            .or(in_state(AppState::GameOver)),
                    ),
            )
            .add_systems(
                Update,
                handle_touch_input
                    .in_set(InputSet)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
```

- [ ] **Step 3: Build to verify it compiles**

```bash
cargo build
```

Expected: no errors.

- [ ] **Step 4: Run all tests**

```bash
cargo test
```

Expected: all tests pass (swipe tests + existing board tests).

- [ ] **Step 5: Commit**

```bash
git add src/input/systems.rs src/input/mod.rs
git commit -m "feat: add touch swipe support for mobile input"
```

---

## Manual Verification

To test on mobile:
1. Build for WASM: `cargo build --target wasm32-unknown-unknown --profile wasm-release`
2. Deploy or use `wasm-bindgen` tooling to serve locally
3. Open on a mobile browser, swipe in each direction — tiles should move accordingly
4. Verify a short tap does not trigger a move

For quick desktop verification: Chrome DevTools → toggle device toolbar → simulate touch events.
