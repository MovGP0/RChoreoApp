#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;
use thiserror::Error;
use time::OffsetDateTime;

#[derive(Debug, Error)]
pub enum ChoreoJsonError {
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("expected object for {0}")]
    ExpectedObject(&'static str),
    #[error("expected array for {0}")]
    ExpectedArray(&'static str),
    #[error("missing field {0}")]
    MissingField(&'static str),
    #[error("invalid reference id {0}")]
    InvalidReference(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DancerId(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SceneId(pub i32);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color {
    #[serde(skip)]
    pub a: u8,
    #[serde(skip)]
    pub r: u8,
    #[serde(skip)]
    pub g: u8,
    #[serde(skip)]
    pub b: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self::transparent()
    }
}

impl Color {
    pub fn transparent() -> Self {
        Self {
            a: 0,
            r: 0,
            g: 0,
            b: 0,
        }
    }

    pub fn from_hex(value: &str) -> Option<Self> {
        let value = value.trim();
        let hex = value.strip_prefix('#').unwrap_or(value);
        if hex.len() != 8 {
            return None;
        }

        let a = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let r = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let g = u8::from_str_radix(&hex[4..6], 16).ok()?;
        let b = u8::from_str_radix(&hex[6..8], 16).ok()?;
        Some(Self { a, r, g, b })
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.a, self.r, self.g, self.b)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontPosition {
    Top,
    Right,
    Bottom,
    Left,
}

impl FrontPosition {
    fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Top),
            1 => Some(Self::Right),
            2 => Some(Self::Bottom),
            3 => Some(Self::Left),
            _ => None,
        }
    }

    fn to_i32(self) -> i32 {
        match self {
            Self::Top => 0,
            Self::Right => 1,
            Self::Bottom => 2,
            Self::Left => 3,
        }
    }
}

impl Serialize for FrontPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.to_i32())
    }
}

