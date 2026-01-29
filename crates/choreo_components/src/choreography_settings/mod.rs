mod choreography_settings_view_model;
mod load_choreography_settings_behavior;
mod load_settings_preferences_behavior;
mod mapper;
mod messages;
mod update_author_behavior;
mod update_comment_behavior;
mod update_date_behavior;
mod update_description_behavior;
mod update_draw_path_from_behavior;
mod update_draw_path_to_behavior;
mod update_floor_back_behavior;
mod update_floor_color_behavior;
mod update_floor_front_behavior;
mod update_floor_left_behavior;
mod update_floor_right_behavior;
mod update_grid_lines_behavior;
mod update_grid_resolution_behavior;
mod update_name_behavior;
mod update_positions_at_side_behavior;
mod update_selected_scene_behavior;
mod update_show_legend_behavior;
mod update_show_timestamps_behavior;
mod update_snap_to_grid_behavior;
mod update_subtitle_behavior;
mod update_transparency_behavior;
mod update_variation_behavior;

pub use choreography_settings_view_model::{ChoreographySettingsViewModel, GridSizeOption};
pub use load_choreography_settings_behavior::LoadChoreographySettingsBehavior;
pub use load_settings_preferences_behavior::LoadSettingsPreferencesBehavior;
pub use messages::{RedrawFloorCommand, ShowTimestampsChangedEvent};
pub use update_author_behavior::UpdateAuthorBehavior;
pub use update_comment_behavior::UpdateCommentBehavior;
pub use update_date_behavior::UpdateDateBehavior;
pub use update_description_behavior::UpdateDescriptionBehavior;
pub use update_draw_path_from_behavior::UpdateDrawPathFromBehavior;
pub use update_draw_path_to_behavior::UpdateDrawPathToBehavior;
pub use update_floor_back_behavior::UpdateFloorBackBehavior;
pub use update_floor_color_behavior::UpdateFloorColorBehavior;
pub use update_floor_front_behavior::UpdateFloorFrontBehavior;
pub use update_floor_left_behavior::UpdateFloorLeftBehavior;
pub use update_floor_right_behavior::UpdateFloorRightBehavior;
pub use update_grid_lines_behavior::UpdateGridLinesBehavior;
pub use update_grid_resolution_behavior::UpdateGridResolutionBehavior;
pub use update_name_behavior::UpdateNameBehavior;
pub use update_positions_at_side_behavior::UpdatePositionsAtSideBehavior;
pub use update_selected_scene_behavior::UpdateSelectedSceneBehavior;
pub use update_show_legend_behavior::UpdateShowLegendBehavior;
pub use update_show_timestamps_behavior::UpdateShowTimestampsBehavior;
pub use update_snap_to_grid_behavior::UpdateSnapToGridBehavior;
pub use update_subtitle_behavior::UpdateSubtitleBehavior;
pub use update_transparency_behavior::UpdateTransparencyBehavior;
pub use update_variation_behavior::UpdateVariationBehavior;

use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::Sender;

use crate::behavior::Behavior;
use crate::global::GlobalStateModel;

pub struct ChoreographySettingsDependencies<P: crate::preferences::Preferences> {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub preferences: P,
    pub redraw_sender: Sender<RedrawFloorCommand>,
    pub show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
}

pub fn build_choreography_settings_behaviors<P: crate::preferences::Preferences + Clone + 'static>(
    deps: ChoreographySettingsDependencies<P>,
) -> Vec<Box<dyn Behavior<ChoreographySettingsViewModel>>> {
    let global_state = deps.global_state;
    let preferences = deps.preferences;
    let redraw_sender = deps.redraw_sender;
    let show_timestamps_sender = deps.show_timestamps_sender;
    vec![
        Box::new(LoadChoreographySettingsBehavior::new(
            global_state.clone(),
        )),
        Box::new(UpdateSelectedSceneBehavior::new(
            global_state.clone(),
        )),
        Box::new(UpdateCommentBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateNameBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateSubtitleBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateDateBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateVariationBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateAuthorBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateDescriptionBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateFloorFrontBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateFloorBackBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateFloorLeftBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateFloorRightBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateGridResolutionBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateDrawPathFromBehavior::new(
            preferences.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateDrawPathToBehavior::new(
            preferences.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateGridLinesBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateSnapToGridBehavior::new(
            global_state.clone(),
            preferences.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateFloorColorBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateShowTimestampsBehavior::new(
            global_state.clone(),
            preferences.clone(),
            redraw_sender.clone(),
            show_timestamps_sender.clone(),
        )),
        Box::new(UpdateShowLegendBehavior::new(
            preferences.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdatePositionsAtSideBehavior::new(
            global_state.clone(),
            preferences,
            redraw_sender.clone(),
        )),
        Box::new(UpdateTransparencyBehavior::new(
            global_state,
            redraw_sender.clone(),
        )),
    ]
}

pub fn build_settings_model_behaviors<P: crate::preferences::Preferences + Clone + 'static>(
    preferences: P,
) -> Vec<Box<dyn Behavior<choreo_models::SettingsModel>>> {
    vec![Box::new(LoadSettingsPreferencesBehavior::new(
        preferences,
    ))]
}
