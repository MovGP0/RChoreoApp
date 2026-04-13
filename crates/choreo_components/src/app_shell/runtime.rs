use egui::Context;
use egui::Visuals;

use crate::choreo_main::MainPageBinding;
use crate::choreo_main::MainPageDependencies;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::state::ChoreoMainState;
use crate::material::ThemeMode;
use crate::material::styling::material_palette::sync_material_theme;
use crate::material::styling::material_typography as typography;

use super::effects::AppShellEffect;

pub struct AppShellRuntime {
    main_page_binding: MainPageBinding,
}

impl AppShellRuntime {
    #[must_use]
    pub fn new(main_page_dependencies: MainPageDependencies) -> Self {
        Self {
            main_page_binding: MainPageBinding::new(main_page_dependencies),
        }
    }

    pub fn apply_effects(&self, context: Option<&Context>, effects: Vec<AppShellEffect>) {
        for effect in effects {
            self.apply_effect(context, effect);
        }
    }

    pub fn tick_audio_runtime(&self) -> bool {
        self.main_page_binding.tick_audio_runtime()
    }

    #[must_use]
    pub fn audio_runtime_is_active(&self) -> bool {
        self.main_page_binding.audio_runtime_is_active()
    }

    #[must_use]
    pub fn snapshot_main_page_state(&self) -> ChoreoMainState {
        let state = self.main_page_binding.state();
        state.borrow().clone()
    }

    pub fn apply_main_page_theme(&self, context: &Context, main_page_state: &ChoreoMainState) {
        sync_material_theme(
            &main_page_state.settings_state.material_scheme,
            main_page_state.settings_state.theme_mode,
        );
        context.set_visuals(if matches!(main_page_state.settings_state.theme_mode, ThemeMode::Dark)
        {
            Visuals::dark()
        } else {
            Visuals::light()
        });
    }

    pub fn dispatch_main_page_actions(&self, actions: Vec<ChoreoMainAction>) {
        for action in actions {
            self.main_page_binding.dispatch(action);
        }
    }

    fn apply_effect(&self, context: Option<&Context>, effect: AppShellEffect) {
        match effect {
            AppShellEffect::ApplyTypography => {
                let context = context.expect("typography effect requires an egui context");
                typography::apply_to_context(context);
            }
            AppShellEffect::InitializeMainPage => {
                self.main_page_binding
                    .dispatch(ChoreoMainAction::Initialize);
            }
            AppShellEffect::RequestRepaint => {
                let context = context.expect("repaint effect requires an egui context");
                context.request_repaint();
            }
            AppShellEffect::RouteExternalFilePath { file_path } => {
                self.main_page_binding.route_external_file_path(&file_path);
            }
        }
    }
}
