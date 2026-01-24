use crate::scenes::SceneViewModel;
use crate::settings::SettingsViewModel;
use choreo_models::{ChoreographyModel, PositionModel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionMode {
    View,
    Move,
    RotateAroundCenter,
    RotateAroundDancer,
    Scale,
    LineOfSight,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionRectangle {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SvgDocument {
    pub picture_bytes: Vec<u8>,
    pub bounds: (f32, f32, f32, f32),
}

#[derive(Debug)]
pub struct GlobalStateModel {
    pub settings_view_model: SettingsViewModel,
    pub choreography: ChoreographyModel,
    pub svg_document: Option<SvgDocument>,
    pub svg_file_path: Option<String>,
    pub scenes: Vec<SceneViewModel>,
    pub selected_scene: Option<SceneViewModel>,
    pub selected_positions: Vec<PositionModel>,
    pub selection_rectangle: Option<SelectionRectangle>,
    pub interaction_mode: InteractionMode,
    pub is_place_mode: bool,
}

impl GlobalStateModel {
    pub fn new(settings_view_model: SettingsViewModel) -> Self {
        Self {
            settings_view_model,
            choreography: ChoreographyModel::default(),
            svg_document: None,
            svg_file_path: None,
            scenes: Vec::new(),
            selected_scene: None,
            selected_positions: Vec::new(),
            selection_rectangle: None,
            interaction_mode: InteractionMode::View,
            is_place_mode: false,
        }
    }
}
