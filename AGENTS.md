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

## MaterialDesignThemes to Slint (Material) replacement list

- App root: `MaterialWindow` for themed root window.
- Buttons: `FilledButton`, `ElevatedButton`, `OutlineButton`, `TonalButton`, `TextButton`, `FloatingActionButton`, `SegmentedButton`, `IconButton`, `OutlineIconButton`, `TonalIconButton`.
- Cards: `FilledCard`, `OutlinedCard`, `ElevatedCard`.
- Text input: `TextField` (use for TextBox/SmartHint-like input), `DropDownMenu` for ComboBox-style selections.
- Selection: `CheckBox`, `CheckBoxTile`, `RadioButton`, `Switch`.
- Chips: `ActionChip`, `InputChip`, `FilterChip`.
- Navigation & app bars: `NavigationBar`, `NavigationRail`, `NavigationDrawer`, `ModalNavigationDrawer`, `ModalDrawer`, `BottomAppBar`, `LargeAppBar`, `SmallAppBar`, `SearchBar`.
- Dialogs: `Dialog`, `FullscreenDialog` (modal flows).
- Progress: `LinearProgressIndicator`, `CircularProgressIndicator`.
- Feedback: `SnackBar`, `ToolTip`, `HorizontalDivider`, `VerticalDivider`.
- Existing but no direct Slint Material component (custom Slint component): `AutoSuggestBox`, `Badged`, `Calendar/DatePicker`, `Clock/TimePicker`, `ColorPicker/ColorZone`, `DataGrid`, `Expander`, `Flipper`, `HamburgerToggleButton`, `Menu/MenuItem`, `PopupBox`, `RatingBar`, `Ripple`, `ScrollViewer/ScrollBar`, `SliderWithTicks`, `SplitButton`, `TabView`, `TreeView/TreeListView`, `Underline`.

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
