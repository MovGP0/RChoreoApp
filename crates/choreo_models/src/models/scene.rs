use std::collections::HashMap;
use std::rc::Rc;

use choreo_master_mobile_json::{Color, SceneId};

use crate::clone_mode::CloneMode;
use crate::models::dancer::DancerModel;
use crate::models::position::PositionModel;
use crate::models::role::RoleModel;

#[derive(Debug, Clone, PartialEq)]
pub struct SceneModel {
    pub scene_id: SceneId,
    pub positions: Vec<PositionModel>,
    pub name: String,
    pub text: Option<String>,
    pub fixed_positions: bool,
    pub timestamp: Option<String>,
    pub variation_depth: i32,
    pub variations: Vec<Vec<SceneModel>>,
    pub current_variation: Vec<SceneModel>,
    pub color: Color,
}

impl SceneModel {
    pub fn clone_with(&self, mode: CloneMode) -> Self {
        if matches!(mode, CloneMode::Shallow) {
            return self.clone();
        }

        let mut dancer_map = HashMap::new();
        let mut role_map = HashMap::new();
        self.clone_internal(&mut dancer_map, &mut role_map)
    }

    pub(crate) fn clone_internal(
        &self,
        dancer_map: &mut HashMap<usize, Rc<DancerModel>>,
        role_map: &mut HashMap<usize, Rc<RoleModel>>,
    ) -> Self {
        let positions = self
            .positions
            .iter()
            .map(|position| PositionModel::clone_internal(position, dancer_map, role_map))
            .collect();
        let variations = self
            .variations
            .iter()
            .map(|variation| {
                variation
                    .iter()
                    .map(|scene| scene.clone_internal(dancer_map, role_map))
                    .collect()
            })
            .collect();
        let current_variation = self
            .current_variation
            .iter()
            .map(|scene| scene.clone_internal(dancer_map, role_map))
            .collect();

        Self {
            scene_id: self.scene_id,
            positions,
            name: self.name.clone(),
            text: self.text.clone(),
            fixed_positions: self.fixed_positions,
            timestamp: self.timestamp.clone(),
            variation_depth: self.variation_depth,
            variations,
            current_variation,
            color: self.color.clone(),
        }
    }
}
