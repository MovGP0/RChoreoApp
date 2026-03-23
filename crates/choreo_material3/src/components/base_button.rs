use std::borrow::Cow;

use egui::Color32;
use egui::Image;
use egui::Rect;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use egui::Vec2;
use egui::vec2;

use crate::components::icon::MaterialIconStyle;
use crate::components::icon::centered_icon_rect;
use crate::components::icon::paint_icon;
use crate::components::icon::show_icon_with_style;
use crate::components::list::AvatarStyle;
use crate::components::list::avatar;
use crate::components::material_text::MaterialTextOverflow;
use crate::components::material_text::material_text;
use crate::components::state_layer::StateLayerStyle;
use crate::components::state_layer::apply_tooltip;
use crate::components::state_layer::paint_state_layer_for_response;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;
use crate::styling::material_typography::MATERIAL_TYPOGRAPHY;

pub struct BaseButtonResponse<R> {
    pub inner: R,
    pub response: Response,
}

pub struct BaseButton<'a> {
    pub icon: Option<Image<'static>>,
    pub icon_color: Option<Color32>,
    pub text: Cow<'a, str>,
    pub button_horizontal_padding: f32,
    pub button_vertical_padding: f32,
    pub button_padding_left: Option<f32>,
    pub button_padding_right: Option<f32>,
    pub button_padding_top: Option<f32>,
    pub button_padding_bottom: Option<f32>,
    pub spacing: f32,
    pub min_layout_width: f32,
    pub min_layout_height: f32,
    pub icon_size: f32,
    pub avatar_icon: Option<Image<'static>>,
    pub avatar_size: Option<f32>,
    pub avatar_background: Option<Color32>,
    pub color: Option<Color32>,
    pub tooltip: Cow<'a, str>,
    pub enabled: bool,
    pub border_radius: Option<f32>,
    pub display_background: bool,
    pub clip_ripple: bool,
}

impl<'a> BaseButton<'a> {
    #[must_use]
    pub fn new() -> Self {
        let metrics = material_style_metrics();
        Self {
            icon: None,
            icon_color: None,
            text: Cow::Borrowed(""),
            button_horizontal_padding: metrics.paddings.padding_24,
            button_vertical_padding: metrics.paddings.padding_10,
            button_padding_left: None,
            button_padding_right: None,
            button_padding_top: None,
            button_padding_bottom: None,
            spacing: metrics.spacings.spacing_8,
            min_layout_width: metrics.sizes.size_40,
            min_layout_height: metrics.sizes.size_40,
            icon_size: metrics.icon_sizes.icon_size_18,
            avatar_icon: None,
            avatar_size: None,
            avatar_background: None,
            color: None,
            tooltip: Cow::Borrowed(""),
            enabled: true,
            border_radius: None,
            display_background: true,
            clip_ripple: true,
        }
    }

    #[must_use]
    pub fn has_icon(&self) -> bool {
        self.icon.is_some()
    }

