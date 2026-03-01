use egui::Context;
use egui_material3::MaterialButton;

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

    pub fn ui(&self, context: &Context) {
        egui::CentralPanel::default().show(context, |ui| {
            ui.heading(&self.title);
            ui.label("egui-material3 component integration target.");
            ui.add(MaterialButton::new("Material 3 Action"));
        });
    }
}
