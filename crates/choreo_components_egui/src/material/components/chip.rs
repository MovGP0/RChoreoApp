use std::borrow::Cow;

use egui::Color32;
use egui::Image;
use egui::Response;
use egui::Ui;
use egui::vec2;

use crate::material::components::BaseButton;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;

pub struct ActionChip<'a> {
    pub icon: Option<Image<'static>>,
    pub text: Cow<'a, str>,
    pub enabled: bool,
    pub tooltip: Cow<'a, str>,
    pub avatar_icon: Option<Image<'static>>,
    pub avatar_background: Option<Color32>,
}

impl<'a> ActionChip<'a> {
    #[must_use]
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            icon: None,
            text: text.into(),
            enabled: true,
            tooltip: Cow::Borrowed(""),
            avatar_icon: None,
            avatar_background: None,
        }
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let has_icon = self.icon.is_some();
        chip_frame(
            ui,
            palette.outline,
            Color32::TRANSPARENT,
            MaterialChipShape::Standard,
            |ui| {
                BaseButton {
                    icon: self.icon,
                    icon_color: Some(if self.enabled {
                        palette.primary
                    } else {
                        palette.on_surface
                    }),
                    text: self.text,
                    button_padding_left: Some(if has_icon {
                        metrics.paddings.padding_8
                    } else {
                        metrics.paddings.padding_16
                    }),
                    button_padding_right: Some(metrics.paddings.padding_16),
                    button_vertical_padding: metrics.paddings.padding_6,
                    min_layout_height: metrics.sizes.size_32,
                    avatar_icon: self.avatar_icon,
                    avatar_size: Some(metrics.sizes.size_18),
                    avatar_background: self.avatar_background,
                    color: Some(palette.on_surface),
                    tooltip: self.tooltip,
                    enabled: self.enabled,
                    border_radius: Some(metrics.corner_radii.border_radius_8),
                    ..BaseButton::new()
                }
                .show(ui, |_| {})
                .response
            },
        )
    }
}

pub struct FilterChip<'a> {
    pub icon: Option<Image<'static>>,
    pub text: Cow<'a, str>,
    pub enabled: bool,
    pub tooltip: Cow<'a, str>,
    pub checked: bool,
}

impl<'a> FilterChip<'a> {
    #[must_use]
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            icon: None,
            text: text.into(),
            enabled: true,
            tooltip: Cow::Borrowed(""),
            checked: false,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let border_color = if self.checked {
            Color32::TRANSPARENT
        } else {
            palette.outline
        };
        let background = if self.checked {
            palette.secondary_container
        } else {
            Color32::TRANSPARENT
        };
        let response = chip_frame(ui, border_color, background, MaterialChipShape::Standard, |ui| {
            BaseButton {
                icon: Some(if self.checked {
                    Image::new(egui::include_image!("../../../assets/icons/Check.svg"))
                } else {
                    self.icon.clone().unwrap_or_else(|| {
                        Image::new(egui::include_image!("../../../assets/icons/Check.svg"))
                    })
                }),
                icon_color: Some(if self.checked {
                    palette.on_secondary_container
                } else if self.enabled {
                    palette.primary
                } else {
                    palette.on_surface
                }),
                text: self.text.clone(),
                button_padding_left: Some(if self.icon.is_some() || self.checked {
                    metrics.paddings.padding_8
                } else {
                    metrics.paddings.padding_16
                }),
                button_padding_right: Some(metrics.paddings.padding_16),
                button_vertical_padding: metrics.paddings.padding_6,
                min_layout_height: metrics.sizes.size_32,
                color: Some(if self.checked {
                    palette.on_secondary_container
                } else {
                    palette.on_surface
                }),
                tooltip: self.tooltip.clone(),
                enabled: self.enabled,
                border_radius: Some(metrics.corner_radii.border_radius_8),
                display_background: !self.enabled,
                ..BaseButton::new()
            }
            .show(ui, |_| {})
            .response
        });
        if response.clicked() && self.enabled {
            self.checked = !self.checked;
        }
        response
    }
}

pub struct InputChip<'a> {
    pub leading_icon: Option<Image<'static>>,
    pub trailing_icon: Option<Image<'static>>,
    pub avatar: Option<Image<'static>>,
    pub avatar_background: Option<Color32>,
    pub text: Cow<'a, str>,
    pub enabled: bool,
    pub tooltip: Cow<'a, str>,
    pub checkable: bool,
    pub checked: bool,
}

