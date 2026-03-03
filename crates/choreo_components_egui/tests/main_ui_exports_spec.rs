use choreo_components_egui::audio_player::state::AudioPlayerState;
use choreo_components_egui::choreo_main::actions::ChoreoMainAction;
use choreo_components_egui::choreo_main::state::ChoreoMainState;
use choreo_components_egui::choreo_main::ui::draw as draw_choreo_main;
use choreo_components_egui::floor::state::FloorState;
use choreo_components_egui::global::GlobalProvider;
use choreo_components_egui::logging::BehaviorLog;
use choreo_components_egui::scenes::state::ScenesState;
use choreo_components_egui::settings::state::SettingsState;
use choreo_components_egui::shell;

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
