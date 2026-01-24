use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use choreo_i18n::{icon_bytes, icon_names, translation_with_fallback};
use choreo_master_mobile_json::DancerId;
use choreo_models::{Colors, DancerModel, RoleModel, SceneModel};

use crate::audio_player::HapticFeedback;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconOption {
    pub key: String,
    pub display_name: String,
    pub icon_name: String,
    pub icon_bytes: Option<&'static [u8]>,
}

pub struct DancerSettingsViewModel {
    pub dancers: Vec<Rc<DancerModel>>,
    pub roles: Vec<Rc<RoleModel>>,
    pub selected_dancer: Option<Rc<DancerModel>>,
    pub selected_role: Option<Rc<RoleModel>>,
    pub selected_icon_option: Option<IconOption>,
    pub has_selected_dancer: bool,
    pub can_delete_dancer: bool,
    pub is_dancer_list_open: bool,
    pub swap_from_dancer: Option<Rc<DancerModel>>,
    pub swap_to_dancer: Option<Rc<DancerModel>>,
    pub can_swap_dancers: bool,
    pub is_dialog_open: bool,
    pub dialog_content: Option<String>,
    pub icon_options: Vec<IconOption>,
    haptic_feedback: Option<Box<dyn HapticFeedback>>,
}

impl DancerSettingsViewModel {
    pub fn new(haptic_feedback: Option<Box<dyn HapticFeedback>>) -> Self {
        Self {
            dancers: Vec::new(),
            roles: Vec::new(),
            selected_dancer: None,
            selected_role: None,
            selected_icon_option: None,
            has_selected_dancer: false,
            can_delete_dancer: false,
            is_dancer_list_open: false,
            swap_from_dancer: None,
            swap_to_dancer: None,
            can_swap_dancers: false,
            is_dialog_open: false,
            dialog_content: None,
            icon_options: load_icon_options(),
            haptic_feedback,
        }
    }

    pub fn add_dancer(&self) {
        self.perform_click();
    }

    pub fn delete_dancer(&self) {
        self.perform_click();
    }

    pub fn swap_dancers(&self) {
        self.perform_click();
    }

    pub fn cancel(&self) {
        self.perform_click();
    }

    pub fn save(&self) {
        self.perform_click();
    }

