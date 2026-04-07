use crate::dancers::state::DancerState;
use crate::dancers::state::DancersState;
use crate::dancers::state::RoleState;
use crate::dancers::state::transparent_color;
use crate::dancers::ui::SwapDialogAction;
use crate::dancers::ui::build_swap_dialog_view_model;
use crate::dancers::ui::draw_swap_dialog_panel;
use crate::dancers::ui::map_swap_dialog_action;
use crate::dancers::ui::selected_dancer_index;
use crate::dancers::ui::selected_icon_index;
use crate::dancers::ui::selected_role_index;
use choreo_i18n::icon_names;

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
fn dancer_settings_page_draw_executes_without_panicking() {
    let state = sample_state();
    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = crate::dancers::ui::draw(ui, &state);
        });
    });
}

#[test]
fn selected_indices_follow_current_state_selection() {
    let state = sample_state();
    let mut errors = Vec::new();

    check_eq!(errors, selected_dancer_index(&state), Some(1));
    check_eq!(errors, selected_role_index(&state), Some(1));
    check_eq!(errors, selected_icon_index(&state), Some(1));

    assert_no_errors(errors);
}

#[test]
fn selected_dancer_index_returns_none_when_missing() {
    let mut state = sample_state();
    state.selected_dancer = None;
    assert_eq!(selected_dancer_index(&state), None);
}

#[test]
fn icon_options_are_loaded_from_i18n_catalog() {
    let state = DancersState::default();
    let mut errors = Vec::new();

    check_eq!(errors, state.icon_options.len(), icon_names().len());
    check!(errors, state.icon_options.len() > 3);

    assert_no_errors(errors);
}

#[test]
fn dropdown_helpers_follow_shared_material_dropdown_contract() {
    let state = sample_state();
    let expected_icon_labels = state
        .icon_options
        .iter()
        .map(|option| option.display_name.clone())
        .collect::<Vec<_>>();

    let mut errors = Vec::new();

    check_eq!(errors, crate::dancers::ui::dropdown_height_token(), 60.0);
    check_eq!(
        errors,
        crate::dancers::ui::role_option_labels(&state),
        vec!["Leader".to_string(), "Follower".to_string()]
    );
    check_eq!(
        errors,
        crate::dancers::ui::dancer_option_labels(&state),
        vec!["Alex".to_string(), "Blake".to_string()]
    );
    check_eq!(
        errors,
        crate::dancers::ui::icon_option_labels(&state),
        expected_icon_labels
    );

    assert_no_errors(errors);
}

#[test]
fn swap_dialog_view_model_formats_translated_message_from_selected_dancers() {
    let mut state = sample_state();
    state.is_dialog_open = true;
    state.dialog_content = Some("swap_dancers".to_string());

    let view_model =
        build_swap_dialog_view_model(&state, "en").expect("swap dialog should be visible");

    let mut errors = Vec::new();

    check_eq!(errors, view_model.title_text, "Swap dancers");
    check_eq!(errors, view_model.first_dancer_name, "Alex");
    check_eq!(errors, view_model.second_dancer_name, "Blake");
    check_eq!(
        errors,
        view_model.message_text,
        "Swap dancers \"Alex\" and \"Blake\"?"
    );
    check_eq!(errors, view_model.cancel_text, "Cancel");
    check_eq!(errors, view_model.confirm_text, "Swap");
    check_eq!(
        errors,
        view_model.first_dancer_color,
        egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255)
    );
    check_eq!(
        errors,
        view_model.second_dancer_color,
        egui::Color32::from_rgba_unmultiplied(0, 0, 255, 255)
    );

    assert_no_errors(errors);
}

#[test]
fn swap_dialog_view_model_is_none_for_non_swap_dialog_content() {
    let mut state = sample_state();
    state.is_dialog_open = true;
    state.dialog_content = Some("other_dialog".to_string());

    assert_eq!(build_swap_dialog_view_model(&state, "en"), None);
}

#[test]
fn swap_dialog_panel_returns_no_action_without_click() {
    let mut state = sample_state();
    state.is_dialog_open = true;

    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            assert_eq!(draw_swap_dialog_panel(ui, &state, "en"), None);
        });
    });
}

#[test]
fn swap_dialog_actions_map_to_dancer_reducer_actions() {
    let mut errors = Vec::new();

    check_eq!(
        errors,
        map_swap_dialog_action(SwapDialogAction::Cancel),
        crate::dancers::actions::DancersAction::HideDialog
    );
    check_eq!(
        errors,
        map_swap_dialog_action(SwapDialogAction::Confirm),
        crate::dancers::actions::DancersAction::ConfirmSwapDancers
    );

    assert_no_errors(errors);
}

#[test]
fn dancer_settings_page_draw_requests_hide_dialog_when_clicking_away() {
    let mut state = sample_state();
    state.is_dialog_open = true;

    let context = egui::Context::default();
    let raw_input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::pos2(0.0, 0.0),
            egui::vec2(480.0, 320.0),
        )),
        events: click_events(egui::pos2(8.0, 8.0)),
        ..egui::RawInput::default()
    };

    let mut emitted_actions = Vec::new();
    let _ = context.run(raw_input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            emitted_actions = crate::dancers::ui::draw(ui, &state);
        });
    });

    assert!(emitted_actions.contains(&crate::dancers::actions::DancersAction::HideDialog));
}

fn click_events(position: egui::Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(position),
        egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
        egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        },
    ]
}

fn sample_state() -> DancersState {
    let leader = RoleState {
        name: "Leader".to_string(),
        color: transparent_color(),
        z_index: 0,
    };
    let follower = RoleState {
        name: "Follower".to_string(),
        color: transparent_color(),
        z_index: 1,
    };

    let mut state = DancersState::default();
    state.roles = vec![leader.clone(), follower.clone()];
    state.dancers = vec![
        DancerState {
            dancer_id: 1,
            role: leader,
            name: "Alex".to_string(),
            shortcut: "A".to_string(),
            color: super::color(255, 0, 0),
            icon: Some("IconCircle".to_string()),
        },
        DancerState {
            dancer_id: 2,
            role: follower.clone(),
            name: "Blake".to_string(),
            shortcut: "B".to_string(),
            color: super::color(0, 0, 255),
            icon: Some("IconSquare".to_string()),
        },
    ];
    state.selected_dancer = state.dancers.get(1).cloned();
    state.selected_role = Some(follower);
    state.selected_icon_option = state.icon_options.get(1).cloned();
    state.can_delete_dancer = true;
    state.swap_from_dancer = state.dancers.first().cloned();
    state.swap_to_dancer = state.dancers.get(1).cloned();
    state.can_swap_dancers = true;
    state.dialog_content = Some("swap_dancers".to_string());
    state.is_dialog_open = false;

    state
}
