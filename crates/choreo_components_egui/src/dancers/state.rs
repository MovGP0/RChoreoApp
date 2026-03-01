use choreo_master_mobile_json::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct IconOption {
    pub key: String,
    pub display_name: String,
    pub icon_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RoleState {
    pub name: String,
    pub color: Color,
    pub z_index: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DancerState {
    pub dancer_id: i32,
    pub role: RoleState,
    pub name: String,
    pub shortcut: String,
    pub color: Color,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PositionState {
    pub dancer_id: Option<i32>,
    pub dancer_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneState {
    pub positions: Vec<PositionState>,
    pub variations: Vec<Vec<SceneState>>,
    pub current_variation: Vec<SceneState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneViewState {
    pub positions: Vec<PositionState>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DancersGlobalState {
    pub roles: Vec<RoleState>,
    pub dancers: Vec<DancerState>,
    pub scenes: Vec<SceneState>,
    pub scene_views: Vec<SceneViewState>,
    pub selected_scene: Option<SceneViewState>,
    pub selected_positions: Vec<PositionState>,
    pub selected_positions_snapshot: Vec<PositionState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DancersState {
    pub roles: Vec<RoleState>,
    pub dancers: Vec<DancerState>,
    pub selected_dancer: Option<DancerState>,
    pub selected_role: Option<RoleState>,
    pub selected_icon_option: Option<IconOption>,
    pub has_selected_dancer: bool,
    pub can_delete_dancer: bool,
    pub swap_from_dancer: Option<DancerState>,
    pub swap_to_dancer: Option<DancerState>,
    pub can_swap_dancers: bool,
    pub is_dialog_open: bool,
    pub dialog_content: Option<String>,
    pub icon_options: Vec<IconOption>,
    pub global: DancersGlobalState,
}

impl Default for DancersState {
    fn default() -> Self {
        Self {
            roles: Vec::new(),
            dancers: Vec::new(),
            selected_dancer: None,
            selected_role: None,
            selected_icon_option: None,
            has_selected_dancer: false,
            can_delete_dancer: false,
            swap_from_dancer: None,
            swap_to_dancer: None,
            can_swap_dancers: false,
            is_dialog_open: false,
            dialog_content: None,
            icon_options: default_icon_options(),
            global: DancersGlobalState::default(),
        }
    }
}

impl DancersState {
    #[must_use]
    pub fn with_global(mut self, global: DancersGlobalState) -> Self {
        self.global = global;
        self
    }
}

#[must_use]
pub fn transparent_color() -> Color {
    Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    }
}

#[must_use]
pub fn default_role(name: &str) -> RoleState {
    RoleState {
        name: name.to_string(),
        color: transparent_color(),
        z_index: 0,
    }
}

#[must_use]
pub fn default_icon_options() -> Vec<IconOption> {
    vec![
        IconOption {
            key: "IconCircle".to_string(),
            display_name: "Circle".to_string(),
            icon_name: "IconCircle".to_string(),
        },
        IconOption {
            key: "IconSquare".to_string(),
            display_name: "Square".to_string(),
            icon_name: "IconSquare".to_string(),
        },
        IconOption {
            key: "IconTriangle".to_string(),
            display_name: "Triangle".to_string(),
            icon_name: "IconTriangle".to_string(),
        },
    ]
}
