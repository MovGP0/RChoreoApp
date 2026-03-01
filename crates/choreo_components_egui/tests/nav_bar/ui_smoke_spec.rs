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
    assert_eq!(nav_button(&closed), ("Open Nav", NavBarAction::ToggleNavigation));

    let open = NavBarState {
        is_nav_open: true,
        ..NavBarState::default()
    };
    assert_eq!(nav_button(&open), ("Close Nav", NavBarAction::CloseNavigation));
}

#[test]
fn settings_button_action_depends_on_settings_state() {
    let closed = NavBarState::default();
    assert_eq!(
        settings_button(&closed),
        ("Open Settings", NavBarAction::ToggleChoreographySettings)
    );

    let open = NavBarState {
        is_choreography_settings_open: true,
        ..NavBarState::default()
    };
    assert_eq!(
        settings_button(&open),
        ("Close Settings", NavBarAction::CloseChoreographySettings)
    );
}

#[test]
fn mode_labels_match_expected_navigation_terms() {
    assert_eq!(mode_label(InteractionMode::View), "View");
    assert_eq!(mode_label(InteractionMode::Move), "Move");
    assert_eq!(mode_label(InteractionMode::RotateAroundCenter), "Rotate Center");
    assert_eq!(mode_label(InteractionMode::RotateAroundDancer), "Rotate Dancer");
    assert_eq!(mode_label(InteractionMode::Scale), "Scale");
    assert_eq!(mode_label(InteractionMode::LineOfSight), "Line of Sight");
}
