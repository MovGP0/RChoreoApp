use super::actions::DancersAction;
use super::state::{
    DancerState, DancersState, PositionState, RoleState, SceneState, SceneViewState, default_role,
};

pub fn reduce(state: &mut DancersState, action: DancersAction) {
    match action {
        DancersAction::LoadFromGlobal | DancersAction::ReloadFromGlobal => load_from_global(state),
        DancersAction::AddDancer => {
            add_dancer(state);
            refresh_selection_state(state);
            ensure_swap_selections(state);
        }
        DancersAction::DeleteSelectedDancer => {
            delete_selected_dancer(state);
            refresh_selection_state(state);
            ensure_swap_selections(state);
        }
        DancersAction::SelectDancer { index } => {
            state.selected_dancer = state.dancers.get(index).cloned();
            refresh_selection_state(state);
        }
        DancersAction::SelectRole { index } => {
            state.selected_role = state.roles.get(index).cloned();
            update_selected_role(state);
            refresh_selection_state(state);
        }
        DancersAction::UpdateDancerName { value } => {
            update_selected_dancer(state, |dancer| dancer.name = value);
            refresh_selection_state(state);
        }
        DancersAction::UpdateDancerShortcut { value } => {
            update_selected_dancer(state, |dancer| dancer.shortcut = value);
            refresh_selection_state(state);
        }
        DancersAction::UpdateDancerColor { value } => {
            update_selected_dancer(state, |dancer| dancer.color = value);
            refresh_selection_state(state);
        }
        DancersAction::UpdateDancerIcon { value } => {
            state.selected_icon_option = if value.trim().is_empty() {
                None
            } else {
                state
                    .icon_options
                    .iter()
                    .find(|option| {
                        option.icon_name.eq_ignore_ascii_case(&value)
                            || option.key.eq_ignore_ascii_case(&value)
                    })
                    .cloned()
            };
            update_selected_icon(state);
            refresh_selection_state(state);
        }
        DancersAction::UpdateSwapFrom { index } => {
            state.swap_from_dancer = state.dancers.get(index).cloned();
            update_can_swap(state);
        }
        DancersAction::UpdateSwapTo { index } => {
            state.swap_to_dancer = state.dancers.get(index).cloned();
            update_can_swap(state);
        }
        DancersAction::SwapDancers => {
            swap_dancers(state);
            refresh_selection_state(state);
            ensure_swap_selections(state);
        }
        DancersAction::ShowDialog { content_id } => {
            state.dialog_content = content_id.clone();
            state.is_dialog_open = content_id.is_some();
        }
        DancersAction::HideDialog => {
            state.is_dialog_open = false;
            state.dialog_content = None;
        }
        DancersAction::Cancel => {}
        DancersAction::SaveToGlobal => save_to_global(state),
    }
}

fn load_from_global(state: &mut DancersState) {
    state.roles = state.global.roles.clone();
    ensure_default_roles(&mut state.roles);

    state.dancers.clear();
    for dancer in &state.global.dancers {
        let mapped_role = state
            .roles
            .iter()
            .find(|candidate| candidate.name.eq_ignore_ascii_case(&dancer.role.name))
            .cloned()
            .or_else(|| state.roles.first().cloned())
            .unwrap_or_else(|| default_role("Dame"));

        state.dancers.push(DancerState {
            role: mapped_role,
            ..dancer.clone()
        });
    }

    state.selected_dancer = state.dancers.first().cloned();
    refresh_selection_state(state);
    ensure_swap_selections(state);
}

fn ensure_default_roles(roles: &mut Vec<RoleState>) {
    if !roles.is_empty() {
        return;
    }
    roles.push(default_role("Dame"));
    roles.push(default_role("Herr"));
}

fn next_dancer_id(dancers: &[DancerState]) -> i32 {
    dancers.iter().map(|dancer| dancer.dancer_id).max().unwrap_or(0) + 1
}

fn add_dancer(state: &mut DancersState) {
    ensure_default_roles(&mut state.roles);
    let role = state
        .roles
        .first()
        .cloned()
        .unwrap_or_else(|| default_role("Dame"));
    let dancer = DancerState {
        dancer_id: next_dancer_id(&state.dancers),
        role: role.clone(),
        name: String::new(),
        shortcut: String::new(),
        color: role.color,
        icon: None,
    };
    state.dancers.push(dancer.clone());
    state.selected_dancer = Some(dancer);
}

