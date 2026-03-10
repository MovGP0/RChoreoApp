use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::UiBuilder;

use crate::material::styling::material_palette::material_palette_for_visuals;

pub struct ModalResponse<R> {
    pub inner: R,
    pub response: Response,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modal;

impl Modal {
    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> ModalResponse<R> {
        let palette = material_palette_for_visuals(ui.visuals());
        let rect = ui.max_rect();
        let response = ui.interact(rect, ui.id().with("material_modal"), Sense::click());
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::ZERO, palette.background_modal);
        let inner = ui
            .scope_builder(UiBuilder::new().max_rect(rect), add_contents)
            .inner;
        ModalResponse { inner, response }
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::Modal;

    #[test]
    fn modal_renders_overlay_without_panicking() {
        let context = Context::default();
        let mut width = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = Modal.show(ui, |ui| {
                    ui.label("Dialog content");
                });
                width = response.response.rect.width();
            });
        });
        assert!(width > 0.0);
    }
}
