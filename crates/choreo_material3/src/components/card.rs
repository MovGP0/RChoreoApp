use egui::Color32;
use egui::CornerRadius;
use egui::Rect;
use egui::Response;
use egui::Sense;
use egui::Stroke;
use egui::StrokeKind;
use egui::Ui;
use egui::UiBuilder;
use egui::Vec2;

use super::elevation::elevation_spec;
use super::elevation::paint_elevation_for_ui;
use super::state_layer::StateLayerStyle;
use super::state_layer::paint_state_layer_for_response;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;

const CARD_ELEVATION_LEVEL: u8 = 1;

pub struct CardResponse<R> {
    pub inner: R,
    pub response: Response,
    pub activated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BaseCard {
    size: Vec2,
    clickable: bool,
    has_elevation: bool,
    background: Color32,
    border_color: Color32,
    border_width: f32,
}

impl BaseCard {
    #[must_use]
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            clickable: false,
            has_elevation: false,
            background: Color32::TRANSPARENT,
            border_color: Color32::TRANSPARENT,
            border_width: 0.0,
        }
    }

    #[must_use]
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    #[must_use]
    pub fn has_elevation(mut self, has_elevation: bool) -> Self {
        self.has_elevation = has_elevation;
        self
    }

    #[must_use]
    pub fn background(mut self, background: Color32) -> Self {
        self.background = background;
        self
    }

    #[must_use]
    pub fn border(mut self, border_width: f32, border_color: Color32) -> Self {
        self.border_width = border_width;
        self.border_color = border_color;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> CardResponse<R> {
        let (rect, response) = ui.allocate_exact_size(self.size, self.sense());
        self.show_with_response(ui, rect, response, add_contents)
    }

    pub fn show_in_rect<R>(
        self,
        ui: &mut Ui,
        rect: Rect,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> CardResponse<R> {
        let response = ui.allocate_rect(rect, self.sense());
        self.show_with_response(ui, rect, response, add_contents)
    }

    fn show_with_response<R>(
        self,
        ui: &mut Ui,
        rect: Rect,
        response: Response,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> CardResponse<R> {
        self.paint_background(ui, rect);
        self.paint_border(ui, rect);
        self.paint_state_layer(ui, &response);

        let clip_rect = rect.intersect(ui.clip_rect());
        let inner = ui
            .scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                ui.set_clip_rect(clip_rect);
                add_contents(ui)
            })
            .inner;

        CardResponse {
            inner,
            activated: self.activated(ui, &response),
            response,
        }
    }

    fn paint_background(self, ui: &Ui, rect: Rect) {
        if self.has_elevation {
            let spec = elevation_spec(
                self.background,
                card_corner_radius(),
                CARD_ELEVATION_LEVEL,
                ui.visuals().dark_mode,
            );
            paint_elevation_for_ui(ui.painter(), rect, spec, ui.visuals());
            return;
        }

        ui.painter()
            .rect_filled(rect, card_rounding(), self.background);
    }

    fn paint_border(self, ui: &Ui, rect: Rect) {
        if self.border_width <= 0.0 {
            return;
        }

        ui.painter().rect_stroke(
            rect,
            card_rounding(),
            Stroke::new(self.border_width, self.border_color),
            StrokeKind::Middle,
        );
    }

    fn paint_state_layer(self, ui: &Ui, response: &Response) {
        if !self.clickable {
            return;
        }

        let palette = material_palette_for_visuals(ui.visuals());
        let mut state_layer = StateLayerStyle::for_ui(ui);
        state_layer.color = palette.on_surface;
        state_layer.border_radius = card_corner_radius();
        state_layer.enabled = ui.is_enabled();
        paint_state_layer_for_response(ui, response, state_layer);
    }

    #[must_use]
    fn sense(self) -> Sense {
        if self.clickable {
            Sense::click()
        } else {
            Sense::hover()
        }
    }

    #[must_use]
    fn activated(self, ui: &Ui, response: &Response) -> bool {
        if !self.clickable {
            return false;
        }
        response.clicked()
            || (response.has_focus()
                && ui.input(|input| {
                    input.key_pressed(egui::Key::Space) || input.key_pressed(egui::Key::Enter)
                }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElevatedCard {
    size: Vec2,
    clickable: bool,
}

impl ElevatedCard {
    #[must_use]
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            clickable: false,
        }
    }

    #[must_use]
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> CardResponse<R> {
        let palette = material_palette_for_visuals(ui.visuals());
        BaseCard::new(self.size)
            .clickable(self.clickable)
            .has_elevation(true)
            .background(palette.surface)
            .show(ui, add_contents)
    }

    pub fn show_in_rect<R>(
        self,
        ui: &mut Ui,
        rect: Rect,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> CardResponse<R> {
        let palette = material_palette_for_visuals(ui.visuals());
        BaseCard::new(rect.size())
            .clickable(self.clickable)
            .has_elevation(true)
            .background(palette.surface)
            .show_in_rect(ui, rect, add_contents)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FilledCard {
    size: Vec2,
    clickable: bool,
}

impl FilledCard {
    #[must_use]
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            clickable: false,
        }
    }

    #[must_use]
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> CardResponse<R> {
        let palette = material_palette_for_visuals(ui.visuals());
        BaseCard::new(self.size)
            .clickable(self.clickable)
            .background(palette.surface_container_highest)
            .show(ui, add_contents)
    }

    pub fn show_in_rect<R>(
        self,
        ui: &mut Ui,
        rect: Rect,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> CardResponse<R> {
        let palette = material_palette_for_visuals(ui.visuals());
        BaseCard::new(rect.size())
            .clickable(self.clickable)
            .background(palette.surface_container_highest)
            .show_in_rect(ui, rect, add_contents)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutlinedCard {
    size: Vec2,
    clickable: bool,
}

impl OutlinedCard {
    #[must_use]
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
            clickable: false,
        }
    }

    #[must_use]
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> CardResponse<R> {
        let palette = material_palette_for_visuals(ui.visuals());
        BaseCard::new(self.size)
            .clickable(self.clickable)
            .background(palette.surface_container_low)
            .border(material_style_metrics().strokes.outline, palette.outline)
            .show(ui, add_contents)
    }

    pub fn show_in_rect<R>(
        self,
        ui: &mut Ui,
        rect: Rect,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> CardResponse<R> {
        let palette = material_palette_for_visuals(ui.visuals());
        BaseCard::new(rect.size())
            .clickable(self.clickable)
            .background(palette.surface_container_low)
            .border(material_style_metrics().strokes.outline, palette.outline)
            .show_in_rect(ui, rect, add_contents)
    }
}

#[must_use]
fn card_corner_radius() -> f32 {
    material_style_metrics().corner_radii.border_radius_12
}

#[must_use]
fn card_rounding() -> CornerRadius {
    CornerRadius::same(card_corner_radius().round() as u8)
}
