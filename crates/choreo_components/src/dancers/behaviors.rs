use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use choreo_master_mobile_json::DancerId;
use choreo_models::{Colors, DancerModel, RoleModel, SceneModel};

use crate::audio_player::HapticFeedback;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;

use super::view_model::{
    CloseDancerDialogCommand, DancerSettingsViewModel, IconOption, ShowDancerDialogCommand,
};

pub struct AddDancerBehavior;

impl AddDancerBehavior {
    pub fn add_dancer(view_model: &mut DancerSettingsViewModel) {
        ensure_default_roles(&mut view_model.roles);

        let next_id = next_dancer_id(&view_model.dancers);
        let role = view_model
            .roles
            .first()
            .cloned()
            .unwrap_or_else(|| Rc::new(default_role("Dame")));
        if !view_model.roles.iter().any(|item| Rc::ptr_eq(item, &role)) {
            view_model.roles.push(role.clone());
        }

        let dancer = Rc::new(DancerModel {
            dancer_id: next_id,
            role: role.clone(),
            name: String::new(),
            shortcut: String::new(),
            color: role.color.clone(),
            icon: None,
        });

        view_model.dancers.push(dancer.clone());
        view_model.selected_dancer = Some(dancer);
    }
}

impl Behavior<DancerSettingsViewModel> for AddDancerBehavior {
    fn activate(&self, _view_model: &mut DancerSettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("AddDancerBehavior", "DancerSettingsViewModel");
    }
}

pub struct CancelDancerSettingsBehavior;

impl CancelDancerSettingsBehavior {
    pub fn navigate_back() {
        // handled by navigation layer
    }
}

impl Behavior<DancerSettingsViewModel> for CancelDancerSettingsBehavior {
    fn activate(&self, _view_model: &mut DancerSettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "CancelDancerSettingsBehavior",
            "DancerSettingsViewModel",
        );
    }
}

pub struct DeleteDancerBehavior;

impl DeleteDancerBehavior {
    pub fn delete_dancer(view_model: &mut DancerSettingsViewModel) {
        let Some(selected) = view_model.selected_dancer.as_ref() else {
            return;
        };
        let selected_id = selected.dancer_id;

        let index = view_model
            .dancers
            .iter()
            .position(|dancer| dancer.dancer_id == selected_id);
        let Some(index) = index else {
            return;
        };

        view_model.dancers.remove(index);
        view_model.selected_dancer = if view_model.dancers.is_empty() {
            None
        } else {
            Some(view_model.dancers[std::cmp::min(index, view_model.dancers.len() - 1)].clone())
        };
    }
}

impl Behavior<DancerSettingsViewModel> for DeleteDancerBehavior {
    fn activate(&self, _view_model: &mut DancerSettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("DeleteDancerBehavior", "DancerSettingsViewModel");
    }
}

pub struct HideDancerDialogBehavior {
    receiver: crossbeam_channel::Receiver<CloseDancerDialogCommand>,
}

impl HideDancerDialogBehavior {
    pub fn new(receiver: crossbeam_channel::Receiver<CloseDancerDialogCommand>) -> Self {
        Self { receiver }
    }

    pub fn hide_dialog(view_model: &mut DancerSettingsViewModel) {
        view_model.is_dialog_open = false;
        view_model.dialog_content = None;
    }

    pub fn try_handle(&self, view_model: &mut DancerSettingsViewModel) -> bool {
        match self.receiver.try_recv() {
            Ok(_) => {
                Self::hide_dialog(view_model);
                true
            }
            Err(_) => false,
        }
    }
}

impl Behavior<DancerSettingsViewModel> for HideDancerDialogBehavior {
    fn activate(&self, _view_model: &mut DancerSettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("HideDancerDialogBehavior", "DancerSettingsViewModel");
    }
}

pub struct LoadDancerSettingsBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
}

