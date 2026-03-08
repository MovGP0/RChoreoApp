use std::borrow::Cow;

use egui::Color32;
use egui::Image;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::vec2;

use crate::material::components::ListTile;
use crate::material::components::StateLayerStyle;
use crate::material::components::apply_tooltip;
use crate::material::components::paint_state_layer_for_response;
use crate::material::components::icon::icon_with_style;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckState {
    Unchecked,
    PartiallyChecked,
    Checked,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckBox {
    pub check_state: CheckState,
    pub tristate: bool,
    pub has_error: bool,
    pub enabled: bool,
    pub tooltip: String,
}

impl CheckBox {
    #[must_use]
    pub fn checked(&self) -> bool {
        matches!(
            self.check_state,
            CheckState::Checked | CheckState::PartiallyChecked
        )
    }

    pub fn toggle(&mut self) {
        if !self.tristate {
            self.check_state = if self.check_state != CheckState::Checked {
                CheckState::Checked
            } else {
                CheckState::Unchecked
            };
            return;
        }

        self.check_state = match self.check_state {
            CheckState::Checked => CheckState::PartiallyChecked,
            CheckState::PartiallyChecked => CheckState::Unchecked,
            CheckState::Unchecked => CheckState::Checked,
        };
    }

    pub fn set_check_state(&mut self, check_state: CheckState) {
        if check_state == CheckState::PartiallyChecked {
            self.tristate = true;
        }
        self.check_state = check_state;
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let (rect, response) = ui.allocate_exact_size(
            vec2(metrics.sizes.size_48, metrics.sizes.size_48),
            Sense::click(),
        );

        if response.clicked() && self.enabled {
            self.toggle();
        }

        let mut state_style = StateLayerStyle::for_ui(ui);
        state_style.color = palette.on_surface;
        state_style.border_radius = rect.width().max(rect.height()) * 0.5;
        state_style.enabled = self.enabled;
        state_style.display_background = self.enabled;
        state_style.tooltip = self.tooltip.as_str();
        paint_state_layer_for_response(ui, &response, state_style);
        let response = apply_tooltip(response, state_style);

        let box_size = vec2(metrics.sizes.size_18, metrics.sizes.size_18);
        let box_rect = egui::Rect::from_center_size(rect.center(), box_size);
        let is_checked = self.checked();
        let border_color = if self.has_error {
            palette.error
        } else if self.enabled {
            palette.on_surface_variant
        } else {
            palette.on_surface.gamma_multiply(0.38)
        };
        let fill_color = if !self.enabled {
            if is_checked {
                palette.on_surface.gamma_multiply(0.38)
            } else {
                Color32::TRANSPARENT
            }
        } else if is_checked {
            if self.has_error {
                palette.error
            } else {
                palette.primary
            }
        } else {
            Color32::TRANSPARENT
        };

        ui.painter().rect(
            box_rect,
            egui::CornerRadius::same(metrics.corner_radii.border_radius_2.round() as u8),
            fill_color,
            egui::Stroke::new(if is_checked { 0.0 } else { 2.0 }, border_color),
            egui::StrokeKind::Middle,
        );

        if is_checked {
            let image = check_state_icon(self.check_state);
            let image = icon_with_style(
                image,
                crate::material::components::icon::MaterialIconStyle {
                    size: box_size,
                    tint: palette.on_primary,
                },
            );
            ui.put(box_rect, image);
        }

        response
    }
}

impl Default for CheckBox {
    fn default() -> Self {
        Self {
            check_state: CheckState::Unchecked,
            tristate: false,
            has_error: false,
            enabled: true,
            tooltip: String::new(),
        }
    }
}

pub struct CheckBoxTile<'a> {
    pub text: Cow<'a, str>,
    pub supporting_text: Cow<'a, str>,
    pub check_box: CheckBox,
}

impl<'a> CheckBoxTile<'a> {
    #[must_use]
    pub fn new(text: impl Into<Cow<'a, str>>) -> Self {
        Self {
            text: text.into(),
            supporting_text: Cow::Borrowed(""),
            check_box: CheckBox::default(),
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> Response {
        let text = self.text.clone();
        let supporting_text = self.supporting_text.clone();
        let check_box = &mut self.check_box;
        let response = ListTile {
            text,
            supporting_text,
            avatar_icon: None,
            avatar_text: Cow::Borrowed(""),
            avatar_background: None,
            avatar_foreground: None,
            enabled: check_box.enabled,
            color: None,
            tooltip: Cow::Borrowed(""),
        }
        .show(ui, |ui| {
            let _ = check_box.show(ui);
        });
        if response.clicked() && check_box.enabled {
            check_box.toggle();
        }
        response
    }
}

fn check_state_icon(check_state: CheckState) -> Image<'static> {
    match check_state {
        CheckState::Unchecked | CheckState::Checked => {
            Image::new(egui::include_image!("../../../assets/icons/Check.svg"))
        }
        CheckState::PartiallyChecked => {
            Image::new(egui::include_image!("../../../assets/icons/Minus.svg"))
        }
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::CheckBox;
    use super::CheckBoxTile;
    use super::CheckState;

    #[test]
    fn toggle_cycles_through_tristate_values() {
        let mut check_box = CheckBox {
            tristate: true,
            ..CheckBox::default()
        };
        check_box.toggle();
        assert_eq!(check_box.check_state, CheckState::Checked);
        check_box.toggle();
        assert_eq!(check_box.check_state, CheckState::PartiallyChecked);
        check_box.toggle();
        assert_eq!(check_box.check_state, CheckState::Unchecked);
    }

    #[test]
    fn check_box_tile_renders_without_panicking() {
        let context = Context::default();
        let mut positive = false;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut tile = CheckBoxTile::new("Option");
                let response = tile.show(ui);
                positive = response.rect.height() > 0.0;
            });
        });
        assert!(positive);
    }
}
