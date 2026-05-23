# Touch Swipe Support — Design Spec

**Date:** 2026-05-23  
**Status:** Approved

## Objective

Add mobile touch swipe support to the 2048 Bevy/WASM game so players can move tiles by swiping on touchscreen devices.

## Architecture

### Approach

Use Bevy's built-in `EventReader<TouchInput>` — idiomatic, works in native and WASM, no external dependencies.

### New system: `handle_touch_input`

Added to `src/input/systems.rs`, registered in `InputPlugin` alongside `handle_input`.

**Flow:**
1. On `TouchPhase::Started` — store touch position in a `Local<Option<Vec2>>`
2. On `TouchPhase::Ended` — compute delta from stored start
3. If `delta.length() > 30.0` — determine dominant axis and write to `Direction` resource
4. On `TouchPhase::Cancelled` — clear stored start without writing direction

**Direction mapping:**
- `|dx| > |dy|` → Left (dx < 0) or Right (dx > 0)  
- `|dy| > |dx|` → Up (dy > 0) or Down (dy < 0) *(Bevy Y axis: up is positive)*

### Multi-touch

Only the first touch (`id == 0` or first event received) is tracked. Subsequent fingers are ignored.

## Integration

- `InputPlugin` registers `handle_touch_input` in the same `InputSet` and same state conditions as `handle_input`
- Both systems write to `Direction` resource — no conflict since only one can fire per frame in practice

## Constraints

- Minimum swipe distance: **30px** — balances accidental taps vs. intentional swipes
- No restart gesture — restart remains keyboard only (R key)
- No visual feedback for swipe — out of scope
