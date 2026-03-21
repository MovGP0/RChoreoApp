# egui + `reducer`

The egui side of this repo should use the [`reducer`](https://docs.rs/reducer/latest/reducer/) crate as the top-level state container pattern.

## Current Project Status

The current codebase is close to a reducer architecture, but it stops short of using a real store boundary.

- `crates/choreo_components_egui/src/choreo_main/reducer.rs` already contains most of the main page state transition logic.
- Feature slices such as `audio_player`, `floor`, `settings`, `scenes`, `dancers`, and `drawer_host` already expose reducer-style `reduce(state, action)` functions.
- `MainViewModel` is mostly a thin wrapper around `ChoreoMainState` plus `dispatch()`.

The main architectural problem is that orchestration still lives outside the reducer layer:

- `AppShellViewModel` decides initialization, splash handling, theme application, and frame-time repaint policy.
- `MainPageBinding` mixes dispatch, behavior activation, runtime polling, file routing, and imperative side effects.
- `runtime.rs` drains `outgoing_*` queues from state, runs IO, and then dispatches more actions.

That means the project currently has reducers, but not a reducer-driven application shell.

## Target Shape

Use one top-level `Store<AppState, AppReactor>` for the egui app host.

- `AppState` owns the full egui application state.
- `AppAction` is the only way state changes.
- `Reducer<AppAction>` is implemented for `AppState`.
- Feature states still keep their own reducer logic.
- A runtime layer executes effects such as file dialogs, audio IO, persistence, and external file opening.
- The egui host renders from store state and dispatches actions back into the store.

The practical goal is to replace:

- `AppShellViewModel`
- `MainPageBinding`
- the hand-rolled `outgoing_*` command queues

with:

- `Store<AppState, AppReactor>`
- a small `AppRuntime`
- typed `AppEffect` values

## Recommended Module Layout

For the egui shell, prefer a dedicated app module:

```text
crates/choreo_components_egui/src/app/
  action.rs
  effect.rs
  reactor.rs
  runtime.rs
  state.rs
  ui.rs
```

Suggested responsibilities:

- `state.rs`: top-level `AppState`
- `action.rs`: top-level `AppAction`
- `effect.rs`: `AppEffect` values for IO/runtime work
- `reactor.rs`: egui-facing reactor for repaint notifications and store subscriptions
- `runtime.rs`: executes effects and dispatches follow-up actions
- `ui.rs`: pure egui rendering functions that read state and emit actions

Keep feature reducers where they already live. Wrap them in `reducer::Reducer` trait impls instead of rewriting everything at once.

## Distinct Page Modules

Yes, this pattern should still keep distinct modules for pages.

Using `reducer::Store` does not mean the app must collapse into one giant reducer file. The store is the top-level state container. Pages remain separate modules with their own state, actions, reducer logic, and egui rendering.

For this repo, a better shape is:

```text
crates/choreo_components_egui/src/
  app/
    action.rs
    effect.rs
    reactor.rs
    runtime.rs
    state.rs
  pages/
    main_page/
      action.rs
      reducer.rs
      state.rs
      ui.rs
    settings_page/
      action.rs
      reducer.rs
      state.rs
      ui.rs
    dancers_page/
      action.rs
      reducer.rs
      state.rs
      ui.rs
    splash_page/
      action.rs
      reducer.rs
      state.rs
      ui.rs
```

Recommended ownership split:

- `app/*`: navigation, global shell state, effect execution, external file routing, repaint policy, and cross-page coordination
- `pages/main_page/*`: choreography editor state and UI
- `pages/settings_page/*`: settings-specific state and UI
- `pages/dancers_page/*`: dancer editing state and UI
- `pages/splash_page/*`: splash-specific state and UI

The important boundary is:

- page reducers handle page-local transitions
- the app reducer decides which page is active and how pages coordinate
- the runtime executes OS and IO side effects

## Page Routing Example

The top-level app state can keep distinct page states while still using one store:

```rust
use reducer::Reducer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Route {
    #[default]
    Splash,
    Main,
    Settings,
    Dancers,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppAction {
    Navigate(Route),
    SplashPage(splash_page::SplashPageAction),
    MainPage(main_page::MainPageAction),
    SettingsPage(settings_page::SettingsPageAction),
    DancersPage(dancers_page::DancersPageAction),
    ExternalFileDropped(std::path::PathBuf),
    EffectsDrained,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AppState {
    pub route: Route,
    pub splash_page: splash_page::SplashPageState,
    pub main_page: main_page::MainPageState,
    pub settings_page: settings_page::SettingsPageState,
    pub dancers_page: dancers_page::DancersPageState,
    pub effects: Vec<AppEffect>,
}

impl Reducer<AppAction> for AppState {
    fn reduce(&mut self, action: AppAction) {
        match action {
            AppAction::Navigate(route) => {
                self.route = route;
            }
            AppAction::SplashPage(action) => {
                splash_page::reducer::reduce(&mut self.splash_page, action);
            }
            AppAction::MainPage(action) => {
                main_page::reducer::reduce(&mut self.main_page, action);
            }
            AppAction::SettingsPage(action) => {
                settings_page::reducer::reduce(&mut self.settings_page, action);
            }
            AppAction::DancersPage(action) => {
                dancers_page::reducer::reduce(&mut self.dancers_page, action);
            }
            AppAction::ExternalFileDropped(path) => {
                self.effects.push(AppEffect::RouteExternalFile(path));
            }
            AppAction::EffectsDrained => {
                self.effects.clear();
            }
        }
    }
}
```

Then the egui host renders the active page, but still dispatches everything into the same store:

```rust
egui::CentralPanel::default().show(context, |ui| match self.store.route {
    Route::Splash => {
        for action in splash_page::ui::draw(ui, &self.store.splash_page) {
            self.store.dispatch(AppAction::SplashPage(action)).unwrap();
        }
    }
    Route::Main => {
        for action in main_page::ui::draw(ui, &self.store.main_page) {
            self.store.dispatch(AppAction::MainPage(action)).unwrap();
        }
    }
    Route::Settings => {
        for action in settings_page::ui::draw(ui, &self.store.settings_page) {
            self.store.dispatch(AppAction::SettingsPage(action)).unwrap();
        }
    }
    Route::Dancers => {
        for action in dancers_page::ui::draw(ui, &self.store.dancers_page) {
            self.store.dispatch(AppAction::DancersPage(action)).unwrap();
        }
    }
});
```

This keeps modules distinct without losing the benefits of a single source of truth.

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

## Example Structure

This example uses the current repo concepts:

- `ChoreoMainState`
- `ChoreoMainAction`
- `AudioPlayerRuntime`
- file picker and file loading side effects

It shows the shape to aim for, not a drop-in patch.

```rust
use reducer::Dispatcher;
use reducer::Reactor;
use reducer::Reducer;
use reducer::Store;
use std::cell::Cell;
use std::convert::Infallible;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum AppAction {
    Startup,
    SplashDismissed,
    ExternalFileDropped(PathBuf),
    Main(ChoreoMainAction),
    AudioTick(AudioRuntimeSnapshot),
    FileDialogResolved(Option<OpenChoreoRequested>),
    EffectsDrained,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppEffect {
    PickChoreoFile,
    OpenExternalFile(PathBuf),
    PollAudio,
    RequestRepaint,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AudioRuntimeSnapshot {
    pub position_seconds: f64,
    pub keep_polling: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppShellState {
    pub is_initialized: bool,
    pub show_splash_screen: bool,
}

impl Default for AppShellState {
    fn default() -> Self {
        Self {
            is_initialized: false,
            show_splash_screen: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AppState {
    pub shell: AppShellState,
    pub main: ChoreoMainState,
    pub effects: Vec<AppEffect>,
}

impl Reducer<AppAction> for AppState {
    fn reduce(&mut self, action: AppAction) {
        match action {
            AppAction::Startup => {
                if self.shell.is_initialized {
                    return;
                }

                self.shell.is_initialized = true;
                self.main.reduce(ChoreoMainAction::Initialize);
                self.effects.push(AppEffect::RequestRepaint);
            }
            AppAction::SplashDismissed => {
                self.shell.show_splash_screen = false;
                self.effects.push(AppEffect::RequestRepaint);
                self.effects.push(AppEffect::PollAudio);
            }
            AppAction::ExternalFileDropped(path) => {
                self.effects.push(AppEffect::OpenExternalFile(path));
            }
            AppAction::Main(action) => {
                let should_pick_choreo = matches!(
                    &action,
                    ChoreoMainAction::RequestOpenChoreo(request) if request.contents.trim().is_empty()
                );
                self.main.reduce(action);

                if self.main.audio_player_state.has_player {
                    self.effects.push(AppEffect::PollAudio);
                }

                if should_pick_choreo {
                    self.effects.push(AppEffect::PickChoreoFile);
                }
            }
            AppAction::AudioTick(sample) => {
                self.main.reduce(ChoreoMainAction::AudioPlayerAction(
                    AudioPlayerAction::PlayerPositionSampled {
                        position: sample.position_seconds,
                    },
                ));

                if sample.keep_polling {
                    self.effects.push(AppEffect::PollAudio);
                }
            }
            AppAction::FileDialogResolved(Some(request)) => {
                self.main
                    .reduce(ChoreoMainAction::RequestOpenChoreo(request));
            }
            AppAction::FileDialogResolved(None) => {}
            AppAction::EffectsDrained => {
                self.effects.clear();
            }
        }
    }
}

impl Reducer<ChoreoMainAction> for ChoreoMainState {
    fn reduce(&mut self, action: ChoreoMainAction) {
        crate::choreo_main::reducer::reduce(self, action);
    }
}

#[derive(Clone)]
pub struct AppReactor {
    needs_repaint: Rc<Cell<bool>>,
}

impl AppReactor {
    pub fn new(needs_repaint: Rc<Cell<bool>>) -> Self {
        Self { needs_repaint }
    }
}

impl Reactor<AppState> for AppReactor {
    type Error = Infallible;

    fn react(&mut self, state: &AppState) -> Result<(), Self::Error> {
        let should_repaint = !state.effects.is_empty()
            || state.shell.show_splash_screen
            || state.main.audio_player_state.is_playing;
        self.needs_repaint.set(should_repaint);
        Ok(())
    }
}

pub struct AppRuntime {
    audio_runtime: AudioPlayerRuntime,
    pick_choreo_file: Rc<dyn Fn() -> Option<OpenChoreoRequested>>,
}

impl AppRuntime {
    pub fn process(
        &mut self,
        store: &mut Store<AppState, AppReactor>,
        context: &egui::Context,
    ) {
        let effects = store.effects.clone();
        if effects.is_empty() {
            return;
        }

        store.dispatch(AppAction::EffectsDrained).expect("reactor should be infallible");

        for effect in effects {
            match effect {
                AppEffect::PickChoreoFile => {
                    let request = (self.pick_choreo_file)();
                    store
                        .dispatch(AppAction::FileDialogResolved(request))
                        .expect("reactor should be infallible");
                }
                AppEffect::OpenExternalFile(path) => {
                    if path.extension().and_then(|value| value.to_str()) == Some("choreo") {
                        if let Ok(contents) = std::fs::read_to_string(&path) {
                            store
                                .dispatch(AppAction::Main(ChoreoMainAction::RequestOpenChoreo(
                                    OpenChoreoRequested {
                                        file_path: Some(path.to_string_lossy().into_owned()),
                                        file_name: path
                                            .file_name()
                                            .map(|value| value.to_string_lossy().into_owned()),
                                        contents,
                                    },
                                )))
                                .expect("reactor should be infallible");
                        }
                    }
                }
                AppEffect::PollAudio => {
                    if let Some(sample) = self.audio_runtime.sample() {
                        store
                            .dispatch(AppAction::AudioTick(AudioRuntimeSnapshot {
                                position_seconds: sample.position,
                                keep_polling: sample.is_playing,
                            }))
                            .expect("reactor should be infallible");
                    }
                }
                AppEffect::RequestRepaint => {
                    context.request_repaint();
                }
            }
        }
    }
}

pub struct DesktopEguiApp {
    store: Store<AppState, AppReactor>,
    runtime: AppRuntime,
    needs_repaint: Rc<Cell<bool>>,
}

impl eframe::App for DesktopEguiApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.store.shell.is_initialized {
            self.store.dispatch(AppAction::Startup).expect("dispatch should succeed");
        }

        let dropped_files = context.input(|input| input.raw.dropped_files.clone());
        for dropped_file in dropped_files {
            if let Some(path) = dropped_file.path {
                self.store
                    .dispatch(AppAction::ExternalFileDropped(path))
                    .expect("dispatch should succeed");
            }
        }

        egui::CentralPanel::default().show(context, |ui| {
            if self.store.shell.show_splash_screen {
                if ui.button("Continue").clicked() {
                    self.store
                        .dispatch(AppAction::SplashDismissed)
                        .expect("dispatch should succeed");
                }
                return;
            }

            let state = &self.store.main;
            for action in crate::choreo_main::ui::draw(ui, state) {
                self.store
                    .dispatch(AppAction::Main(action))
                    .expect("dispatch should succeed");
            }
        });

        self.runtime.process(&mut self.store, context);

        if self.needs_repaint.replace(false) {
            context.request_repaint();
        }
    }
}
```

## Why This Is Better For This Repo

This structure fixes the specific problems visible in the current code.

### 1. `MainViewModel` stops being a second store

Right now `MainViewModel` owns state and exposes `dispatch()`, which duplicates what `Store` should already do.

With `reducer::Store`:

- the store owns state
- the reducer owns transitions
- the egui host reads state directly from the store

### 2. `MainPageBinding` stops mixing unrelated responsibilities

Right now `MainPageBinding` does all of these:

- dispatching actions
- invoking behaviors
- draining outgoing commands
- polling audio runtime
- routing files by extension

That is too much for one type.

Instead:

- the store handles state updates
- the runtime handles IO/effects
- the egui view only renders and emits actions

### 3. `outgoing_*` queues become explicit effects

The current `outgoing_open_choreo_requests`, `outgoing_audio_requests`, and `outgoing_open_svg_commands` fields are a hand-rolled effect queue inside application state.

That should become an explicit `AppEffect` layer.

This makes side effects:

- easier to name
- easier to test
- easier to centralize
- harder to accidentally trigger from random view-model methods

### 4. Existing feature reducers can be migrated incrementally

Do not rewrite the whole egui layer in one step.

The lowest-risk migration is:

1. Keep the existing feature `reduce(state, action)` functions.
2. Add `impl Reducer<Action> for State` wrappers around them.
3. Introduce a top-level `AppState` and `AppAction`.
4. Move `MainPageBinding` responsibilities into `AppRuntime`.
5. Replace `AppShellViewModel` with a `Store<AppState, AppReactor>` host.

## Practical Rules

When adding egui code in this repo:

- Prefer `Store<AppState, AppReactor>` over custom view-model `dispatch()` wrappers.
- Prefer `AppAction` enums over ad-hoc callback structs for state changes.
- Prefer `AppEffect` plus runtime execution over `outgoing_*` state queues.
- Keep `ui::draw()` functions read-only with respect to state; they should emit actions.
- Keep file dialogs, persistence, audio IO, timers, and OS integration outside reducers.
- Keep feature reducers pure and deterministic.

This is the direction new egui work should follow.
