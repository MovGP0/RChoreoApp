use std::borrow::Cow;

use egui::Response;
use egui::Ui;
use egui::Vec2;
use egui::vec2;

use crate::material::components::IconButtonItem;
use crate::material::components::material_text;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;
use crate::material::styling::material_typography::TextStyle;

pub struct BaseAppBar<'a> {
    pub title: Cow<'a, str>,
    pub leading_button: Option<&'a IconButtonItem>,
    pub show_background: bool,
    pub trailing_buttons: &'a [IconButtonItem],
    pub horizontal_alignment: egui::Align,
    pub spacing: f32,
    pub vertical_spacing: f32,
    pub two_rows: bool,
    pub text_style: TextStyle,
    pub text_padding: f32,
    pub appbar_padding_right: f32,
    pub appbar_padding_top: f32,
    pub appbar_padding_bottom: f32,
}

pub struct AppBarResponse {
    pub leading: Option<Response>,
    pub trailing: Vec<Response>,
}

impl<'a> BaseAppBar<'a> {
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        let metrics = material_style_metrics();
        Self {
            title: title.into(),
            leading_button: None,
            show_background: false,
            trailing_buttons: &[],
            horizontal_alignment: egui::Align::Center,
            spacing: metrics.spacings.spacing_6,
            vertical_spacing: 0.0,
            two_rows: false,
            text_style: MATERIAL_TYPOGRAPHY.title_large,
            text_padding: metrics.paddings.padding_16,
            appbar_padding_right: 0.0,
            appbar_padding_top: 0.0,
            appbar_padding_bottom: 0.0,
        }
    }

    pub fn show(self, ui: &mut Ui) -> AppBarResponse {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let mut leading = None;
        let mut trailing = Vec::with_capacity(self.trailing_buttons.len());

        egui::Frame::new()
            .fill(if self.show_background {
                palette.surface_container
            } else {
                palette.surface
            })
            .show(ui, |ui| {
                ui.add_space(self.appbar_padding_top);
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = self.vertical_spacing;
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = metrics.spacings.spacing_6;
                        ui.add_space(self.spacing);

                        if let Some(button) = self.leading_button {
                            let response = ui.add_enabled(
                                button.enabled,
                                egui::Button::image(button.icon.clone()),
                            );
                            leading = Some(response.on_hover_text(button.tooltip.clone()));
                        }

                        if !self.two_rows {
                            let trailing_width = estimated_trailing_width(
                                self.trailing_buttons.len(),
                                metrics.sizes.size_40,
                                metrics.spacings.spacing_6,
                            );
                            let title_width = (ui.available_width() - trailing_width).max(0.0);
                            let title_height = self.text_style.font_size_px.max(metrics.sizes.size_40);
                            ui.allocate_ui_with_layout(
                                vec2(title_width, title_height),
                                title_lane_layout(self.horizontal_alignment),
                                |ui| {
                                    let _ = material_text(ui, self.title.clone())
                                        .text_style(self.text_style)
                                        .color(palette.on_surface)
                                        .show(ui);
                                },
                            );
                        } else {
                            ui.add_space(ui.available_width().max(0.0));
                        }

                        for button in self.trailing_buttons {
                            let response = ui.add_enabled(
                                button.enabled,
                                egui::Button::image(button.icon.clone()),
                            );
                            trailing.push(response.on_hover_text(button.tooltip.clone()));
                        }
                        ui.add_space(self.appbar_padding_right);
                    });

                    if self.two_rows && !self.title.is_empty() {
                        ui.add_space(self.text_padding);
                        let _ = material_text(ui, self.title)
                            .text_style(self.text_style)
                            .color(palette.on_surface)
                            .show(ui);
                    }
                });
                ui.add_space(self.appbar_padding_bottom);
            });

        AppBarResponse { leading, trailing }
    }
}

pub struct AppBar<'a> {
    pub title: Cow<'a, str>,
    pub leading_button: Option<&'a IconButtonItem>,
    pub trailing_button: Option<&'a IconButtonItem>,
    pub show_background: bool,
}

impl<'a> AppBar<'a> {
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
            leading_button: None,
            trailing_button: None,
            show_background: false,
        }
    }

    pub fn show(self, ui: &mut Ui) -> AppBarResponse {
        let metrics = material_style_metrics();
        let trailing = self.trailing_button.map_or(&[][..], std::slice::from_ref);
        BaseAppBar {
            title: self.title,
            leading_button: self.leading_button,
            show_background: self.show_background,
            trailing_buttons: trailing,
            horizontal_alignment: egui::Align::Center,
            spacing: metrics.spacings.spacing_6,
            vertical_spacing: 0.0,
            two_rows: false,
            text_style: MATERIAL_TYPOGRAPHY.title_large,
            text_padding: metrics.paddings.padding_16,
            appbar_padding_right: 0.0,
            appbar_padding_top: metrics.paddings.padding_8,
            appbar_padding_bottom: 0.0,
        }
        .show(ui)
    }
}

