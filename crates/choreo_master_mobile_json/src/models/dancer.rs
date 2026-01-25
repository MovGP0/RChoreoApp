use serde::{Deserialize, Serialize};

use super::{Color, DancerId, Role};

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
