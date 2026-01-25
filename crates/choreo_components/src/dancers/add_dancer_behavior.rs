use std::rc::Rc;

use choreo_models::DancerModel;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::mapper::{default_role, ensure_default_roles, next_dancer_id};
use super::dancer_settings_view_model::DancerSettingsViewModel;

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
    fn activate(
        &self,
        _view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("AddDancerBehavior", "DancerSettingsViewModel");
    }
}


