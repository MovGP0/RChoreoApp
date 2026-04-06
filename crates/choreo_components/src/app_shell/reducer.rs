use super::actions::AppShellAction;
use super::effects::AppShellEffect;
use super::state::AppShellState;

pub fn reduce(state: &mut AppShellState, action: AppShellAction) -> Vec<AppShellEffect> {
    match action {
        AppShellAction::FrameStarted => reduce_frame_started(state),
        AppShellAction::SplashPresented => reduce_splash_presented(state),
        AppShellAction::ExternalFilePathReceived { file_path } => {
            reduce_external_file_path_received(file_path)
        }
    }
}

fn reduce_frame_started(state: &mut AppShellState) -> Vec<AppShellEffect> {
    let mut effects = Vec::new();

    if !state.is_typography_initialized {
        state.is_typography_initialized = true;
        effects.push(AppShellEffect::ApplyTypography);
    }

    if !state.is_main_page_initialized {
        state.is_main_page_initialized = true;
        effects.push(AppShellEffect::InitializeMainPage);
    }

    effects
}

fn reduce_splash_presented(state: &mut AppShellState) -> Vec<AppShellEffect> {
    if !state.show_splash_screen {
        return Vec::new();
    }

    state.show_splash_screen = false;
    vec![AppShellEffect::RequestRepaint]
}

fn reduce_external_file_path_received(file_path: String) -> Vec<AppShellEffect> {
    if file_path.trim().is_empty() {
        return Vec::new();
    }

    vec![AppShellEffect::RouteExternalFilePath { file_path }]
}
