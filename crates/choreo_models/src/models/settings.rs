use choreo_master_mobile_json::{Color, FrontPosition};

use crate::clone_mode::CloneMode;

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
