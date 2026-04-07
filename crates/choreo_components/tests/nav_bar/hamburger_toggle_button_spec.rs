use egui::Rect;
use egui::pos2;
use egui::vec2;

use crate::nav_bar::nav_bar_component::hamburger_toggle_button::checked_animation_spec;
use crate::nav_bar::nav_bar_component::hamburger_toggle_button::content_padding_token;
use crate::nav_bar::nav_bar_component::hamburger_toggle_button::desired_size;
use crate::nav_bar::nav_bar_component::hamburger_toggle_button::geometry_for_rect;
use crate::nav_bar::nav_bar_component::hamburger_toggle_button::geometry_for_rect_with_progress;
use crate::nav_bar::nav_bar_component::hamburger_toggle_button::minimum_button_size_token;
use crate::nav_bar::nav_bar_component::hamburger_toggle_button::state_layer_animation_spec;
use crate::nav_bar::nav_bar_component::hamburger_toggle_button::toggled_state_after_click;
use choreo_components::material::styling::material_animations::MaterialAnimation;
use choreo_components::material::styling::material_animations::MaterialAnimations;
use choreo_components::material::styling::material_style_metrics::material_style_metrics;

#[test]
fn unchecked_geometry_keeps_three_parallel_bars() {
    let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(48.0, 48.0));
    let geometry = geometry_for_rect(rect, false);

    macro_rules! check {
        ($errors:expr, $condition:expr) => {
            if !$condition {
                $errors.push(format!("{} is false", stringify!($condition)));
            }
        };
    }

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

    let mut errors = Vec::new();

    check!(errors, geometry.top_start.y < geometry.middle_start.y);
    check!(errors, geometry.middle_start.y < geometry.bottom_start.y);
    check_eq!(errors, geometry.top_start.y, geometry.top_end.y);
    check_eq!(errors, geometry.middle_start.y, geometry.middle_end.y);
    check_eq!(errors, geometry.bottom_start.y, geometry.bottom_end.y);
    check_eq!(errors, geometry.top_start.x, geometry.middle_start.x);
    check_eq!(errors, geometry.middle_start.x, geometry.bottom_start.x);

    assert!(errors.is_empty(), "Assertion failures:\n{}", errors.join("\n"));
}

#[test]
fn checked_geometry_collapses_top_and_bottom_to_middle_with_rotations() {
    let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(48.0, 48.0));
    let geometry = geometry_for_rect(rect, true);

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
                $errors.push(format!("{} is false", stringify!($condition)));
            }
        };
    }

    let mut errors = Vec::new();

    check_eq!(errors, geometry.top_start.y, geometry.middle_start.y);
    check_eq!(errors, geometry.bottom_start.y, geometry.middle_start.y);

    let top_delta_y = geometry.top_end.y - geometry.top_start.y;
    let bottom_delta_y = geometry.bottom_end.y - geometry.bottom_start.y;
    check!(errors, top_delta_y < 0.0);
    check!(errors, bottom_delta_y > 0.0);

    let middle_delta_y = geometry.middle_end.y - geometry.middle_start.y;
    check_eq!(errors, middle_delta_y, 0.0);

    assert!(errors.is_empty(), "Assertion failures:\n{}", errors.join("\n"));
}

#[test]
fn partial_transition_geometry_interpolates_between_unchecked_and_checked() {
    let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(48.0, 48.0));
    let unchecked = geometry_for_rect(rect, false);
    let checked = geometry_for_rect(rect, true);
    let halfway = geometry_for_rect_with_progress(rect, 0.5);

    macro_rules! check {
        ($errors:expr, $condition:expr) => {
            if !$condition {
                $errors.push(format!("{} is false", stringify!($condition)));
            }
        };
    }

    let mut errors = Vec::new();

    check!(errors, halfway.top_start.y > unchecked.top_start.y);
    check!(errors, halfway.top_start.y < checked.top_start.y);
    check!(errors, halfway.bottom_start.y < unchecked.bottom_start.y);
    check!(errors, halfway.bottom_start.y > checked.bottom_start.y);
    check!(errors, halfway.top_end.y < unchecked.top_end.y);
    check!(errors, halfway.top_end.y > checked.top_end.y);

    assert!(errors.is_empty(), "Assertion failures:\n{}", errors.join("\n"));
}

#[test]
fn desired_size_defaults_to_slint_minimum() {
    let size = desired_size(None);

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

    let mut errors = Vec::new();

    check_eq!(errors, size.x, minimum_button_size_token());
    check_eq!(errors, size.y, minimum_button_size_token());

    assert!(errors.is_empty(), "Assertion failures:\n{}", errors.join("\n"));
}

#[test]
fn desired_size_clamps_to_slint_minimum_when_too_small() {
    let size = desired_size(Some(vec2(24.0, 30.0)));

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

    let mut errors = Vec::new();

    check_eq!(errors, size.x, minimum_button_size_token());
    check_eq!(errors, size.y, minimum_button_size_token());

    assert!(errors.is_empty(), "Assertion failures:\n{}", errors.join("\n"));
}

#[test]
fn hamburger_button_tokens_map_to_shared_material_metrics() {
    let metrics = material_style_metrics();

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

    let mut errors = Vec::new();

    check_eq!(errors, minimum_button_size_token(), metrics.sizes.size_40);
    check_eq!(errors, content_padding_token(), metrics.paddings.padding_10);

    assert!(errors.is_empty(), "Assertion failures:\n{}", errors.join("\n"));
}

#[test]
fn hamburger_button_uses_shared_material_animation_specs() {
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

    let mut errors = Vec::new();

    check_eq!(
        errors,
        checked_animation_spec(),
        MaterialAnimations::spec(MaterialAnimation::Emphasized)
    );
    check_eq!(
        errors,
        state_layer_animation_spec(),
        MaterialAnimations::spec(MaterialAnimation::Opacity)
    );

    assert!(errors.is_empty(), "Assertion failures:\n{}", errors.join("\n"));
}

#[test]
fn toggle_on_click_matches_slint_toggle_semantics() {
    assert!(toggled_state_after_click(false, true, true));
    assert!(!toggled_state_after_click(false, false, true));
    assert!(!toggled_state_after_click(false, true, false));
}
