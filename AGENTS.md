# Agent Instructions

- This project is a Rust port of the .NET project in https://github.com/MovGP0/ChoreoApp.git
- You may clone the project in the `.temp/` folder for reference
- We need the project to execute in Windows, MacOS, Linux, Android, iOS, and Web (WASM)
- Consider a dedicated project for each build target

## Crates

- use [Slint](https://slint.dev/) as UI layer
- use [rspec](https://github.com/rust-rspec/rspec) for BDD testing
- use [material-color-utilities](https://github.com/deminearchiver/material-color-utilities-rust) for material colors
- use [nject](https://github.com/nicolascotton/nject) for dependency injection
- use [rodio](https://docs.rs/rodio/latest/rodio/) for audio playback
- use [log](https://crates.io/crates/log) and [env_logger](https://crates.io/crates/env_logger) for logging

## Dependency Injection
- use nject for dependency injection
- All ViewModels and Behaviors need to be injectable
- Behaviors are always transient

## Unit Testing
See `UnitTesting.md` for detailed instructions.

## Model View Behavior Pattern
See `ModelViewBehavior.md` for detailed instructions.

## MaterialDesignThemes to Slint (Material) replacement list

- App root: `material_window.slint` (MaterialWindow).
- Buttons: `filled_button.slint`, `elevated_button.slint`, `outline_button.slint`, `tonal_button.slint`, `text_button.slint`, `floating_action_button.slint`, `segmented_button.slint`, `icon_button.slint`, `outline_icon_button.slint`, `tonal_icon_button.slint`.
- Cards: `card.slint` (filled/outlined/elevated variants).
- Text input: `text_field.slint`, `drop_down_menu.slint`.
- Selection: `check_box.slint`, `radio_button.slint`, `switch.slint`.
- Chips: `chip.slint` (action/input/filter).
- Navigation & app bars: `app_bar.slint`, `bottom_app_bar.slint`, `navigation_bar.slint`, `navigation_rail.slint`, `navigation_drawer.slint`, `drawer.slint`, `search_bar.slint`.
- Dialogs/sheets: `dialog.slint`, `bottom_sheet.slint`, `modal.slint`.
- Progress: `progress_indicator.slint` (linear/circular).
- Feedback: `snack_bar.slint`, `tooltip.slint`, `divider.slint`.
- Other available: `date_picker.slint`, `time_picker.slint`, `menu.slint`, `tab_bar.slint`, `slider.slint`, `scroll_view.slint`, `badge.slint`, `list.slint`, `list_view.slint`, `icon.slint`.
- Still custom (no direct material.slint component): `AutoSuggestBox`, `ColorPicker/ColorZone`, `DataGrid`, `Expander`, `Flipper`, `HamburgerToggleButton`, `PopupBox`, `RatingBar`, `Ripple`, `SliderWithTicks`, `SplitButton`, `TreeView/TreeListView`, `Underline`.

Tips:
- Clone https://github.com/slint-ui/slint.git to .temp/ to inspect the source code.
- The material themed controls are located in `D:\RChoreoApp\.temp\slint\ui-libraries\material\src\`.

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
Only when the changes are verified, you can close the bd ticket.

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
- place message (Event, Command, Query, Response) types into `messages.rs`
- place the slint views into the shared `ui` folder

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
- In Slint, callbacks can be declared as `callback name(type);` or `callback name(arg: type);` (named args are ok); `callback name(type arg);` is invalid.
- For Material components, wire a `material` library alias in `build.rs` and import from `@material` (e.g., `import { FilledButton } from "@material";`).
- If Material widgets like `Slider` are missing, ensure the import is from `@material` and that the `material` library path points to `material.slint`.
- Material `CheckBox` has no `text`/`checked`/`toggled`; use `CheckBoxTile` with `check_state` and `checked_state_changed`.
- Material `Slider` emits `value_changed(value)` (not `changed`).
- If `material-1.0` mismatches the Slint version (e.g., `radio-button` accessibility role errors), sync to the template’s `material-1.0` or patch the role to `checkbox`.
- Keep `material-1.0` synced with the Slint tag in use (`v1.14.1` from `.temp/slint`); do not edit `material-1.0` directly.
- In Slint functions returning `length`, use units (e.g., `0px`) and avoid binding `width` to parent/root inside layouts; prefer `horizontal-stretch` to prevent layout loops.

# Slint

See `Slint.md` for additional instructions.
