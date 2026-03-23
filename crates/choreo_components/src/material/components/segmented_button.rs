use egui::CornerRadius;
use egui::Image;
use egui::Response;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::vec2;

use crate::material::components::icon::MaterialIconStyle;
use crate::material::components::icon::icon_with_style;
use crate::material::components::material_text::material_text;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;

#[derive(Clone)]
pub struct SegmentedItem {
    pub icon: Option<Image<'static>>,
    pub text: String,
}

impl SegmentedItem {
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            icon: None,
            text: text.into(),
        }
    }
}

pub struct SegmentedButtonResponse {
    pub changed_to: Option<usize>,
    pub item_responses: Vec<Response>,
}

pub struct SegmentedButton<'a> {
    pub items: &'a [SegmentedItem],
    pub current_index: usize,
}

impl<'a> SegmentedButton<'a> {
    pub fn show(self, ui: &mut Ui) -> SegmentedButtonResponse {
        let metrics = material_style_metrics();
        let palette = material_palette_for_visuals(ui.visuals());
        let total_width = ui.available_width().max(metrics.sizes.size_40 * 2.0);
        let item_width = if self.items.is_empty() {
            total_width
        } else {
            (total_width / self.items.len() as f32).max(metrics.sizes.size_40 * 2.0)
        };
        let mut changed_to = None;
        let mut item_responses = Vec::with_capacity(self.items.len());

        egui::Frame::new()
            .stroke(Stroke::new(1.0, palette.outline))
            .corner_radius(CornerRadius::same(
                (metrics.sizes.size_40 * 0.5).round() as u8
            ))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                ui.horizontal(|ui| {
                    for (index, item) in self.items.iter().enumerate() {
                        let response = show_segment_item(
                            ui,
                            item,
                            index == self.current_index,
                            item_width,
                            index + 1 == self.items.len(),
                        );
                        if response.clicked() && index != self.current_index {
                            changed_to = Some(index);
                        }
                        item_responses.push(response);
                    }
                });
            });

        SegmentedButtonResponse {
            changed_to,
            item_responses,
        }
    }
}

fn show_segment_item(
    ui: &mut Ui,
    item: &SegmentedItem,
    selected: bool,
    width: f32,
    last: bool,
) -> Response {
    let metrics = material_style_metrics();
    let palette = material_palette_for_visuals(ui.visuals());
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, metrics.sizes.size_40), Sense::click());

    if selected {
        ui.painter()
            .rect_filled(rect, 0.0, palette.secondary_container);
    }
    if !last {
        ui.painter().line_segment(
            [rect.right_top(), rect.right_bottom()],
            Stroke::new(1.0, palette.outline),
        );
    }

    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = metrics.spacings.spacing_8;
            let tint = palette.on_surface;
            if selected {
                let _ = ui.add(icon_with_style(
                    Image::new(egui::include_image!("../../../assets/icons/Check.svg")),
                    MaterialIconStyle {
                        size: vec2(
                            metrics.icon_sizes.icon_size_18,
                            metrics.icon_sizes.icon_size_18,
                        ),
                        tint,
                    },
                ));
            } else if let Some(icon) = item.icon.clone() {
                let _ = ui.add(icon_with_style(
                    icon,
                    MaterialIconStyle {
                        size: vec2(
                            metrics.icon_sizes.icon_size_18,
                            metrics.icon_sizes.icon_size_18,
                        ),
                        tint,
                    },
                ));
            }
            let _ = material_text(ui, item.text.as_str())
                .text_style(MATERIAL_TYPOGRAPHY.label_large)
                .color(tint)
                .show(ui);
        });
    });

    response
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::SegmentedButton;
    use super::SegmentedItem;

    #[test]
    fn segmented_button_renders_without_panicking() {
        let context = Context::default();
        let mut item_count = 0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let items = vec![SegmentedItem::new("One"), SegmentedItem::new("Two")];
                item_count = SegmentedButton {
                    items: &items,
                    current_index: 0,
                }
                .show(ui)
                .item_responses
                .len();
            });
        });
        assert_eq!(item_count, 2);
    }
}