impl LoadDancerSettingsBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>) -> Self {
        Self { global_state }
    }

    pub fn load(&self, view_model: &mut DancerSettingsViewModel) {
        view_model.roles.clear();
        view_model.dancers.clear();

        let mut role_map: HashMap<usize, Rc<RoleModel>> = HashMap::new();
        let global_state = self.global_state.borrow();
        for role in &global_state.choreography.roles {
            let copy = Rc::new(RoleModel {
                name: role.name.clone(),
                color: role.color.clone(),
                z_index: role.z_index,
            });
            role_map.insert(Rc::as_ptr(role) as usize, copy.clone());
            view_model.roles.push(copy);
        }

        ensure_default_roles(&mut view_model.roles);

        for dancer in &global_state.choreography.dancers {
            let role = role_map
                .get(&(Rc::as_ptr(&dancer.role) as usize))
                .cloned()
                .or_else(|| {
                    view_model.roles.iter().find(|candidate| {
                        candidate.name.eq_ignore_ascii_case(&dancer.role.name)
                    }).cloned()
                })
                .or_else(|| view_model.roles.first().cloned())
                .unwrap_or_else(|| Rc::new(default_role("Dame")));

            let copy = Rc::new(DancerModel {
                dancer_id: dancer.dancer_id,
                role,
                name: dancer.name.clone(),
                shortcut: dancer.shortcut.clone(),
                color: dancer.color.clone(),
                icon: dancer.icon.clone(),
            });
            view_model.dancers.push(copy);
        }

        view_model.selected_dancer = view_model.dancers.first().cloned();
    }
}

impl Behavior<DancerSettingsViewModel> for LoadDancerSettingsBehavior {
    fn activate(&self, view_model: &mut DancerSettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "LoadDancerSettingsBehavior",
            "DancerSettingsViewModel",
        );
        self.load(view_model);
    }
}

pub struct SaveDancerSettingsBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
}

impl SaveDancerSettingsBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>) -> Self {
        Self { global_state }
    }

    pub fn apply_changes(&mut self, view_model: &DancerSettingsViewModel) {
        let mut global_state = self.global_state.borrow_mut();
        let choreography = &mut global_state.choreography;
        choreography.roles.clear();
        choreography
            .roles
            .extend(view_model.roles.iter().cloned());

        choreography.dancers.clear();
        choreography
            .dancers
            .extend(view_model.dancers.iter().cloned());

        let dancer_map: HashMap<DancerId, Rc<DancerModel>> = view_model
            .dancers
            .iter()
            .map(|dancer| (dancer.dancer_id, dancer.clone()))
            .collect();

        for scene in &mut choreography.scenes {
            update_scene_dancers(scene, &dancer_map);
        }
    }
}

impl Behavior<DancerSettingsViewModel> for SaveDancerSettingsBehavior {
    fn activate(&self, _view_model: &mut DancerSettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "SaveDancerSettingsBehavior",
            "DancerSettingsViewModel",
        );
    }
}

pub struct SelectedDancerStateBehavior;

impl SelectedDancerStateBehavior {
    pub fn update_selected_dancer(view_model: &mut DancerSettingsViewModel) {
        let dancer = view_model.selected_dancer.clone();
        view_model.has_selected_dancer = dancer.is_some();
        view_model.can_delete_dancer = dancer.is_some();
        view_model.selected_icon_option = dancer
            .as_ref()
            .and_then(|value| {
                view_model
                    .icon_options
                    .iter()
                    .find(|option| is_icon_match(option, value.icon.as_deref()))
                    .cloned()
            });
        view_model.selected_role = dancer.as_ref().map(|value| value.role.clone());
    }
}

impl Behavior<DancerSettingsViewModel> for SelectedDancerStateBehavior {
    fn activate(&self, _view_model: &mut DancerSettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "SelectedDancerStateBehavior",
            "DancerSettingsViewModel",
        );
    }
}

pub struct SelectedIconBehavior;

impl SelectedIconBehavior {
    pub fn update_selected_icon(view_model: &mut DancerSettingsViewModel) {
        let Some(selected) = view_model.selected_dancer.as_ref() else {
            return;
        };
        let icon_value = view_model
            .selected_icon_option
            .as_ref()
            .map(|option| normalize_icon_name(&option.icon_name));

        let updated = Rc::new(DancerModel {
            icon: icon_value,
            ..(**selected).clone()
        });

        if let Some(index) = view_model
            .dancers
            .iter()
            .position(|dancer| dancer.dancer_id == updated.dancer_id)
        {
            view_model.dancers[index] = updated.clone();
        }
        view_model.selected_dancer = Some(updated);
    }
}

impl Behavior<DancerSettingsViewModel> for SelectedIconBehavior {
    fn activate(&self, _view_model: &mut DancerSettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("SelectedIconBehavior", "DancerSettingsViewModel");
    }
}

fn default_role(name: &str) -> RoleModel {
    RoleModel {
        z_index: 0,
        name: name.to_string(),
        color: Colors::transparent(),
    }
}

fn ensure_default_roles(roles: &mut Vec<Rc<RoleModel>>) {
    if !roles.is_empty() {
        return;
    }

    roles.push(Rc::new(default_role("Dame")));
    roles.push(Rc::new(default_role("Herr")));
}

