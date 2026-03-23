use egui::Response;
use egui::Sense;
use egui::Ui;

use crate::components::badge;
use crate::components::icon::icon;
use crate::components::material_text;
use crate::items::navigation_item::NavigationItem;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;

pub struct BaseNavigationItemTemplate<'a> {
    pub item: &'a NavigationItem,
    pub index: usize,
    pub selected: bool,
}

pub struct BaseNavigationResponse {
    pub changed_to: Option<usize>,
    pub item_responses: Vec<Response>,
}

pub fn base_navigation_item_template(
    ui: &mut Ui,
    template: BaseNavigationItemTemplate<'_>,
) -> Response {
    let palette = material_palette_for_visuals(ui.visuals());
    let metrics = material_style_metrics();
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width().max(0.0), metrics.sizes.size_56),
        Sense::click(),
    );

    let fill = if template.selected {
        palette.secondary_container
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter().rect_filled(
        rect,
        egui::CornerRadius::same(metrics.corner_radii.border_radius_12.round() as u8),
        fill,
    );

    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = metrics.spacings.spacing_12;
            if let Some(image) = if template.selected {
                template
                    .item
                    .selected_icon
                    .clone()
                    .or_else(|| template.item.icon.clone())
            } else {
                template.item.icon.clone()
            } {
                let _ = ui.add(icon(ui, image));
            }
            let text_color = if template.selected {
                palette.on_secondary_container
            } else {
                palette.on_surface
            };
            let _ = material_text(ui, template.item.text.as_str())
                .color(text_color)
                .show(ui);
            ui.add_space(ui.available_width().max(0.0));
            if template.item.show_badge {
                let _ = badge(ui, template.item.badge.as_str());
            }
        });
    });

    response
}

pub fn base_navigation(
    ui: &mut Ui,
    items: &[NavigationItem],
    current_index: &mut usize,
) -> BaseNavigationResponse {
    let mut changed_to = None;
    let mut item_responses = Vec::with_capacity(items.len());
    for (index, item) in items.iter().enumerate() {
        let response = base_navigation_item_template(
            ui,
            BaseNavigationItemTemplate {
                item,
                index,
                selected: *current_index == index,
            },
        );
        if response.clicked() && *current_index != index {
            *current_index = index;
            changed_to = Some(index);
        }
        item_responses.push(response);
    }
    BaseNavigationResponse {
        changed_to,
        item_responses,
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::base_navigation;
    use crate::items::navigation_item::NavigationItem;

    #[test]
    fn base_navigation_renders_without_panicking() {
        let context = Context::default();
        let mut changed_to = None;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let items = vec![NavigationItem::new("Home"), NavigationItem::new("Settings")];
                let mut current_index = 0;
                changed_to = base_navigation(ui, &items, &mut current_index).changed_to;
            });
        });
        assert_eq!(changed_to, None);
    }
}
