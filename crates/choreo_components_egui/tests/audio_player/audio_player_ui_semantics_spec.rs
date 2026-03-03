use crate::audio_player::audio_player_component::ui::denormalize_speed_from_slider_value;
use crate::audio_player::audio_player_component::ui::normalize_speed_to_slider_value;
use crate::audio_player::audio_player_component::ui::play_pause_icon_label;

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
    assert_eq!(play_pause_icon_label(false), "[>]");
    assert_eq!(play_pause_icon_label(true), "[||]");
}
