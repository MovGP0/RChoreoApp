use choreo_components_egui::audio_player::state::AudioPlayerState;
use choreo_components_egui::choreo_main::actions::ChoreoMainAction;
use choreo_components_egui::choreo_main::state::ChoreoMainState;
use choreo_components_egui::choreo_main::ui::draw as draw_choreo_main;
use choreo_components_egui::floor::state::FloorState;
use choreo_components_egui::scenes::state::ScenesState;
use choreo_components_egui::settings::state::SettingsState;
use choreo_components_egui::shell::state::ShellState;

#[test]
fn main_ui_exports_spec() {
    let _shell = ShellState::default();
    let _main = ChoreoMainState::default();
    let _settings = SettingsState::default();
    let _scenes = ScenesState::default();
    let _audio = AudioPlayerState::default();
    let _floor = FloorState::default();
    let _draw: fn(&mut egui::Ui, &ChoreoMainState) -> Vec<ChoreoMainAction> = draw_choreo_main;
}
