use egui::CornerRadius;
use egui::Rect;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use egui::Vec2;
use egui::pos2;
use egui::vec2;

use crate::material::components::badge;
use crate::material::components::icon::MaterialIconStyle;
use crate::material::components::icon::icon_with_style;
use crate::material::components::material_text;
use crate::material::items::navigation_item::NavigationItem;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;

pub struct NavigationBarItemTemplate<'a> {
    pub item: &'a NavigationItem,
    pub index: usize,
    pub selected: bool,
    pub navitem_padding_top: f32,
    pub navitem_padding_bottom: f32,
}

impl<'a> NavigationBarItemTemplate<'a> {
    #[must_use]
    pub fn new(item: &'a NavigationItem, index: usize, selected: bool) -> Self {
        let metrics = material_style_metrics();
        Self {
            item,
            index,
            selected,
            navitem_padding_top: metrics.paddings.padding_12,
            navitem_padding_bottom: metrics.paddings.padding_16,
        }
    }

    pub fn show(self, ui: &mut Ui, size: Vec2) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let (rect, response) = ui.allocate_exact_size(size, Sense::click());
        let pill_width = metrics
            .sizes
            .size_64
            .max(size.x - 2.0 * metrics.paddings.padding_20);
        let pill_height = metrics.sizes.size_32;
        let pill_rect = Rect::from_center_size(
            pos2(
                rect.center().x,
                rect.top() + self.navitem_padding_top + pill_height * 0.5,
            ),
            vec2(pill_width, pill_height),
        );
        if self.selected {
            ui.painter().rect_filled(
                pill_rect,
                CornerRadius::same((pill_height * 0.5).round() as u8),
                palette.secondary_container,
            );
        }

        let content_color = if self.selected {
            palette.on_secondary_container
        } else {
            palette.on_surface
        };
        let icon_rect = Rect::from_center_size(
            pill_rect.center(),
            vec2(
                metrics.icon_sizes.icon_size_24,
                metrics.icon_sizes.icon_size_24,
            ),
        );
        if let Some(icon) = selected_icon(self.item, self.selected) {
            let _ = ui.put(
                icon_rect,
                icon_with_style(
                    icon,
                    MaterialIconStyle {
                        size: icon_rect.size(),
                        tint: content_color,
                    },
                ),
            );
        } else {
            let dot_radius = 6.0;
            ui.painter()
                .circle_filled(icon_rect.center(), dot_radius, content_color);
        }

        let label_rect = Rect::from_min_max(
            pos2(rect.left(), pill_rect.bottom() + metrics.spacings.spacing_4),
            pos2(
                rect.right(),
                rect.bottom() - self.navitem_padding_bottom + metrics.paddings.padding_8,
            ),
        );
        ui.scope_builder(UiBuilder::new().max_rect(label_rect), |ui| {
            ui.vertical_centered(|ui| {
                let _ = material_text(ui, self.item.text.as_str())
                    .text_style(if self.selected {
                        MATERIAL_TYPOGRAPHY.label_medium_prominent
                    } else {
                        MATERIAL_TYPOGRAPHY.label_medium
                    })
                    .color(content_color)
                    .show(ui);
            });
        });

        if self.item.show_badge || !self.item.badge.is_empty() {
            let badge_size = vec2(metrics.sizes.size_16, metrics.sizes.size_16);
            let badge_rect = Rect::from_min_size(
                pos2(
                    pill_rect.center().x + metrics.paddings.padding_8,
                    rect.top(),
                ),
                badge_size,
            );
            let _ = ui.scope_builder(UiBuilder::new().max_rect(badge_rect), |ui| {
                badge(ui, self.item.badge.as_str())
            });
        }

        response
    }
}

pub struct NavigationBarResponse {
    pub changed_to: Option<usize>,
    pub item_responses: Vec<Response>,
}

pub struct NavigationBar<'a> {
    pub items: &'a [NavigationItem],
    pub current_index: &'a mut usize,
}

impl<'a> NavigationBar<'a> {
    #[must_use]
    pub fn new(items: &'a [NavigationItem], current_index: &'a mut usize) -> Self {
        Self {
            items,
            current_index,
        }
    }

    pub fn show(self, ui: &mut Ui) -> NavigationBarResponse {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let item_count = self.items.len().max(1);
        let desired_size = vec2(
            ui.available_width().max(metrics.sizes.size_80),
            metrics.sizes.size_80,
        );
        let (rect, _) = ui.allocate_exact_size(desired_size, Sense::hover());
        ui.painter()
            .rect_filled(rect, CornerRadius::ZERO, palette.surface_container);

        let item_width = (rect.width() - 2.0 * metrics.paddings.padding_8) / item_count as f32;
        let item_size = vec2(item_width.max(metrics.sizes.size_56), rect.height());
        let mut changed_to = None;
        let mut item_responses = Vec::with_capacity(self.items.len());

        ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = metrics.spacings.spacing_8;
                ui.add_space(metrics.paddings.padding_8);
                for (index, item) in self.items.iter().enumerate() {
                    let response =
                        NavigationBarItemTemplate::new(item, index, *self.current_index == index)
                            .show(ui, item_size);
                    if response.clicked() && *self.current_index != index {
                        *self.current_index = index;
                        changed_to = Some(index);
                    }
                    item_responses.push(response);
                }
            });
        });

        NavigationBarResponse {
            changed_to,
            item_responses,
        }
    }
}

fn selected_icon(item: &NavigationItem, selected: bool) -> Option<egui::Image<'static>> {
    if selected {
        item.selected_icon.clone().or_else(|| item.icon.clone())
    } else {
        item.icon.clone()
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;
    use egui::Image;

    use super::NavigationBar;
    use crate::material::items::navigation_item::NavigationItem;

    #[test]
    fn navigation_bar_renders_without_panicking() {
        let context = Context::default();
        let mut item_count = 0;
        let mut changed_to = None;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut items = vec![NavigationItem::new("Home"), NavigationItem::new("Settings")];
                items[0].icon = Some(Image::new(egui::include_image!(
                    "../../../assets/icons/Home.svg"
                )));
                items[1].icon = Some(Image::new(egui::include_image!(
                    "../../../assets/icons/Pen.svg"
                )));
                let mut current_index = 0;
                let response = NavigationBar::new(&items, &mut current_index).show(ui);
                item_count = response.item_responses.len();
                changed_to = response.changed_to;
            });
        });
        assert_eq!(item_count, 2);
        assert_eq!(changed_to, None);
    }
}
