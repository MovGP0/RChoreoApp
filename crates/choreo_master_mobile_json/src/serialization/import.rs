use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use time::OffsetDateTime;

use crate::clock::SystemClock;
use crate::errors::ChoreoJsonError;
use crate::models::{
    Choreography, Color, Dancer, DancerId, Floor, Position, Role, Scene, SceneId, Settings,
};
use crate::serialization::helpers::get_string;

pub fn import(json: &str) -> Result<Choreography, ChoreoJsonError> {
    let value: Value = serde_json::from_str(json)?;
    from_value(&value)
}

pub fn import_from_file(path: impl AsRef<Path>) -> Result<Choreography, ChoreoJsonError> {
    let json = fs::read_to_string(path)?;
    import(&json)
}

fn from_value(value: &Value) -> Result<Choreography, ChoreoJsonError> {
    let root = value
        .as_object()
        .ok_or(ChoreoJsonError::ExpectedObject("Choreography"))?;

    let comment = root.get("_Comment").and_then(|v| v.as_str()).map(String::from);
    let name = get_string(root, "Name")?.unwrap_or_default();
    let subtitle = get_string(root, "Subtitle")?;
    let date = get_string(root, "Date")?;
    let variation = get_string(root, "Variation")?;
    let author = get_string(root, "Author")?;
    let description = get_string(root, "Description")?;
    let last_save_date = get_string(root, "LastSaveDate")?
        .and_then(|value| {
            OffsetDateTime::parse(&value, &time::format_description::well_known::Rfc3339).ok()
        })
        .unwrap_or_else(SystemClock::now_utc);

    let settings_value = root
        .get("Settings")
        .ok_or(ChoreoJsonError::MissingField("Settings"))?;
    let mut settings: Settings = serde_json::from_value(settings_value.clone())?;
    if let Some(color_str) = settings_value.get("FloorColor").and_then(|v| v.as_str()) {
        settings.floor_color = Color::from_hex(color_str).unwrap_or_else(Color::transparent);
    }

    let floor_value = root
        .get("Floor")
        .ok_or(ChoreoJsonError::MissingField("Floor"))?;
    let floor: Floor = serde_json::from_value(floor_value.clone())?;

    let (roles, role_ids) = parse_roles(root.get("Roles"))?;
    let (dancers, dancer_ids) = parse_dancers(root.get("Dancers"), &role_ids)?;
    let scenes = parse_scenes(root.get("Scenes"), &dancer_ids)?;

    Ok(Choreography {
        comment,
        settings,
        floor,
        roles,
        dancers,
        scenes,
        name,
        subtitle,
        date,
        variation,
        author,
        description,
        last_save_date,
    })
}

fn parse_roles(value: Option<&Value>) -> Result<(Vec<Role>, HashMap<String, Role>), ChoreoJsonError> {
    let list = value
        .and_then(Value::as_array)
        .ok_or(ChoreoJsonError::ExpectedArray("Roles"))?;

    let mut roles = Vec::with_capacity(list.len());
    let mut by_id = HashMap::new();

    for item in list {
        let obj = item
            .as_object()
            .ok_or(ChoreoJsonError::ExpectedObject("Role"))?;
        let mut role: Role = serde_json::from_value(Value::Object(obj.clone()))?;
        if let Some(color) = obj.get("Color").and_then(|v| v.as_str()) {
            role.color = Color::from_hex(color).unwrap_or_else(Color::transparent);
        }
        let id = obj.get("$id").and_then(|v| v.as_str()).unwrap_or_default();
        if !id.is_empty() {
            by_id.insert(id.to_string(), role.clone());
        }
        roles.push(role);
    }

    Ok((roles, by_id))
}

fn parse_dancers(
    value: Option<&Value>,
    roles_by_id: &HashMap<String, Role>,
) -> Result<(Vec<Dancer>, HashMap<String, Dancer>), ChoreoJsonError> {
    let list = value
        .and_then(Value::as_array)
        .ok_or(ChoreoJsonError::ExpectedArray("Dancers"))?;

    let mut dancers = Vec::with_capacity(list.len());
    let mut by_id = HashMap::new();

    for item in list {
        let obj = item
            .as_object()
            .ok_or(ChoreoJsonError::ExpectedObject("Dancer"))?;
        let mut dancer: Dancer = serde_json::from_value(Value::Object(obj.clone()))?;
        if let Some(color) = obj.get("Color").and_then(|v| v.as_str()) {
            dancer.color = Color::from_hex(color).unwrap_or_else(Color::transparent);
        }
        dancer.role = parse_role_ref(obj.get("Role"), roles_by_id)?;

        let id = obj.get("$id").and_then(|v| v.as_str()).unwrap_or_default();
        if let Ok(parsed) = id.parse::<i32>() {
            dancer.dancer_id = DancerId(parsed);
        }
        if !id.is_empty() {
            by_id.insert(id.to_string(), dancer.clone());
        }
        dancers.push(dancer);
    }

    Ok((dancers, by_id))
}

