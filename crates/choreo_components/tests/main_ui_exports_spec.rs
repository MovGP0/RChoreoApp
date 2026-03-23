use choreo_components::audio_player::state::AudioPlayerState;
use choreo_components::choreo_main::actions::ChoreoMainAction;
use choreo_components::choreo_main::state::ChoreoMainState;
use choreo_components::choreo_main::ui::draw as draw_choreo_main;
use choreo_components::floor::state::FloorState;
use choreo_components::global::GlobalProvider;
use choreo_components::logging::BehaviorLog;
use choreo_components::scenes::state::ScenesState;
use choreo_components::settings::state::SettingsState;
use choreo_components::shell;

#[test]
fn main_ui_exports_spec() {
    let _shell_title = shell::app_title();
    let _shell_host = shell::create_shell_host();
    let _main = ChoreoMainState::default();
    let _settings = SettingsState::default();
    let _scenes = ScenesState::default();
    let _audio = AudioPlayerState::default();
    let _floor = FloorState::default();
    let _global_provider = GlobalProvider::new();
    BehaviorLog::behavior_activated("Behavior", "ViewModel");
    let _draw: fn(&mut egui::Ui, &ChoreoMainState) -> Vec<ChoreoMainAction> = draw_choreo_main;
}
