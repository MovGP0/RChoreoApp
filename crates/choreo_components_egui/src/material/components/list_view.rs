use egui::Ui;

use crate::material::components::scroll_view::ScrollView;
use crate::material::components::scroll_view::ScrollViewResponse;
use crate::material::components::scroll_view::ScrollViewState;

pub struct ListView<'a> {
    state: &'a mut ScrollViewState,
}

impl<'a> ListView<'a> {
    #[must_use]
    pub fn new(state: &'a mut ScrollViewState) -> Self {
        Self { state }
    }

    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> ScrollViewResponse<R> {
        ScrollView::new(self.state).show(ui, add_contents)
    }
}

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::ListView;
    use crate::material::components::scroll_view::ScrollViewState;

    #[test]
    fn list_view_reuses_scroll_view_shell() {
        let context = Context::default();
        let mut state = ScrollViewState::default();
        let mut viewport_height = 0.0;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = ListView::new(&mut state).show(ui, |ui| {
                    ui.add_sized([200.0, 120.0], egui::Label::new("row"));
                });
                viewport_height = response.viewport_rect.height();
            });
        });
        assert!(viewport_height > 0.0);
    }
}
