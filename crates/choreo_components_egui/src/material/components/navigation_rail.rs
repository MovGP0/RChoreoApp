use egui::Align;
use egui::Image;
use egui::Layout;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::vec2;

use crate::material::components::FabStyle;
use crate::material::components::FloatingActionButton;
use crate::material::components::badge;
use crate::material::components::icon::icon_with_style;
use crate::material::components::icon_button::MaterialIconButton;
use crate::material::components::material_text;
use crate::material::items::navigation_item::NavigationItem;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;

pub struct NavigationRailResponse {
    pub changed_to: Option<usize>,
    pub menu_clicked: bool,
    pub fab_clicked: bool,
    pub item_responses: Vec<Response>,
}

pub struct NavigationRail<'a> {
    pub items: &'a [NavigationItem],
    pub current_index: usize,
    pub has_menu: bool,
    pub fab_icon: Option<Image<'static>>,
    pub alignment: Align,
}

impl<'a> NavigationRail<'a> {
    pub fn show(self, ui: &mut Ui) -> NavigationRailResponse {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let rail_width = metrics
            .sizes
            .size_80
            .max(ui.available_width().min(metrics.sizes.size_80));
        let mut changed_to = None;
        let mut menu_clicked = false;
        let mut fab_clicked = false;
        let mut item_responses = Vec::with_capacity(self.items.len());

        let frame = egui::Frame::new().fill(palette.surface);
        frame.show(ui, |ui| {
            ui.set_min_width(rail_width);
            ui.spacing_mut().item_spacing.y = metrics.spacings.spacing_4;
            ui.add_space(metrics.paddings.padding_44);

            if self.has_menu {
                let response = MaterialIconButton::new(Image::new(egui::include_image!(
                    "../../../assets/icons/Menu.svg"
                )))
                .show(ui);
                menu_clicked = response.response.clicked();
            }

            if let Some(icon) = self.fab_icon {
                ui.horizontal_centered(|ui| {
                    let response = FloatingActionButton {
                        icon: Some(icon),
                        style: FabStyle::Standard,
                        ..FloatingActionButton::new()
                    }
                    .show(ui);
                    fab_clicked = response.clicked();
                });
            }

            if self.items.is_empty() {
                ui.add_space(ui.available_height().max(0.0));
            }

            ui.with_layout(Layout::top_down(self.alignment), |ui| {
                for (index, item) in self.items.iter().enumerate() {
                    let response = show_navigation_rail_item(ui, item, index == self.current_index);
                    if response.clicked() && index != self.current_index {
                        changed_to = Some(index);
                    }
                    item_responses.push(response);
                }
            });

            ui.add_space(metrics.paddings.padding_56);
        });

        NavigationRailResponse {
            changed_to,
            menu_clicked,
            fab_clicked,
            item_responses,
        }
    }
}

fn show_navigation_rail_item(ui: &mut Ui, item: &NavigationItem, selected: bool) -> Response {
    let palette = material_palette_for_visuals(ui.visuals());
    let metrics = material_style_metrics();
    let width = metrics.sizes.size_80;
    let height = metrics.sizes.size_56 + metrics.paddings.padding_12 + metrics.paddings.padding_4;
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());
    let color = if selected {
        palette.on_secondary_container
    } else {
        palette.on_surface
    };

    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
            ui.add_space(0.0);
            let pill_rect = egui::Rect::from_center_size(
                egui::pos2(
                    rect.center().x,
                    rect.top() + metrics.paddings.padding_12 + metrics.sizes.size_16,
                ),
                vec2(metrics.sizes.size_56, metrics.sizes.size_32),
            );
            if selected {
                ui.painter().rect_filled(
                    pill_rect,
                    egui::CornerRadius::same((metrics.sizes.size_32 * 0.5).round() as u8),
                    palette.secondary_container,
                );
            }

            let icon_image = if selected {
                item.selected_icon.clone().or_else(|| item.icon.clone())
            } else {
                item.icon.clone()
            };
            if let Some(image) = icon_image {
                let _ = ui.put(
                    pill_rect,
                    icon_with_style(
                        image,
                        crate::material::components::icon::MaterialIconStyle {
                            size: vec2(
                                metrics.icon_sizes.icon_size_24,
                                metrics.icon_sizes.icon_size_24,
                            ),
                            tint: color,
                        },
                    ),
                );
            }

            if item.show_badge || !item.badge.is_empty() {
                let badge_pos = egui::pos2(
                    pill_rect.center().x + metrics.paddings.padding_16,
                    pill_rect.top(),
                );
                let badge_size = ui.scope_builder(
                    egui::UiBuilder::new()
                        .max_rect(egui::Rect::from_min_size(badge_pos, vec2(24.0, 16.0))),
                    |ui| badge(ui, item.badge.as_str()),
                );
                let _ = badge_size;
            }

            ui.add_space(metrics.spacings.spacing_4);
            let _ = material_text(ui, item.text.as_str())
                .text_style(MATERIAL_TYPOGRAPHY.label_medium)
                .color(color)
                .show(ui);
        });
    });

    response
}

#[cfg(test)]
mod tests {
    use egui::Align;
    use egui::Context;
    use egui::Image;

    use super::NavigationRail;
    use crate::material::items::navigation_item::NavigationItem;

    #[test]
    fn navigation_rail_renders_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let items = vec![NavigationItem::new("Home"), NavigationItem::new("Settings")];
                let response = NavigationRail {
                    items: &items,
                    current_index: 0,
                    has_menu: true,
                    fab_icon: Some(Image::new(egui::include_image!(
                        "../../../assets/icons/Home.svg"
                    ))),
                    alignment: Align::Center,
                }
                .show(ui);
                width = response
                    .item_responses
                    .first()
                    .map_or(0.0, |response| response.rect.width());
            });
        });
        assert!(width >= 80.0);
    }
}
