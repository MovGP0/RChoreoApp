use egui::Align;
use egui::CornerRadius;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::vec2;

use crate::components::icon::MaterialIconStyle;
use crate::components::icon::icon_with_style;
use crate::components::material_text::material_text;
use crate::items::navigation_item::NavigationItem;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;
use crate::styling::material_typography::MATERIAL_TYPOGRAPHY;

pub struct TabBarResponse {
    pub changed_to: Option<usize>,
    pub item_responses: Vec<Response>,
}

pub struct TabBar<'a> {
    pub items: &'a [NavigationItem],
    pub current_index: usize,
}

pub struct SecondaryTabBar<'a> {
    pub items: &'a [NavigationItem],
    pub current_index: usize,
}

impl<'a> TabBar<'a> {
    pub fn show(self, ui: &mut Ui) -> TabBarResponse {
        show_tab_bar(ui, self.items, self.current_index, false)
    }
}

impl<'a> SecondaryTabBar<'a> {
    pub fn show(self, ui: &mut Ui) -> TabBarResponse {
        show_tab_bar(ui, self.items, self.current_index, true)
    }
}

fn show_tab_bar(
    ui: &mut Ui,
    items: &[NavigationItem],
    current_index: usize,
    secondary: bool,
) -> TabBarResponse {
    let palette = material_palette_for_visuals(ui.visuals());
    let metrics = material_style_metrics();
    let total_width = ui.available_width().max(metrics.sizes.size_80);
    let item_width = if items.is_empty() {
        total_width
    } else {
        total_width / items.len() as f32
    };
    let height = metrics.sizes.size_56;
    let mut changed_to = None;
    let mut item_responses = Vec::with_capacity(items.len());
    let mut selected_rect = None;

    egui::Frame::new().fill(palette.surface).show(ui, |ui| {
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        ui.horizontal(|ui| {
            for (index, item) in items.iter().enumerate() {
                let response = show_tab_item(
                    ui,
                    item,
                    index == current_index,
                    item_width,
                    height,
                    secondary,
                );
                if index == current_index {
                    selected_rect = Some(response.rect);
                }
                if response.clicked() && index != current_index {
                    changed_to = Some(index);
                }
                item_responses.push(response);
            }
        });

        if let Some(rect) = selected_rect {
            let indicator_height = if secondary { 2.0 } else { 3.0 };
            let indicator_width = if secondary {
                rect.width()
            } else {
                (rect.width() * 0.5).max(metrics.sizes.size_24)
            };
            let indicator_rect = egui::Rect::from_center_size(
                egui::pos2(rect.center().x, rect.bottom() - indicator_height * 0.5),
                vec2(indicator_width, indicator_height),
            );
            ui.painter().rect_filled(
                indicator_rect,
                CornerRadius::same((indicator_height * 0.5).round() as u8),
                palette.primary,
            );
        }
    });

    TabBarResponse {
        changed_to,
        item_responses,
    }
}

fn show_tab_item(
    ui: &mut Ui,
    item: &NavigationItem,
    selected: bool,
    width: f32,
    height: f32,
    secondary: bool,
) -> Response {
    let palette = material_palette_for_visuals(ui.visuals());
    let metrics = material_style_metrics();
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::click());
    let color = if selected {
        if secondary {
            palette.on_surface
        } else {
            palette.primary
        }
    } else {
        palette.on_surface_variant
    };

    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        if secondary {
            ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                ui.add_space(metrics.paddings.padding_16);
                if let Some(icon) = selected
                    .then(|| item.selected_icon.clone())
                    .flatten()
                    .or_else(|| item.icon.clone())
                {
                    let _ = ui.add(icon_with_style(
                        icon,
                        MaterialIconStyle {
                            size: vec2(
                                metrics.icon_sizes.icon_size_24,
                                metrics.icon_sizes.icon_size_24,
                            ),
                            tint: color,
                        },
                    ));
                }
                let _ = material_text(ui, item.text.as_str())
                    .text_style(MATERIAL_TYPOGRAPHY.title_small)
                    .color(color)
                    .show(ui);
                ui.add_space(metrics.paddings.padding_16);
            });
        } else {
            ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                ui.add_space(metrics.paddings.padding_10);
                if let Some(icon) = selected
                    .then(|| item.selected_icon.clone())
                    .flatten()
                    .or_else(|| item.icon.clone())
                {
                    let _ = ui.add(icon_with_style(
                        icon,
                        MaterialIconStyle {
                            size: vec2(
                                metrics.icon_sizes.icon_size_24,
                                metrics.icon_sizes.icon_size_24,
                            ),
                            tint: color,
                        },
                    ));
                }
                let _ = material_text(ui, item.text.as_str())
                    .text_style(MATERIAL_TYPOGRAPHY.title_small)
                    .color(color)
                    .show(ui);
            });
        }
    });

    response
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::SecondaryTabBar;
    use super::TabBar;
    use crate::items::navigation_item::NavigationItem;

    #[test]
    fn tab_bar_renders_without_panicking() {
        let context = Context::default();
        let mut item_count = 0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let items = vec![NavigationItem::new("Home"), NavigationItem::new("Files")];
                item_count = TabBar {
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

    #[test]
    fn secondary_tab_bar_renders_without_panicking() {
        let context = Context::default();
        let mut item_count = 0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let items = vec![NavigationItem::new("Home"), NavigationItem::new("Files")];
                item_count = SecondaryTabBar {
                    items: &items,
                    current_index: 1,
                }
                .show(ui)
                .item_responses
                .len();
            });
        });
        assert_eq!(item_count, 2);
    }
}
