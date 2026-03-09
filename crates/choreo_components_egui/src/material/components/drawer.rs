use std::borrow::Cow;

use egui::Area;
use egui::Context;
use egui::CornerRadius;
use egui::Frame;
use egui::Id;
use egui::Order;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use egui::Vec2;
use egui::pos2;
use egui::vec2;

use crate::material::components::MaterialTextOverflow;
use crate::material::components::material_text;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;

pub struct DrawerHeader<'a> {
    pub title: Cow<'a, str>,
}

impl<'a> DrawerHeader<'a> {
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let desired_height = metrics.sizes.size_56;
        let available_width = ui.available_width().max(metrics.sizes.size_56);
        let (rect, response) = ui.allocate_exact_size(vec2(available_width, desired_height), Sense::hover());
        ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
            ui.add_space(metrics.paddings.padding_16);
            let _ = material_text(ui, self.title)
                .text_style(MATERIAL_TYPOGRAPHY.title_small)
                .color(palette.on_surface_variant)
                .overflow(MaterialTextOverflow::Elide)
                .show(ui);
        });
        response
    }
}

impl Default for DrawerHeader<'_> {
    fn default() -> Self {
        Self::new("")
    }
}

pub struct Drawer<'a> {
    pub title: Cow<'a, str>,
    pub min_width: f32,
}

pub struct DrawerResponse<R> {
    pub inner: R,
    pub response: Response,
}

impl<'a> Drawer<'a> {
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        let metrics = material_style_metrics();
        Self {
            title: title.into(),
            min_width: metrics.sizes.size_360,
        }
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> DrawerResponse<R> {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let width = self.min_width.max(ui.available_width().min(self.min_width));
        let frame = Frame::new()
            .fill(palette.surface_container_low)
            .corner_radius(CornerRadius::same(metrics.corner_radii.border_radius_16.round() as u8))
            .inner_margin(egui::Margin::same(metrics.paddings.padding_12.round() as i8));
        let inner = frame.show(ui, |ui| {
            ui.set_min_width(width - metrics.paddings.padding_12 * 2.0);
            ui.vertical(|ui| {
                if !self.title.is_empty() {
                    let _ = DrawerHeader::new(self.title).show(ui);
                }
                add_contents(ui)
            })
            .inner
        });
        DrawerResponse {
            inner: inner.inner,
            response: inner.response,
        }
    }
}

impl Default for Drawer<'_> {
    fn default() -> Self {
        Self::new("")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerPosition {
    Left,
    Right,
}

pub struct ModalDrawer<'a> {
    pub title: Cow<'a, str>,
    pub position: DrawerPosition,
    pub size: Vec2,
}

pub struct ModalDrawerResponse<R> {
    pub inner: R,
    pub close_requested: bool,
    pub backdrop_response: Response,
}

impl<'a> ModalDrawer<'a> {
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
            position: DrawerPosition::Left,
            size: vec2(material_style_metrics().sizes.size_360, 0.0),
        }
    }

    pub fn show<R>(
        self,
        ctx: &Context,
        id_source: impl std::hash::Hash,
        available_rect: egui::Rect,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> ModalDrawerResponse<R> {
        let palette = material_palette_for_visuals(&ctx.style().visuals);
        let drawer_width = self.size.x.max(material_style_metrics().sizes.size_360);
        let drawer_height = if self.size.y > 0.0 {
            self.size.y
        } else {
            available_rect.height()
        };
        let backdrop_id = Id::new(("material_modal_drawer_backdrop", &id_source));
        let drawer_id = Id::new(("material_modal_drawer_panel", id_source));

        let backdrop_response = Area::new(backdrop_id)
            .order(Order::Foreground)
            .fixed_pos(available_rect.min)
            .show(ctx, |ui| {
                let (rect, response) = ui.allocate_exact_size(available_rect.size(), Sense::click());
                ui.painter()
                    .rect_filled(rect, CornerRadius::ZERO, palette.background_modal);
                response
            })
            .inner;

        let drawer_origin = match self.position {
            DrawerPosition::Left => available_rect.min,
            DrawerPosition::Right => {
                pos2(available_rect.right() - drawer_width, available_rect.top())
            }
        };
        let inner = Area::new(drawer_id)
            .order(Order::Foreground)
            .fixed_pos(drawer_origin)
            .show(ctx, |ui| {
                ui.set_min_size(vec2(drawer_width, drawer_height));
                Drawer {
                    title: self.title,
                    min_width: drawer_width,
                }
                .show(ui, add_contents)
                .inner
            })
            .inner;

        ModalDrawerResponse {
            inner,
            close_requested: backdrop_response.clicked(),
            backdrop_response,
        }
    }
}

impl Default for ModalDrawer<'_> {
    fn default() -> Self {
        Self::new("")
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use egui::Context;
    use egui::pos2;
    use egui::vec2;

    use super::Drawer;
    use super::DrawerHeader;
    use super::DrawerPosition;
    use super::ModalDrawer;

    #[test]
    fn drawer_header_renders_without_panicking() {
        let context = Context::default();
        let mut height = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = DrawerHeader::new("Scenes").show(ui);
                height = response.rect.height();
            });
        });
        assert!(height >= 56.0);
    }

    #[test]
    fn drawer_renders_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = Drawer::new("Scenes").show(ui, |ui| {
                    ui.label("Item");
                });
                width = response.response.rect.width();
            });
        });
        assert!(width >= 360.0);
    }

    #[test]
    fn modal_drawer_renders_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            let available = egui::Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0));
            let response = ModalDrawer {
                title: Cow::Borrowed("Scenes"),
                position: DrawerPosition::Right,
                size: vec2(360.0, 600.0),
            }
            .show(ctx, "drawer_test", available, |ui| {
                ui.label("Item");
            });
            width = response.backdrop_response.rect.width();
        });
        assert!(width >= 800.0);
    }
}
