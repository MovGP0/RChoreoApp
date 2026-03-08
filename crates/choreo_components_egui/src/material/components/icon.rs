use egui::Color32;
use egui::Image;
use egui::Response;
use egui::Ui;
use egui::Vec2;
use egui::vec2;

use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialIconStyle {
    pub size: Vec2,
    pub tint: Color32,
}

impl MaterialIconStyle {
    #[must_use]
    pub fn for_ui(ui: &Ui) -> Self {
        let metrics = material_style_metrics();
        let palette = material_palette_for_visuals(ui.visuals());
        Self {
            size: vec2(metrics.icon_sizes.icon_size_18, metrics.icon_sizes.icon_size_18),
            tint: palette.on_background,
        }
    }
}

pub type MaterialIcon<'a> = Image<'a>;

pub fn icon<'a>(ui: &Ui, image: Image<'a>) -> MaterialIcon<'a> {
    let style = MaterialIconStyle::for_ui(ui);
    image.fit_to_exact_size(style.size).tint(style.tint)
}

pub fn icon_with_style<'a>(image: Image<'a>, style: MaterialIconStyle) -> MaterialIcon<'a> {
    image.fit_to_exact_size(style.size).tint(style.tint)
}

pub fn show_icon<'a>(ui: &mut Ui, image: Image<'a>) -> Response {
    ui.add(icon(ui, image))
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::MaterialIconStyle;

    #[test]
    fn default_style_matches_slint_defaults() {
        let context = Context::default();
        let mut style = None;
        let mut expected_tint = None;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                style = Some(MaterialIconStyle::for_ui(ui));
                expected_tint = Some(
                    crate::material::styling::material_palette::material_palette_for_visuals(
                        ui.visuals(),
                    )
                    .on_background,
                );
            });
        });
        let style = style.expect("icon style");
        assert_eq!(style.size, egui::vec2(18.0, 18.0));
        assert_eq!(style.tint, expected_tint.expect("expected tint"));
    }
}
