use std::collections::HashMap;
use std::rc::Rc;

use crate::clone_mode::CloneMode;
use crate::models::dancer::DancerModel;
use crate::models::role::RoleModel;

#[derive(Debug, Clone, PartialEq)]
pub struct PositionModel {
    pub dancer: Option<Rc<DancerModel>>,
    pub orientation: Option<f64>,
    pub x: f64,
    pub y: f64,
    pub curve1_x: Option<f64>,
    pub curve1_y: Option<f64>,
    pub curve2_x: Option<f64>,
    pub curve2_y: Option<f64>,
    pub movement1_x: Option<f64>,
    pub movement1_y: Option<f64>,
    pub movement2_x: Option<f64>,
    pub movement2_y: Option<f64>,
}

impl PositionModel {
    pub fn clone_with(&self, mode: CloneMode) -> Self {
        if matches!(mode, CloneMode::Shallow) {
            return self.clone();
        }

        let mut dancer_map = HashMap::new();
        let mut role_map = HashMap::new();
        Self::clone_internal(self, &mut dancer_map, &mut role_map)
    }

    pub(crate) fn clone_internal(
        source: &Self,
        dancer_map: &mut HashMap<usize, Rc<DancerModel>>,
        role_map: &mut HashMap<usize, Rc<RoleModel>>,
    ) -> Self {
        Self {
            dancer: crate::mapping::clone_dancer(source.dancer.as_ref(), dancer_map, role_map),
            orientation: source.orientation,
            x: source.x,
            y: source.y,
            curve1_x: source.curve1_x,
            curve1_y: source.curve1_y,
            curve2_x: source.curve2_x,
            curve2_y: source.curve2_y,
            movement1_x: source.movement1_x,
            movement1_y: source.movement1_y,
            movement2_x: source.movement2_x,
            movement2_y: source.movement2_y,
        }
    }
}
