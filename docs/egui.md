# egui + `reducer`

The egui side of this repo should use the [`reducer`](https://docs.rs/reducer/latest/reducer/) crate as the top-level state container pattern.

## Recommended Module Layout

For the egui shell, prefer a dedicated app module:

```text
crates/choreo_components/src/MODULENAME/
  action.rs
  effect.rs
  reactor.rs
  runtime.rs
  state.rs
  ui.rs
```

Object responsibilities:

- `state.rs`: contains the state of the component
- `action.rs`: contains the actions that can be dispatched to update the state
- `effect.rs`: `AppEffect` values for IO/runtime work
- `reducer.rs`: pure state transition logic that updates the state based on an action
- `ui.rs`: pure egui rendering functions

## Component Submodule Responsibilities

Not every egui component should stop at `action.rs`, `state.rs`, `reducer.rs`, and `ui.rs`.

When a page or shared widget starts mixing layout math, animation helpers, style tokens, and painting code in one file, split those responsibilities into focused submodules.

The current `hamburger_toggle_button` module is a good example:

```text
crates/choreo_components/src/hamburger_toggle_button/
  mod.rs
  geometry.rs
  state.rs
  tokens.rs
  widget.rs
```

Use this ownership split:

- `mod.rs`: keep it thin. It declares submodules and re-exports the supported public surface.
- `geometry.rs`: own size calculations, rect-to-shape projection, interpolation math, and other deterministic layout helpers.
- `tokens.rs`: own style metrics, animation specs, padding constants, and other theme/material values that should be shared instead of duplicated.
- `widget.rs`: own the egui-facing widget type, `show()` or `draw()` entry points, response handling, and painting by composing `geometry`, `state`, and `tokens`.

Practical rules:

- Put pure geometry math in `geometry.rs`, not in `widget.rs`.
- Put egui `Context`/`Id` animation bookkeeping in `state.rs` when it is widget-local and not part of app/page reducer state.
- Put Material sizing, motion, and opacity lookups in `tokens.rs` so other widgets can reuse the same contract.
- Keep `widget.rs` focused on request/response flow and painting, not on owning every helper.
- Keep `mod.rs` free of implementation detail; it is the module boundary, not the implementation dump.

This split is especially useful for shared egui controls, reusable page panels, and page-local widgets that need parity math or animation behavior but do not justify becoming full reducer features on their own.

## Cross-Page Coordination

The app reducer should own transitions that affect more than one page.

Examples:

- opening Settings from Main
- saving dancer edits back into choreography state
- loading a `.choreo` file and updating multiple page projections
- changing theme settings that affect the whole shell

That logic should not live in a page UI function or in one page reducer mutating another page directly.

Instead:

1. a page emits a page action
2. the app reducer translates it into route changes and/or app effects
3. the runtime performs any required IO
4. follow-up actions are dispatched back into the store

## Practical Rules

When adding egui code in this repo:

- Prefer `Store<AppState, AppReactor>` over custom view-model `dispatch()` wrappers.
- Prefer `AppAction` enums over ad-hoc callback structs for state changes.
- Prefer `AppEffect` plus runtime execution over `outgoing_*` state queues.
- Keep `ui::draw()` functions read-only with respect to state; they should emit actions.
- Keep file dialogs, persistence, audio IO, timers, and OS integration outside reducers.
- Keep feature reducers pure and deterministic.
