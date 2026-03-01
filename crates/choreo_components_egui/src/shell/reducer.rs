use super::actions::ShellAction;
use super::state::ShellState;
use super::state::ShellThemeMode;

pub fn reduce(state: &mut ShellState, action: ShellAction) {
    match action {
        ShellAction::Initialize => {
            state.app_title = "ChoreoApp".to_string();
        }
        ShellAction::SetThemeMode { is_dark } => {
            state.theme_mode = if is_dark {
                ShellThemeMode::Dark
            } else {
                ShellThemeMode::Light
            };
            sync_active_background(state);
        }
        ShellAction::ApplyMaterialSchemes {
            light_background_hex,
            dark_background_hex,
        } => {
            state.schemes.light_background_hex = light_background_hex;
            state.schemes.dark_background_hex = dark_background_hex;
            sync_active_background(state);
        }
    }
}

fn sync_active_background(state: &mut ShellState) {
    state.active_background_hex = match state.theme_mode {
        ShellThemeMode::Light => state.schemes.light_background_hex.clone(),
        ShellThemeMode::Dark => state.schemes.dark_background_hex.clone(),
    };
}
