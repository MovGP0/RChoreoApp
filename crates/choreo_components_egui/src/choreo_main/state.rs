use super::actions::OpenAudioRequested;
use super::actions::OpenSvgFileCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MainContent {
    #[default]
    Main,
    Settings,
    Dancers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InteractionMode {
    #[default]
    None,
    Move,
    RotateAroundCenter,
    RotateAroundDancer,
    Scale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InteractionStateMachineState {
    #[default]
    Idle,
    MovePositions,
    RotateAroundCenter,
    RotateAroundCenterSelection,
    ScaleAroundDancer,
    ScaleAroundDancerSelection,
    ScalePositions,
    ScalePositionsSelection,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneState {
    pub name: String,
    pub timestamp_seconds: Option<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChoreoMainState {
    pub content: MainContent,
    pub is_dialog_open: bool,
    pub dialog_content: Option<String>,
    pub interaction_mode: InteractionMode,
    pub interaction_state_machine: InteractionStateMachineState,
    pub selected_positions_count: usize,
    pub outgoing_audio_requests: Vec<OpenAudioRequested>,
    pub outgoing_open_svg_commands: Vec<OpenSvgFileCommand>,
    pub svg_file_path: Option<String>,
    pub last_opened_svg_preference: Option<String>,
    pub draw_floor_request_count: usize,
    pub scenes: Vec<SceneState>,
    pub selected_scene_index: Option<usize>,
    pub audio_position_seconds: f64,
    pub floor_scene_name: Option<String>,
}

impl Default for ChoreoMainState {
    fn default() -> Self {
        Self {
            content: MainContent::Main,
            is_dialog_open: false,
            dialog_content: None,
            interaction_mode: InteractionMode::None,
            interaction_state_machine: InteractionStateMachineState::Idle,
            selected_positions_count: 0,
            outgoing_audio_requests: Vec::new(),
            outgoing_open_svg_commands: Vec::new(),
            svg_file_path: None,
            last_opened_svg_preference: None,
            draw_floor_request_count: 0,
            scenes: Vec::new(),
            selected_scene_index: None,
            audio_position_seconds: 0.0,
            floor_scene_name: None,
        }
    }
}