    fn perform_click(&self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloseDancerDialogCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowDancerDialogCommand {
    pub content_id: Option<String>,
}

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

fn load_icon_options() -> Vec<IconOption> {
    let mut options: Vec<IconOption> = icon_names()
        .iter()
        .map(|&name| IconOption {
            key: name.to_string(),
            display_name: to_display_name(name),
            icon_name: name.to_string(),
            icon_bytes: icon_bytes(name),
        })
        .collect();

    options.sort_by(|left, right| {
        let left = left.display_name.to_ascii_lowercase();
        let right = right.display_name.to_ascii_lowercase();
        left.cmp(&right)
    });

    options
}

fn to_display_name(key: &str) -> String {
    let mut key = key;
    if let Some(stripped) = key.strip_prefix("Icon") {
        key = stripped;
    }

    if key.trim().is_empty() {
        return "Icon".to_string();
    }

    let mut builder = Vec::with_capacity(key.len() + 4);
    let mut previous_is_upper = false;

    for (index, current) in key.chars().enumerate() {
        let is_upper = current.is_ascii_uppercase();
        if index > 0 && is_upper && !previous_is_upper {
            builder.push(' ');
        }
        builder.push(current);
        previous_is_upper = is_upper;
    }

    builder.into_iter().collect()
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

pub struct IconNameToImageSourceConverter;

impl IconNameToImageSourceConverter {
    pub fn convert(icon_name: &str) -> Option<&'static [u8]> {
        let normalized = normalize_icon_name_for_resource(icon_name);
        icon_bytes(&normalized)
            .or_else(|| {
                let fallback = format!("Icon{}", to_pascal_case(&normalized));
                icon_bytes(&fallback)
            })
    }
}

fn normalize_icon_name_for_resource(icon_name: &str) -> String {
    let normalized = icon_name.replace('\\', "/");
    let mut trimmed = normalized.as_str();
    if !trimmed.contains('/') && trimmed.to_ascii_lowercase().starts_with("icon") {
        trimmed = &trimmed[4..];
    }

    let name = trimmed
        .split('/')
        .next_back()
        .unwrap_or(trimmed)
        .trim();
    let name = name.strip_suffix(".png").unwrap_or(name);
    if name.to_ascii_lowercase().starts_with("icon") {
        name.to_string()
    } else {
        format!("Icon{}", to_pascal_case(name))
    }
}

fn to_pascal_case(value: &str) -> String {
    let mut result = String::new();
    let mut upper_next = true;
    for ch in value.chars() {
        if ch == '_' || ch == '-' || ch == ' ' {
            upper_next = true;
            continue;
        }
        if upper_next {
            result.push(ch.to_ascii_uppercase());
            upper_next = false;
        } else {
            result.push(ch);
        }
    }
    result
}

pub struct SwapDancersDialogViewModel {
    close_dialog_sender: crossbeam_channel::Sender<CloseDancerDialogCommand>,
    haptic_feedback: Option<Box<dyn HapticFeedback>>,
    first_dancer: Rc<DancerModel>,
    second_dancer: Rc<DancerModel>,
    pub message: String,
    pub first_dancer_name: String,
    pub second_dancer_name: String,
    pub first_dancer_color: choreo_master_mobile_json::Color,
    pub second_dancer_color: choreo_master_mobile_json::Color,
}

impl SwapDancersDialogViewModel {
    pub fn new(
        close_dialog_sender: crossbeam_channel::Sender<CloseDancerDialogCommand>,
        haptic_feedback: Option<Box<dyn HapticFeedback>>,
        first_dancer: Rc<DancerModel>,
        second_dancer: Rc<DancerModel>,
        locale: &str,
    ) -> Self {
        let first_dancer_name = display_name(&first_dancer);
        let second_dancer_name = display_name(&second_dancer);
        let message = format_swap_message(locale, &first_dancer_name, &second_dancer_name);
        let first_dancer_color = first_dancer.color.clone();
        let second_dancer_color = second_dancer.color.clone();

        Self {
            close_dialog_sender,
            haptic_feedback,
            first_dancer,
            second_dancer,
            message,
            first_dancer_name,
            second_dancer_name,
            first_dancer_color,
            second_dancer_color,
        }
    }

    pub fn confirm_swap(&mut self) {
        self.perform_click();
        swap_properties(&mut self.first_dancer, &mut self.second_dancer);
        self.close_dialog();
    }

    pub fn cancel(&mut self) {
        self.perform_click();
        self.close_dialog();
    }

    fn perform_click(&self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }
    }

    fn close_dialog(&self) {
        let _ = self.close_dialog_sender.send(CloseDancerDialogCommand);
    }
}

fn swap_properties(first: &mut Rc<DancerModel>, second: &mut Rc<DancerModel>) {
    let first_role = first.role.clone();
    let first_name = first.name.clone();
    let first_shortcut = first.shortcut.clone();
    let first_color = first.color.clone();
    let first_icon = first.icon.clone();
    let second_role = second.role.clone();
    let second_name = second.name.clone();
    let second_shortcut = second.shortcut.clone();
    let second_color = second.color.clone();
    let second_icon = second.icon.clone();

    let first_mut = Rc::make_mut(first);
    let second_mut = Rc::make_mut(second);

    first_mut.role = second_role;
    first_mut.name = second_name;
    first_mut.shortcut = second_shortcut;
    first_mut.color = second_color;
    first_mut.icon = second_icon;

    second_mut.role = first_role;
    second_mut.name = first_name;
    second_mut.shortcut = first_shortcut;
    second_mut.color = first_color;
    second_mut.icon = first_icon;
}

fn display_name(dancer: &DancerModel) -> String {
    if !dancer.name.trim().is_empty() {
        return dancer.name.clone();
    }
    if !dancer.shortcut.trim().is_empty() {
        return dancer.shortcut.clone();
    }
    format!("#{}", dancer.dancer_id.0)
}

fn format_swap_message(locale: &str, first: &str, second: &str) -> String {
    let template = translation_with_fallback(locale, "DancerSwapDialogMessage")
        .unwrap_or("{0} ↔ {1}");
    template
        .replace("{0}", first)
        .replace("{1}", second)
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
