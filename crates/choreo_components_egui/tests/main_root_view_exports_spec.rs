use choreo_components_egui::main_root_view;
use choreo_components_egui::main_root_view::AudioPlayerInfo;
use choreo_components_egui::main_root_view::AxisLabel;
use choreo_components_egui::main_root_view::ChoreoInfo;
use choreo_components_egui::main_root_view::ChoreographySettings;
use choreo_components_egui::main_root_view::FloorCurve;
use choreo_components_egui::main_root_view::FloorInfo;
use choreo_components_egui::main_root_view::FloorLegendEntries;
use choreo_components_egui::main_root_view::FloorMetricsInfo;
use choreo_components_egui::main_root_view::FloorPosition;
use choreo_components_egui::main_root_view::LegendEntry;
use choreo_components_egui::main_root_view::LineSegment;
use choreo_components_egui::main_root_view::MainRootAction;
use choreo_components_egui::main_root_view::MainRootState;
use choreo_components_egui::main_root_view::MaterialPalette;
use choreo_components_egui::main_root_view::SceneInfo;
use choreo_components_egui::main_root_view::SceneListItem;
use choreo_components_egui::main_root_view::ScenesInfo;
use choreo_components_egui::main_root_view::SettingsInfo;
use choreo_components_egui::main_root_view::ShellHost;
use choreo_components_egui::main_root_view::Translations;
use choreo_components_egui::main_root_view::ZoomPanInfo;

#[test]
fn main_root_view_exports_spec() {
    let _palette = MaterialPalette::default();
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
    let _choreo_info: ChoreoInfo =
        choreo_components_egui::choreo_info::state::ChoreoInfoState::default();
    let _choreo_settings = ChoreographySettings::default();
    let _audio = AudioPlayerInfo::default();
    let _settings = SettingsInfo::default();
    let _position = FloorPosition::new(1.0, 2.0);
    let _segment = LineSegment {
        from: choreo_components_egui::floor::state::Point::new(1.0, 2.0),
        to: choreo_components_egui::floor::state::Point::new(3.0, 4.0),
    };
    let _label = AxisLabel {
        text: "Front".to_string(),
        position: choreo_components_egui::floor::state::Point::new(0.0, 1.0),
    };
    let legend_entry = LegendEntry {
        shortcut: "L".to_string(),
        name: "Lead".to_string(),
        position_text: "1".to_string(),
        color: [255, 0, 0, 255],
    };
    let _legend_entries: FloorLegendEntries = vec![legend_entry];
    let _curve = FloorCurve::default();
    let _floor = FloorInfo::default();
    let _metrics = FloorMetricsInfo::from_zoom(1.0);
    let _zoom_pan = ZoomPanInfo::default();
    let _shell_host: ShellHost = choreo_components_egui::shell::create_shell_host();
    let _draw: fn(&mut egui::Ui, &MainRootState) -> Vec<MainRootAction> = main_root_view::draw;

    assert_eq!(Translations::app_title("en"), "Choreography Viewer");
    assert_eq!(Translations::text("en", "mode_view"), Some("View"));
    assert_eq!(ZoomPanInfo::user_scale(0.0), 1.0);
    assert_eq!(ZoomPanInfo::base_scale(4.0, 2.0), 2.0);
}
