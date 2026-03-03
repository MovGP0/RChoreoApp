use choreo_components_egui::main_root_view;
use choreo_components_egui::main_root_view::AudioPlayerInfo;
use choreo_components_egui::main_root_view::ChoreographySettings;
use choreo_components_egui::main_root_view::FloorInfo;
use choreo_components_egui::main_root_view::FloorMetricsInfo;
use choreo_components_egui::main_root_view::MainRootAction;
use choreo_components_egui::main_root_view::MainRootState;
use choreo_components_egui::main_root_view::SceneInfo;
use choreo_components_egui::main_root_view::SceneListItem;
use choreo_components_egui::main_root_view::ScenesInfo;
use choreo_components_egui::main_root_view::SettingsInfo;
use choreo_components_egui::main_root_view::ShellHost;

#[test]
fn main_root_view_exports_spec() {
    let _state = MainRootState::default();
    let _scenes = ScenesInfo::default();
    let _scene_item = SceneListItem::new(
        choreo_master_mobile_json::SceneId(1),
        "Scene 1",
        choreo_master_mobile_json::Color::transparent(),
    );
    let _scene = SceneInfo::new(
        choreo_master_mobile_json::SceneId(2),
        "Scene 2",
        choreo_master_mobile_json::Color::transparent(),
    );
    let _choreo_settings = ChoreographySettings::default();
    let _audio = AudioPlayerInfo::default();
    let _settings = SettingsInfo::default();
    let _floor = FloorInfo::default();
    let _metrics = FloorMetricsInfo::from_zoom(1.0);
    let _shell_host: ShellHost = choreo_components_egui::shell::create_shell_host();
    let _draw: fn(&mut egui::Ui, &MainRootState) -> Vec<MainRootAction> = main_root_view::draw;
}
