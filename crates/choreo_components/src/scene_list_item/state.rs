use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::SceneId;
use choreo_models::PositionModel;
use choreo_models::SceneModel;

#[derive(Debug, Clone, PartialEq)]
pub struct SceneItemState {
    pub scene_id: SceneId,
    pub name: String,
    pub text: String,
    pub fixed_positions: bool,
    pub timestamp: Option<f64>,
    pub is_selected: bool,
    pub positions: Vec<PositionModel>,
    pub variation_depth: i32,
    pub variations: Vec<Vec<SceneModel>>,
    pub current_variation: Vec<SceneModel>,
    pub color: Color,
}

impl SceneItemState {
    #[must_use]
    pub fn new(scene_id: SceneId, name: impl Into<String>, color: Color) -> Self {
        Self {
            scene_id,
            name: name.into(),
            text: String::new(),
            fixed_positions: false,
            timestamp: None,
            is_selected: false,
            positions: Vec::new(),
            variation_depth: 0,
            variations: Vec::new(),
            current_variation: Vec::new(),
            color,
        }
    }
}
