use serde::{Deserialize, Serialize};

use super::Dancer;

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