fn next_dancer_id(dancers: &[Rc<DancerModel>]) -> DancerId {
    let mut max_id = 0;
    for dancer in dancers {
        let value = dancer.dancer_id.0;
        if value > max_id {
            max_id = value;
        }
    }
    DancerId(max_id + 1)
}

fn update_scene_dancers(scene: &mut SceneModel, dancer_map: &HashMap<DancerId, Rc<DancerModel>>) {
    for index in (0..scene.positions.len()).rev() {
        let dancer_id = scene.positions[index]
            .dancer
            .as_ref()
            .map(|dancer| dancer.dancer_id);
        let Some(dancer_id) = dancer_id else {
            continue;
        };

        if let Some(new_dancer) = dancer_map.get(&dancer_id) {
            scene.positions[index].dancer = Some(new_dancer.clone());
        } else {
            scene.positions.remove(index);
        }
    }

    for variation in &mut scene.variations {
        for variation_scene in variation {
            update_scene_dancers(variation_scene, dancer_map);
        }
    }

    for variation_scene in &mut scene.current_variation {
        update_scene_dancers(variation_scene, dancer_map);
    }
}

fn is_icon_match(option: &IconOption, icon_value: Option<&str>) -> bool {
    let Some(icon_value) = icon_value else {
        return false;
    };

    if option.key.eq_ignore_ascii_case(icon_value) {
        return true;
    }

    let normalized = icon_value.replace('\\', "/");
    let file_name = normalized
        .split('/')
        .next_back()
        .unwrap_or(icon_value);
    let name = file_name
        .split('.')
        .next()
        .unwrap_or(file_name);

    option.key.eq_ignore_ascii_case(name)
        || option.icon_name.eq_ignore_ascii_case(name)
}

fn normalize_icon_name(icon_name: &str) -> String {
    let normalized = icon_name.replace('\\', "/");
    let file_name = normalized
        .split('/')
        .next_back()
        .unwrap_or(&normalized);
    let name = file_name
        .split('.')
        .next()
        .unwrap_or(file_name);
    if name.trim().is_empty() {
        icon_name.to_string()
    } else {
        name.to_string()
    }
}

pub struct SelectedRoleBehavior;

impl SelectedRoleBehavior {
    pub fn update_selected_role(view_model: &mut DancerSettingsViewModel) {
        let (Some(selected), Some(role)) = (
            view_model.selected_dancer.as_ref(),
            view_model.selected_role.as_ref(),
        ) else {
            return;
        };

        let updated = Rc::new(DancerModel {
            role: role.clone(),
            ..(**selected).clone()
        });

        if let Some(index) = view_model
            .dancers
            .iter()
            .position(|dancer| dancer.dancer_id == updated.dancer_id)
        {
            view_model.dancers[index] = updated.clone();
        }
        view_model.selected_dancer = Some(updated);
    }
}

impl Behavior<DancerSettingsViewModel> for SelectedRoleBehavior {
    fn activate(&self, _view_model: &mut DancerSettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("SelectedRoleBehavior", "DancerSettingsViewModel");
    }
}

pub struct ShowDancerDialogBehavior {
    receiver: crossbeam_channel::Receiver<ShowDancerDialogCommand>,
}

impl ShowDancerDialogBehavior {
    pub fn new(receiver: crossbeam_channel::Receiver<ShowDancerDialogCommand>) -> Self {
        Self { receiver }
    }

    pub fn try_handle(&self, view_model: &mut DancerSettingsViewModel) -> bool {
        match self.receiver.try_recv() {
            Ok(command) => {
                view_model.dialog_content = command.content_id.clone();
                view_model.is_dialog_open = command.content_id.is_some();
                true
            }
            Err(_) => false,
        }
    }
}

impl Behavior<DancerSettingsViewModel> for ShowDancerDialogBehavior {
    fn activate(&self, _view_model: &mut DancerSettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("ShowDancerDialogBehavior", "DancerSettingsViewModel");
    }
}

pub struct SwapDancersBehavior {
    haptic_feedback: Option<Box<dyn HapticFeedback>>,
    show_dialog_sender: crossbeam_channel::Sender<ShowDancerDialogCommand>,
}

impl SwapDancersBehavior {
    pub fn new(
        haptic_feedback: Option<Box<dyn HapticFeedback>>,
        show_dialog_sender: crossbeam_channel::Sender<ShowDancerDialogCommand>,
    ) -> Self {
        Self {
            haptic_feedback,
            show_dialog_sender,
        }
    }

