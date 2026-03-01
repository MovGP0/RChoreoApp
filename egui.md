# What Works Well with egui

## 1) Model + View Function + Update/Reducer (Elm/Redux Style)

This is usually the most natural pattern.

- Model: application state (domain + UI state).
- View: functions that build egui UI (`fn ui(&mut self, ui: &mut egui::Ui, model: &Model) -> Vec<Action>`) or mutate through commands.
- Update/Reducer: a single place that applies actions/events to the model.

Why this fits egui: egui already emits per-frame events, so you can convert them into domain actions and apply them.

```rust
enum Action {
    ZoomBy(f32),
    PanBy(egui::Vec2),
    OpenFile,
    SetTool(Tool),
    // ...
}

struct Model {
    camera: CameraState,
    tool: Tool,
    // domain data...
}

fn view(ui: &mut egui::Ui, model: &Model) -> Vec<Action> {
    let mut actions = Vec::new();

    if ui.button("Open").clicked() {
        actions.push(Action::OpenFile);
    }

    // ...
    actions
}

fn update(model: &mut Model, action: Action) {
    match action {
        Action::ZoomBy(factor) => model.camera.zoom *= factor,
        Action::PanBy(delta) => model.camera.pan += delta,
        Action::OpenFile => {
            /* ... */
        }
        Action::SetTool(tool) => model.tool = tool,
    }
}
```

In `App::update(ctx, frame)`:

- Call view(s) to collect actions.
- Apply actions via `update()`.
- Render scene using the resulting model state.

Benefits:

- Deterministic state transitions.
- Easy testing of the reducer (pure logic).
- Clean separation between UI and domain.

## 2) MVU + Commands/Effects (for Async/IO)

Immediate-mode UIs benefit from explicit effects.

- Reducer returns an `Effect` (spawn task, file dialog, network).
- Completion posts a message back (action).

This avoids doing IO inside UI code and keeps things testable.

## 3) Controller Objects for Complex Interaction (Gestures, Tools)

For viewport toolchains (pan/zoom/rotate, selection, drag constraints), use dedicated controllers.

- `ViewportController` owns gesture state (active pointers, last centroid, etc.).
- It outputs actions (`PanBy`, `ZoomTo`, `RotateBy`) or directly updates camera state.

This maps cleanly to multi-touch input and your `wgpu` scene.
