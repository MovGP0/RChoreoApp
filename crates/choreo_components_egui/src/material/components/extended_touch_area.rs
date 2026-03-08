use std::borrow::Cow;

use egui::Key;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::vec2;

use crate::material::components::ToolTip;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtendedTouchAreaKeyResult {
    Accept,
    Reject,
}

pub struct ExtendedTouchAreaResponse {
    pub response: Response,
    pub has_focus: bool,
    pub activated: bool,
    pub enter_pressed: bool,
    pub key_result: ExtendedTouchAreaKeyResult,
}

pub struct ExtendedTouchArea<'a> {
    pub enabled: bool,
    pub tooltip: Cow<'a, str>,
    pub tooltip_offset: f32,
    pub sense: Sense,
}

impl<'a> ExtendedTouchArea<'a> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            enabled: true,
            tooltip: Cow::Borrowed(""),
            tooltip_offset: 0.0,
            sense: Sense::click(),
        }
    }

    pub fn show(
        self,
        ui: &mut Ui,
        add_children: impl FnOnce(&mut Ui),
    ) -> ExtendedTouchAreaResponse {
        let inner = ui.scope(add_children);
        let mut response = ui.interact(inner.response.rect, inner.response.id, self.sense);
        let has_focus = response.has_focus();
        let activated = self.enabled
            && has_focus
            && ui.input(|input| {
                input.key_pressed(Key::Enter) || input.key_pressed(Key::Space)
            });
        if self.enabled
            && !self.tooltip.is_empty()
            && response.hovered()
            && !response.is_pointer_button_down_on()
        {
            let anchor = response.rect.left_bottom() + vec2(0.0, self.tooltip_offset);
            let _ = ToolTip::new(self.tooltip.clone()).show(ui, anchor);
        }
        if activated {
            response.mark_changed();
        }
        let key_result = if activated {
            ExtendedTouchAreaKeyResult::Accept
        } else {
            ExtendedTouchAreaKeyResult::Reject
        };
        ExtendedTouchAreaResponse {
            response,
            has_focus,
            activated,
            enter_pressed: activated,
            key_result,
        }
    }
}

impl Default for ExtendedTouchArea<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::ExtendedTouchArea;

    #[test]
    fn extended_touch_area_wraps_child_rect() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let outcome = ExtendedTouchArea::new().show(ui, |ui| {
                    ui.add_sized([96.0, 24.0], egui::Label::new("target"));
                });
                width = outcome.response.rect.width();
            });
        });
        assert!(width >= 96.0);
    }
}
