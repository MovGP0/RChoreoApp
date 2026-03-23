use std::borrow::Cow;

use egui::Color32;
use egui::CornerRadius;
use egui::Id;
use egui::Image;
use egui::Rect;
use egui::Response;
use egui::Sense;
use egui::TextEdit;
use egui::Ui;
use egui::UiBuilder;
use egui::vec2;

use crate::components::icon_button::MaterialIconButton;
use crate::components::material_text::material_text;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;
use crate::styling::material_typography::MATERIAL_TYPOGRAPHY;

const FIELD_CORNER_RADIUS: u8 = 4;

pub struct TextFieldResponse {
    pub response: Response,
    pub text_changed: bool,
    pub accepted: bool,
    pub leading_icon_clicked: bool,
    pub trailing_icon_clicked: bool,
    pub has_focus: bool,
}

pub struct TextField<'a> {
    pub id: Id,
    pub label: Cow<'a, str>,
    pub placeholder_text: Cow<'a, str>,
    pub supporting_text: Cow<'a, str>,
    pub text: &'a mut String,
    pub leading_icon: Option<Image<'static>>,
    pub trailing_icon: Option<Image<'static>>,
    pub enabled: bool,
    pub read_only: bool,
    pub has_error: bool,
    pub width: Option<f32>,
}

impl<'a> TextField<'a> {
    #[must_use]
    pub fn new(id_source: impl std::hash::Hash, text: &'a mut String) -> Self {
        Self {
            id: Id::new(id_source),
            label: Cow::Borrowed(""),
            placeholder_text: Cow::Borrowed(""),
            supporting_text: Cow::Borrowed(""),
            text,
            leading_icon: None,
            trailing_icon: None,
            enabled: true,
            read_only: false,
            has_error: false,
            width: None,
        }
    }

    #[must_use]
    pub fn label(mut self, label: impl Into<Cow<'a, str>>) -> Self {
        self.label = label.into();
        self
    }

    #[must_use]
    pub fn placeholder_text(mut self, placeholder_text: impl Into<Cow<'a, str>>) -> Self {
        self.placeholder_text = placeholder_text.into();
        self
    }

    #[must_use]
    pub fn supporting_text(mut self, supporting_text: impl Into<Cow<'a, str>>) -> Self {
        self.supporting_text = supporting_text.into();
        self
    }

    #[must_use]
    pub fn leading_icon(mut self, leading_icon: Image<'static>) -> Self {
        self.leading_icon = Some(leading_icon);
        self
    }

