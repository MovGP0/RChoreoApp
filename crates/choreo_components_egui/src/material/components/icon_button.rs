use std::borrow::Cow;

use egui::Button;
use egui::Color32;
use egui::Image;
use egui::Response;
use egui::Ui;
use egui::vec2;

use crate::material::components::BaseButton;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;

const ICON_BUTTON_SIZE_PX: f32 = 48.0;
const ICON_GLYPH_SIZE_PX: f32 = 24.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopBarIcon {
    Settings,
    Home,
    Image,
    Audio,
}

#[must_use]
pub const fn icon_uri(icon: TopBarIcon) -> &'static str {
    match icon {
        TopBarIcon::Settings => "bytes://top_bar/settings.svg",
        TopBarIcon::Home => "bytes://top_bar/home.svg",
        TopBarIcon::Image => "bytes://top_bar/image.svg",
        TopBarIcon::Audio => "bytes://top_bar/audio.svg",
    }
}

pub fn icon_image(icon: TopBarIcon) -> Image<'static> {
    match icon {
        TopBarIcon::Settings => Image::new(egui::include_image!("../../../assets/icons/Pen.svg")),
        TopBarIcon::Home => Image::new(egui::include_image!("../../../assets/icons/Home.svg")),
        TopBarIcon::Image => Image::new(egui::include_image!("../../../assets/icons/Svg.svg")),
        TopBarIcon::Audio => {
            Image::new(egui::include_image!("../../../assets/icons/PlayCircle.svg"))
        }
    }
}

#[must_use]
pub fn icon_button(ui: &mut Ui, image: Image<'static>, checked: bool) -> Response {
    let tint = if checked {
        ui.visuals().selection.stroke.color
    } else {
        ui.visuals().widgets.inactive.fg_stroke.color
    };
    let button = Button::image(
        image
            .fit_to_exact_size(vec2(ICON_GLYPH_SIZE_PX, ICON_GLYPH_SIZE_PX))
            .tint(tint),
    )
    .selected(checked)
    .frame(true)
    .frame_when_inactive(false)
    .corner_radius(ICON_BUTTON_SIZE_PX / 2.0)
    .min_size(vec2(ICON_BUTTON_SIZE_PX, ICON_BUTTON_SIZE_PX))
    .image_tint_follows_text_color(false);
    ui.add(button)
}

pub struct MaterialIconButtonResponse {
    pub response: Response,
    pub checked: bool,
}

pub struct MaterialIconButton<'a> {
    pub icon: Image<'static>,
    pub checked_icon: Option<Image<'static>>,
    pub icon_color: Option<Color32>,
    pub checked_icon_color: Option<Color32>,
    pub disabled_icon_color: Option<Color32>,
    pub checkable: bool,
    pub checked: bool,
    pub tooltip: Cow<'a, str>,
    pub enabled: bool,
    pub inverse: bool,
    pub inline: bool,
    pub has_error: bool,
}

impl<'a> MaterialIconButton<'a> {
    #[must_use]
    pub fn new(icon: Image<'static>) -> Self {
        Self {
            icon,
            checked_icon: None,
            icon_color: None,
            checked_icon_color: None,
            disabled_icon_color: None,
            checkable: false,
            checked: false,
            tooltip: Cow::Borrowed(""),
            enabled: true,
            inverse: false,
            inline: false,
            has_error: false,
        }
    }

    pub fn show(self, ui: &mut Ui) -> MaterialIconButtonResponse {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let min_size = if self.inline {
            metrics.icon_sizes.icon_size_18
        } else {
            metrics.sizes.size_40
        };
        let glyph_size = if self.inline {
            metrics.icon_sizes.icon_size_18
        } else {
            metrics.icon_sizes.icon_size_24
        };
        let default_enabled_color = if self.has_error {
            palette.error
        } else if self.inverse {
            palette.inverse_on_surface
        } else {
            palette.on_surface_variant
        };
        let enabled_color = if self.checked {
            self.checked_icon_color
                .or(self.icon_color)
                .unwrap_or(palette.primary)
        } else {
            self.icon_color.unwrap_or(default_enabled_color)
        };
        let content_color = if self.enabled {
            enabled_color
        } else {
            self.disabled_icon_color.unwrap_or(enabled_color)
        };
        let icon = if self.checked {
            self.checked_icon.clone().unwrap_or_else(|| self.icon.clone())
        } else {
            self.icon.clone()
        };
        let response = BaseButton {
            icon: Some(icon),
            icon_color: Some(content_color),
            text: Cow::Borrowed(""),
            button_horizontal_padding: 0.0,
            button_vertical_padding: 0.0,
            min_layout_width: min_size,
            min_layout_height: min_size,
            icon_size: glyph_size,
            tooltip: self.tooltip,
            enabled: self.enabled,
            border_radius: Some(min_size * 0.5),
            display_background: false,
            clip_ripple: !self.inline,
            ..BaseButton::new()
        }
        .show(ui, |_| {})
        .response;
        let checked = if self.checkable && response.clicked() {
            !self.checked
        } else {
            self.checked
        };
        MaterialIconButtonResponse { response, checked }
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::MaterialIconButton;
    use super::TopBarIcon;
    use super::icon_image;

    #[test]
    fn material_icon_button_renders_without_panicking() {
        let context = Context::default();
        let mut min_size = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = MaterialIconButton::new(icon_image(TopBarIcon::Home)).show(ui);
                min_size = response.response.rect.width().min(response.response.rect.height());
            });
        });
        assert!(min_size >= 40.0);
    }

    #[test]
    fn inline_material_icon_button_uses_compact_extent() {
        let context = Context::default();
        let mut min_size = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = MaterialIconButton {
                    inline: true,
                    ..MaterialIconButton::new(icon_image(TopBarIcon::Home))
                }
                .show(ui);
                min_size = response.response.rect.width().min(response.response.rect.height());
            });
        });
        assert!(min_size >= 18.0);
        assert!(min_size < 40.0);
    }
}
