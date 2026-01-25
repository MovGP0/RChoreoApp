use crate::audio_player::HapticFeedback;
use crate::behavior::{Behavior, CompositeDisposable};
use crate::logging::BehaviorLog;

use super::dancer_settings_view_model::DancerSettingsViewModel;
use super::messages::ShowDancerDialogCommand;

pub struct SwapDancersBehavior {
    haptic_feedback: Option<Box<dyn HapticFeedback>>,
    show_dialog_sender: crossbeam_channel::Sender<ShowDancerDialogCommand>,
}

impl SwapDancersBehavior {
    pub fn new(
        haptic_feedback: Option<Box<dyn HapticFeedback>>,
        show_dialog_sender: crossbeam_channel::Sender<ShowDancerDialogCommand>,
    ) -> Self {
        Self {
            haptic_feedback,
            show_dialog_sender,
        }
    }

    pub fn show_swap_dialog(&self, view_model: &DancerSettingsViewModel) {
        let (Some(from), Some(to)) = (
            view_model.swap_from_dancer.as_ref(),
            view_model.swap_to_dancer.as_ref(),
        ) else {
            return;
        };

        if from.dancer_id == to.dancer_id {
            return;
        }

        if let Some(haptic) = &self.haptic_feedback
            && haptic.is_supported()
        {
            haptic.perform_click();
        }

        let _ = self.show_dialog_sender.send(ShowDancerDialogCommand {
            content_id: Some("swap_dancers".to_string()),
        });
    }
}

impl Behavior<DancerSettingsViewModel> for SwapDancersBehavior {
    fn activate(
        &self,
        _view_model: &mut DancerSettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("SwapDancersBehavior", "DancerSettingsViewModel");
    }
}

