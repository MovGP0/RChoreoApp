use egui::Color32;
use egui::Frame;
use egui::Image;
use egui::Response;
use egui::Ui;
use egui::vec2;

use crate::components::centered_icon_rect;
use crate::components::paint_icon;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;

#[derive(Clone)]
pub struct IconButtonItem {
    pub icon: Image<'static>,
    pub tooltip: String,
    pub enabled: bool,
}

impl IconButtonItem {
    #[must_use]
    pub fn new(icon: Image<'static>) -> Self {
        Self {
            icon,
            tooltip: String::new(),
            enabled: true,
        }
    }
}

pub struct BottomAppBar<'a> {
    pub icon_buttons: &'a [IconButtonItem],
    pub fab_icon: Option<Image<'static>>,
}

pub struct BottomAppBarResponse {
    pub icon_button_responses: Vec<Response>,
    pub fab_response: Option<Response>,
}

pub fn bottom_app_bar(ui: &mut Ui, content: BottomAppBar<'_>) -> BottomAppBarResponse {
    let palette = material_palette_for_visuals(ui.visuals());
    let metrics = material_style_metrics();
    let frame = Frame::new()
        .fill(palette.surface_container)
        .inner_margin(egui::Margin {
            left: metrics.paddings.padding_4.round() as i8,
            right: metrics.paddings.padding_16.round() as i8,
            top: 0,
            bottom: 0,
        });

    let mut icon_button_responses = Vec::with_capacity(content.icon_buttons.len());
    let mut fab_response = None;

    frame.show(ui, |ui| {
        ui.set_min_height(metrics.sizes.size_80);
        ui.horizontal(|ui| {
            for item in content.icon_buttons {
                let response = ui.add_enabled(
                    item.enabled,
                    egui::Button::new("")
                        .min_size(vec2(metrics.sizes.size_40, metrics.sizes.size_40)),
                );
                let tint = ui.style().interact(&response).fg_stroke.color;
                paint_icon(
                    ui,
                    &item.icon,
                    centered_icon_rect(
                        response.rect,
                        vec2(
                            metrics.icon_sizes.icon_size_24,
                            metrics.icon_sizes.icon_size_24,
                        ),
                    ),
                    tint,
                );
                icon_button_responses.push(response.on_hover_text(item.tooltip.clone()));
            }
            ui.add_space(ui.available_width().max(0.0));
            if let Some(fab_icon) = content.fab_icon {
                let button = egui::Button::new("")
                    .fill(palette.primary)
                    .corner_radius(metrics.corner_radii.border_radius_16)
                    .min_size(vec2(metrics.sizes.size_56, metrics.sizes.size_56));
                let response = ui.add(button);
                paint_icon(
                    ui,
                    &fab_icon,
                    centered_icon_rect(response.rect, vec2(24.0, 24.0)),
                    Color32::WHITE,
                );
                fab_response = Some(response);
            }
        });
    });

    BottomAppBarResponse {
        icon_button_responses,
        fab_response,
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;
    use egui::Image;

    use super::BottomAppBar;
    use super::bottom_app_bar;

    #[test]
    fn bottom_app_bar_renders_without_panicking() {
        let context = Context::default();
        let mut icon_count = 0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let content = BottomAppBar {
                    icon_buttons: &[],
                    fab_icon: None::<Image<'static>>,
                };
                let response = bottom_app_bar(ui, content);
                icon_count = response.icon_button_responses.len();
            });
        });
        assert_eq!(icon_count, 0);
    }
}
