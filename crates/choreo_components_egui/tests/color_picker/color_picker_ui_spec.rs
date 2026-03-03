#[path = "../../src/color_picker/actions.rs"]
mod actions;
#[path = "../../src/color_picker/state.rs"]
mod state;
#[path = "../../src/color_picker/ui.rs"]
mod ui;

use egui::Color32;

#[test]
fn draw_uses_selection_thumb_size_and_stroke_settings() {
    let context = egui::Context::default();
    let mut state = state::ColorPickerState::new();
    state.selection_thumb_size = 24.0;
    state.selection_stroke_thickness = 3.0;
    state.selection_stroke_color = Color32::from_rgb(4, 5, 6);

    let output = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = ui::draw(ui, &state);
        });
    });

    let mut saw_thumb_fill = false;
    let mut saw_thumb_stroke = false;
    for clipped in output.shapes {
        if let egui::Shape::Circle(circle) = clipped.shape {
            if (circle.radius - (state.selection_thumb_size * 0.5)).abs() < 0.01
                && circle.fill == state.selected_color
            {
                saw_thumb_fill = true;
            }
            if (circle.radius - (state.selection_thumb_size * 0.5)).abs() < 0.01
                && (circle.stroke.width - state.selection_stroke_thickness).abs() < 0.01
                && circle.stroke.color == state.selection_stroke_color
            {
                saw_thumb_stroke = true;
            }
        }
    }

    assert!(saw_thumb_fill);
    assert!(saw_thumb_stroke);
}
