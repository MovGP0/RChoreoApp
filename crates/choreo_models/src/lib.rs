#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use choreo_master_mobile_json::{
    Choreography, Color, Dancer, DancerId, Floor, FrontPosition, Position, Role, Scene, SceneId,
    Settings,
};
use std::collections::HashMap;
use std::rc::Rc;
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloneMode {
    Deep,
    Shallow,
}

pub struct SettingsPreferenceKeys;

impl SettingsPreferenceKeys {
    pub const THEME: &str = "Theme";
    pub const USE_SYSTEM_THEME: &str = "UseSystemTheme";
    pub const USE_PRIMARY_COLOR: &str = "UsePrimaryColor";
    pub const USE_SECONDARY_COLOR: &str = "UseSecondaryColor";
    pub const USE_TERTIARY_COLOR: &str = "UseTertiaryColor";
    pub const PRIMARY_COLOR: &str = "PrimaryColor";
    pub const SECONDARY_COLOR: &str = "SecondaryColor";
    pub const TERTIARY_COLOR: &str = "TertiaryColor";
    pub const LAST_OPENED_CHOREO_FILE: &str = "LastOpenedChoreoFile";
    pub const LAST_OPENED_AUDIO_FILE: &str = "LastOpenedAudioFile";
    pub const LAST_OPENED_SVG_FILE: &str = "LastOpenedSvgFile";
    pub const DRAW_PATH_FROM: &str = "DrawPathFrom";
    pub const DRAW_PATH_TO: &str = "DrawPathTo";
    pub const POSITIONS_AT_SIDE: &str = "PositionsAtSide";
    pub const SHOW_TIMESTAMPS: &str = "ShowTimestamps";
    pub const SNAP_TO_GRID: &str = "SnapToGrid";
    pub const SHOW_LEGEND: &str = "ShowLegend";
}

pub struct Colors;

impl Colors {
    pub fn transparent() -> Color {
        Color::transparent()
    }

    pub fn blue() -> Color {
        Color {
            a: 255,
            r: 0,
            g: 0,
            b: 255,
        }
    }

    pub fn red() -> Color {
        Color {
            a: 255,
            r: 255,
            g: 0,
            b: 0,
        }
    }

    pub fn purple() -> Color {
        Color {
            a: 255,
            r: 128,
            g: 0,
            b: 128,
        }
    }

    pub fn orange() -> Color {
        Color {
            a: 255,
            r: 255,
            g: 165,
            b: 0,
        }
    }

    pub fn teal() -> Color {
        Color {
            a: 255,
            r: 0,
            g: 128,
            b: 128,
        }
    }

    pub fn green() -> Color {
        Color {
            a: 255,
            r: 0,
            g: 128,
            b: 0,
        }
    }

    pub fn gold() -> Color {
        Color {
            a: 255,
            r: 255,
            g: 215,
            b: 0,
        }
    }

