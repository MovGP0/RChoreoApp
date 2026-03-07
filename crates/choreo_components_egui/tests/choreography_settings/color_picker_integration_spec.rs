use egui::Color32;

use super::color;
use super::create_state;
use super::ui;

#[test]
fn floor_color_picker_state_tracks_floor_color() {
    let mut state = create_state();
    state.floor_color = color(255, 12, 34, 56);

    let picker_state = ui::floor_color_picker_state(&state);

    assert_eq!(
        picker_state.selected_color,
        Color32::from_rgba_premultiplied(12, 34, 56, 255)
    );
}

#[test]
fn selected_scene_color_picker_state_tracks_scene_color() {
    let mut state = create_state();
    state.scene_color = color(255, 78, 90, 123);

    let picker_state = ui::selected_scene_color_picker_state(&state);

    assert_eq!(
        picker_state.selected_color,
        Color32::from_rgba_premultiplied(78, 90, 123, 255)
    );
}
