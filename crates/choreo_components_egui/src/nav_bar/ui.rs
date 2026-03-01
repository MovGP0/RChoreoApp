use egui::Ui;

use super::actions::NavBarAction;
use super::state::InteractionMode;
use super::state::NavBarState;
use super::state::all_modes;

#[must_use]
pub fn nav_button(state: &NavBarState) -> (&'static str, NavBarAction) {
    if state.is_nav_open {
        ("Close Nav", NavBarAction::CloseNavigation)
    } else {
        ("Open Nav", NavBarAction::ToggleNavigation)
    }
}

#[must_use]
pub fn settings_button(state: &NavBarState) -> (&'static str, NavBarAction) {
    if state.is_choreography_settings_open {
        ("Close Settings", NavBarAction::CloseChoreographySettings)
    } else {
        ("Open Settings", NavBarAction::ToggleChoreographySettings)
    }
}

#[must_use]
pub fn mode_label(mode: InteractionMode) -> &'static str {
    match mode {
        InteractionMode::View => "View",
        InteractionMode::Move => "Move",
        InteractionMode::RotateAroundCenter => "Rotate Center",
        InteractionMode::RotateAroundDancer => "Rotate Dancer",
        InteractionMode::Scale => "Scale",
        InteractionMode::LineOfSight => "Line of Sight",
    }
}

pub fn draw(ui: &mut Ui, state: &NavBarState) -> Vec<NavBarAction> {
    let mut actions: Vec<NavBarAction> = Vec::new();
    ui.horizontal(|ui| {
        ui.heading("Navigation");
        let (nav_label, nav_action) = nav_button(state);
        if ui.button(nav_label).clicked() {
            actions.push(nav_action);
        }
        let (settings_label, settings_action) = settings_button(state);
        if ui.button(settings_label).clicked() {
            actions.push(settings_action);
        }
    });

    ui.horizontal_wrapped(|ui| {
        if ui.button("Open Audio").clicked() {
            actions.push(NavBarAction::OpenAudio);
        }
        if ui.button("Open Image").clicked() {
            actions.push(NavBarAction::OpenImage);
        }
        if ui.button("Reset Viewport").clicked() {
            actions.push(NavBarAction::ResetFloorViewport);
        }
    });

    let mut selected_mode = state.selected_mode;
    ui.add_enabled_ui(state.is_mode_selection_enabled, |ui| {
        egui::ComboBox::from_label("Mode")
            .selected_text(mode_label(selected_mode))
            .show_ui(ui, |ui| {
                for mode in all_modes() {
                    ui.selectable_value(&mut selected_mode, *mode, mode_label(*mode));
                }
            });
    });

    if selected_mode != state.selected_mode {
        actions.push(NavBarAction::SetSelectedMode {
            mode: selected_mode,
        });
    }

    actions
}
