use std::collections::HashMap;
use std::rc::Rc;

use choreo_master_mobile_json::{Color, DancerId};

use crate::clone_mode::CloneMode;
use crate::models::role::RoleModel;

#[derive(Debug, Clone, PartialEq)]
pub struct DancerModel {
    pub dancer_id: DancerId,
    pub role: Rc<RoleModel>,
    pub name: String,
    pub shortcut: String,
    pub color: Color,
    pub icon: Option<String>,
}

impl DancerModel {
    pub fn clone_with(&self, mode: CloneMode) -> Self {
        if matches!(mode, CloneMode::Shallow) {
            return self.clone();
        }

        let mut role_map = HashMap::new();
        Self::clone_internal(self, &mut role_map)
    }

    pub(crate) fn clone_internal(
        source: &Self,
        role_map: &mut HashMap<usize, Rc<RoleModel>>,
    ) -> Self {
        let role = crate::mapping::clone_role(&source.role, role_map);
        Self {
            dancer_id: source.dancer_id,
            role,
            name: source.name.clone(),
            shortcut: source.shortcut.clone(),
            color: source.color.clone(),
            icon: source.icon.clone(),
        }
    }
}
