use std::borrow::Cow;

use egui::Align;
use egui::Area;
use egui::Context;
use egui::CornerRadius;
use egui::Id;
use egui::Order;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use egui::Vec2;
use egui::pos2;
use egui::vec2;

use crate::components::ElevationSpec;
use crate::components::Modal;
use crate::components::paint_elevation;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;

pub struct BottomSheetResponse<R> {
    pub inner: R,
    pub response: Response,
    pub drag_delta_y: f32,
    pub pressed: bool,
    pub released: bool,
}

pub struct BottomSheet<'a> {
    pub size: Vec2,
    pub title: Cow<'a, str>,
}

impl<'a> BottomSheet<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            size: vec2(material_style_metrics().sizes.size_640, 0.0),
            title: Cow::Borrowed(""),
        }
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> BottomSheetResponse<R> {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let available_width = ui.available_width();
        let width =
            self.size.x.min(metrics.sizes.size_640).min(
                (available_width - 2.0 * metrics.paddings.padding_56).max(metrics.sizes.size_200),
            );
        let inner = egui::Frame::new()
            .fill(palette.surface_container_low)
            .corner_radius(CornerRadius {
                nw: metrics.corner_radii.border_radius_16.round() as u8,
                ne: metrics.corner_radii.border_radius_16.round() as u8,
                sw: 0,
                se: 0,
            })
            .show(ui, |ui| {
                ui.set_width(width);
                ui.vertical(|ui| {
                    ui.with_layout(egui::Layout::top_down(Align::Center), |ui| {
                        let (drag_rect, response) = ui.allocate_exact_size(
                            vec2(width, metrics.sizes.size_36),
                            Sense::click_and_drag(),
                        );
                        let handle_rect = egui::Rect::from_center_size(
                            drag_rect.center(),
                            vec2(metrics.sizes.size_32, metrics.sizes.size_4),
                        );
                        ui.painter().rect_filled(
                            handle_rect,
                            CornerRadius::same((metrics.sizes.size_4 * 0.5).round() as u8),
                            palette.outline,
                        );
                        let inner = add_contents(ui);
                        (inner, response)
                    })
                    .inner
                })
                .inner
            });
        let (inner, drag_response) = inner.inner;
        let drag_delta_y = drag_response.drag_delta().y;
        let pressed = drag_response.drag_started();
        let released = drag_response.drag_stopped();
        BottomSheetResponse {
            inner,
            response: drag_response,
            drag_delta_y,
            pressed,
            released,
        }
    }
}

impl Default for BottomSheet<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ModalBottomSheetResponse<R> {
    pub inner: R,
    pub close_requested: bool,
    pub backdrop_response: Response,
    pub sheet_response: Response,
}

pub struct ModalBottomSheet<'a> {
    pub title: Cow<'a, str>,
    pub size: Vec2,
    pub drag_margin: f32,
}

impl<'a> ModalBottomSheet<'a> {
    #[must_use]
    pub fn new() -> Self {
        let metrics = material_style_metrics();
        Self {
            title: Cow::Borrowed(""),
            size: vec2(metrics.sizes.size_640, 0.0),
            drag_margin: metrics.paddings.padding_56,
        }
    }

    pub fn show<R>(
        self,
        ctx: &Context,
        id_source: impl std::hash::Hash,
        available_rect: egui::Rect,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> ModalBottomSheetResponse<R> {
        let palette = material_palette_for_visuals(&ctx.style().visuals);
        let metrics = material_style_metrics();
        let modal = Area::new(Id::new(("material_modal_bottom_sheet_overlay", &id_source)))
            .order(Order::Foreground)
            .fixed_pos(available_rect.min)
            .show(ctx, |ui| {
                ui.set_min_size(available_rect.size());
                Modal.show(ui, |ui| {
                    let width = self.size.x.min(metrics.sizes.size_640).min(
                        (available_rect.width() - 2.0 * metrics.paddings.padding_56)
                            .max(metrics.sizes.size_200),
                    );
                    let origin = pos2(
                        available_rect.center().x - width * 0.5,
                        available_rect.bottom(),
                    );
                    Area::new(Id::new(("material_modal_bottom_sheet_panel", id_source)))
                        .order(Order::Foreground)
                        .fixed_pos(origin)
                        .anchor(egui::Align2::LEFT_BOTTOM, vec2(0.0, 0.0))
                        .show(ui.ctx(), |ui| {
                            let desired_height = self.size.y.max(metrics.sizes.size_200);
                            let (rect, _) =
                                ui.allocate_exact_size(vec2(width, desired_height), Sense::hover());
                            paint_elevation(
                                ui.painter(),
                                rect,
                                ElevationSpec {
                                    background: palette.surface_container_low,
                                    border_radius: metrics.corner_radii.border_radius_16,
                                    level: 3,
                                    dark_mode: ui.visuals().dark_mode,
                                },
                                palette,
                            );
                            ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                                BottomSheet {
                                    size: vec2(width, desired_height),
                                    title: self.title,
                                }
                                .show(ui, add_contents)
                            })
                            .inner
                        })
                        .inner
                })
            })
            .inner;
        let sheet = modal.inner;
        let close_requested =
            modal.response.clicked() || (sheet.drag_delta_y > self.drag_margin && sheet.released);
        ModalBottomSheetResponse {
            inner: sheet.inner,
            close_requested,
            backdrop_response: modal.response,
            sheet_response: sheet.response,
        }
    }
}

impl Default for ModalBottomSheet<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;
    use egui::pos2;
    use egui::vec2;

    use super::BottomSheet;
    use super::ModalBottomSheet;

    #[test]
    fn bottom_sheet_renders_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = BottomSheet::new().show(ui, |ui| ui.label("Body"));
                width = response.response.rect.width();
            });
        });
        assert!(width > 0.0);
    }

    #[test]
    fn modal_bottom_sheet_renders_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            let available = egui::Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0));
            let response =
                ModalBottomSheet::new().show(ctx, "sheet", available, |ui| ui.label("Body"));
            width = response.backdrop_response.rect.width();
        });
        assert!(width >= 800.0);
    }
}
