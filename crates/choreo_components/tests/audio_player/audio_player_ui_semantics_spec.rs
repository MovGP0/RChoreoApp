use choreo_components::audio_player::ui::audio_player_fixed_controls_width_px;
use choreo_components::audio_player::ui::denormalize_speed_from_slider_value;
use choreo_components::audio_player::ui::link_icon_name;
use choreo_components::audio_player::ui::link_icon_svg;
use choreo_components::audio_player::ui::normalize_speed_to_slider_value;
use choreo_components::audio_player::ui::play_pause_icon_label;
use choreo_components::audio_player::ui::play_pause_icon_svg;
use choreo_components::audio_player::ui::position_slider_width_for_panel_width;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn speed_normalization_round_trips_between_slider_and_speed_range() {
    let minimum_speed = 0.8;
    let maximum_speed = 1.1;

    let mut errors = Vec::new();

    let slider_value = normalize_speed_to_slider_value(0.95, minimum_speed, maximum_speed);
    check!(errors, (slider_value - 50.0).abs() < 0.000_1);

    let speed = denormalize_speed_from_slider_value(50.0, minimum_speed, maximum_speed);
    check!(errors, (speed - 0.95).abs() < 0.000_1);

    assert_no_errors(errors);
}

#[test]
fn play_pause_icon_label_maps_to_icon_text_tokens() {
    let mut errors = Vec::new();

    check_eq!(errors, play_pause_icon_label(false), "play_arrow");
    check_eq!(errors, play_pause_icon_label(true), "pause");

    assert_no_errors(errors);
}

#[test]
fn icon_controls_preserve_image_semantics() {
    let mut errors = Vec::new();

    check_eq!(errors, link_icon_name(), "link");
    check!(errors, link_icon_svg().contains("<svg"));
    check!(errors, play_pause_icon_svg(false).contains("<svg"));
    check!(errors, play_pause_icon_svg(true).contains("<svg"));

    assert_no_errors(errors);
}

#[test]
fn position_slider_width_uses_panel_width_minus_fixed_controls() {
    let panel_width = 960.0;
    let fixed_controls_width = audio_player_fixed_controls_width_px();

    let mut errors = Vec::new();

    let slider_width = position_slider_width_for_panel_width(panel_width);

    check!(
        errors,
        (slider_width - (panel_width - fixed_controls_width)).abs() < 0.000_1
    );

    assert_no_errors(errors);
}

#[test]
fn position_slider_width_clamps_to_minimum_when_panel_is_tight() {
    let panel_width = audio_player_fixed_controls_width_px() - 24.0;

    let mut errors = Vec::new();

    let slider_width = position_slider_width_for_panel_width(panel_width);

    check_eq!(errors, slider_width, 120.0);

    assert_no_errors(errors);
}
