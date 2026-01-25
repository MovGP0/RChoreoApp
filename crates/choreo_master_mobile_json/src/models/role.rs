use serde::{Deserialize, Serialize};

use super::Color;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Role {
    #[serde(rename = "ZIndex")]
    pub z_index: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Color", skip_serializing, skip_deserializing)]
    pub color: Color,
}
