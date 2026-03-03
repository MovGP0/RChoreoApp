use crate::dancers::state::DancerState;
use crate::dancers::state::DancersState;
use crate::dancers::state::RoleState;
use crate::dancers::state::transparent_color;
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
            color: transparent_color(),
            icon: Some("IconCircle".to_string()),
        },
        DancerState {
            dancer_id: 2,
            role: follower.clone(),
            name: "Blake".to_string(),
            shortcut: "B".to_string(),
            color: transparent_color(),
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
    state.dialog_content = Some("Confirm".to_string());
    state.is_dialog_open = false;

    state
}
