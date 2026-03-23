use std::borrow::Cow;

use egui::Align;
use egui::Align2;
use egui::Id;
use egui::Image;
use egui::Layout;
use egui::Order;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;
use egui::vec2;

use crate::components::FilledButton;
use crate::components::MaterialIconButton;
use crate::components::TextButton;
use crate::components::icon::MaterialIconStyle;
use crate::components::icon::icon_with_style;
use crate::components::material_text;
use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;
use crate::styling::material_typography::MATERIAL_TYPOGRAPHY;

pub struct DialogResponse<R> {
    pub inner: R,
    pub response: Response,
    pub action_clicked: Option<usize>,
    pub action_button_clicked: Option<usize>,
    pub default_action_clicked: bool,
    pub close_requested: bool,
}

pub struct BaseDialog<'a> {
    pub title: Cow<'a, str>,
    pub icon: Option<Image<'static>>,
    pub action_button_icons: Vec<Image<'static>>,
    pub default_action_text: Cow<'a, str>,
    pub actions: Vec<Cow<'a, str>>,
}

impl<'a> BaseDialog<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: Cow::Borrowed(""),
            icon: None,
            action_button_icons: Vec::new(),
            default_action_text: Cow::Borrowed(""),
            actions: Vec::new(),
        }
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> DialogResponse<R> {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let overlay_rect = ui.max_rect();
        let modal_response = ui.allocate_rect(overlay_rect, Sense::click());
        ui.painter()
            .rect_filled(overlay_rect, 0.0, palette.background_modal);

        let mut action_clicked = None;
        let mut action_button_clicked = None;
        let mut default_action_clicked = false;
        let mut close_requested = ui.input(|input| input.key_pressed(egui::Key::Escape));
        if !self.default_action_text.is_empty()
            && ui.input(|input| input.key_pressed(egui::Key::Enter))
        {
            default_action_clicked = true;
        }

        let area = egui::Area::new(Id::new(("material_base_dialog", self.title.as_ref())))
            .order(Order::Foreground)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .show(ui.ctx(), |ui| {
                egui::Frame::new()
                    .fill(palette.surface_container_high)
                    .corner_radius(egui::CornerRadius::same(
                        metrics.corner_radii.border_radius_28.round() as u8,
                    ))
                    .inner_margin(egui::Margin::same(metrics.paddings.padding_24.round() as i8))
                    .show(ui, |ui| {
                        ui.set_max_width(metrics.sizes.size_572);
                        ui.spacing_mut().item_spacing.y = metrics.spacings.spacing_16;

                        if let Some(icon) = self.icon {
                            ui.vertical_centered(|ui| {
                                let _ = ui.add(icon_with_style(
                                    icon,
                                    MaterialIconStyle {
                                        size: vec2(
                                            metrics.icon_sizes.icon_size_24,
                                            metrics.icon_sizes.icon_size_24,
                                        ),
                                        tint: palette.on_surface,
                                    },
                                ));
                            });
                        }

                        if !self.title.is_empty() {
                            let _ = material_text(ui, self.title)
                                .text_style(MATERIAL_TYPOGRAPHY.headline_small)
                                .color(palette.on_surface)
                                .show(ui);
                        }

                        let inner = add_contents(ui);

                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            ui.spacing_mut().item_spacing.x = metrics.spacings.spacing_8;

                            for (index, icon) in self.action_button_icons.into_iter().enumerate() {
                                let response = MaterialIconButton {
                                    icon,
                                    tooltip: Cow::Borrowed(""),
                                    ..MaterialIconButton::new(Image::new(egui::include_image!(
                                        "../../assets/icons/Close.svg"
                                    )))
                                }
                                .show(ui);
                                if response.response.clicked() {
                                    action_button_clicked = Some(index);
                                }
                            }

                            ui.add_space(0.0);
                            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                                if !self.default_action_text.is_empty() {
                                    let response =
                                        FilledButton::new(self.default_action_text).show(ui);
                                    if response.clicked() {
                                        default_action_clicked = true;
                                    }
                                }

                                for (index, action) in self.actions.into_iter().enumerate().rev() {
                                    let response = TextButton::new(action).show(ui);
                                    if response.clicked() {
                                        action_clicked = Some(index);
                                    }
                                }
                            });
                        });

                        inner
                    })
            });

        let card_rect = area.response.rect;
        if modal_response.clicked()
            && let Some(pointer_pos) = modal_response.interact_pointer_pos()
            && !card_rect.contains(pointer_pos)
        {
            close_requested = true;
        }

        DialogResponse {
            inner: area.inner.inner,
            response: area.response,
            action_clicked,
            action_button_clicked,
            default_action_clicked,
            close_requested,
        }
    }
}

