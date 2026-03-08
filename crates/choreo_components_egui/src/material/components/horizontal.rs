use egui::InnerResponse;
use egui::Margin;
use egui::Ui;

use crate::material::styling::material_style_metrics::material_style_metrics;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Horizontal {
    pub padding: f32,
    pub spacing: f32,
    pub vertical_align: egui::Align,
}

impl Horizontal {
    #[must_use]
    pub fn new() -> Self {
        let metrics = material_style_metrics();
        Self {
            padding: metrics.paddings.padding_8,
            spacing: metrics.spacings.spacing_8,
            vertical_align: egui::Align::Center,
        }
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        egui::Frame::new()
            .inner_margin(Margin::same(self.padding.round() as i8))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = self.spacing;
                    ui.with_layout(
                        egui::Layout::left_to_right(self.vertical_align),
                        add_contents,
                    )
                    .inner
                })
                .inner
            })
    }
}

impl Default for Horizontal {
    fn default() -> Self {
        Self::new()
    }
}

pub fn horizontal<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    Horizontal::new().show(ui, add_contents)
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::Horizontal;

    #[test]
    fn horizontal_defaults_match_slint_metrics() {
        let horizontal = Horizontal::new();
        assert_eq!(horizontal.padding, 8.0);
        assert_eq!(horizontal.spacing, 8.0);
        assert_eq!(horizontal.vertical_align, egui::Align::Center);
    }

    #[test]
    fn horizontal_renders_without_panicking() {
        let context = Context::default();
        let mut min_width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = Horizontal::new().show(ui, |ui| {
                    ui.label("A");
                    ui.label("B");
                });
                min_width = response.response.rect.width();
            });
        });
        assert!(min_width > 0.0);
    }
}
