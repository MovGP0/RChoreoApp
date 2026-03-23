use egui::InnerResponse;
use egui::Margin;
use egui::Ui;

use crate::styling::material_style_metrics::material_style_metrics;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertical {
    pub padding: f32,
    pub spacing: f32,
    pub horizontal_align: egui::Align,
}

impl Vertical {
    #[must_use]
    pub fn new() -> Self {
        let metrics = material_style_metrics();
        Self {
            padding: metrics.paddings.padding_8,
            spacing: metrics.spacings.spacing_8,
            horizontal_align: egui::Align::Min,
        }
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        egui::Frame::new()
            .inner_margin(Margin::same(self.padding.round() as i8))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = self.spacing;
                    ui.with_layout(egui::Layout::top_down(self.horizontal_align), add_contents)
                        .inner
                })
                .inner
            })
    }
}

impl Default for Vertical {
    fn default() -> Self {
        Self::new()
    }
}

pub fn vertical<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    Vertical::new().show(ui, add_contents)
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::Vertical;

    #[test]
    fn vertical_defaults_match_slint_metrics() {
        let vertical = Vertical::new();
        assert_eq!(vertical.padding, 8.0);
        assert_eq!(vertical.spacing, 8.0);
        assert_eq!(vertical.horizontal_align, egui::Align::Min);
    }

    #[test]
    fn vertical_renders_without_panicking() {
        let context = Context::default();
        let mut min_height = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = Vertical::new().show(ui, |ui| {
                    ui.label("A");
                    ui.label("B");
                });
                min_height = response.response.rect.height();
            });
        });
        assert!(min_height > 0.0);
    }
}
