use egui::Area;
use egui::Color32;
use egui::CornerRadius;
use egui::Frame;
use egui::Id;
use egui::Margin;
use egui::Order;
use egui::Pos2;
use egui::Rect;
use egui::Sense;
use egui::Ui;
use egui::pos2;
use egui::vec2;

use crate::ui_style::material_style_metrics::material_style_metrics;

#[cfg_attr(test, allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialogMetricsTokens {
    pub dialog_padding: i8,
    pub dialog_margin: f32,
    pub dialog_corner_radius: u8,
}

#[must_use]
#[cfg_attr(test, allow(dead_code))]
pub const fn dialog_metrics_tokens() -> DialogMetricsTokens {
    let metrics = material_style_metrics();
    DialogMetricsTokens {
        dialog_padding: metrics.paddings.padding_24 as i8,
        dialog_margin: metrics.paddings.padding_24,
        dialog_corner_radius: metrics.corner_radii.border_radius_12 as u8,
    }
}

pub struct DialogHostProps<'a> {
    pub id_source: &'a str,
    pub is_open: bool,
    pub close_on_click_away: bool,
    pub overlay_color: Color32,
    pub dialog_background: Color32,
    pub dialog_text_color: Color32,
    pub dialog_padding: i8,
    pub dialog_margin: f32,
    pub dialog_corner_radius: u8,
    pub dialog_content: &'a str,
}

#[must_use]
pub fn dialog_panel_rect(bounds: Rect, margin: f32) -> Rect {
    let left = bounds.left() + margin;
    let top = bounds.top() + margin;
    let width = (bounds.width() - (margin * 2.0)).max(0.0);
    let height = (bounds.height() - (margin * 2.0)).max(0.0);
    Rect::from_min_size(pos2(left, top), vec2(width, height))
}

pub fn draw_dialog_host(
    ui: &mut Ui,
    props: &DialogHostProps<'_>,
    add_children: impl FnOnce(&mut Ui),
) -> bool {
    add_children(ui);

    if !props.is_open {
        return false;
    }

    let host_rect = ui.max_rect();
    let panel_rect = dialog_panel_rect(host_rect, props.dialog_margin);
    let local_panel_rect = panel_rect.translate(-host_rect.min.to_vec2());

    let close_requested = Area::new(Id::new((props.id_source, "overlay")))
        .order(Order::Foreground)
        .fixed_pos(host_rect.min)
        .show(ui.ctx(), |ui| {
            let overlay_rect = Rect::from_min_size(Pos2::ZERO, host_rect.size());
            let _response = ui.allocate_rect(overlay_rect, Sense::click());
            ui.painter()
                .rect_filled(overlay_rect, 0.0, props.overlay_color);

            if !props.close_on_click_away {
                return false;
            }

            let pointer_release_pos = ui.input(|input| {
                input.events.iter().rev().find_map(|event| {
                    if let egui::Event::PointerButton {
                        pos,
                        button: egui::PointerButton::Primary,
                        pressed: false,
                        ..
                    } = event
                    {
                        return Some(*pos);
                    }
                    None
                })
            });

            pointer_release_pos.is_some_and(|position| {
                overlay_rect.contains(position) && !local_panel_rect.contains(position)
            })
        })
        .inner;

    Area::new(Id::new((props.id_source, "panel")))
        .order(Order::Foreground)
        .fixed_pos(panel_rect.min)
        .show(ui.ctx(), |ui| {
            ui.set_min_size(panel_rect.size());
            Frame::new()
                .fill(props.dialog_background)
                .corner_radius(CornerRadius::same(props.dialog_corner_radius))
                .inner_margin(Margin::same(props.dialog_padding))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(props.dialog_content).color(props.dialog_text_color),
                    );
                });
        });

    close_requested
}
