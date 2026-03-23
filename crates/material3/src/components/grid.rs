use std::hash::Hash;

use egui::InnerResponse;
use egui::Margin;
use egui::Ui;
use egui::vec2;

use crate::styling::material_style_metrics::material_style_metrics;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Grid {
    pub padding: f32,
    pub spacing_horizontal: f32,
    pub spacing_vertical: f32,
    pub striped: bool,
}

impl Grid {
    #[must_use]
    pub fn new() -> Self {
        let metrics = material_style_metrics();
        Self {
            padding: metrics.paddings.padding_8,
            spacing_horizontal: metrics.spacings.spacing_8,
            spacing_vertical: metrics.spacings.spacing_8,
            striped: false,
        }
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        id_source: impl Hash,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        egui::Frame::new()
            .inner_margin(Margin::same(self.padding.round() as i8))
            .show(ui, |ui| {
                egui::Grid::new(id_source)
                    .spacing(vec2(self.spacing_horizontal, self.spacing_vertical))
                    .striped(self.striped)
                    .show(ui, add_contents)
                    .inner
            })
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

pub fn grid<R>(
    ui: &mut Ui,
    id_source: impl Hash,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    Grid::new().show(ui, id_source, add_contents)
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::Grid;

    #[test]
    fn grid_defaults_match_slint_metrics() {
        let grid = Grid::new();
        assert_eq!(grid.padding, 8.0);
        assert_eq!(grid.spacing_horizontal, 8.0);
        assert_eq!(grid.spacing_vertical, 8.0);
        assert!(!grid.striped);
    }

    #[test]
    fn grid_renders_without_panicking() {
        let context = Context::default();
        let mut min_height = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = Grid::new().show(ui, "grid_test", |ui| {
                    ui.label("A");
                    ui.label("B");
                    ui.end_row();
                });
                min_height = response.response.rect.height();
            });
        });
        assert!(min_height > 0.0);
    }
}
