use egui::Context;
use egui_material3::MaterialButton;

use crate::shell::reducer::reduce;
use crate::shell::state::ShellState;
use crate::shell::ui::draw;

#[derive(Debug, Clone, Default)]
pub struct AppShellViewModel {
    state: ShellState,
}

impl AppShellViewModel {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            state: ShellState {
                app_title: title.into(),
                ..ShellState::default()
            },
        }
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.state.app_title
    }

    pub fn ui(&mut self, context: &Context) {
        egui::CentralPanel::default().show(context, |ui| {
            let actions = draw(ui, &self.state);
            for action in actions {
                reduce(&mut self.state, action);
            }
            ui.add(MaterialButton::new("Material 3 Action"));
        });
    }
}
