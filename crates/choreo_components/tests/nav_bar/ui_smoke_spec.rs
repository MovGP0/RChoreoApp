use crate::nav_bar::nav_bar_component::actions::NavBarAction;
use crate::nav_bar::nav_bar_component::state::InteractionMode;
use crate::nav_bar::nav_bar_component::state::NavBarState;
use crate::nav_bar::nav_bar_component::translations::nav_bar_translations;
use crate::nav_bar::nav_bar_component::ui::action_button_icon_uris;
use crate::nav_bar::nav_bar_component::ui::action_button_tokens;
use crate::nav_bar::nav_bar_component::ui::image_button_checked;
use crate::nav_bar::nav_bar_component::ui::mode_label;
use crate::nav_bar::nav_bar_component::ui::mode_option_labels;
use crate::nav_bar::nav_bar_component::ui::mode_selector_height_token;
use crate::nav_bar::nav_bar_component::ui::mode_selector_width_token;
use crate::nav_bar::nav_bar_component::ui::nav_button;
use crate::nav_bar::nav_bar_component::ui::settings_button;
use crate::nav_bar::nav_bar_component::ui::settings_button_checked;
use crate::nav_bar::nav_bar_component::ui::top_bar_action_count;

#[test]
fn nav_bar_ui_draw_executes_without_panicking() {
    let state = NavBarState::default();
    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = crate::nav_bar::nav_bar_component::ui::draw(ui, &state);
        });
    });
}

#[test]
fn nav_button_action_depends_on_nav_state() {
    let mut errors = Vec::new();
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

    let closed = NavBarState::default();
    check_eq!(
        errors,
        nav_button(&closed),
        ("open_navigation", NavBarAction::ToggleNavigation)
    );

    let open = NavBarState {
        is_nav_open: true,
        ..NavBarState::default()
    };
    check_eq!(
        errors,
        nav_button(&open),
        ("close_navigation", NavBarAction::CloseNavigation)
    );

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn settings_button_action_depends_on_settings_state() {
    let mut errors = Vec::new();
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

    let closed = NavBarState::default();
    check_eq!(
        errors,
        settings_button(&closed),
        ("open_settings", NavBarAction::ToggleChoreographySettings)
    );

    let open = NavBarState {
        is_choreography_settings_open: true,
        ..NavBarState::default()
    };
    check_eq!(
        errors,
        settings_button(&open),
        ("close_settings", NavBarAction::CloseChoreographySettings)
    );

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn toggle_icon_buttons_reflect_original_checked_state_mappings() {
    let mut errors = Vec::new();
    macro_rules! check {
        ($errors:expr, $condition:expr) => {
            if !$condition {
                $errors.push(format!("assertion failed: {}", stringify!($condition)));
            }
        };
    }

    let closed = NavBarState::default();
    check!(errors, !settings_button_checked(&closed));
    check!(errors, !image_button_checked(&closed));

    let open = NavBarState {
        is_choreography_settings_open: true,
        is_floor_svg_overlay_open: true,
        ..NavBarState::default()
    };
    check!(errors, settings_button_checked(&open));
    check!(errors, image_button_checked(&open));

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn mode_labels_map_to_translation_keys() {
    let mut errors = Vec::new();
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

    check_eq!(errors, mode_label(InteractionMode::View), "ModeView");
    check_eq!(errors, mode_label(InteractionMode::Move), "ModeMove");
    check_eq!(
        errors,
        mode_label(InteractionMode::RotateAroundCenter),
        "ModeRotateAroundCenter"
    );
    check_eq!(
        errors,
        mode_label(InteractionMode::RotateAroundDancer),
        "ModeRotateAroundDancer"
    );
    check_eq!(errors, mode_label(InteractionMode::Scale), "ModeScale");
    check_eq!(
        errors,
        mode_label(InteractionMode::LineOfSight),
        "ModeLineOfSight"
    );

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn mode_selector_uses_original_top_bar_size_tokens() {
    let mut errors = Vec::new();
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

    check_eq!(errors, mode_selector_width_token(), 180.0);
    check_eq!(errors, mode_selector_height_token(), 56.0);

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn nav_bar_exposes_all_mode_options_in_order() {
    let strings = nav_bar_translations("en");
    assert_eq!(
        mode_option_labels(&strings),
        [
            "View",
            "Move",
            "Rotate around center",
            "Rotate around dancer",
            "Scale",
            "Line of sight",
        ]
    );
}

#[test]
fn nav_bar_top_bar_actions_keep_expected_order() {
    let state = NavBarState::default();
    assert_eq!(top_bar_action_count(), 6);
    assert_eq!(
        action_button_tokens(&state),
        ["menu", "edit", "home", "image", "play_circle"]
    );
}

#[test]
fn nav_bar_action_icons_map_to_distinct_svg_sources() {
    assert_eq!(
        action_button_icon_uris(),
        [
            "bytes://top_bar/settings.svg",
            "bytes://top_bar/home.svg",
            "bytes://top_bar/image.svg",
            "bytes://top_bar/audio.svg",
        ]
    );
}

#[test]
fn nav_bar_mode_dropdown_popup_renders_all_labels() {
    let state = NavBarState::default();
    let context = egui::Context::default();
    let closed_output = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(1280.0);
            let _ = crate::nav_bar::nav_bar_component::ui::draw(ui, &state);
        });
    });

    egui::Popup::open_id(
        &context,
        egui::Id::new("nav_bar_mode_dropdown").with("popup"),
    );
    let open_output = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(1280.0);
            let _ = crate::nav_bar::nav_bar_component::ui::draw(ui, &state);
        });
    });

    assert!(open_output.shapes.len() > closed_output.shapes.len());
}

#[test]
fn nav_bar_does_not_render_extra_mode_heading_text() {
    let state = NavBarState::default();
    let context = egui::Context::default();
    let output = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_width(1280.0);
            let _ = crate::nav_bar::nav_bar_component::ui::draw(ui, &state);
        });
    });

    let mut rendered_mode_heading = false;
    for clipped in output.shapes {
        if format!("{:?}", clipped.shape).contains("\"Mode\"") {
            rendered_mode_heading = true;
            break;
        }
    }

    assert!(!rendered_mode_heading);
}
