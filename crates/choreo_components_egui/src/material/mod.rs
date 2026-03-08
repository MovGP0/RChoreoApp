pub mod components;
pub mod icons;
pub mod items;
pub mod styling;

/// Registers image loaders so embedded SVG assets can render in egui views.
pub fn install_image_loaders(context: &egui::Context) {
    egui_extras::install_image_loaders(context);
}
