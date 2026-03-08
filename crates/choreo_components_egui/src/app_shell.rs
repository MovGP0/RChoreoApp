use egui::Context;
use std::time::Duration;

use crate::choreo_main::MainPageBinding;
use crate::choreo_main::MainPageDependencies;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::ui;
use crate::splash_screen_host;
use crate::ui_style::typography;

pub struct AppShellViewModel {
    title: String,
    is_initialized: bool,
    is_typography_initialized: bool,
    show_splash_screen: bool,
    main_page_binding: Option<MainPageBinding>,
    splash_screen_state: splash_screen_host::ui::SplashScreenUiState,
}

impl Default for AppShellViewModel {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl AppShellViewModel {
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self::new_with_dependencies(title, MainPageDependencies::default())
    }

    #[must_use]
    pub fn new_with_dependencies(
        title: impl Into<String>,
        main_page_dependencies: MainPageDependencies,
    ) -> Self {
        Self {
            title: title.into(),
            is_initialized: false,
            is_typography_initialized: false,
            show_splash_screen: true,
            main_page_binding: Some(MainPageBinding::new(main_page_dependencies)),
            splash_screen_state: splash_screen_host::ui::SplashScreenUiState::default(),
        }
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn ui(&mut self, context: &Context) {
        if !self.is_typography_initialized {
            typography::apply_to_context(context);
            self.is_typography_initialized = true;
        }

        if !self.is_initialized {
            if let Some(binding) = self.main_page_binding.as_ref() {
                binding.dispatch(ChoreoMainAction::Initialize);
            }
            self.is_initialized = true;
        }

        if self.show_splash_screen {
            egui::CentralPanel::default().show(context, |ui| {
                splash_screen_host::ui::draw(ui, &self.splash_screen_state);
            });
            self.show_splash_screen = false;
            context.request_repaint();
            return;
        }

        let mut request_audio_repaint = false;
        if let Some(binding) = self.main_page_binding.as_ref() {
            request_audio_repaint = binding.tick_audio_runtime();
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

        if let Some(binding) = self.main_page_binding.as_ref() {
            request_audio_repaint |= binding.audio_runtime_is_active();
        }
        if request_audio_repaint {
            context.request_repaint_after(Duration::from_millis(16));
        }
    }

    pub fn route_external_file_path(&self, file_path: &str) {
        if let Some(binding) = self.main_page_binding.as_ref() {
            binding.route_external_file_path(file_path);
        }
    }
}

#[cfg(test)]
mod app_shell_splash_spec {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::AppShellViewModel;
    use crate::choreo_main::MainPageActionHandlers;
    use crate::choreo_main::MainPageDependencies;
    use crate::choreo_main::actions::ChoreoMainAction;
    use crate::choreo_main::actions::OpenChoreoRequested;

    #[test]
    fn first_frame_shows_splash_then_hands_off_to_main_ui() {
        let context = egui::Context::default();
        let mut shell = AppShellViewModel::new("ChoreoApp");

        let _ = context.run(egui::RawInput::default(), |ctx| {
            shell.ui(ctx);
        });

        assert!(shell.is_initialized);
        assert!(shell.is_typography_initialized);
        assert!(!shell.show_splash_screen);

        let _ = context.run(egui::RawInput::default(), |ctx| {
            shell.ui(ctx);
        });

        assert!(!shell.show_splash_screen);
    }

    #[test]
    fn injected_main_page_dependencies_route_open_choreo_requests() {
        let routed_requests: Rc<RefCell<Vec<OpenChoreoRequested>>> =
            Rc::new(RefCell::new(Vec::new()));
        let routed_requests_for_handler = Rc::clone(&routed_requests);
        let shell = AppShellViewModel::new_with_dependencies(
            "ChoreoApp",
            MainPageDependencies {
                action_handlers: MainPageActionHandlers {
                    request_open_choreo: Some(Rc::new(move |request| {
                        routed_requests_for_handler.borrow_mut().push(request);
                    })),
                    ..MainPageActionHandlers::default()
                },
                ..MainPageDependencies::default()
            },
        );

        let binding = shell
            .main_page_binding
            .as_ref()
            .expect("shell should create a main page binding");
        binding.dispatch(ChoreoMainAction::RequestOpenChoreo(OpenChoreoRequested {
            file_path: Some("C:/demo.choreo".to_string()),
            file_name: Some("demo.choreo".to_string()),
            contents: "demo".to_string(),
        }));

        let routed_requests = routed_requests.borrow();
        assert_eq!(routed_requests.len(), 1);
        assert_eq!(routed_requests[0].file_name.as_deref(), Some("demo.choreo"));
    }
}
