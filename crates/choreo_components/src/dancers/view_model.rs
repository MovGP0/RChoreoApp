use std::rc::Rc;

use choreo_i18n::{icon_bytes, icon_names, translation_with_fallback};
use choreo_master_mobile_json::Color;
use choreo_models::{DancerModel, RoleModel};

use crate::audio_player::HapticFeedback;

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