pub struct SmallAppBar<'a> {
    pub title: Cow<'a, str>,
    pub leading_button: Option<&'a IconButtonItem>,
    pub trailing_buttons: &'a [IconButtonItem],
    pub show_background: bool,
}

impl<'a> SmallAppBar<'a> {
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
            leading_button: None,
            trailing_buttons: &[],
            show_background: false,
        }
    }

    pub fn show(self, ui: &mut Ui) -> AppBarResponse {
        let metrics = material_style_metrics();
        BaseAppBar {
            title: self.title,
            leading_button: self.leading_button,
            show_background: self.show_background,
            trailing_buttons: self.trailing_buttons,
            horizontal_alignment: egui::Align::Min,
            spacing: metrics.spacings.spacing_4,
            vertical_spacing: 0.0,
            two_rows: false,
            text_style: MATERIAL_TYPOGRAPHY.title_large,
            text_padding: metrics.paddings.padding_16,
            appbar_padding_right: 0.0,
            appbar_padding_top: metrics.paddings.padding_8,
            appbar_padding_bottom: 0.0,
        }
        .show(ui)
    }
}

pub struct MediumAppBar<'a> {
    pub title: Cow<'a, str>,
    pub leading_button: Option<&'a IconButtonItem>,
    pub trailing_buttons: &'a [IconButtonItem],
    pub show_background: bool,
}

impl<'a> MediumAppBar<'a> {
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
            leading_button: None,
            trailing_buttons: &[],
            show_background: false,
        }
    }

    pub fn show(self, ui: &mut Ui) -> AppBarResponse {
        let metrics = material_style_metrics();
        BaseAppBar {
            title: self.title,
            leading_button: self.leading_button,
            show_background: self.show_background,
            trailing_buttons: self.trailing_buttons,
            horizontal_alignment: egui::Align::Min,
            spacing: metrics.spacings.spacing_4,
            vertical_spacing: metrics.spacings.spacing_4,
            two_rows: true,
            text_style: MATERIAL_TYPOGRAPHY.headline_small,
            text_padding: metrics.paddings.padding_16,
            appbar_padding_right: 0.0,
            appbar_padding_top: metrics.paddings.padding_8,
            appbar_padding_bottom: metrics.paddings.padding_24,
        }
        .show(ui)
    }
}

pub struct LargeAppBar<'a> {
    pub title: Cow<'a, str>,
    pub leading_button: Option<&'a IconButtonItem>,
    pub trailing_buttons: &'a [IconButtonItem],
    pub show_background: bool,
}

impl<'a> LargeAppBar<'a> {
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
            leading_button: None,
            trailing_buttons: &[],
            show_background: false,
        }
    }

    pub fn show(self, ui: &mut Ui) -> AppBarResponse {
        let metrics = material_style_metrics();
        BaseAppBar {
            title: self.title,
            leading_button: self.leading_button,
            show_background: self.show_background,
            trailing_buttons: self.trailing_buttons,
            horizontal_alignment: egui::Align::Min,
            spacing: metrics.spacings.spacing_4,
            vertical_spacing: metrics.spacings.spacing_40,
            two_rows: true,
            text_style: MATERIAL_TYPOGRAPHY.headline_medium,
            text_padding: metrics.paddings.padding_16,
            appbar_padding_right: 0.0,
            appbar_padding_top: metrics.paddings.padding_8,
            appbar_padding_bottom: metrics.paddings.padding_28,
        }
        .show(ui)
    }
}

#[must_use]
pub fn app_bar_min_size() -> Vec2 {
    let metrics = material_style_metrics();
    vec2(0.0, metrics.sizes.size_64)
}

#[must_use]
fn estimated_trailing_width(button_count: usize, button_width: f32, spacing: f32) -> f32 {
    if button_count == 0 {
        return 0.0;
    }
    let gaps = button_count.saturating_sub(1) as f32 * spacing;
    button_count as f32 * button_width + gaps
}

#[must_use]
fn title_lane_layout(alignment: egui::Align) -> egui::Layout {
    match alignment {
        egui::Align::Min => egui::Layout::left_to_right(egui::Align::Center),
        egui::Align::Center => {
            egui::Layout::centered_and_justified(egui::Direction::LeftToRight)
        }
        egui::Align::Max => egui::Layout::right_to_left(egui::Align::Center),
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::AppBar;

    #[test]
    fn app_bar_renders_without_panicking() {
        let context = Context::default();
        let mut trailing_count = 0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = AppBar::new("Title").show(ui);
                trailing_count = response.trailing.len();
            });
        });
        assert_eq!(trailing_count, 0);
    }

    #[test]
    fn estimated_trailing_width_counts_buttons_and_gaps() {
        assert_eq!(super::estimated_trailing_width(0, 40.0, 6.0), 0.0);
        assert_eq!(super::estimated_trailing_width(1, 40.0, 6.0), 40.0);
        assert_eq!(super::estimated_trailing_width(3, 40.0, 6.0), 132.0);
    }
}