    pub fn show_swap_dialog(&self, view_model: &DancerSettingsViewModel) {
        let (Some(from), Some(to)) = (
            view_model.swap_from_dancer.as_ref(),
            view_model.swap_to_dancer.as_ref(),
        ) else {
            return;
        };

        if from.dancer_id == to.dancer_id {
            return;
        }

        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }

        let _ = self.show_dialog_sender.send(ShowDancerDialogCommand {
            content_id: Some("swap_dancers".to_string()),
        });
    }
}

impl Behavior<DancerSettingsViewModel> for SwapDancersBehavior {
    fn activate(&self, _view_model: &mut DancerSettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated("SwapDancersBehavior", "DancerSettingsViewModel");
    }
}

pub struct SwapDancerSelectionBehavior;

impl SwapDancerSelectionBehavior {
    pub fn ensure_swap_selections(view_model: &mut DancerSettingsViewModel) {
        if view_model.dancers.is_empty() {
            view_model.swap_from_dancer = None;
            view_model.swap_to_dancer = None;
            Self::update_can_swap(view_model);
            return;
        }

        if view_model
            .swap_from_dancer
            .as_ref()
            .map(|dancer| !view_model.dancers.iter().any(|item| item.dancer_id == dancer.dancer_id))
            .unwrap_or(true)
        {
            view_model.swap_from_dancer = view_model.dancers.first().cloned();
        }

        if view_model.dancers.len() < 2 {
            view_model.swap_to_dancer = None;
            Self::update_can_swap(view_model);
            return;
        }

        if view_model
            .swap_to_dancer
            .as_ref()
            .map(|dancer| !view_model.dancers.iter().any(|item| item.dancer_id == dancer.dancer_id))
            .unwrap_or(true)
            || view_model.swap_to_dancer.as_ref().map(|dancer| {
                view_model.swap_from_dancer.as_ref().map(|from| from.dancer_id == dancer.dancer_id).unwrap_or(false)
            }).unwrap_or(false)
        {
            view_model.swap_to_dancer = view_model.dancers.iter().find(|dancer| {
                view_model.swap_from_dancer.as_ref().map(|from| from.dancer_id != dancer.dancer_id).unwrap_or(true)
            }).cloned();
        }

        Self::update_can_swap(view_model);
    }

    pub fn update_can_swap(view_model: &mut DancerSettingsViewModel) {
        view_model.can_swap_dancers = view_model.swap_from_dancer.is_some()
            && view_model.swap_to_dancer.is_some()
            && view_model
                .swap_from_dancer
                .as_ref()
                .zip(view_model.swap_to_dancer.as_ref())
                .map(|(from, to)| from.dancer_id != to.dancer_id)
                .unwrap_or(false);
    }
}

impl Behavior<DancerSettingsViewModel> for SwapDancerSelectionBehavior {
    fn activate(&self, _view_model: &mut DancerSettingsViewModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "SwapDancerSelectionBehavior",
            "DancerSettingsViewModel",
        );
    }
}

pub struct DancerSettingsDependencies {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub show_dialog_sender: crossbeam_channel::Sender<ShowDancerDialogCommand>,
    pub close_dialog_sender: crossbeam_channel::Sender<CloseDancerDialogCommand>,
    pub show_dialog_receiver: crossbeam_channel::Receiver<ShowDancerDialogCommand>,
    pub close_dialog_receiver: crossbeam_channel::Receiver<CloseDancerDialogCommand>,
    pub haptic_feedback: Option<Box<dyn HapticFeedback>>,
}

pub fn build_dancer_settings_behaviors(
    deps: DancerSettingsDependencies,
) -> Vec<Box<dyn Behavior<DancerSettingsViewModel>>> {
    vec![
        Box::new(LoadDancerSettingsBehavior::new(deps.global_state.clone())),
        Box::new(AddDancerBehavior),
        Box::new(DeleteDancerBehavior),
        Box::new(SelectedDancerStateBehavior),
        Box::new(SelectedIconBehavior),
        Box::new(SelectedRoleBehavior),
        Box::new(SwapDancerSelectionBehavior),
        Box::new(SwapDancersBehavior::new(
            deps.haptic_feedback,
            deps.show_dialog_sender,
        )),
        Box::new(HideDancerDialogBehavior::new(deps.close_dialog_receiver)),
        Box::new(ShowDancerDialogBehavior::new(deps.show_dialog_receiver)),
        Box::new(CancelDancerSettingsBehavior),
        Box::new(SaveDancerSettingsBehavior::new(deps.global_state)),
    ]
}
