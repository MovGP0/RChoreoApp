use egui::Color32;

use super::dancer;
use super::role;
use super::state::DancersState;
use super::ui;

#[test]
fn selected_dancer_color_picker_state_tracks_selected_dancer_color() {
    let mut state = DancersState::default();
    let mut selected_dancer = dancer(1, role("Leader"), "Alex", "A", None);
    selected_dancer.color = super::color(12, 34, 56);
    state.selected_dancer = Some(selected_dancer);

    let picker_state = ui::selected_dancer_color_picker_state(&state);

    assert_eq!(
        picker_state.selected_color,
        Color32::from_rgba_unmultiplied(12, 34, 56, 255)
    );
}

#[test]
fn selected_dancer_color_picker_state_is_transparent_without_selection() {
    let state = DancersState::default();

    let picker_state = ui::selected_dancer_color_picker_state(&state);

    assert_eq!(picker_state.selected_color, Color32::TRANSPARENT);
}