    #[must_use]
    pub fn has_avatar(&self) -> bool {
        self.avatar_icon.is_some()
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_children: impl FnOnce(&mut Ui) -> R,
    ) -> BaseButtonResponse<R> {
        let desired_size = self.desired_size(ui);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
        let mut state_style = StateLayerStyle::for_ui(ui);
        let palette = material_palette_for_visuals(ui.visuals());
        state_style.color = self.color.unwrap_or(palette.on_surface);
        state_style.border_radius = self.border_radius.unwrap_or(rect.height() * 0.5);
        state_style.display_background = self.display_background;
        state_style.clip_ripple = self.clip_ripple;
        state_style.enabled = self.enabled;
        state_style.tooltip = self.tooltip.as_ref();
        paint_state_layer_for_response(ui, &response, state_style);
        let response = apply_tooltip(ui, response, state_style);

        let inner = ui
            .scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                self.paint_contents(ui, rect, add_children)
            })
            .inner;

        BaseButtonResponse { inner, response }
    }

    fn desired_size(&self, ui: &Ui) -> Vec2 {
        let font_height = ui.fonts(|fonts| {
            fonts.row_height(&egui::FontId::proportional(
                MATERIAL_TYPOGRAPHY.label_large.font_size_px,
            ))
        });
        let leading_width = match (self.has_avatar(), self.has_icon()) {
            (true, true) => self
                .avatar_size
                .unwrap_or(self.icon_size)
                .max(self.icon_size),
            (true, false) => self.avatar_size.unwrap_or(self.icon_size),
            (false, true) => self.icon_size,
            (false, false) => 0.0,
        };
        let leading_height = match (self.has_avatar(), self.has_icon()) {
            (true, true) => self.avatar_size.unwrap_or(self.icon_size) + self.icon_size,
            (true, false) => self.avatar_size.unwrap_or(self.icon_size),
            (false, true) => self.icon_size,
            (false, false) => 0.0,
        };
        let content_height = self.text_height_or_zero(font_height).max(leading_height)
            + self.padding_top()
            + self.padding_bottom();
        let content_width = self.padding_left()
            + self.padding_right()
            + leading_width
            + if leading_width > 0.0 && !self.text.is_empty() {
                self.spacing
            } else {
                0.0
            }
            + self.text_width_or_zero(ui);
        vec2(
            self.min_layout_width.max(content_width),
            self.min_layout_height.max(content_height),
        )
    }

    fn paint_contents<R>(
        self,
        ui: &mut Ui,
        rect: Rect,
        add_children: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        let palette = material_palette_for_visuals(ui.visuals());
        let content_color = self.color.unwrap_or(palette.on_surface);
        let inner_rect = Rect::from_min_max(
            rect.min + vec2(self.padding_left(), self.padding_top()),
            rect.max - vec2(self.padding_right(), self.padding_bottom()),
        );

        if self.text.is_empty() && self.avatar_icon.is_none() && self.icon.is_some() {
            if let Some(icon) = self.icon.as_ref() {
                paint_icon(
                    ui,
                    icon,
                    centered_icon_rect(inner_rect, vec2(self.icon_size, self.icon_size)),
                    self.icon_color.unwrap_or(content_color),
                );
            }
            return ui
                .scope_builder(UiBuilder::new().max_rect(inner_rect), add_children)
                .inner;
        }

        ui.scope_builder(UiBuilder::new().max_rect(inner_rect), |ui| {
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::Center)
                    .with_main_align(egui::Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = self.spacing;
                    if self.has_avatar() || self.has_icon() {
                        ui.with_layout(
                            egui::Layout::top_down(egui::Align::Center)
                                .with_main_align(egui::Align::Center),
                            |ui| {
                                if let Some(avatar_icon) = self.avatar_icon {
                                    let avatar_style = AvatarStyle {
                                        background: self
                                            .avatar_background
                                            .unwrap_or(Color32::TRANSPARENT),
                                        size: vec2(
                                            self.avatar_size.unwrap_or(self.icon_size),
                                            self.avatar_size.unwrap_or(self.icon_size),
                                        ),
                                    };
                                    let _ = avatar(ui, Some(avatar_icon), avatar_style);
                                }
                                if let Some(icon) = self.icon {
                                    let _ = show_icon_with_style(
                                        ui,
                                        &icon,
                                        MaterialIconStyle {
                                            size: vec2(self.icon_size, self.icon_size),
                                            tint: self.icon_color.unwrap_or(content_color),
                                        },
                                    );
                                }
                            },
                        );
                    }
                    if !self.text.is_empty() {
                        let _ = material_text(ui, self.text)
                            .text_style(MATERIAL_TYPOGRAPHY.label_large)
                            .color(content_color)
                            .overflow(MaterialTextOverflow::Clip)
                            .show(ui);
                    }
                    add_children(ui)
                },
            )
            .inner
        })
        .inner
    }

    #[must_use]
    fn padding_left(&self) -> f32 {
        self.button_padding_left
            .unwrap_or(self.button_horizontal_padding)
    }

    #[must_use]
    fn padding_right(&self) -> f32 {
        self.button_padding_right
            .unwrap_or(self.button_horizontal_padding)
    }

    #[must_use]
    fn padding_top(&self) -> f32 {
        self.button_padding_top
            .unwrap_or(self.button_vertical_padding)
    }

    #[must_use]
    fn padding_bottom(&self) -> f32 {
        self.button_padding_bottom
            .unwrap_or(self.button_vertical_padding)
    }

    #[must_use]
    fn text_width_or_zero(&self, ui: &Ui) -> f32 {
        if self.text.is_empty() {
            return 0.0;
        }
        ui.fonts(|fonts| {
            fonts
                .layout_no_wrap(
                    self.text.clone().into_owned(),
                    egui::FontId::proportional(MATERIAL_TYPOGRAPHY.label_large.font_size_px),
                    Color32::WHITE,
                )
                .size()
                .x
        })
    }

    #[must_use]
    fn text_height_or_zero(&self, font_height: f32) -> f32 {
        if self.text.is_empty() {
            return 0.0;
        }
        font_height
    }
}

impl Default for BaseButton<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::BaseButton;

    #[test]
    fn base_button_renders_without_panicking() {
        let context = Context::default();
        let mut positive = false;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = BaseButton::new().show(ui, |_| {});
                positive = response.response.rect.height() >= 40.0;
            });
        });
        assert!(positive);
    }
}
