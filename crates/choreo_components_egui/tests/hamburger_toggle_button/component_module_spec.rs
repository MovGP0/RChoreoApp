use choreo_components_egui::hamburger_toggle_button;
use choreo_components_egui::hamburger_toggle_button::HamburgerToggleButton;

#[test]
fn hamburger_toggle_button_component_module_exports_reusable_widget_surface() {
    let _draw = hamburger_toggle_button::draw
        as fn(&mut egui::Ui, bool, bool, &str, Option<egui::Vec2>) -> egui::Response;
    let _widget = HamburgerToggleButton::new(false)
        .enabled(true)
        .toggle_on_click(true)
        .tooltip("Menu")
        .size(egui::vec2(48.0, 48.0));
}
