use super::actions::OpenAudioRequested;
use super::actions::OpenChoreoRequested;
use super::actions::OpenSvgFileCommand;
use crate::audio_player::state::AudioPlayerState;
use crate::dancers::state::DancersState;
use crate::floor::state::FloorState;

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
    View,
    Move,
    RotateAroundCenter,
    RotateAroundDancer,
    Scale,
    LineOfSight,
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
    pub selected_mode_index: i32,
    pub is_mode_selection_enabled: bool,
    pub is_nav_open: bool,
    pub is_choreography_settings_open: bool,
    pub is_audio_player_open: bool,
    pub is_dialog_open: bool,
    pub dialog_content: Option<String>,
    pub interaction_mode: InteractionMode,
    pub interaction_state_machine: InteractionStateMachineState,
    pub selected_positions_count: usize,
    pub outgoing_open_choreo_requests: Vec<OpenChoreoRequested>,
    pub outgoing_audio_requests: Vec<OpenAudioRequested>,
    pub outgoing_open_svg_commands: Vec<OpenSvgFileCommand>,
    pub svg_file_path: Option<String>,
    pub last_opened_svg_preference: Option<String>,
    pub draw_floor_request_count: usize,
    pub scenes: Vec<SceneState>,
    pub selected_scene_index: Option<usize>,
    pub audio_position_seconds: f64,
    pub floor_scene_name: Option<String>,
    pub floor_state: FloorState,
    pub audio_player_state: AudioPlayerState,
    pub dancers_state: DancersState,
}

impl Default for ChoreoMainState {
    fn default() -> Self {
        Self {
            content: MainContent::Main,
            selected_mode_index: -1,
            is_mode_selection_enabled: true,
            is_nav_open: false,
            is_choreography_settings_open: false,
            is_audio_player_open: false,
            is_dialog_open: false,
            dialog_content: None,
            interaction_mode: InteractionMode::View,
            interaction_state_machine: InteractionStateMachineState::Idle,
            selected_positions_count: 0,
            outgoing_open_choreo_requests: Vec::new(),
            outgoing_audio_requests: Vec::new(),
            outgoing_open_svg_commands: Vec::new(),
            svg_file_path: None,
            last_opened_svg_preference: None,
            draw_floor_request_count: 0,
            scenes: Vec::new(),
            selected_scene_index: None,
            audio_position_seconds: 0.0,
            floor_scene_name: None,
            floor_state: FloorState::default(),
            audio_player_state: AudioPlayerState::default(),
            dancers_state: DancersState::default(),
        }
    }
}
