# Agent Instructions

- This project is a Rust port of the .NET project in https://github.com/MovGP0/ChoreoApp.git
- You may clone the project in the `.temp/` folder for reference
- We need the project to execute in Windows, MacOS, Linux, Android, iOS, and Web (WASM)
- Consider a dedicated project for each build target

## Crates

- use [egui](https://github.com/emilk/egui) as UI layer
- use [egui-material3](https://github.com/nikescar/egui-material3/) for Material 3 components
- use [rspec](https://github.com/rust-rspec/rspec) for BDD testing
- use [material-color-utilities](https://github.com/deminearchiver/material-color-utilities-rust) for material colors
- use [nject](https://github.com/nicolascotton/nject) for dependency injection
- use [rodio](https://docs.rs/rodio/latest/rodio/) for audio playback on desktop/mobile
- use [web-audio-api](https://docs.rs/web-audio-api/latest/web_audio_api/) for audio playback on WASM
- use [log](https://crates.io/crates/log) and [env_logger](https://crates.io/crates/env_logger) for logging

## Dependency Injection
- use nject for dependency injection
- All ViewModels and Behaviors need to be injectable
- Behaviors are always transient

## Unit Testing
See `UnitTesting.md` for detailed instructions.

## Spec Driven Development

Use Spec Driven Development (SDD) as the default workflow for behavior changes and bug fixes:

1. Write or extend `_spec.rs` tests first for the target scenario.
2. Run the smallest relevant test target and confirm it fails (red).
3. Implement the minimal production change needed to satisfy the spec.
4. Re-run the same focused spec until it passes (green).
5. Refactor only when specs stay green and behavior is unchanged.
6. Add regression coverage for discovered edge cases before closing the task.

Rules:
- Prefer `rspec` BDD style (`describe`/`it`) for new behavior scenarios.
- Keep specs behavior-focused (user-visible outcome), not implementation-coupled.
- For synchronization/state-flow work, codify scenarios in a document first (for example `Timestamp.md`), then map each scenario to one or more specs.
- Do not introduce business logic changes without a corresponding failing spec first, unless fixing build/lint breakages required to run tests.
- When a bug is reported, first reproduce it with a spec; the fix is complete only when that spec passes and remains in the suite.
- If essential logic required by a spec is missing (for example a class/function does not exist yet), first create a clear `TODO:` list in the spec file that documents the planned implementation steps and required seams; once the classes/functions are in place, replace those `TODO:` entries with the actual spec code.

## Model View Behavior Pattern
See `ModelViewBehavior.md` for detailed instructions.
Key rules:
- Activate behaviors before attaching the View-ViewModel adapter, since the adapter holds a mutable borrow.
- Do not borrow the ViewModel inside subscription callbacks; the adapter is still attached with a mutable borrow.

## UI Translations

- Use a dedicated translation module in the egui layer for all UI strings.
- Bind labels/text in egui from centralized translation lookups.
- ViewModels must not contain UI strings or translation keys.

## UI Colors and Styling

- Apply colors and typography through egui/egui-material3 theming.
- Keep styling in dedicated egui theme modules; behaviors are reserved for business logic.
- Do not add behavior classes for styling-only concerns.

## UI Layout Grid

- Use a strict `12px` base grid for layout dimensions.
- Allowed values for spacing, padding, margins, gaps, widths, and heights are multiples of `12px` only:
  - `12px`, `24px`, `36px`, `48px`, `60px`, `72px`, `84px`, `96px`, ...
- Values like `10px`, `14px`, `56px`, etc. are not allowed for layout sizing/spacing.
- Exceptions are only allowed for:
  - Hairlines/borders/strokes (`1px` or `2px`)
  - Corner radii and icon glyph sizes when required by Material components
  - Third-party control internals that are not configurable
- For exceptions, add a short code comment explaining why a non-grid value is required.

## Material 3 Component Mapping (egui)

- Use egui-material3 widgets/themes as first choice for Material 3 UI.
- Map previous window/root views to egui app/root panels.
- Map button/card/input/selection/navigation/dialog/progress/feedback controls to egui-material3 equivalents.
- Keep unsupported controls as explicit custom egui widgets.
- Prefer shared widget modules so behavior parity is implemented once and reused across targets.

## Code style

- Place usings into separate lines to improve readability and reduce merge conflicts
```rust
use choreo_state_machine::{
    ApplicationStateMachine,
    MovePositionsCompletedTrigger
};

use crate::global::{
    GlobalStateModel,
    InteractionMode
}; 
```

## Linting

- Run: `cargo clippy --all-targets --all-features -- -D warnings`
- Add to crate roots where applicable:
  ```rust
  #![deny(warnings)]
  #![deny(unsafe_code)]
  #![deny(rust_2018_idioms)]
  #![deny(unused_must_use)]
  #![deny(unreachable_pub)]
  #![deny(elided_lifetimes_in_paths)]
  #![deny(clippy::all)]
  ```
- Set panic abort in `Cargo.toml`:
  ```toml
  [profile.release]
  panic = "abort"

  [profile.dev]
  panic = "abort"
  ```

## Check progress
After making changes, YOU MUST do the following:

Check if the changed project builds
```sh
cargo build -p PROJECTNAME
```
Check if the linter finds any errors
```sh
cargo clippy -p PROJECTNAME --all-targets --all-features -- -D warnings
```
Check if the unit tests work
```sh
cargo test -p PROJECTNAME
```
Let the user test and confirm the changes manually. Do not create a commit before the user confirms the changes work as expected.
If the user finds any issues, fix them and repeat the checks above until the user confirms the changes are good.

Only when the changes are verified, you can close the bd ticket.

**Example:** Build and run with Open Telemetry (OTEL) support:
```sh
cargo run -p rchoreo_desktop --bin rchoreo_desktop --features debug-otel
```

## Rust Common Mistakes to Avoid

- Avoid `&String` when `&str` is sufficient; prefer slices in APIs.
- Avoid `Rc`/`RefCell` unless shared mutable ownership is required; prefer ownership/borrows.
- Avoid slice indexing; use iterators or `array_windows()` to prevent off-by-one bugs.
- Use correct integer types for domain values; avoid lossy or overflowing casts.
- Avoid sentinel values (`-1`, `""`, `null`); use `Option<T>`.
- Prefer enums over magic strings for state/roles.
- Use proper error propagation (`?`) and implement `std::error::Error` for custom errors.
- Implement standard traits where appropriate to integrate with the ecosystem.
- Prefer standard library macros/helpers over hand-rolled versions.
- Use tooling: `cargo fmt` and `cargo clippy`.

# Architecture

## Specific to `choreo_components` module

- each component is a module
- structure into a folder per module (`main` is not allowed in rust as a folder or module name)
- place view models into a NAME_view_module.rs file (e.g. `floor_view_model.rs`)
- place behaviors into dedicated NAME_behavior.rs files (e.g. `draw_floor_behavior.rs`)
- place View-ViewModel adapters into dedicated NAME_adapter.rs files (e.g. `floor_adapter.rs`)
- place message (Event, Command, Query, Response) types into `messages.rs`
- place egui views/widgets into shared `ui` or `widgets` modules

### Floor Component

- `FloorCanvasView` uses `content.floor_x`, `content.floor_y`, `content.floor_width`, and `content.floor_height` as the single source of truth for transformed floor placement.
- In Y direction, header space is reserved first (`header_height_px`), then floor Y is computed from the remaining height.
- Header overlay is rendered above all floor layers (`z` high), but its position/size are still bound to transformed floor coordinates:
  - `x = content.floor_x`
  - `y = content.floor_y - content.header_height_px`
  - `width = content.floor_width`
- SVG overlay must scale from the same layout basis as the floor (`content.layout_width_px`, `content.content_height_px`, and `content.zoom`) and remain centered at `content.center_x/content.center_y`.
- Layering order inside floor rendering:
  1. floor background
  2. grid lines
  3. floor SVG
  4. from/to paths
  5. position circles
  6. position numbers
- Curves and dashed path segments are command-based geometry generated in `floor_adapter.rs`; when layout/bounds change, redraw/apply must run so path commands are regenerated with current transform.

# Issue Tracking

This project uses **bd** (beads) for issue tracking. Run `bd onboard` to get started.

## Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --status in_progress  # Claim work
bd close <id>         # Complete work
bd sync               # Sync with git
```

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds

# Lessons Learned

If you encounter a compile error after a code change you did, keep a note here how to avoid the problem in the future:

- `rspec::describe` requires an explicit environment argument (use `()` when none), and the suite type is `rspec::block::Suite<T>` with `Report` imported for `is_success()`.
- When using `rspec` in tests, set `exit_on_failure(false)` in `ConfigurationBuilder` to avoid aborting the whole test process and to surface failures in `SuiteReport`.
- Clippy denies range loops used only for indexing; prefer iterators with `enumerate()` and direct `contains()` for sentinel checks.
- For egui event/callback flow, route UI events into typed actions and reduce state in one place.
- Keep egui-material3 theme wiring centralized; avoid duplicating palette logic across widgets.
- For responsive layout in egui, prefer panels/layout APIs and avoid hard-coded pixel constants unless required.
- For platform-independent current time, do not call `OffsetDateTime::now_utc()` directly in app/model code; route through a shared clock abstraction (for example `SystemClock::now_utc()`), and on `wasm32` compute time via `web-sys` (`window.performance().time_origin() + now()`) before converting to `OffsetDateTime`.

# egui

See `egui.md` for additional instructions.

# Open Telemetry

See `OpenTelemetry.md` for activity tracing.
