use egui::Context;

use crate::choreo_main::MainPageBinding;
use crate::choreo_main::MainPageDependencies;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::ui;

pub struct AppShellViewModel {
    title: String,
    is_initialized: bool,
    main_page_binding: Option<MainPageBinding>,
}

impl Default for AppShellViewModel {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl AppShellViewModel {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            is_initialized: false,
            main_page_binding: Some(MainPageBinding::new(MainPageDependencies::default())),
        }
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn ui(&mut self, context: &Context) {
        if !self.is_initialized {
            if let Some(binding) = self.main_page_binding.as_ref() {
                binding.dispatch(ChoreoMainAction::Initialize);
            }
            self.is_initialized = true;
        }

        egui::CentralPanel::default().show(context, |ui| {
            if let Some(binding) = self.main_page_binding.as_ref() {
                let state = {
                    let view_model = binding.view_model();
                    view_model.borrow().state().clone()
                };
                for action in ui::draw(ui, &state) {
                    binding.dispatch(action);
                }
            }
        });
    }
}