    #[must_use]
    pub fn trailing_icon(mut self, trailing_icon: Image<'static>) -> Self {
        self.trailing_icon = Some(trailing_icon);
        self
    }

    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    #[must_use]
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    #[must_use]
    pub fn has_error(mut self, has_error: bool) -> Self {
        self.has_error = has_error;
        self
    }

    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn show(self, ui: &mut Ui) -> TextFieldResponse {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let label = self.label.clone();
        let placeholder_text = self.placeholder_text.clone();
        let supporting_text = self.supporting_text.clone();
        let leading_icon = self.leading_icon.clone();
        let trailing_icon = self.trailing_icon.clone();
        let has_leading = leading_icon.is_some();
        let has_trailing = trailing_icon.is_some();
        let supporting_height = if self.supporting_text.is_empty() {
            0.0
        } else {
            metrics.spacings.spacing_4 + MATERIAL_TYPOGRAPHY.body_small.font_size_px
        };
        let field_width = self.width.unwrap_or_else(|| ui.available_width().max(0.0));
        let total_size = vec2(field_width, metrics.sizes.size_56 + supporting_height);
        let (outer_rect, outer_response) = ui.allocate_exact_size(total_size, Sense::hover());
        let field_rect =
            Rect::from_min_size(outer_rect.min, vec2(total_size.x, metrics.sizes.size_56));
        let supporting_rect = Rect::from_min_max(
            egui::pos2(
                outer_rect.min.x + metrics.paddings.padding_16,
                field_rect.max.y + metrics.spacings.spacing_4,
            ),
            outer_rect.max,
        );

        let mut response = outer_response.clone();
        let mut text_changed = false;
        let mut accepted = false;
        let mut leading_icon_clicked = false;
        let mut trailing_icon_clicked = false;
        let mut has_focus = ui.memory(|memory| memory.has_focus(self.id));

        ui.painter().rect_filled(
            field_rect,
            CornerRadius::same(FIELD_CORNER_RADIUS),
            if self.enabled {
                palette.surface_container_highest
            } else {
                palette
                    .surface_container_highest
                    .gamma_multiply(palette.disable_opacity)
            },
        );

        let show_floating_label = should_float_label(self.text, has_focus);
        let initial_highlight = field_highlight(
            self.enabled,
            self.has_error,
            has_focus,
            palette.error,
            palette.primary,
            palette.on_surface_variant,
        );

        ui.scope_builder(UiBuilder::new().max_rect(field_rect), |ui| {
            let left_padding = if has_leading {
                metrics.paddings.padding_4
            } else {
                metrics.paddings.padding_16
            };
            let right_padding = if has_trailing {
                metrics.paddings.padding_4
            } else {
                metrics.paddings.padding_16
            };
            let inner_rect = Rect::from_min_max(
                field_rect.min + vec2(left_padding, metrics.paddings.padding_4),
                field_rect.max - vec2(right_padding, metrics.paddings.padding_4),
            );
            ui.scope_builder(UiBuilder::new().max_rect(inner_rect), |ui| {
                ui.spacing_mut().item_spacing.x = metrics.spacings.spacing_2;
                ui.horizontal(|ui| {
                    if let Some(icon) = leading_icon.clone() {
                        let icon_response = MaterialIconButton {
                            inline: true,
                            enabled: self.enabled,
                            ..MaterialIconButton::new(icon)
                        }
                        .show(ui);
                        leading_icon_clicked = icon_response.response.clicked();
                        response = response.union(icon_response.response);
                    }

                    ui.vertical(|ui| {
                        ui.set_width(
                            (field_rect.width()
                                - left_padding
                                - right_padding
                                - if has_leading {
                                    metrics.icon_sizes.icon_size_18 + metrics.spacings.spacing_2
                                } else {
                                    0.0
                                }
                                - if has_trailing {
                                    metrics.icon_sizes.icon_size_18 + metrics.spacings.spacing_2
                                } else {
                                    0.0
                                })
                            .max(0.0),
                        );

                        if show_floating_label {
                            let _ = material_text(ui, label.clone())
                                .text_style(MATERIAL_TYPOGRAPHY.body_small)
                                .color(initial_highlight)
                                .show(ui);
                        } else {
                            ui.add_space(metrics.paddings.padding_6);
                        }

                        let mut text_edit = TextEdit::singleline(self.text)
                            .id(self.id)
                            .frame(false)
                            .desired_width(f32::INFINITY)
                            .hint_text(if show_floating_label {
                                placeholder_text.as_ref()
                            } else {
                                label.as_ref()
                            });
                        if self.read_only {
                            text_edit = text_edit.interactive(false);
                        }
                        let text_response = ui.add_enabled(self.enabled, text_edit);
                        text_changed = text_response.changed();
                        has_focus = text_response.has_focus();
                        accepted = text_response.lost_focus()
                            && ui.input(|input| input.key_pressed(egui::Key::Enter));
                        response = response.union(text_response);
                    });

                    if let Some(icon) = trailing_icon.clone() {
                        let icon_response = MaterialIconButton {
                            inline: true,
                            enabled: self.enabled,
                            has_error: self.has_error,
                            ..MaterialIconButton::new(icon)
                        }
                        .show(ui);
                        trailing_icon_clicked = icon_response.response.clicked();
                        response = response.union(icon_response.response);
                    }
                });
            });
        });

        let highlight = field_highlight(
            self.enabled,
            self.has_error,
            has_focus,
            palette.error,
            palette.primary,
            palette.on_surface_variant,
        );
        let indicator_height = active_indicator_height(has_focus);
        let indicator_rect = Rect::from_min_max(
            egui::pos2(field_rect.min.x, field_rect.max.y - indicator_height),
            field_rect.max,
        );
        ui.painter().rect_filled(indicator_rect, 0.0, highlight);

        if !supporting_text.is_empty() {
            ui.scope_builder(UiBuilder::new().max_rect(supporting_rect), |ui| {
                let _ = material_text(ui, supporting_text)
                    .text_style(MATERIAL_TYPOGRAPHY.body_small)
                    .color(highlight)
                    .show(ui);
            });
        }

        TextFieldResponse {
            response,
            text_changed,
            accepted,
            leading_icon_clicked,
            trailing_icon_clicked,
            has_focus,
        }
    }
}

#[must_use]
pub fn field_highlight(
    enabled: bool,
    has_error: bool,
    has_focus: bool,
    error_color: Color32,
    focus_color: Color32,
    fallback_color: Color32,
) -> Color32 {
    if enabled && has_error {
        return error_color;
    }
    if has_focus {
        return focus_color;
    }
    fallback_color
}

#[must_use]
pub fn should_float_label(text: &str, has_focus: bool) -> bool {
    has_focus || !text.is_empty()
}

#[must_use]
pub const fn active_indicator_height(has_focus: bool) -> f32 {
    if has_focus { 3.0 } else { 1.0 }
}

#[cfg(test)]
mod tests {
    use egui::Color32;
    use egui::Context;
    use egui::Image;

    use super::TextField;
    use super::active_indicator_height;
    use super::field_highlight;
    use super::should_float_label;

    #[test]
    fn floating_label_matches_text_and_focus_state() {
        assert!(!should_float_label("", false));
        assert!(should_float_label("", true));
        assert!(should_float_label("value", false));
    }

    #[test]
    fn highlight_prioritizes_error_before_focus() {
        let highlight =
            field_highlight(true, true, true, Color32::RED, Color32::BLUE, Color32::GRAY);
        assert_eq!(highlight, Color32::RED);
        assert_eq!(active_indicator_height(true), 3.0);
        assert_eq!(active_indicator_height(false), 1.0);
    }

    #[test]
    fn text_field_renders_with_icons_and_supporting_text() {
        let context = Context::default();
        let mut text = String::from("hello");
        let mut positive_height = false;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = TextField::new("text-field", &mut text)
                    .label("Label")
                    .placeholder_text("Placeholder")
                    .supporting_text("Supporting")
                    .leading_icon(Image::new(egui::include_image!(
                        "../../assets/icons/Magnify.svg"
                    )))
                    .trailing_icon(Image::new(egui::include_image!(
                        "../../assets/icons/Close.svg"
                    )))
                    .show(ui);
                positive_height = response.response.rect.height() >= 56.0;
            });
        });
        assert!(positive_height);
    }
}
