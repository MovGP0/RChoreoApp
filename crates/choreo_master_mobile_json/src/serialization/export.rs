use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::clock::SystemClock;
use crate::errors::ChoreoJsonError;
use crate::models::{Choreography, Dancer, DancerId, Position, Role, Scene};
use crate::serialization::helpers::ref_map;

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
    root.insert(
        "Floor".to_string(),
        serde_json::to_value(&choreography.floor)?,
    );

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
        root.insert(
            "Description".to_string(),
            Value::String(description.clone()),
        );
    }

    let timestamp = choreography
        .last_save_date
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| {
            SystemClock::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default()
        });
    root.insert("LastSaveDate".to_string(), Value::String(timestamp));

    Ok(Value::Object(root))
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
        map.insert("Role".to_string(), Value::Object(ref_map("$ref", &role_id)));
        map.insert("Name".to_string(), Value::String(dancer.name.clone()));
        map.insert(
            "Shortcut".to_string(),
            Value::String(dancer.shortcut.clone()),
        );
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
        map.insert(
            "Positions".to_string(),
            export_positions(positions, dancer_ids)?,
        );
    }
    map.insert("Name".to_string(), Value::String(scene.name.clone()));
    if let Some(text) = &scene.text {
        map.insert("Text".to_string(), Value::String(text.clone()));
    }
    map.insert(
        "FixedPositions".to_string(),
        Value::Bool(scene.fixed_positions),
    );
    if let Some(timestamp) = &scene.timestamp {
        map.insert("Timestamp".to_string(), Value::String(timestamp.clone()));
    }
    map.insert(
        "VariationDepth".to_string(),
        Value::Number(scene.variation_depth.into()),
    );
    if let Some(variations) = &scene.variations {
        map.insert(
            "Variations".to_string(),
            export_scene_variations(variations, dancer_ids)?,
        );
    }
    if let Some(current) = &scene.current_variation {
        map.insert(
            "CurrentVariation".to_string(),
            export_scene_list(current, dancer_ids)?,
        );
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
        let mut map = serde_json::to_value(position)?
            .as_object()
            .cloned()
            .unwrap_or_default();
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