impl Default for BaseDialog<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Dialog<'a> {
    pub base: BaseDialog<'a>,
}

impl<'a> Dialog<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            base: BaseDialog::new(),
        }
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> DialogResponse<R> {
        self.base.show(ui, add_contents)
    }
}

impl Default for Dialog<'_> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FullscreenDialog<'a> {
    pub title: Cow<'a, str>,
    pub actions: Vec<Cow<'a, str>>,
}

impl<'a> FullscreenDialog<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            title: Cow::Borrowed(""),
            actions: Vec::new(),
        }
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> DialogResponse<R> {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let rect = ui.max_rect();
        let response = ui.allocate_rect(rect, Sense::click());
        ui.painter().rect_filled(rect, 0.0, palette.surface);

        let mut action_clicked = None;
        let close_requested = ui.input(|input| input.key_pressed(egui::Key::Escape));

        let inner = ui
            .scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = metrics.spacings.spacing_16;
                    ui.horizontal(|ui| {
                        ui.add_space(metrics.paddings.padding_24);
                        let close = MaterialIconButton::new(Image::new(egui::include_image!(
                            "../../assets/icons/Close.svg"
                        )))
                        .show(ui);
                        let _ = close;

                        let _ = material_text(ui, self.title)
                            .text_style(MATERIAL_TYPOGRAPHY.headline_small)
                            .color(palette.on_surface)
                            .show(ui);

                        for (index, action) in self.actions.into_iter().enumerate() {
                            let action_response = TextButton::new(action).show(ui);
                            if action_response.clicked() {
                                action_clicked = Some(index);
                            }
                        }
                    });

                    add_contents(ui)
                })
                .inner
            })
            .inner;

        DialogResponse {
            inner,
            response,
            action_clicked,
            action_button_clicked: None,
            default_action_clicked: false,
            close_requested,
        }
    }
}

impl Default for FullscreenDialog<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;
    use egui::Image;

    use super::BaseDialog;
    use super::Dialog;
    use super::FullscreenDialog;

    #[test]
    fn base_dialog_renders_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = BaseDialog {
                    title: "Dialog".into(),
                    icon: Some(Image::new(egui::include_image!(
                        "../../assets/icons/Home.svg"
                    ))),
                    default_action_text: "Ok".into(),
                    actions: vec!["Cancel".into()],
                    ..BaseDialog::new()
                }
                .show(ui, |ui| ui.label("Body"));
                width = response.response.rect.width();
            });
        });
        assert!(width > 0.0);
    }

    #[test]
    fn dialog_wrapper_renders_without_panicking() {
        let context = Context::default();
        let mut height = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = Dialog::new().show(ui, |ui| ui.label("Body"));
                height = response.response.rect.height();
            });
        });
        assert!(height > 0.0);
    }

    #[test]
    fn fullscreen_dialog_renders_without_panicking() {
        let context = Context::default();
        let mut height = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = FullscreenDialog {
                    title: "Title".into(),
                    actions: vec!["Save".into()],
                }
                .show(ui, |ui| ui.label("Body"));
                height = response.response.rect.height();
            });
        });
        assert!(height > 0.0);
    }
}
