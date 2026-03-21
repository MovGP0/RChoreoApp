use crate::dancers::actions::DancersAction;

use super::action::DancerSettingsPageAction;
use super::action::SwapDialogAction;

#[must_use]
pub fn map_action(action: DancerSettingsPageAction) -> Vec<DancersAction> {
    vec![match action {
        DancerSettingsPageAction::ToggleDancerList => DancersAction::ToggleDancerList,
        DancerSettingsPageAction::CloseDancerList => DancersAction::CloseDancerList,
        DancerSettingsPageAction::AddDancer => DancersAction::AddDancer,
        DancerSettingsPageAction::DeleteSelectedDancer => DancersAction::DeleteSelectedDancer,
        DancerSettingsPageAction::SelectDancer { index } => DancersAction::SelectDancer { index },
        DancerSettingsPageAction::SelectRole { index } => DancersAction::SelectRole { index },
        DancerSettingsPageAction::UpdateDancerName { value } => {
            DancersAction::UpdateDancerName { value }
        }
        DancerSettingsPageAction::UpdateDancerShortcut { value } => {
            DancersAction::UpdateDancerShortcut { value }
        }
        DancerSettingsPageAction::UpdateDancerColor { value } => {
            DancersAction::UpdateDancerColor { value }
        }
        DancerSettingsPageAction::UpdateDancerIcon { value } => {
            DancersAction::UpdateDancerIcon { value }
        }
        DancerSettingsPageAction::UpdateSwapFrom { index } => {
            DancersAction::UpdateSwapFrom { index }
        }
        DancerSettingsPageAction::UpdateSwapTo { index } => DancersAction::UpdateSwapTo { index },
        DancerSettingsPageAction::RequestSwapDancers => DancersAction::RequestSwapDancers,
        DancerSettingsPageAction::DismissDialog => DancersAction::HideDialog,
        DancerSettingsPageAction::ConfirmSwapDialog => DancersAction::ConfirmSwapDancers,
        DancerSettingsPageAction::CancelPage => DancersAction::Cancel,
        DancerSettingsPageAction::SavePage => DancersAction::SaveToGlobal,
    }]
}

#[must_use]
pub const fn map_swap_dialog_action(action: SwapDialogAction) -> DancerSettingsPageAction {
    match action {
        SwapDialogAction::Cancel => DancerSettingsPageAction::DismissDialog,
        SwapDialogAction::Confirm => DancerSettingsPageAction::ConfirmSwapDialog,
    }
}
