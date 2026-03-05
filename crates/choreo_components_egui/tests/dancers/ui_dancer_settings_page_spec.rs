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
    assert_eq!(selected_dancer_index(&state), Some(1));
    assert_eq!(selected_role_index(&state), Some(1));
    assert_eq!(selected_icon_index(&state), Some(1));
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
    assert_eq!(state.icon_options.len(), icon_names().len());
    assert!(state.icon_options.len() > 3);
}

#[test]
fn swap_dialog_view_model_formats_translated_message_from_selected_dancers() {
    let mut state = sample_state();
    state.is_dialog_open = true;
    state.dialog_content = Some("swap_dancers".to_string());

    let view_model =
        build_swap_dialog_view_model(&state, "en").expect("swap dialog should be visible");

    assert_eq!(view_model.title_text, "Swap dancers");
    assert_eq!(view_model.first_dancer_name, "Alex");
    assert_eq!(view_model.second_dancer_name, "Blake");
    assert_eq!(
        view_model.message_text,
        "Swap dancers \"Alex\" and \"Blake\"?"
    );
    assert_eq!(view_model.cancel_text, "Cancel");
    assert_eq!(view_model.confirm_text, "Swap");
    assert_eq!(
        view_model.first_dancer_color,
        egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255)
    );
    assert_eq!(
        view_model.second_dancer_color,
        egui::Color32::from_rgba_unmultiplied(0, 0, 255, 255)
    );
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
    assert_eq!(
        map_swap_dialog_action(SwapDialogAction::Cancel),
        crate::dancers::actions::DancersAction::HideDialog
    );
    assert_eq!(
        map_swap_dialog_action(SwapDialogAction::Confirm),
        crate::dancers::actions::DancersAction::ConfirmSwapDancers
    );
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
