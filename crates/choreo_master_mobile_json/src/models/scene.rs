use serde::{Deserialize, Serialize};

use super::{Color, Position, SceneId};

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
