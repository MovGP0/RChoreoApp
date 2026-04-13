use egui::Context;
use std::time::Duration;

use crate::choreo_main::MainPageDependencies;

pub mod actions;
pub mod effects;
pub mod reducer;
pub mod runtime;
pub mod state;
pub mod ui;

pub use state::AppShellState;

use actions::AppShellAction;
use reducer::reduce;
use runtime::AppShellRuntime;

pub struct AppShellStore {
    state: AppShellState,
    runtime: AppShellRuntime,
}

impl Default for AppShellStore {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl AppShellStore {
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
            state: AppShellState::new(title),
            runtime: AppShellRuntime::new(main_page_dependencies),
        }
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.state.title
    }

    #[must_use]
    pub fn state(&self) -> &AppShellState {
        &self.state
    }

    pub fn ui(&mut self, context: &Context) {
        self.dispatch_with_context(AppShellAction::FrameStarted, context);

        if self.state.show_splash_screen {
            let main_page_state = self.runtime.snapshot_main_page_state();
            self.runtime
                .apply_main_page_theme(context, &main_page_state);
            ui::draw_splash(context, &self.state, &main_page_state);
            self.dispatch_with_context(AppShellAction::SplashPresented, context);
            return;
        }

        let mut request_audio_repaint = self.runtime.tick_audio_runtime();
        let main_page_state = self.runtime.snapshot_main_page_state();
        self.runtime
            .apply_main_page_theme(context, &main_page_state);
        let actions = ui::draw_main_page(context, &main_page_state);
        self.runtime.dispatch_main_page_actions(actions);

        request_audio_repaint |= self.runtime.audio_runtime_is_active();
        if request_audio_repaint {
            context.request_repaint_after(Duration::from_millis(16));
        }
    }

    pub fn route_external_file_path(&mut self, file_path: &str) {
        self.dispatch_without_context(AppShellAction::ExternalFilePathReceived {
            file_path: file_path.to_string(),
        });
    }

    fn dispatch_with_context(&mut self, action: AppShellAction, context: &Context) {
        let effects = reduce(&mut self.state, action);
        self.runtime.apply_effects(Some(context), effects);
    }

    fn dispatch_without_context(&mut self, action: AppShellAction) {
        let effects = reduce(&mut self.state, action);
        self.runtime.apply_effects(None, effects);
    }
}

#[cfg(test)]
mod app_shell_store_spec {
    use std::cell::RefCell;
    use std::fs;
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use super::AppShellStore;
    use crate::choreo_main::MainPageActionHandlers;
    use crate::choreo_main::MainPageDependencies;
    use crate::choreo_main::actions::OpenChoreoRequested;
    use crate::settings::actions::SettingsAction;

    #[test]
    fn first_frame_shows_splash_then_hands_off_to_main_ui() {
        let context = egui::Context::default();
        let mut shell = AppShellStore::new("ChoreoApp");

        let _ = context.run(egui::RawInput::default(), |ctx| {
            shell.ui(ctx);
        });

        assert!(shell.state().is_main_page_initialized);
        assert!(shell.state().is_typography_initialized);
        assert!(!shell.state().show_splash_screen);

        let _ = context.run(egui::RawInput::default(), |ctx| {
            shell.ui(ctx);
        });

        assert!(!shell.state().show_splash_screen);
    }

    #[test]
    fn injected_main_page_dependencies_route_open_choreo_requests() {
        let routed_requests: Rc<RefCell<Vec<OpenChoreoRequested>>> =
            Rc::new(RefCell::new(Vec::new()));
        let routed_requests_for_handler = Rc::clone(&routed_requests);
        let mut shell = AppShellStore::new_with_dependencies(
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
        let path = unique_temp_file("choreo");
        fs::write(&path, "demo").expect("test should write .choreo file");

        shell.route_external_file_path(path.to_string_lossy().as_ref());

        let routed_requests = routed_requests.borrow();
        assert_eq!(routed_requests.len(), 1);
        assert_eq!(
            routed_requests[0].file_name.as_deref(),
            path.file_name().and_then(|value| value.to_str())
        );

        let _ = fs::remove_file(path);
    }

    #[test]
    fn ui_syncs_dynamic_material_theme_from_settings_state() {
        let context = egui::Context::default();
        let mut shell = AppShellStore::new("ChoreoApp");

        let _ = context.run(egui::RawInput::default(), |ctx| {
            shell.ui(ctx);
        });

        shell.runtime.dispatch_main_page_actions(vec![
            crate::choreo_main::actions::ChoreoMainAction::SettingsAction(
                SettingsAction::UpdateUseSystemTheme { enabled: false },
            ),
            crate::choreo_main::actions::ChoreoMainAction::SettingsAction(
                SettingsAction::UpdateUsePrimaryColor { enabled: true },
            ),
            crate::choreo_main::actions::ChoreoMainAction::SettingsAction(
                SettingsAction::UpdatePrimaryColorHex {
                    value: "#FF336699".to_string(),
                },
            ),
            crate::choreo_main::actions::ChoreoMainAction::SettingsAction(
                SettingsAction::UpdateIsDarkMode { enabled: true },
            ),
        ]);

        let main_page_state = shell.runtime.snapshot_main_page_state();
        let expected = crate::material::styling::material_palette::material_palette_for_theme(
            &main_page_state.settings_state.material_scheme,
            main_page_state.settings_state.theme_mode,
        );

        let _ = context.run(egui::RawInput::default(), |ctx| {
            shell.ui(ctx);
        });

        let visuals = context.style().visuals.clone();
        assert!(visuals.dark_mode);
        assert_eq!(
            egui_material3::get_global_color("primary"),
            expected.primary
        );
        assert_eq!(
            egui_material3::get_global_color("onSurface"),
            expected.on_surface
        );
    }

    fn unique_temp_file(extension: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let mut path = std::env::temp_dir();
        path.push(format!("rchoreo_app_shell_{nanos}.{extension}"));
        path
    }
}
