use serde::{Deserialize, Serialize};

use super::{Color, FrontPosition};

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