pub struct InputChipResponse {
    pub response: Response,
    pub trailing_clicked: bool,
}

impl<'a> InputChip<'a> {
    #[must_use]
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            leading_icon: None,
            trailing_icon: None,
            avatar: None,
            avatar_background: None,
            text: text.into(),
            enabled: true,
            tooltip: Cow::Borrowed(""),
            checkable: false,
            checked: false,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> InputChipResponse {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let has_avatar = self.avatar.is_some();
        let border_color = if self.checked {
            Color32::TRANSPARENT
        } else {
            palette.outline
        };
        let background = if self.checked {
            palette.secondary_container
        } else {
            Color32::TRANSPARENT
        };
        let mut trailing_clicked = false;
        let response = chip_frame(
            ui,
            border_color,
            background,
            if has_avatar {
                MaterialChipShape::Avatar
            } else {
                MaterialChipShape::Standard
            },
            |ui| {
                BaseButton {
                    icon: self.leading_icon.clone(),
                    icon_color: Some(if self.checked {
                        palette.on_secondary_container
                    } else if self.enabled {
                        palette.primary
                    } else {
                        palette.on_surface
                    }),
                    text: self.text.clone(),
                    button_padding_left: Some(if has_avatar {
                        metrics.paddings.padding_4
                    } else if self.leading_icon.is_some() {
                        metrics.paddings.padding_8
                    } else {
                        metrics.paddings.padding_12
                    }),
                    button_padding_right: Some(if self.trailing_icon.is_some() {
                        metrics.paddings.padding_8
                    } else {
                        metrics.paddings.padding_12
                    }),
                    button_vertical_padding: metrics.paddings.padding_6,
                    min_layout_height: metrics.sizes.size_32,
                    avatar_icon: self.avatar.clone(),
                    avatar_size: Some(metrics.sizes.size_18),
                    avatar_background: self.avatar_background,
                    color: Some(if self.checked {
                        palette.on_secondary_container
                    } else {
                        palette.on_surface
                    }),
                    tooltip: self.tooltip.clone(),
                    enabled: self.enabled,
                    border_radius: Some(if has_avatar {
                        metrics.sizes.size_32 * 0.5
                    } else {
                        metrics.corner_radii.border_radius_8
                    }),
                    display_background: !self.enabled,
                    ..BaseButton::new()
                }
                .show(ui, |ui| {
                    if let Some(icon) = self.trailing_icon.clone() {
                        let response = ui.add_enabled(
                            self.enabled,
                            egui::Button::image(icon.fit_to_exact_size(vec2(
                                metrics.icon_sizes.icon_size_18,
                                metrics.icon_sizes.icon_size_18,
                            )))
                            .frame(false),
                        );
                        if response.clicked() {
                            trailing_clicked = true;
                        }
                    }
                })
                .response
            },
        );
        if response.clicked() && self.checkable && self.enabled {
            self.checked = !self.checked;
        }
        InputChipResponse {
            response,
            trailing_clicked,
        }
    }
}

enum MaterialChipShape {
    Standard,
    Avatar,
}

fn chip_frame(
    ui: &mut Ui,
    border_color: Color32,
    background: Color32,
    shape: MaterialChipShape,
    add_contents: impl FnOnce(&mut Ui) -> Response,
) -> Response {
    let metrics = material_style_metrics();
    let rounding = match shape {
        MaterialChipShape::Standard => metrics.corner_radii.border_radius_8,
        MaterialChipShape::Avatar => metrics.sizes.size_32 * 0.5,
    };
    egui::Frame::new()
        .fill(background)
        .corner_radius(rounding)
        .stroke(egui::Stroke::new(1.0, border_color))
        .show(ui, add_contents)
        .inner
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::ActionChip;
    use super::FilterChip;
    use super::InputChip;

    #[test]
    fn chips_render_without_panicking() {
        let context = Context::default();
        let mut positive = false;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = ActionChip::new("Action").show(ui);
                let mut filter = FilterChip::new("Filter");
                let _ = filter.show(ui);
                let mut input = InputChip::new("Input");
                let response = input.show(ui);
                positive = response.response.rect.height() > 0.0;
            });
        });
        assert!(positive);
    }
}
