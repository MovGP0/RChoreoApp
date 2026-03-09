use egui::CornerRadius;
use egui::Id;
use egui::PopupCloseBehavior;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::vec2;

use crate::material::components::icon::MaterialIconStyle;
use crate::material::components::icon::icon_with_style;
use crate::material::components::material_text::material_text;
use crate::material::items::menu_item::MenuItem;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;

pub struct MenuInnerResponse {
    pub activated: Option<usize>,
    pub item_responses: Vec<Response>,
}

pub struct PopupMenuResponse {
    pub activated: Option<usize>,
    pub popup_open: bool,
    pub item_responses: Vec<Response>,
}

pub struct MenuInner<'a> {
    pub items: &'a [MenuItem],
    pub current_index: Option<usize>,
    pub width: Option<f32>,
}

pub struct PopupMenu<'a> {
    pub id: Id,
    pub items: &'a [MenuItem],
    pub current_index: Option<usize>,
    pub width: f32,
}

impl<'a> MenuInner<'a> {
    pub fn show(self, ui: &mut Ui) -> MenuInnerResponse {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let width = self.width.unwrap_or(metrics.sizes.size_200);
        let mut activated = None;
        let mut item_responses = Vec::with_capacity(self.items.len());

        egui::Frame::new()
            .fill(palette.surface_container)
            .corner_radius(CornerRadius::same(
                metrics.corner_radii.border_radius_4.round() as u8,
            ))
            .show(ui, |ui| {
                ui.set_min_width(width);
                ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                ui.add_space(metrics.paddings.padding_8);

                for (index, item) in self.items.iter().enumerate() {
                    let response = show_menu_item(
                        ui,
                        item,
                        self.current_index == Some(index),
                        width,
                    );
                    if response.clicked() && item.enabled {
                        activated = Some(index);
                    }
                    item_responses.push(response);
                }

                ui.add_space(metrics.paddings.padding_8);
            });

        MenuInnerResponse {
            activated,
            item_responses,
        }
    }
}

impl<'a> PopupMenu<'a> {
    pub fn show_from_response(self, ui: &mut Ui, anchor: &Response) -> PopupMenuResponse {
        let popup_id = self.id;
        let mut activated = None;
        let mut item_responses = Vec::new();
        let mut popup_open = egui::Popup::is_id_open(ui.ctx(), popup_id);

        let _ = egui::Popup::from_response(anchor)
            .id(popup_id)
            .open_memory(None)
            .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
            .align(egui::RectAlign::TOP_START)
            .width(self.width)
            .show(|ui| {
                popup_open = true;
                let response = MenuInner {
                    items: self.items,
                    current_index: self.current_index,
                    width: Some(self.width),
                }
                .show(ui);
                activated = response.activated;
                item_responses = response.item_responses;
                if activated.is_some() {
                    egui::Popup::close_id(ui.ctx(), popup_id);
                }
            });

        PopupMenuResponse {
            activated,
            popup_open,
            item_responses,
        }
    }
}

fn show_menu_item(ui: &mut Ui, item: &MenuItem, selected: bool, width: f32) -> Response {
    let palette = material_palette_for_visuals(ui.visuals());
    let metrics = material_style_metrics();
    let (rect, response) = ui.allocate_exact_size(
        vec2(width, metrics.sizes.size_56),
        if item.enabled { Sense::click() } else { Sense::hover() },
    );

    if selected {
        ui.painter().rect_filled(
            rect,
            CornerRadius::same(metrics.corner_radii.border_radius_4.round() as u8),
            palette.surface_container_highest,
        );
    }

    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = metrics.spacings.spacing_12;
            ui.add_space(metrics.paddings.padding_12);

            if let Some(icon) = item.icon.clone() {
                let tint = if item.enabled {
                    palette.on_surface
                } else {
                    palette.on_surface.gamma_multiply(palette.disable_opacity)
                };
                let _ = ui.add(icon_with_style(
                    icon,
                    MaterialIconStyle {
                        size: vec2(metrics.icon_sizes.icon_size_24, metrics.icon_sizes.icon_size_24),
                        tint,
                    },
                ));
            } else {
                ui.add_space(metrics.icon_sizes.icon_size_24);
            }

            let text_color = if item.enabled {
                palette.on_surface
            } else {
                palette.on_surface.gamma_multiply(palette.disable_opacity)
            };
            let trailing_color = if item.enabled {
                palette.on_surface_variant
            } else {
                palette
                    .on_surface_variant
                    .gamma_multiply(palette.disable_opacity)
            };

            let _ = material_text(ui, item.text.as_str())
                .text_style(MATERIAL_TYPOGRAPHY.body_large)
                .color(text_color)
                .show(ui);
            ui.add_space(ui.available_width().max(0.0));
            if !item.trailing_text.is_empty() {
                let _ = material_text(ui, item.trailing_text.as_str())
                    .text_style(MATERIAL_TYPOGRAPHY.body_large)
                    .color(trailing_color)
                    .show(ui);
            }
            ui.add_space(metrics.paddings.padding_12);
        });
    });

    response
}

#[cfg(test)]
mod tests {
    use egui::Context;
    use egui::Id;
    use egui::Sense;
    use egui::vec2;

    use super::MenuInner;
    use super::PopupMenu;
    use crate::material::components::divider::draw_horizontal_divider;
    use crate::material::items::menu_item::MenuItem;

    #[test]
    fn menu_inner_renders_without_panicking() {
        let context = Context::default();
        let mut item_count = 0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let items = vec![MenuItem::new("Open"), MenuItem::new("Save")];
                let response = MenuInner {
                    items: &items,
                    current_index: Some(0),
                    width: Some(200.0),
                }
                .show(ui);
                item_count = response.item_responses.len();
            });
        });
        assert_eq!(item_count, 2);
    }

    #[test]
    fn popup_menu_renders_without_panicking() {
        let context = Context::default();
        let mut popup_open = false;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let (_, anchor) = ui.allocate_exact_size(vec2(200.0, 56.0), Sense::click());
                egui::Popup::open_id(ui.ctx(), Id::new("menu-popup"));
                let items = vec![MenuItem::new("Open"), MenuItem::new("Save")];
                popup_open = PopupMenu {
                    id: Id::new("menu-popup"),
                    items: &items,
                    current_index: None,
                    width: 200.0,
                }
                .show_from_response(ui, &anchor)
                .popup_open;
            });
        });
        assert!(popup_open);
    }

    #[test]
    fn divider_helper_stays_available_for_menu_composition() {
        let context = Context::default();
        let mut divider_height = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                divider_height = draw_horizontal_divider(ui).rect.height();
            });
        });
        assert!(divider_height >= 1.0);
    }
}
