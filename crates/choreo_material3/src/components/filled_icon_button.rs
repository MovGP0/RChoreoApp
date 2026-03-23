use egui::CornerRadius;
use egui::Image;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;

use crate::components::BaseButton;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;

pub struct FilledIconButton {
    pub icon: Image<'static>,
    pub checked_icon: Option<Image<'static>>,
    pub checkable: bool,
    pub checked: bool,
    pub tooltip: String,
    pub enabled: bool,
}

pub struct FilledIconButtonResponse {
    pub response: Response,
    pub checked: bool,
}

impl FilledIconButton {
    #[must_use]
    pub fn new(icon: Image<'static>) -> Self {
        Self {
            icon,
            checked_icon: None,
            checkable: false,
            checked: false,
            tooltip: String::new(),
            enabled: true,
        }
    }

    pub fn show(mut self, ui: &mut Ui) -> FilledIconButtonResponse {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let size = metrics.sizes.size_40;
        let border_radius = size * 0.5;
        let icon = if self.checked {
            self.checked_icon
                .clone()
                .unwrap_or_else(|| self.icon.clone())
        } else {
            self.icon.clone()
        };
        let icon_color = if self.enabled {
            if self.checkable {
                if self.checked {
                    palette.on_primary
                } else {
                    palette.primary
                }
            } else {
                palette.on_primary
            }
        } else {
            palette.on_surface_variant
        };
        let background = if !self.enabled {
            None
        } else if self.checkable && !self.checked {
            Some(palette.surface_container_highest)
        } else {
            Some(palette.primary)
        };
        let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), Sense::hover());
        if let Some(background) = background {
            ui.painter().rect_filled(
                rect,
                CornerRadius::same(border_radius.round() as u8),
                background,
            );
        }
        let response = ui
            .scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                BaseButton {
                    icon: Some(icon),
                    icon_color: Some(icon_color),
                    text: "".into(),
                    tooltip: self.tooltip.clone().into(),
                    enabled: self.enabled,
                    border_radius: Some(border_radius),
                    button_horizontal_padding: 0.0,
                    button_vertical_padding: 0.0,
                    min_layout_width: size,
                    min_layout_height: size,
                    icon_size: metrics.icon_sizes.icon_size_24,
                    display_background: false,
                    ..BaseButton::new()
                }
                .show(ui, |_| {})
                .response
            })
            .inner;
        if self.checkable && response.clicked() && self.enabled {
            self.checked = !self.checked;
        }
        FilledIconButtonResponse {
            response,
            checked: self.checked,
        }
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;
    use egui::Image;

    use super::FilledIconButton;

    #[test]
    fn filled_icon_button_renders_without_panicking() {
        let context = Context::default();
        let mut min_height = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = FilledIconButton::new(Image::new(egui::include_image!(
                    "../../assets/icons/Home.svg"
                )))
                .show(ui);
                min_height = response.response.rect.height();
            });
        });
        assert!(min_height >= 40.0);
    }
}
