use crate::behavior::Behavior;
use crate::behavior::CompositeDisposable;
use crate::logging::BehaviorLog;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;
use super::messages::ChoreographySettingsCommand;
use super::state::DateParts;

macro_rules! define_update_behavior {
    ($name:ident, $command:expr, $value_ty:ty, $vm_ty:expr) => {
        pub struct $name;

        impl $name {
            pub fn apply(view_model: &mut ChoreographySettingsViewModel, value: $value_ty) {
                view_model.dispatch($command(value));
            }
        }

        impl Behavior<ChoreographySettingsViewModel> for $name {
            fn activate(
                &self,
                _view_model: &mut ChoreographySettingsViewModel,
                _disposables: &mut CompositeDisposable,
            ) {
                BehaviorLog::behavior_activated(stringify!($name), $vm_ty);
            }
        }
    };
}

pub struct LoadChoreographySettingsBehavior;
impl LoadChoreographySettingsBehavior {
    pub fn apply(
        view_model: &mut ChoreographySettingsViewModel,
        command: ChoreographySettingsCommand,
    ) {
        view_model.dispatch(command);
    }
}
impl Behavior<ChoreographySettingsViewModel> for LoadChoreographySettingsBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "LoadChoreographySettingsBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

pub struct LoadSettingsPreferencesBehavior;
impl LoadSettingsPreferencesBehavior {
    pub fn apply(
        view_model: &mut ChoreographySettingsViewModel,
        show_timestamps: bool,
        positions_at_side: bool,
        snap_to_grid: bool,
    ) {
        view_model.dispatch(ChoreographySettingsCommand::LoadSettingsPreferences {
            show_timestamps,
            positions_at_side,
            snap_to_grid,
        });
    }
}
impl Behavior<ChoreographySettingsViewModel> for LoadSettingsPreferencesBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "LoadSettingsPreferencesBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

define_update_behavior!(
    UpdateCommentBehavior,
    ChoreographySettingsCommand::UpdateComment,
    String,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateNameBehavior,
    ChoreographySettingsCommand::UpdateName,
    String,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateSubtitleBehavior,
    ChoreographySettingsCommand::UpdateSubtitle,
    String,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateVariationBehavior,
    ChoreographySettingsCommand::UpdateVariation,
    String,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateAuthorBehavior,
    ChoreographySettingsCommand::UpdateAuthor,
    String,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateDescriptionBehavior,
    ChoreographySettingsCommand::UpdateDescription,
    String,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateFloorFrontBehavior,
    ChoreographySettingsCommand::UpdateFloorFront,
    i32,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateFloorBackBehavior,
    ChoreographySettingsCommand::UpdateFloorBack,
    i32,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateFloorLeftBehavior,
    ChoreographySettingsCommand::UpdateFloorLeft,
    i32,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateFloorRightBehavior,
    ChoreographySettingsCommand::UpdateFloorRight,
    i32,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateGridResolutionBehavior,
    ChoreographySettingsCommand::UpdateGridResolution,
    i32,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateDrawPathFromBehavior,
    ChoreographySettingsCommand::UpdateDrawPathFrom,
    bool,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateDrawPathToBehavior,
    ChoreographySettingsCommand::UpdateDrawPathTo,
    bool,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateGridLinesBehavior,
    ChoreographySettingsCommand::UpdateGridLines,
    bool,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateSnapToGridBehavior,
    ChoreographySettingsCommand::UpdateSnapToGrid,
    bool,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateShowTimestampsBehavior,
    ChoreographySettingsCommand::UpdateShowTimestamps,
    bool,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateShowLegendBehavior,
    ChoreographySettingsCommand::UpdateShowLegend,
    bool,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdatePositionsAtSideBehavior,
    ChoreographySettingsCommand::UpdatePositionsAtSide,
    bool,
    "ChoreographySettingsViewModel"
);
define_update_behavior!(
    UpdateTransparencyBehavior,
    ChoreographySettingsCommand::UpdateTransparency,
    f64,
    "ChoreographySettingsViewModel"
);

pub struct UpdateDateBehavior;
impl UpdateDateBehavior {
    pub fn apply(view_model: &mut ChoreographySettingsViewModel, year: i32, month: u8, day: u8) {
        view_model.dispatch(ChoreographySettingsCommand::UpdateDate(DateParts {
            year,
            month,
            day,
        }));
    }
}
impl Behavior<ChoreographySettingsViewModel> for UpdateDateBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("UpdateDateBehavior", "ChoreographySettingsViewModel");
    }
}
