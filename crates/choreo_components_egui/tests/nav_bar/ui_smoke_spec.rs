use crate::nav_bar::nav_bar_component::actions::NavBarAction;
use crate::nav_bar::nav_bar_component::state::InteractionMode;
use crate::nav_bar::nav_bar_component::state::NavBarState;
use crate::nav_bar::nav_bar_component::ui::mode_label;
use crate::nav_bar::nav_bar_component::ui::nav_button;
use crate::nav_bar::nav_bar_component::ui::settings_button;

#[test]
fn nav_bar_ui_draw_executes_without_panicking() {
    let state = NavBarState::default();
    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = crate::nav_bar::nav_bar_component::ui::draw(ui, &state);
        });
    });
}

#[test]
fn nav_button_action_depends_on_nav_state() {
    let closed = NavBarState::default();
    assert_eq!(
        nav_button(&closed),
        ("open_navigation", NavBarAction::ToggleNavigation)
    );

    let open = NavBarState {
        is_nav_open: true,
        ..NavBarState::default()
    };
    assert_eq!(
        nav_button(&open),
        ("close_navigation", NavBarAction::CloseNavigation)
    );
}

#[test]
fn settings_button_action_depends_on_settings_state() {
    let closed = NavBarState::default();
    assert_eq!(
        settings_button(&closed),
        ("open_settings", NavBarAction::ToggleChoreographySettings)
    );

    let open = NavBarState {
        is_choreography_settings_open: true,
        ..NavBarState::default()
    };
    assert_eq!(
        settings_button(&open),
        ("close_settings", NavBarAction::CloseChoreographySettings)
    );
}

#[test]
fn mode_labels_map_to_translation_keys() {
    assert_eq!(mode_label(InteractionMode::View), "ModeView");
    assert_eq!(mode_label(InteractionMode::Move), "ModeMove");
    assert_eq!(
        mode_label(InteractionMode::RotateAroundCenter),
        "ModeRotateAroundCenter"
    );
    assert_eq!(
        mode_label(InteractionMode::RotateAroundDancer),
        "ModeRotateAroundDancer"
    );
    assert_eq!(mode_label(InteractionMode::Scale), "ModeScale");
    assert_eq!(mode_label(InteractionMode::LineOfSight), "ModeLineOfSight");
}