fn delete_selected_dancer(state: &mut DancersState) {
    let Some(selected_id) = state.selected_dancer.as_ref().map(|dancer| dancer.dancer_id) else {
        return;
    };

    let Some(index) = state
        .dancers
        .iter()
        .position(|dancer| dancer.dancer_id == selected_id)
    else {
        return;
    };

    state.dancers.remove(index);
    state.selected_dancer = if state.dancers.is_empty() {
        None
    } else {
        Some(state.dancers[std::cmp::min(index, state.dancers.len() - 1)].clone())
    };
}

fn update_selected_dancer<F>(state: &mut DancersState, mutator: F)
where
    F: FnOnce(&mut DancerState),
{
    let Some(mut selected) = state.selected_dancer.clone() else {
        return;
    };

    mutator(&mut selected);

    if let Some(index) = state
        .dancers
        .iter()
        .position(|dancer| dancer.dancer_id == selected.dancer_id)
    {
        state.dancers[index] = selected.clone();
    }
    state.selected_dancer = Some(selected);
}

fn update_selected_role(state: &mut DancersState) {
    let Some(role) = state.selected_role.clone() else {
        return;
    };
    update_selected_dancer(state, |dancer| dancer.role = role);
}

fn normalize_icon_name(icon_name: &str) -> String {
    let normalized = icon_name.replace('\\', "/");
    let file_name = normalized.split('/').next_back().unwrap_or(&normalized);
    let name = file_name.split('.').next().unwrap_or(file_name);
    if name.trim().is_empty() {
        icon_name.to_string()
    } else {
        name.to_string()
    }
}

fn update_selected_icon(state: &mut DancersState) {
    let icon_value = state
        .selected_icon_option
        .as_ref()
        .map(|option| normalize_icon_name(&option.icon_name));
    update_selected_dancer(state, |dancer| dancer.icon = icon_value);
}

fn is_icon_match(key: &str, icon_value: Option<&str>) -> bool {
    let Some(icon_value) = icon_value else {
        return false;
    };

    if key.eq_ignore_ascii_case(icon_value) {
        return true;
    }

    let normalized = icon_value.replace('\\', "/");
    let file_name = normalized.split('/').next_back().unwrap_or(icon_value);
    let name = file_name.split('.').next().unwrap_or(file_name);
    key.eq_ignore_ascii_case(name)
}

fn refresh_selection_state(state: &mut DancersState) {
    let dancer = state.selected_dancer.clone();
    state.has_selected_dancer = dancer.is_some();
    state.can_delete_dancer = dancer.is_some();
    state.selected_role = dancer.as_ref().map(|value| value.role.clone());
    state.selected_icon_option = dancer.as_ref().and_then(|value| {
        state
            .icon_options
            .iter()
            .find(|option| {
                is_icon_match(&option.key, value.icon.as_deref())
                    || is_icon_match(&option.icon_name, value.icon.as_deref())
            })
            .cloned()
    });
}

fn ensure_swap_selections(state: &mut DancersState) {
    if state.dancers.is_empty() {
        state.swap_from_dancer = None;
        state.swap_to_dancer = None;
        update_can_swap(state);
        return;
    }

    if state
        .swap_from_dancer
        .as_ref()
        .map(|dancer| {
            !state
                .dancers
                .iter()
                .any(|item| item.dancer_id == dancer.dancer_id)
        })
        .unwrap_or(true)
    {
        state.swap_from_dancer = state.dancers.first().cloned();
    }

    if state.dancers.len() < 2 {
        state.swap_to_dancer = None;
        update_can_swap(state);
        return;
    }

    if state
        .swap_to_dancer
        .as_ref()
        .map(|dancer| {
            !state
                .dancers
                .iter()
                .any(|item| item.dancer_id == dancer.dancer_id)
        })
        .unwrap_or(true)
        || state
            .swap_to_dancer
            .as_ref()
            .zip(state.swap_from_dancer.as_ref())
            .map(|(to, from)| to.dancer_id == from.dancer_id)
            .unwrap_or(false)
    {
        state.swap_to_dancer = state
            .dancers
            .iter()
            .find(|dancer| {
                state
                    .swap_from_dancer
                    .as_ref()
                    .map(|from| from.dancer_id != dancer.dancer_id)
                    .unwrap_or(true)
            })
            .cloned();
    }

    update_can_swap(state);
}

