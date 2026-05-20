# Plugin Architecture Design — 2048 Bevy

**Date:** 2026-05-20  
**Status:** Approved

---

## Objectif

Refactoriser le projet 2048 Bevy pour adopter une architecture à base de plugins Bevy et un contrôle de flux par `States`. Le code est actuellement câblé manuellement dans `main.rs` avec une `GameState` Resource qui sera supprimée.

---

## Architecture

### `AppState` (remplace `GameState` Resource)

```rust
// src/states.rs
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    Paused,
    Won,
    GameOver,
}
```

`GameState` (Resource) est supprimée. Les transitions d'état se font via `NextState<AppState>`.

---

### Structure des fichiers cible

```
src/
├── main.rs                    # App::new() + add_plugins(...)  ~10 lignes
├── states.rs                  # pub enum AppState
│
├── components/                # composants partagés entre plusieurs plugins
│   ├── mod.rs
│   ├── position.rs            # Position { row, col }
│   └── tile.rs                # Tile { value }
│
├── camera/
│   └── mod.rs                 # CameraPlugin + setup_camera
│
├── input/
│   ├── mod.rs                 # InputPlugin
│   ├── resources.rs           # Direction (déplacé depuis utils/)
│   └── systems.rs             # handle_input
│
├── board/
│   ├── mod.rs                 # BoardPlugin
│   ├── resources.rs           # Board, Score (déplacés depuis resources/)
│   └── systems.rs             # process_turn, spawn_random_tile
│
├── render/
│   ├── mod.rs                 # RenderPlugin
│   └── systems.rs             # setup_grid_background
│
└── ui/
    ├── mod.rs                 # UIPlugin
    ├── components.rs          # marqueurs UI spécifiques (ScoreText, etc.)
    └── systems.rs             # setup_ui, update_ui
```

**Modules supprimés :** `src/resources/`, `src/systems/`, `src/utils/`

---

### Mapping État / Plugin

| Plugin | OnEnter | Update (run_if) | OnExit |
|--------|---------|-----------------|--------|
| `CameraPlugin` | `Startup` (global) | — | — |
| `RenderPlugin` | `OnEnter(InGame)` | — | — |
| `BoardPlugin` | `OnEnter(InGame)` | `in_state(InGame)` | `OnExit(InGame)` despawn tuiles + reset Board/Score |
| `InputPlugin` | — | `in_state(InGame)` | — |
| `UIPlugin` | `OnEnter(InGame)` (setup_ui) | `in_state(InGame)` (update_ui) | — |

---

### Pattern plugin (uniforme dans tous les `mod.rs`)

```rust
pub struct XxxPlugin;

impl Plugin for XxxPlugin {
    fn build(&self, app: &mut App) {
        app
            // ressources éventuelles
            // systèmes enregistrés avec run_if(in_state(...))
    }
}
```

---

### `main.rs` final

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()           // avant les plugins
        .add_plugins((
            CameraPlugin,
            RenderPlugin,
            BoardPlugin,
            InputPlugin,
            UIPlugin,
        ))
        .run();
}
```

---

## Contraintes

- `.init_state::<AppState>()` doit précéder `.add_plugins(...)` — les plugins référencent `AppState` au moment du `build()`.
- `Position` et `Tile` restent dans `components/` car utilisés par `board/` et `render/` simultanément.
- `Direction` (ex-`utils/`) est encapsulée dans `input/` car elle n'est lue que par `InputPlugin`.
- `Board` et `Score` sont encapsulées dans `board/` car elles ne sont modifiées que par `BoardPlugin`.

---

## Ce qui n'est pas dans ce spec

- Implémentation du menu principal (UI de `MainMenu`) — les systèmes `UIPlugin` pour cet état sont à définir dans une prochaine itération.
- Logique de pause (les keybinds Échap et la transition `InGame ↔ Paused`) — à implémenter dans `InputPlugin` une fois l'architecture en place.
