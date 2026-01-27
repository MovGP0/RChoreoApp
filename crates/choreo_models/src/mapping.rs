use std::collections::HashMap;
use std::rc::Rc;

use choreo_master_mobile_json::{
    Choreography, Dancer, DancerId, Floor, Position, Role, Scene, Settings,
};
use time::Date;
use time::format_description::{self, FormatItem};

use crate::models::{
    ChoreographyModel, DancerModel, FloorModel, PositionModel, RoleModel, SceneModel, SettingsModel,
};

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
            date: parse_date(source.date.as_deref()),
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
            date: format_date(source.date.as_ref()),
            variation: source.variation.clone(),
            author: source.author.clone(),
            description: source.description.clone(),
            last_save_date: source.last_save_date,
        }
    }
}

fn parse_date(source: Option<&str>) -> Option<Date>
{
    let source = source?;
    let format = date_format();
    Date::parse(source, &format).ok()
}

fn format_date(source: Option<&Date>) -> Option<String>
{
    let source = source?;
    let format = date_format();
    source.format(&format).ok()
}

fn date_format() -> Vec<FormatItem<'static>>
{
    format_description::parse("[year]-[month]-[day]")
        .expect("valid date format")
}

pub(crate) fn role_ptr(role: &Rc<RoleModel>) -> usize {
    Rc::as_ptr(role) as usize
}

pub(crate) fn dancer_ptr(dancer: &Rc<DancerModel>) -> usize {
    Rc::as_ptr(dancer) as usize
}

pub(crate) fn clone_role(role: &Rc<RoleModel>, role_map: &mut HashMap<usize, Rc<RoleModel>>) -> Rc<RoleModel> {
    let key = role_ptr(role);
    if let Some(existing) = role_map.get(&key) {
        return existing.clone();
    }
    let cloned = Rc::new(role.clone_with(crate::clone_mode::CloneMode::Deep));
    role_map.insert(key, cloned.clone());
    cloned
}

pub(crate) fn clone_dancer(
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
