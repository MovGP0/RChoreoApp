use choreo_components_egui::audio_player::ui::audio_player_fixed_controls_width_px;
use choreo_components_egui::audio_player::ui::denormalize_speed_from_slider_value;
use choreo_components_egui::audio_player::ui::link_icon_name;
use choreo_components_egui::audio_player::ui::link_icon_svg;
use choreo_components_egui::audio_player::ui::normalize_speed_to_slider_value;
use choreo_components_egui::audio_player::ui::play_pause_icon_label;
use choreo_components_egui::audio_player::ui::play_pause_icon_svg;
use choreo_components_egui::audio_player::ui::position_slider_width_for_panel_width;

#[test]
fn speed_normalization_round_trips_between_slider_and_speed_range() {
    let minimum_speed = 0.8;
    let maximum_speed = 1.1;

    let slider_value = normalize_speed_to_slider_value(0.95, minimum_speed, maximum_speed);
    assert!((slider_value - 50.0).abs() < 0.000_1);

    let speed = denormalize_speed_from_slider_value(50.0, minimum_speed, maximum_speed);
    assert!((speed - 0.95).abs() < 0.000_1);
}

#[test]
fn play_pause_icon_label_maps_to_icon_text_tokens() {
    assert_eq!(play_pause_icon_label(false), "play_arrow");
    assert_eq!(play_pause_icon_label(true), "pause");
}

#[test]
fn icon_controls_preserve_image_semantics() {
    assert_eq!(link_icon_name(), "link");
    assert!(link_icon_svg().contains("<svg"));
    assert!(play_pause_icon_svg(false).contains("<svg"));
    assert!(play_pause_icon_svg(true).contains("<svg"));
}

#[test]
fn position_slider_width_uses_panel_width_minus_fixed_controls() {
    let panel_width = 960.0;
    let fixed_controls_width = audio_player_fixed_controls_width_px();

    let slider_width = position_slider_width_for_panel_width(panel_width);

    assert!((slider_width - (panel_width - fixed_controls_width)).abs() < 0.000_1);
}

#[test]
fn position_slider_width_clamps_to_minimum_when_panel_is_tight() {
    let panel_width = audio_player_fixed_controls_width_px() - 24.0;

    let slider_width = position_slider_width_for_panel_width(panel_width);

    assert_eq!(slider_width, 120.0);
}
