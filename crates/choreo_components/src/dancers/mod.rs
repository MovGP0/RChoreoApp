mod add_dancer_behavior;
mod cancel_dancer_settings_behavior;
mod dancer_settings_view_model;
mod delete_dancer_behavior;
mod hide_dancer_dialog_behavior;
mod load_dancer_settings_behavior;
mod mapper;
mod messages;
mod save_dancer_settings_behavior;
mod selected_dancer_state_behavior;
mod selected_icon_behavior;
mod selected_role_behavior;
mod show_dancer_dialog_behavior;
mod swap_dancer_selection_behavior;
mod swap_dancers_behavior;
mod translate_behavior;

pub use add_dancer_behavior::AddDancerBehavior;
pub use cancel_dancer_settings_behavior::CancelDancerSettingsBehavior;
pub use dancer_settings_view_model::{
    DancerSettingsViewModel, IconNameToImageSourceConverter, IconOption,
    SwapDancersDialogViewModel,
};
pub use delete_dancer_behavior::DeleteDancerBehavior;
pub use hide_dancer_dialog_behavior::HideDancerDialogBehavior;
pub use load_dancer_settings_behavior::LoadDancerSettingsBehavior;
pub use messages::{CloseDancerDialogCommand, ShowDancerDialogCommand};
pub use save_dancer_settings_behavior::SaveDancerSettingsBehavior;
pub use selected_dancer_state_behavior::SelectedDancerStateBehavior;
pub use selected_icon_behavior::SelectedIconBehavior;
pub use selected_role_behavior::SelectedRoleBehavior;
pub use show_dancer_dialog_behavior::ShowDancerDialogBehavior;
pub use swap_dancer_selection_behavior::SwapDancerSelectionBehavior;
pub use swap_dancers_behavior::SwapDancersBehavior;
pub use translate_behavior::DancerSettingsTranslateBehavior;

use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::{Receiver, Sender};

use crate::audio_player::HapticFeedback;
use crate::behavior::Behavior;
use crate::global::GlobalStateModel;

pub struct DancerSettingsDependencies {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub show_dialog_sender: Sender<ShowDancerDialogCommand>,
    pub close_dialog_sender: Sender<CloseDancerDialogCommand>,
    pub show_dialog_receiver: Receiver<ShowDancerDialogCommand>,
    pub close_dialog_receiver: Receiver<CloseDancerDialogCommand>,
    pub haptic_feedback: Option<Box<dyn HapticFeedback>>,
    pub locale: String,
}

pub fn build_dancer_settings_behaviors(
    deps: DancerSettingsDependencies,
) -> Vec<Box<dyn Behavior<DancerSettingsViewModel>>> {
    vec![
        Box::new(DancerSettingsTranslateBehavior::new(deps.locale)),
        Box::new(LoadDancerSettingsBehavior::new(deps.global_state.clone())),
        Box::new(AddDancerBehavior),
        Box::new(DeleteDancerBehavior),
        Box::new(SelectedDancerStateBehavior),
        Box::new(SelectedIconBehavior),
        Box::new(SelectedRoleBehavior),
        Box::new(SwapDancerSelectionBehavior),
        Box::new(SwapDancersBehavior::new(
            deps.haptic_feedback,
            deps.show_dialog_sender,
        )),
        Box::new(HideDancerDialogBehavior::new(deps.close_dialog_receiver)),
        Box::new(ShowDancerDialogBehavior::new(deps.show_dialog_receiver)),
        Box::new(CancelDancerSettingsBehavior),
        Box::new(SaveDancerSettingsBehavior::new(deps.global_state)),
    ]
}
