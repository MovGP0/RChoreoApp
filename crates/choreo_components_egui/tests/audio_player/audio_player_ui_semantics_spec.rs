use crate::audio_player::audio_player_component::ui::denormalize_speed_from_slider_value;
use crate::audio_player::audio_player_component::ui::link_icon_name;
use crate::audio_player::audio_player_component::ui::link_icon_svg;
use crate::audio_player::audio_player_component::ui::normalize_speed_to_slider_value;
use crate::audio_player::audio_player_component::ui::play_pause_icon_label;
use crate::audio_player::audio_player_component::ui::play_pause_icon_svg;

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