impl<'de> Deserialize<'de> for FrontPosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FrontPositionVisitor;

        impl<'de> serde::de::Visitor<'de> for FrontPositionVisitor {
            type Value = FrontPosition;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an integer 0..=3 or a string front position")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let value = i32::try_from(value)
                    .map_err(|_| E::custom("front position out of range"))?;
                FrontPosition::from_i32(value)
                    .ok_or_else(|| E::custom("front position out of range"))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let value = i32::try_from(value)
                    .map_err(|_| E::custom("front position out of range"))?;
                FrontPosition::from_i32(value)
                    .ok_or_else(|| E::custom("front position out of range"))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value.to_ascii_lowercase().as_str() {
                    "top" => Ok(FrontPosition::Top),
                    "right" => Ok(FrontPosition::Right),
                    "bottom" => Ok(FrontPosition::Bottom),
                    "left" => Ok(FrontPosition::Left),
                    _ => Err(E::custom("unknown front position string")),
                }
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_any(FrontPositionVisitor)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Floor {
    #[serde(rename = "SizeFront")]
    pub size_front: i32,
    #[serde(rename = "SizeBack")]
    pub size_back: i32,
    #[serde(rename = "SizeLeft")]
    pub size_left: i32,
    #[serde(rename = "SizeRight")]
    pub size_right: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Settings {
    #[serde(rename = "AnimationMilliseconds")]
    pub animation_milliseconds: i32,
    #[serde(rename = "FrontPosition")]
    pub front_position: FrontPosition,
    #[serde(rename = "DancerPosition")]
    pub dancer_position: FrontPosition,
    #[serde(rename = "Resolution")]
    pub resolution: i32,
    #[serde(rename = "Transparency")]
    pub transparency: f64,
    #[serde(rename = "PositionsAtSide")]
    pub positions_at_side: bool,
    #[serde(rename = "GridLines")]
    pub grid_lines: bool,
    #[serde(rename = "SnapToGrid", default = "default_snap_to_grid")]
    pub snap_to_grid: bool,
    #[serde(rename = "FloorColor", skip_serializing, skip_deserializing)]
    pub floor_color: Color,
    #[serde(rename = "DancerSize")]
    pub dancer_size: f64,
    #[serde(rename = "ShowTimestamps")]
    pub show_timestamps: bool,
    #[serde(rename = "MusicPathAbsolute")]
    pub music_path_absolute: Option<String>,
    #[serde(rename = "MusicPathRelative")]
    pub music_path_relative: Option<String>,
}

fn default_snap_to_grid() -> bool {
    true
}

impl Default for Settings {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Role {
    #[serde(rename = "ZIndex")]
    pub z_index: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Color", skip_serializing, skip_deserializing)]
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Dancer {
    #[serde(skip)]
    pub dancer_id: DancerId,
    #[serde(rename = "Role", skip_serializing, skip_deserializing)]
    pub role: Role,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Shortcut")]
    pub shortcut: String,
    #[serde(rename = "Color", skip_serializing, skip_deserializing)]
    pub color: Color,
    #[serde(rename = "Icon")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Position {
    #[serde(rename = "Dancer", skip_serializing, skip_deserializing)]
    pub dancer: Option<Dancer>,
    #[serde(rename = "O", skip_serializing_if = "Option::is_none")]
    pub orientation: Option<f64>,
    #[serde(rename = "X")]
    pub x: f64,
    #[serde(rename = "Y")]
    pub y: f64,
    #[serde(rename = "BX", skip_serializing_if = "Option::is_none")]
    pub curve1_x: Option<f64>,
    #[serde(rename = "BY", skip_serializing_if = "Option::is_none")]
    pub curve1_y: Option<f64>,
    #[serde(rename = "CX", skip_serializing_if = "Option::is_none")]
    pub curve2_x: Option<f64>,
    #[serde(rename = "CY", skip_serializing_if = "Option::is_none")]
    pub curve2_y: Option<f64>,
    #[serde(rename = "Movement1X", skip_serializing_if = "Option::is_none")]
    pub movement1_x: Option<f64>,
    #[serde(rename = "Movement1Y", skip_serializing_if = "Option::is_none")]
    pub movement1_y: Option<f64>,
    #[serde(rename = "Movement2X", skip_serializing_if = "Option::is_none")]
    pub movement2_x: Option<f64>,
    #[serde(rename = "Movement2Y", skip_serializing_if = "Option::is_none")]
    pub movement2_y: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Scene {
    #[serde(skip)]
    pub scene_id: SceneId,
    #[serde(rename = "Positions", skip_deserializing)]
    pub positions: Option<Vec<Position>>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Text")]
    pub text: Option<String>,
    #[serde(rename = "FixedPositions")]
    pub fixed_positions: bool,
    #[serde(rename = "Timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "VariationDepth", default)]
    pub variation_depth: i32,
    #[serde(rename = "Variations", skip_deserializing)]
    pub variations: Option<Vec<Vec<Scene>>>,
    #[serde(rename = "CurrentVariation", skip_deserializing)]
    pub current_variation: Option<Vec<Scene>>,
    #[serde(rename = "Color", skip_serializing, skip_deserializing)]
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Choreography {
    #[serde(rename = "_Comment")]
    pub comment: Option<String>,
    #[serde(rename = "Settings")]
    pub settings: Settings,
    #[serde(rename = "Floor")]
    pub floor: Floor,
    #[serde(rename = "Roles")]
    pub roles: Vec<Role>,
    #[serde(rename = "Dancers")]
    pub dancers: Vec<Dancer>,
    #[serde(rename = "Scenes")]
    pub scenes: Vec<Scene>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<String>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "Variation")]
    pub variation: Option<String>,
    #[serde(rename = "Author")]
    pub author: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "LastSaveDate", with = "time::serde::rfc3339")]
    pub last_save_date: OffsetDateTime,
}

impl Default for Choreography {
    fn default() -> Self {
        Self {
            comment: None,
            settings: Settings::default(),
            floor: Floor::default(),
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

pub fn import(json: &str) -> Result<Choreography, ChoreoJsonError> {
    let value: Value = serde_json::from_str(json)?;
    from_value(&value)
}

pub fn import_from_file(path: impl AsRef<Path>) -> Result<Choreography, ChoreoJsonError> {
    let json = fs::read_to_string(path)?;
    import(&json)
}

pub fn export(choreography: &Choreography) -> Result<String, ChoreoJsonError> {
    let value = to_value(choreography)?;
    Ok(serde_json::to_string_pretty(&value)?)
}

pub fn export_to_file(
    path: impl AsRef<Path>,
    choreography: &Choreography,
) -> Result<(), ChoreoJsonError> {
    let json = export(choreography)?;
    fs::write(path, json)?;
    Ok(())
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
        .unwrap_or_else(OffsetDateTime::now_utc);

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

fn to_value(choreography: &Choreography) -> Result<Value, ChoreoJsonError> {
    let mut root = Map::new();
    if let Some(comment) = &choreography.comment {
        root.insert("_Comment".to_string(), Value::String(comment.clone()));
    }

    let mut settings = serde_json::to_value(&choreography.settings)?;
    if let Value::Object(map) = &mut settings {
        map.insert(
            "FloorColor".to_string(),
            Value::String(choreography.settings.floor_color.to_hex()),
        );
    }

    root.insert("Settings".to_string(), settings);
    root.insert("Floor".to_string(), serde_json::to_value(&choreography.floor)?);

    let roles_value = export_roles(&choreography.roles)?;
    let (dancers_value, dancer_ids) = export_dancers(&choreography.dancers, &choreography.roles)?;
    let scenes_value = export_scenes(&choreography.scenes, &dancer_ids)?;

    root.insert("Roles".to_string(), roles_value);
    root.insert("Dancers".to_string(), dancers_value);
    root.insert("Scenes".to_string(), scenes_value);
    root.insert("Name".to_string(), Value::String(choreography.name.clone()));

    if let Some(subtitle) = &choreography.subtitle {
        root.insert("Subtitle".to_string(), Value::String(subtitle.clone()));
    }
    if let Some(date) = &choreography.date {
        root.insert("Date".to_string(), Value::String(date.clone()));
    }
    if let Some(variation) = &choreography.variation {
        root.insert("Variation".to_string(), Value::String(variation.clone()));
    }
    if let Some(author) = &choreography.author {
        root.insert("Author".to_string(), Value::String(author.clone()));
    }
    if let Some(description) = &choreography.description {
        root.insert("Description".to_string(), Value::String(description.clone()));
    }

    let timestamp = choreography.last_save_date.format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| OffsetDateTime::now_utc().format(&time::format_description::well_known::Rfc3339).unwrap_or_default());
    root.insert("LastSaveDate".to_string(), Value::String(timestamp));

    Ok(Value::Object(root))
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
        let id = obj
            .get("$id")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
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

fn parse_role_ref(value: Option<&Value>, roles_by_id: &HashMap<String, Role>) -> Result<Role, ChoreoJsonError> {
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

fn export_roles(roles: &[Role]) -> Result<Value, ChoreoJsonError> {
    let mut list = Vec::with_capacity(roles.len());

    for (index, role) in roles.iter().enumerate() {
        let id = (index + 1) as i32;
        let mut map = Map::new();
        map.insert("$id".to_string(), Value::String(id.to_string()));
        map.insert("ZIndex".to_string(), Value::Number(role.z_index.into()));
        map.insert("Name".to_string(), Value::String(role.name.clone()));
        map.insert("Color".to_string(), Value::String(role.color.to_hex()));
        let value = Value::Object(map);
        list.push(value);
    }

    Ok(Value::Array(list))
}

fn export_dancers(
    dancers: &[Dancer],
    roles: &[Role],
) -> Result<(Value, HashMap<DancerId, String>), ChoreoJsonError> {
    let mut list = Vec::with_capacity(dancers.len());
    let mut ids = HashMap::new();

    for (index, dancer) in dancers.iter().enumerate() {
        let id = if dancer.dancer_id.0 > 0 {
            dancer.dancer_id.0
        } else {
            (index + 1) as i32
        };
        let role_id = role_id_for_dancer(roles, dancer).unwrap_or_else(|| "1".to_string());

        let mut map = Map::new();
        map.insert("$id".to_string(), Value::String(id.to_string()));
        map.insert(
            "Role".to_string(),
            Value::Object(ref_map("$ref", &role_id)),
        );
        map.insert("Name".to_string(), Value::String(dancer.name.clone()));
        map.insert("Shortcut".to_string(), Value::String(dancer.shortcut.clone()));
        map.insert("Color".to_string(), Value::String(dancer.color.to_hex()));
        if let Some(icon) = &dancer.icon {
            map.insert("Icon".to_string(), Value::String(icon.clone()));
        }
        list.push(Value::Object(map));
        ids.insert(DancerId(id), id.to_string());
    }

    Ok((Value::Array(list), ids))
}

fn role_id_for_dancer(roles: &[Role], dancer: &Dancer) -> Option<String> {
    roles
        .iter()
        .position(|role| role == &dancer.role)
        .map(|index| (index + 1).to_string())
}

fn export_scenes(
    scenes: &[Scene],
    dancer_ids: &HashMap<DancerId, String>,
) -> Result<Value, ChoreoJsonError> {
    let mut list = Vec::with_capacity(scenes.len());
    for (index, scene) in scenes.iter().enumerate() {
        let id = if scene.scene_id.0 > 0 {
            scene.scene_id.0
        } else {
            (index + 1) as i32
        };
        list.push(export_scene(scene, id, dancer_ids)?);
    }
    Ok(Value::Array(list))
}

fn export_scene(
    scene: &Scene,
    id: i32,
    dancer_ids: &HashMap<DancerId, String>,
) -> Result<Value, ChoreoJsonError> {
    let mut map = Map::new();
    map.insert("$id".to_string(), Value::String(id.to_string()));
    if let Some(positions) = &scene.positions {
        map.insert("Positions".to_string(), export_positions(positions, dancer_ids)?);
    }
    map.insert("Name".to_string(), Value::String(scene.name.clone()));
    if let Some(text) = &scene.text {
        map.insert("Text".to_string(), Value::String(text.clone()));
    }
    map.insert("FixedPositions".to_string(), Value::Bool(scene.fixed_positions));
    if let Some(timestamp) = &scene.timestamp {
        map.insert("Timestamp".to_string(), Value::String(timestamp.clone()));
    }
    map.insert("VariationDepth".to_string(), Value::Number(scene.variation_depth.into()));
    if let Some(variations) = &scene.variations {
        map.insert("Variations".to_string(), export_scene_variations(variations, dancer_ids)?);
    }
    if let Some(current) = &scene.current_variation {
        map.insert("CurrentVariation".to_string(), export_scene_list(current, dancer_ids)?);
    }
    map.insert("Color".to_string(), Value::String(scene.color.to_hex()));

    Ok(Value::Object(map))
}

fn export_positions(
    positions: &[Position],
    dancer_ids: &HashMap<DancerId, String>,
) -> Result<Value, ChoreoJsonError> {
    let mut list = Vec::with_capacity(positions.len());
    for position in positions {
        let mut map = serde_json::to_value(position)?.as_object().cloned().unwrap_or_default();
        if let Some(dancer) = &position.dancer
            && let Some(ref_id) = dancer_ids.get(&dancer.dancer_id)
        {
            map.insert("Dancer".to_string(), Value::Object(ref_map("$ref", ref_id)));
        }
        list.push(Value::Object(map));
    }
    Ok(Value::Array(list))
}

fn export_scene_variations(
    variations: &[Vec<Scene>],
    dancer_ids: &HashMap<DancerId, String>,
) -> Result<Value, ChoreoJsonError> {
    let mut list = Vec::with_capacity(variations.len());
    for variation in variations {
        list.push(export_scene_list(variation, dancer_ids)?);
    }
    Ok(Value::Array(list))
}

fn export_scene_list(
    scenes: &[Scene],
    dancer_ids: &HashMap<DancerId, String>,
) -> Result<Value, ChoreoJsonError> {
    let mut list = Vec::with_capacity(scenes.len());
    for (index, scene) in scenes.iter().enumerate() {
        let id = if scene.scene_id.0 > 0 {
            scene.scene_id.0
        } else {
            (index + 1) as i32
        };
        list.push(export_scene(scene, id, dancer_ids)?);
    }
    Ok(Value::Array(list))
}

fn ref_map(key: &str, value: &str) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert(key.to_string(), Value::String(value.to_string()));
    map
}

fn get_string(root: &Map<String, Value>, key: &'static str) -> Result<Option<String>, ChoreoJsonError> {
    Ok(root.get(key).and_then(|value| value.as_str()).map(str::to_string))
}