fn update_can_swap(state: &mut DancersState) {
    state.can_swap_dancers = state.swap_from_dancer.is_some()
        && state.swap_to_dancer.is_some()
        && state
            .swap_from_dancer
            .as_ref()
            .zip(state.swap_to_dancer.as_ref())
            .map(|(from, to)| from.dancer_id != to.dancer_id)
            .unwrap_or(false);
}

fn swap_dancers(state: &mut DancersState) -> bool {
    let (Some(from), Some(to)) = (
        state.swap_from_dancer.as_ref(),
        state.swap_to_dancer.as_ref(),
    ) else {
        return false;
    };
    if from.dancer_id == to.dancer_id {
        return false;
    }

    let Some(from_index) = state
        .dancers
        .iter()
        .position(|dancer| dancer.dancer_id == from.dancer_id)
    else {
        return false;
    };
    let Some(to_index) = state
        .dancers
        .iter()
        .position(|dancer| dancer.dancer_id == to.dancer_id)
    else {
        return false;
    };
    if from_index == to_index {
        return false;
    }

    let mut left = state.dancers[from_index].clone();
    let mut right = state.dancers[to_index].clone();
    swap_dancer_properties(&mut left, &mut right);

    state.dancers[from_index] = left.clone();
    state.dancers[to_index] = right.clone();

    if let Some(selected) = state.selected_dancer.as_ref() {
        if selected.dancer_id == left.dancer_id {
            state.selected_dancer = Some(left.clone());
        } else if selected.dancer_id == right.dancer_id {
            state.selected_dancer = Some(right.clone());
        }
    }
    state.swap_from_dancer = Some(left);
    state.swap_to_dancer = Some(right);
    true
}

fn swap_dancer_properties(first: &mut DancerState, second: &mut DancerState) {
    let first_role = first.role.clone();
    let first_name = first.name.clone();
    let first_shortcut = first.shortcut.clone();
    let first_color = first.color.clone();
    let first_icon = first.icon.clone();

    first.role = second.role.clone();
    first.name = second.name.clone();
    first.shortcut = second.shortcut.clone();
    first.color = second.color.clone();
    first.icon = second.icon.clone();

    second.role = first_role;
    second.name = first_name;
    second.shortcut = first_shortcut;
    second.color = first_color;
    second.icon = first_icon;
}

fn save_to_global(state: &mut DancersState) {
    state.global.roles = state.roles.clone();
    state.global.dancers = state.dancers.clone();

    let dancers = state.global.dancers.clone();

    for scene in &mut state.global.scenes {
        update_scene_dancers(scene, &dancers);
    }

    for scene in &mut state.global.scene_views {
        update_scene_view_dancers(scene, &dancers);
    }

    if let Some(selected_scene) = state.global.selected_scene.as_mut() {
        update_scene_view_dancers(selected_scene, &dancers);
    }

    update_positions(&mut state.global.selected_positions, &dancers);
    update_positions(&mut state.global.selected_positions_snapshot, &dancers);
}

fn update_scene_dancers(scene: &mut SceneState, dancers: &[DancerState]) {
    update_positions(&mut scene.positions, dancers);
    for variation in &mut scene.variations {
        for variation_scene in variation {
            update_scene_dancers(variation_scene, dancers);
        }
    }
    for variation_scene in &mut scene.current_variation {
        update_scene_dancers(variation_scene, dancers);
    }
}

fn update_scene_view_dancers(scene: &mut SceneViewState, dancers: &[DancerState]) {
    update_positions(&mut scene.positions, dancers);
}

fn update_positions(positions: &mut Vec<PositionState>, dancers: &[DancerState]) {
    positions.retain_mut(|position| {
        let Some(dancer_id) = position.dancer_id else {
            return true;
        };
        if let Some(new_dancer) = dancers.iter().find(|dancer| dancer.dancer_id == dancer_id) {
            position.dancer_name = Some(new_dancer.name.clone());
            true
        } else {
            false
        }
    });
}
