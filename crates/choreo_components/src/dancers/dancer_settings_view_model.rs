use std::cell::RefCell;
use std::rc::{Rc, Weak};

use choreo_i18n::{icon_bytes, icon_names, translation_with_fallback};
use choreo_master_mobile_json::Color;
use choreo_models::{DancerModel, RoleModel};
use nject::injectable;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::haptics::HapticFeedback;

use super::messages::CloseDancerDialogCommand;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IconOption {
    pub key: String,
    pub display_name: String,
    pub icon_name: String,
    pub icon_bytes: Option<&'static [u8]>,
}

type DancerActionHandler = Rc<dyn Fn(&mut DancerSettingsViewModel)>;
type DancerIndexHandler = Rc<dyn Fn(&mut DancerSettingsViewModel, usize)>;
type DancerTextHandler = Rc<dyn Fn(&mut DancerSettingsViewModel, String)>;

#[derive(Clone, Default)]
pub struct DancerSettingsViewModelActions {
    pub select_dancer: Option<DancerIndexHandler>,
    pub add_dancer: Option<DancerActionHandler>,
    pub delete_dancer: Option<DancerActionHandler>,
    pub select_role: Option<DancerIndexHandler>,
    pub update_dancer_name: Option<DancerTextHandler>,
    pub update_dancer_shortcut: Option<DancerTextHandler>,
    pub update_dancer_icon: Option<DancerTextHandler>,
    pub swap_dancers: Option<DancerActionHandler>,
    pub save: Option<DancerActionHandler>,
    pub cancel: Option<DancerActionHandler>,
}

#[injectable]
#[inject(
    |haptic_feedback: Option<Box<dyn HapticFeedback>>| {
        Self::new(haptic_feedback)
    }
)]
pub struct DancerSettingsViewModel {
    actions: DancerSettingsViewModelActions,
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
    disposables: CompositeDisposable,
    self_handle: Option<Weak<RefCell<DancerSettingsViewModel>>>,
}

impl DancerSettingsViewModel {
    pub fn new(haptic_feedback: Option<Box<dyn HapticFeedback>>) -> Self {
        Self {
            actions: DancerSettingsViewModelActions::default(),
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
            disposables: CompositeDisposable::new(),
            self_handle: None,
        }
    }

    pub fn activate(
        view_model: &Rc<RefCell<DancerSettingsViewModel>>,
        behaviors: Vec<Box<dyn Behavior<DancerSettingsViewModel>>>,
    ) {
        let mut disposables = CompositeDisposable::new();
        {
            let mut view_model = view_model.borrow_mut();
            for behavior in behaviors.iter() {
                behavior.activate(&mut view_model, &mut disposables);
            }
        }
        view_model.borrow_mut().disposables = disposables;
    }

    pub fn set_actions(&mut self, actions: DancerSettingsViewModelActions) {
        self.actions = actions;
    }

    pub fn set_self_handle(&mut self, handle: Weak<RefCell<DancerSettingsViewModel>>) {
        self.self_handle = Some(handle);
    }

    pub fn self_handle(&self) -> Option<Weak<RefCell<DancerSettingsViewModel>>> {
        self.self_handle.clone()
    }

    pub fn add_dancer(&mut self) {
        self.perform_click();
        if let Some(handler) = self.actions.add_dancer.clone() {
            handler(self);
        }
    }

    pub fn delete_dancer(&mut self) {
        self.perform_click();
        if let Some(handler) = self.actions.delete_dancer.clone() {
            handler(self);
        }
    }

    pub fn select_dancer(&mut self, index: usize) {
        if let Some(handler) = self.actions.select_dancer.clone() {
            handler(self, index);
        }
    }

    pub fn select_role(&mut self, index: usize) {
        if let Some(handler) = self.actions.select_role.clone() {
            handler(self, index);
        }
    }

    pub fn update_dancer_name(&mut self, value: String) {
        if let Some(handler) = self.actions.update_dancer_name.clone() {
            handler(self, value);
        }
    }

    pub fn update_dancer_shortcut(&mut self, value: String) {
        if let Some(handler) = self.actions.update_dancer_shortcut.clone() {
            handler(self, value);
        }
    }

    pub fn update_dancer_icon(&mut self, value: String) {
        if let Some(handler) = self.actions.update_dancer_icon.clone() {
            handler(self, value);
        }
    }

    pub fn swap_dancers(&mut self) {
        self.perform_click();
        if let Some(handler) = self.actions.swap_dancers.clone() {
            handler(self);
        }
    }

    pub fn cancel(&mut self) {
        self.perform_click();
        if let Some(handler) = self.actions.cancel.clone() {
            handler(self);
        }
    }

    pub fn save(&mut self) {
        self.perform_click();
        if let Some(handler) = self.actions.save.clone() {
            handler(self);
        }
    }

    fn perform_click(&self) {
        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }
    }
}

impl Drop for DancerSettingsViewModel {
    fn drop(&mut self) {
        self.disposables.dispose_all();
    }
}

pub struct IconNameToImageSourceConverter;

impl IconNameToImageSourceConverter {
    pub fn convert(icon_name: &str) -> Option<&'static [u8]> {
        let normalized = normalize_icon_name_for_resource(icon_name);
        icon_bytes(&normalized).or_else(|| {
            let fallback = format!("Icon{}", to_pascal_case(&normalized));
            icon_bytes(&fallback)
        })
    }
}

pub struct SwapDancersDialogViewModel {
    close_dialog_sender: crossbeam_channel::Sender<CloseDancerDialogCommand>,
    haptic_feedback: Option<Box<dyn HapticFeedback>>,
    first_dancer: Rc<DancerModel>,
    second_dancer: Rc<DancerModel>,
    pub message: String,
    pub first_dancer_name: String,
    pub second_dancer_name: String,
    pub first_dancer_color: Color,
    pub second_dancer_color: Color,
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

fn normalize_icon_name_for_resource(icon_name: &str) -> String {
    let normalized = icon_name.replace('\\', "/");
    let mut trimmed = normalized.as_str();
    if !trimmed.contains('/') && trimmed.to_ascii_lowercase().starts_with("icon") {
        trimmed = &trimmed[4..];
    }

    let name = trimmed.split('/').next_back().unwrap_or(trimmed).trim();
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
    let template =
        translation_with_fallback(locale, "DancerSwapDialogMessage").unwrap_or("{0} ↔ {1}");
    template.replace("{0}", first).replace("{1}", second)
}
