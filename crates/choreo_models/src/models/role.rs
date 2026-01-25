use choreo_master_mobile_json::Color;

use crate::clone_mode::CloneMode;

#[derive(Debug, Clone, PartialEq)]
pub struct RoleModel {
    pub z_index: i32,
    pub name: String,
    pub color: Color,
}

impl RoleModel {
    pub fn clone_with(&self, _mode: CloneMode) -> Self {
        Self {
            z_index: self.z_index,
            name: self.name.clone(),
            color: self.color.clone(),
        }
    }
}