fn parse_role_ref(
    value: Option<&Value>,
    roles_by_id: &HashMap<String, Role>,
) -> Result<Role, ChoreoJsonError> {
    let role_value = value.ok_or(ChoreoJsonError::MissingField("Role"))?;
    let obj = role_value
        .as_object()
        .ok_or(ChoreoJsonError::ExpectedObject("Role"))?;
    if let Some(reference) = obj.get("$ref").and_then(|v| v.as_str()) {
        return roles_by_id
            .get(reference)
            .cloned()
            .ok_or_else(|| ChoreoJsonError::InvalidReference(reference.to_string()));
    }

    let mut role: Role = serde_json::from_value(Value::Object(obj.clone()))?;
    if let Some(color) = obj.get("Color").and_then(|v| v.as_str()) {
        role.color = Color::from_hex(color).unwrap_or_else(Color::transparent);
    }
    Ok(role)
}

fn parse_scenes(
    value: Option<&Value>,
    dancers_by_id: &HashMap<String, Dancer>,
) -> Result<Vec<Scene>, ChoreoJsonError> {
    let list = value
        .and_then(Value::as_array)
        .ok_or(ChoreoJsonError::ExpectedArray("Scenes"))?;

    let mut scenes = Vec::with_capacity(list.len());
    for item in list {
        scenes.push(parse_scene(item, dancers_by_id)?);
    }
    Ok(scenes)
}

fn parse_scene(value: &Value, dancers_by_id: &HashMap<String, Dancer>) -> Result<Scene, ChoreoJsonError> {
    let obj = value
        .as_object()
        .ok_or(ChoreoJsonError::ExpectedObject("Scene"))?;

    let mut scene: Scene = serde_json::from_value(Value::Object(obj.clone()))?;
    if let Some(color) = obj.get("Color").and_then(|v| v.as_str()) {
        scene.color = Color::from_hex(color).unwrap_or_else(Color::transparent);
    }

    if let Some(id) = obj.get("$id").and_then(|v| v.as_str())
        && let Ok(parsed) = id.parse::<i32>()
    {
        scene.scene_id = SceneId(parsed);
    }

    if let Some(positions_value) = obj.get("Positions") {
        scene.positions = Some(parse_positions(positions_value, dancers_by_id)?);
    }

    if let Some(variations) = obj.get("Variations") {
        scene.variations = Some(parse_scene_variations(variations, dancers_by_id)?);
    }
    if let Some(current) = obj.get("CurrentVariation") {
        scene.current_variation = Some(parse_scene_list(current, dancers_by_id)?);
    }

    Ok(scene)
}

fn parse_positions(
    value: &Value,
    dancers_by_id: &HashMap<String, Dancer>,
) -> Result<Vec<Position>, ChoreoJsonError> {
    let list = value
        .as_array()
        .ok_or(ChoreoJsonError::ExpectedArray("Positions"))?;
    let mut positions = Vec::with_capacity(list.len());
    for item in list {
        let obj = item
            .as_object()
            .ok_or(ChoreoJsonError::ExpectedObject("Position"))?;
        let mut position: Position = serde_json::from_value(Value::Object(obj.clone()))?;
        position.dancer = parse_dancer_ref(obj.get("Dancer"), dancers_by_id)?;
        positions.push(position);
    }
    Ok(positions)
}

fn parse_dancer_ref(
    value: Option<&Value>,
    dancers_by_id: &HashMap<String, Dancer>,
) -> Result<Option<Dancer>, ChoreoJsonError> {
    let Some(value) = value else { return Ok(None); };
    let obj = value
        .as_object()
        .ok_or(ChoreoJsonError::ExpectedObject("Dancer"))?;
    if let Some(reference) = obj.get("$ref").and_then(|v| v.as_str()) {
        return dancers_by_id
            .get(reference)
            .cloned()
            .ok_or_else(|| ChoreoJsonError::InvalidReference(reference.to_string()))
            .map(Some);
    }

    let mut dancer: Dancer = serde_json::from_value(Value::Object(obj.clone()))?;
    if let Some(color) = obj.get("Color").and_then(|v| v.as_str()) {
        dancer.color = Color::from_hex(color).unwrap_or_else(Color::transparent);
    }
    if let Some(role_value) = obj.get("Role") {
        let role_obj = role_value
            .as_object()
            .ok_or(ChoreoJsonError::ExpectedObject("Role"))?;
        let mut role: Role = serde_json::from_value(Value::Object(role_obj.clone()))?;
        if let Some(color) = role_obj.get("Color").and_then(|v| v.as_str()) {
            role.color = Color::from_hex(color).unwrap_or_else(Color::transparent);
        }
        dancer.role = role;
    }

    Ok(Some(dancer))
}

fn parse_scene_variations(
    value: &Value,
    dancers_by_id: &HashMap<String, Dancer>,
) -> Result<Vec<Vec<Scene>>, ChoreoJsonError> {
    let outer = value
        .as_array()
        .ok_or(ChoreoJsonError::ExpectedArray("Variations"))?;
    let mut variations = Vec::with_capacity(outer.len());
    for item in outer {
        variations.push(parse_scene_list(item, dancers_by_id)?);
    }
    Ok(variations)
}

fn parse_scene_list(
    value: &Value,
    dancers_by_id: &HashMap<String, Dancer>,
) -> Result<Vec<Scene>, ChoreoJsonError> {
    let list = value
        .as_array()
        .ok_or(ChoreoJsonError::ExpectedArray("SceneList"))?;
    let mut scenes = Vec::with_capacity(list.len());
    for item in list {
        scenes.push(parse_scene(item, dancers_by_id)?);
    }
    Ok(scenes)
}
