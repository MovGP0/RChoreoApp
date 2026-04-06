use egui::Context;
use egui::Frame;
use egui::Margin;
use std::time::Duration;

use crate::choreo_main::MainPageBinding;
use crate::choreo_main::MainPageDependencies;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::ui;
use crate::material::styling::material_palette::apply_material_visuals;
use crate::material::styling::material_palette::material_palette_for_theme;
use crate::material::styling::material_palette::with_current_material_palette;
use crate::material::styling::material_typography as typography;
use crate::splash_screen_host;

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
        Self::new_with_dependencies(title, crate::shell::default_main_page_dependencies())
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
            if let Some(binding) = self.main_page_binding.as_ref() {
                let state = {
                    let state = binding.state();
                    state.borrow().clone()
                };
                apply_material_visuals(
                    context,
                    &state.settings_state.material_scheme,
                    state.settings_state.theme_mode,
                );
            }
            egui::CentralPanel::default()
                .frame(root_panel_frame(context))
                .show(context, |ui| {
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

        if let Some(binding) = self.main_page_binding.as_ref() {
            let state = {
                let state = binding.state();
                state.borrow().clone()
            };
            apply_material_visuals(
                context,
                &state.settings_state.material_scheme,
                state.settings_state.theme_mode,
            );
            let palette = material_palette_for_theme(
                &state.settings_state.material_scheme,
                state.settings_state.theme_mode,
            );

            egui::CentralPanel::default()
                .frame(root_panel_frame(context))
                .show(context, |ui| {
                    with_current_material_palette(palette, || {
                        for action in ui::draw(ui, &state) {
                            binding.dispatch(action);
                        }
                    });
                });
        }

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

fn root_panel_frame(context: &Context) -> Frame {
    Frame::new()
        .fill(context.style().visuals.panel_fill)
        .inner_margin(Margin::same(0))
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
    use crate::settings::actions::SettingsAction;

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

    #[test]
    fn ui_applies_dynamic_material_visuals_from_settings_state() {
        let context = egui::Context::default();
        let mut shell = AppShellViewModel::new("ChoreoApp");
        let binding = shell
            .main_page_binding
            .as_ref()
            .expect("shell should create a main page binding");

        binding.dispatch(ChoreoMainAction::SettingsAction(
            SettingsAction::UpdateUseSystemTheme { enabled: false },
        ));
        binding.dispatch(ChoreoMainAction::SettingsAction(
            SettingsAction::UpdateUsePrimaryColor { enabled: true },
        ));
        binding.dispatch(ChoreoMainAction::SettingsAction(
            SettingsAction::UpdatePrimaryColorHex {
                value: "#FF336699".to_string(),
            },
        ));
        binding.dispatch(ChoreoMainAction::SettingsAction(
            SettingsAction::UpdateIsDarkMode { enabled: true },
        ));

        let expected = {
            let state = binding.state();
            let state = state.borrow();
            crate::material::styling::material_palette::material_palette_for_theme(
                &state.settings_state.material_scheme,
                state.settings_state.theme_mode,
            )
        };

        let _ = context.run(egui::RawInput::default(), |ctx| {
            shell.ui(ctx);
        });

        let visuals = context.style().visuals.clone();
        assert!(visuals.dark_mode);
        assert_eq!(visuals.panel_fill, expected.background);
        assert_eq!(visuals.selection.bg_fill, expected.secondary_container);
        assert_eq!(
            egui_material3::get_global_color("primary"),
            expected.primary
        );
        assert_eq!(
            egui_material3::get_global_color("onSurface"),
            expected.on_surface
        );
    }
}
