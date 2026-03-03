use egui::Context;
use egui_material3::MaterialButton;

use crate::shell;

#[derive(Debug, Clone, Default)]
pub struct AppShellViewModel {
    title: String,
}

impl AppShellViewModel {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn ui(&mut self, context: &Context) {
        egui::CentralPanel::default().show(context, |ui| {
            ui.heading(self.title());
            ui.label(shell::app_title());
            ui.add(MaterialButton::new("Material 3 Action"));
        });
    }
}