    pub fn cyan() -> Color {
        Color {
            a: 255,
            r: 0,
            g: 255,
            b: 255,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FloorModel {
    pub size_front: i32,
    pub size_back: i32,
    pub size_left: i32,
    pub size_right: i32,
}

impl FloorModel {
    pub fn clone_with(&self, _mode: CloneMode) -> Self {
        self.clone()
    }
}

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

    fn clone_internal(source: &Self, role_map: &mut HashMap<usize, Rc<RoleModel>>) -> Self {
        let role = clone_role(&source.role, role_map);
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

    fn clone_internal(
        source: &Self,
        dancer_map: &mut HashMap<usize, Rc<DancerModel>>,
        role_map: &mut HashMap<usize, Rc<RoleModel>>,
    ) -> Self {
        Self {
            dancer: clone_dancer(source.dancer.as_ref(), dancer_map, role_map),
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

    fn clone_internal(
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

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsModel {
    pub animation_milliseconds: i32,
    pub front_position: FrontPosition,
    pub dancer_position: FrontPosition,
    pub resolution: i32,
    pub transparency: f64,
    pub positions_at_side: bool,
    pub grid_lines: bool,
    pub snap_to_grid: bool,
    pub floor_color: Color,
    pub dancer_size: f64,
    pub show_timestamps: bool,
    pub music_path_absolute: Option<String>,
    pub music_path_relative: Option<String>,
}

impl SettingsModel {
    pub fn clone_with(&self, _mode: CloneMode) -> Self {
        self.clone()
    }
}

impl Default for SettingsModel {
    fn default() -> Self {
        Self {
            animation_milliseconds: 0,
            front_position: FrontPosition::Top,
            dancer_position: FrontPosition::Top,
            resolution: 0,
            transparency: 0.0,
            positions_at_side: false,
            grid_lines: false,
            snap_to_grid: true,
            floor_color: Color::transparent(),
            dancer_size: 0.0,
            show_timestamps: false,
            music_path_absolute: None,
            music_path_relative: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoreographyModel {
    pub comment: Option<String>,
    pub settings: SettingsModel,
    pub floor: FloorModel,
    pub roles: Vec<Rc<RoleModel>>,
    pub dancers: Vec<Rc<DancerModel>>,
    pub scenes: Vec<SceneModel>,
    pub name: String,
    pub subtitle: Option<String>,
    pub date: Option<String>,
    pub variation: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub last_save_date: OffsetDateTime,
}

impl ChoreographyModel {
    pub fn clone_with(&self, mode: CloneMode) -> Self {
        if matches!(mode, CloneMode::Shallow) {
            return self.clone();
        }

        let mut role_map = HashMap::new();
        let mut dancer_map = HashMap::new();

        let roles = self
            .roles
            .iter()
            .map(|role| clone_role(role, &mut role_map))
            .collect::<Vec<_>>();
        let dancers = self
            .dancers
            .iter()
            .map(|dancer| {
                clone_dancer(Some(dancer), &mut dancer_map, &mut role_map)
                    .unwrap_or_else(|| dancer.clone())
            })
            .collect::<Vec<_>>();
        let scenes = self
            .scenes
            .iter()
            .map(|scene| scene.clone_internal(&mut dancer_map, &mut role_map))
            .collect();

        Self {
            comment: self.comment.clone(),
            settings: self.settings.clone(),
            floor: self.floor.clone(),
            roles,
            dancers,
            scenes,
            name: self.name.clone(),
            subtitle: self.subtitle.clone(),
            date: self.date.clone(),
            variation: self.variation.clone(),
            author: self.author.clone(),
            description: self.description.clone(),
            last_save_date: self.last_save_date,
        }
    }
}

impl Default for ChoreographyModel {
    fn default() -> Self {
        Self {
            comment: None,
            settings: SettingsModel::default(),
            floor: FloorModel::default(),
            roles: Vec::new(),
            dancers: Vec::new(),
            scenes: Vec::new(),
            name: String::new(),
            subtitle: None,
            date: None,
            variation: None,
            author: None,
            description: None,
            last_save_date: OffsetDateTime::now_utc(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoreographyModelMapper;

impl ChoreographyModelMapper {
    pub fn map_to_model(&self, source: &Choreography) -> ChoreographyModel {
        let settings = map_settings_to_model(&source.settings);
        let floor = map_floor_to_model(&source.floor);

        let mut roles = Vec::with_capacity(source.roles.len());
        for role in &source.roles {
            roles.push(Rc::new(map_role_to_model(role)));
        }

        let mut dancers = Vec::with_capacity(source.dancers.len());
        let mut dancers_by_id = HashMap::new();
        for dancer in &source.dancers {
            let role_model = find_role_model(&roles, &dancer.role)
                .unwrap_or_else(|| Rc::new(map_role_to_model(&dancer.role)));
            let dancer_model = Rc::new(map_dancer_to_model(dancer, role_model));
            if dancer.dancer_id.0 != 0 {
                dancers_by_id.insert(dancer.dancer_id, dancer_model.clone());
            }
            dancers.push(dancer_model);
        }

        let scenes = source
            .scenes
            .iter()
            .map(|scene| map_scene_to_model(scene, &dancers, &dancers_by_id))
            .collect();

        ChoreographyModel {
            comment: source.comment.clone(),
            settings,
            floor,
            roles,
            dancers,
            scenes,
            name: source.name.clone(),
            subtitle: source.subtitle.clone(),
            date: source.date.clone(),
            variation: source.variation.clone(),
            author: source.author.clone(),
            description: source.description.clone(),
            last_save_date: source.last_save_date,
        }
    }

    pub fn map_to_json(&self, source: &ChoreographyModel) -> Choreography {
        let settings = map_settings_from_model(&source.settings);
        let floor = map_floor_from_model(&source.floor);

        let mut roles = Vec::with_capacity(source.roles.len());
        let mut role_map = HashMap::new();
        for role in &source.roles {
            let mapped = map_role_from_model(role);
            role_map.insert(role_ptr(role), mapped.clone());
            roles.push(mapped);
        }

        let mut dancers = Vec::with_capacity(source.dancers.len());
        let mut dancer_map = HashMap::new();
        for dancer in &source.dancers {
            let role = role_map
                .get(&role_ptr(&dancer.role))
                .cloned()
                .unwrap_or_else(|| map_role_from_model(&dancer.role));
            let mapped = map_dancer_from_model(dancer, role);
            dancer_map.insert(dancer_ptr(dancer), mapped.clone());
            dancers.push(mapped);
        }

        let scenes = source
            .scenes
            .iter()
            .map(|scene| map_scene_from_model(scene, &dancer_map))
            .collect();

        Choreography {
            comment: source.comment.clone(),
            settings,
            floor,
            roles,
            dancers,
            scenes,
            name: source.name.clone(),
            subtitle: source.subtitle.clone(),
            date: source.date.clone(),
            variation: source.variation.clone(),
            author: source.author.clone(),
            description: source.description.clone(),
            last_save_date: source.last_save_date,
        }
    }
}

fn role_ptr(role: &Rc<RoleModel>) -> usize {
    Rc::as_ptr(role) as usize
}

fn dancer_ptr(dancer: &Rc<DancerModel>) -> usize {
    Rc::as_ptr(dancer) as usize
}

fn clone_role(role: &Rc<RoleModel>, role_map: &mut HashMap<usize, Rc<RoleModel>>) -> Rc<RoleModel> {
    let key = role_ptr(role);
    if let Some(existing) = role_map.get(&key) {
        return existing.clone();
    }
    let cloned = Rc::new(role.clone_with(CloneMode::Deep));
    role_map.insert(key, cloned.clone());
    cloned
}

fn clone_dancer(
    dancer: Option<&Rc<DancerModel>>,
    dancer_map: &mut HashMap<usize, Rc<DancerModel>>,
    role_map: &mut HashMap<usize, Rc<RoleModel>>,
) -> Option<Rc<DancerModel>> {
    let dancer = dancer?;
    let key = dancer_ptr(dancer);
    if let Some(existing) = dancer_map.get(&key) {
        return Some(existing.clone());
    }
    let role = clone_role(&dancer.role, role_map);
    let cloned = Rc::new(DancerModel {
        dancer_id: dancer.dancer_id,
        role,
        name: dancer.name.clone(),
        shortcut: dancer.shortcut.clone(),
        color: dancer.color.clone(),
        icon: dancer.icon.clone(),
    });
    dancer_map.insert(key, cloned.clone());
    Some(cloned)
}

fn map_settings_to_model(source: &Settings) -> SettingsModel {
    SettingsModel {
        animation_milliseconds: source.animation_milliseconds,
        front_position: source.front_position,
        dancer_position: source.dancer_position,
        resolution: source.resolution,
        transparency: source.transparency,
        positions_at_side: source.positions_at_side,
        grid_lines: source.grid_lines,
        snap_to_grid: source.snap_to_grid,
        floor_color: source.floor_color.clone(),
        dancer_size: source.dancer_size,
        show_timestamps: source.show_timestamps,
        music_path_absolute: source.music_path_absolute.clone(),
        music_path_relative: source.music_path_relative.clone(),
    }
}

fn map_settings_from_model(source: &SettingsModel) -> Settings {
    Settings {
        animation_milliseconds: source.animation_milliseconds,
        front_position: source.front_position,
        dancer_position: source.dancer_position,
        resolution: source.resolution,
        transparency: source.transparency,
        positions_at_side: source.positions_at_side,
        grid_lines: source.grid_lines,
        snap_to_grid: source.snap_to_grid,
        floor_color: source.floor_color.clone(),
        dancer_size: source.dancer_size,
        show_timestamps: source.show_timestamps,
        music_path_absolute: source.music_path_absolute.clone(),
        music_path_relative: source.music_path_relative.clone(),
    }
}

fn map_floor_to_model(source: &Floor) -> FloorModel {
    FloorModel {
        size_front: source.size_front,
        size_back: source.size_back,
        size_left: source.size_left,
        size_right: source.size_right,
    }
}

fn map_floor_from_model(source: &FloorModel) -> Floor {
    Floor {
        size_front: source.size_front,
        size_back: source.size_back,
        size_left: source.size_left,
        size_right: source.size_right,
    }
}

fn map_role_to_model(source: &Role) -> RoleModel {
    RoleModel {
        z_index: source.z_index,
        name: source.name.clone(),
        color: source.color.clone(),
    }
}

fn map_role_from_model(source: &RoleModel) -> Role {
    Role {
        z_index: source.z_index,
        name: source.name.clone(),
        color: source.color.clone(),
    }
}

fn map_dancer_to_model(source: &Dancer, role: Rc<RoleModel>) -> DancerModel {
    DancerModel {
        dancer_id: source.dancer_id,
        role,
        name: source.name.clone(),
        shortcut: source.shortcut.clone(),
        color: source.color.clone(),
        icon: source.icon.clone(),
    }
}

fn map_dancer_from_model(source: &DancerModel, role: Role) -> Dancer {
    Dancer {
        dancer_id: source.dancer_id,
        role,
        name: source.name.clone(),
        shortcut: source.shortcut.clone(),
        color: source.color.clone(),
        icon: source.icon.clone(),
    }
}

fn map_scene_to_model(
    source: &Scene,
    dancers: &[Rc<DancerModel>],
    dancers_by_id: &HashMap<DancerId, Rc<DancerModel>>,
) -> SceneModel {
    let mut positions = Vec::new();
    if let Some(source_positions) = &source.positions {
        positions = source_positions
            .iter()
            .map(|position| map_position_to_model(position, dancers, dancers_by_id))
            .collect();
    }

    SceneModel {
        scene_id: source.scene_id,
        positions,
        name: source.name.clone(),
        text: source.text.clone(),
        fixed_positions: source.fixed_positions,
        timestamp: source.timestamp.clone(),
        variation_depth: source.variation_depth,
        variations: map_scene_variations_to_model(&source.variations, dancers, dancers_by_id),
        current_variation: map_scene_list_to_model(&source.current_variation, dancers, dancers_by_id),
        color: source.color.clone(),
    }
}

fn map_scene_from_model(source: &SceneModel, dancer_map: &HashMap<usize, Dancer>) -> Scene {
    let positions = if source.positions.is_empty() {
        None
    } else {
        Some(
            source
                .positions
                .iter()
                .map(|position| map_position_from_model(position, dancer_map))
                .collect(),
        )
    };

    Scene {
        scene_id: source.scene_id,
        positions,
        name: source.name.clone(),
        text: source.text.clone(),
        fixed_positions: source.fixed_positions,
        timestamp: source.timestamp.clone(),
        variation_depth: source.variation_depth,
        variations: map_scene_variations_from_model(&source.variations, dancer_map),
        current_variation: map_scene_list_from_model(&source.current_variation, dancer_map),
        color: source.color.clone(),
    }
}

fn map_position_to_model(
    source: &Position,
    dancers: &[Rc<DancerModel>],
    dancers_by_id: &HashMap<DancerId, Rc<DancerModel>>,
) -> PositionModel {
    let dancer = source.dancer.as_ref().and_then(|dancer| {
        if dancer.dancer_id.0 != 0 {
            dancers_by_id.get(&dancer.dancer_id).cloned()
        } else {
            dancers
                .iter()
                .find(|candidate| dancer_matches_model(candidate, dancer))
                .cloned()
        }
    });

    PositionModel {
        dancer,
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

fn dancer_matches_model(candidate: &Rc<DancerModel>, dancer: &Dancer) -> bool {
    candidate.dancer_id == dancer.dancer_id
        && candidate.name == dancer.name
        && candidate.shortcut == dancer.shortcut
        && candidate.color == dancer.color
        && candidate.icon == dancer.icon
        && role_matches_model(&candidate.role, &dancer.role)
}

fn role_matches_model(candidate: &Rc<RoleModel>, role: &Role) -> bool {
    candidate.z_index == role.z_index && candidate.name == role.name && candidate.color == role.color
}

fn map_position_from_model(
    source: &PositionModel,
    dancer_map: &HashMap<usize, Dancer>,
) -> Position {
    let dancer = source
        .dancer
        .as_ref()
        .and_then(|dancer| dancer_map.get(&dancer_ptr(dancer)).cloned())
        .or_else(|| source.dancer.as_ref().map(|dancer| {
            let role = map_role_from_model(&dancer.role);
            map_dancer_from_model(dancer, role)
        }));

    Position {
        dancer,
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

fn map_scene_variations_to_model(
    variations: &Option<Vec<Vec<Scene>>>,
    dancers: &[Rc<DancerModel>],
    dancers_by_id: &HashMap<DancerId, Rc<DancerModel>>,
) -> Vec<Vec<SceneModel>> {
    let Some(variations) = variations else {
        return Vec::new();
    };

    variations
        .iter()
        .map(|list| {
            list.iter()
                .map(|scene| map_scene_to_model(scene, dancers, dancers_by_id))
                .collect()
        })
        .collect()
}

fn map_scene_variations_from_model(
    variations: &[Vec<SceneModel>],
    dancer_map: &HashMap<usize, Dancer>,
) -> Option<Vec<Vec<Scene>>> {
    if variations.is_empty() {
        return None;
    }

    Some(
        variations
            .iter()
            .map(|list| list.iter().map(|scene| map_scene_from_model(scene, dancer_map)).collect())
            .collect(),
    )
}

fn map_scene_list_to_model(
    scenes: &Option<Vec<Scene>>,
    dancers: &[Rc<DancerModel>],
    dancers_by_id: &HashMap<DancerId, Rc<DancerModel>>,
) -> Vec<SceneModel> {
    let Some(scenes) = scenes else {
        return Vec::new();
    };

    scenes
        .iter()
        .map(|scene| map_scene_to_model(scene, dancers, dancers_by_id))
        .collect()
}

fn map_scene_list_from_model(
    scenes: &[SceneModel],
    dancer_map: &HashMap<usize, Dancer>,
) -> Option<Vec<Scene>> {
    if scenes.is_empty() {
        return None;
    }

    Some(
        scenes
            .iter()
            .map(|scene| map_scene_from_model(scene, dancer_map))
            .collect(),
    )
}

fn find_role_model(roles: &[Rc<RoleModel>], role: &Role) -> Option<Rc<RoleModel>> {
    roles
        .iter()
        .find(|candidate| {
            candidate.z_index == role.z_index
                && candidate.name == role.name
                && candidate.color == role.color
        })
        .cloned()
}
