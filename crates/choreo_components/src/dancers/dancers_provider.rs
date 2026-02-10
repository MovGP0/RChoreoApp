use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::{Sender, unbounded};

use crate::global::GlobalStateActor;
use crate::haptics::HapticFeedback;

use super::add_dancer_behavior::AddDancerBehavior;
use super::cancel_dancer_settings_behavior::CancelDancerSettingsBehavior;
use super::dancer_settings_view_model::{DancerSettingsViewModel, DancerSettingsViewModelActions};
use super::delete_dancer_behavior::DeleteDancerBehavior;
use super::hide_dancer_dialog_behavior::HideDancerDialogBehavior;
use super::load_dancer_settings_behavior::LoadDancerSettingsBehavior;
use super::messages::{
    AddDancerCommand, CancelDancerSettingsCommand, CloseDancerDialogCommand,
    DancerSelectionCommand, DeleteDancerCommand, ReloadDancerSettingsCommand,
    SaveDancerSettingsCommand, SelectRoleCommand, ShowDancerDialogCommand, SwapDancersCommand,
    UpdateDancerDetailsCommand, UpdateDancerIconCommand, UpdateSwapSelectionCommand,
};
use super::save_dancer_settings_behavior::SaveDancerSettingsBehavior;
use super::selected_dancer_state_behavior::SelectedDancerStateBehavior;
use super::update_dancer_details_behavior::UpdateDancerDetailsBehavior;
use super::selected_icon_behavior::SelectedIconBehavior;
use super::selected_role_behavior::SelectedRoleBehavior;
use super::show_dancer_dialog_behavior::ShowDancerDialogBehavior;
use super::swap_dancer_selection_behavior::SwapDancerSelectionBehavior;
use super::swap_dancers_behavior::SwapDancersBehavior;

pub struct DancersProviderDependencies {
    pub global_state: Rc<GlobalStateActor>,
    pub haptic_feedback: Option<Box<dyn HapticFeedback>>,
}

pub struct DancersProvider {
    dancer_settings_view_model: Rc<RefCell<DancerSettingsViewModel>>,
    show_dialog_sender: Sender<ShowDancerDialogCommand>,
    close_dialog_sender: Sender<CloseDancerDialogCommand>,
    reload_sender: Sender<ReloadDancerSettingsCommand>,
}

impl DancersProvider {
    pub fn new(deps: DancersProviderDependencies) -> Self {
        let (show_dialog_sender, show_dialog_receiver) = unbounded();
        let (close_dialog_sender, close_dialog_receiver) = unbounded();
        let (reload_sender, reload_receiver) = unbounded();
        let (add_dancer_sender, add_dancer_receiver) = unbounded();
        let (delete_dancer_sender, delete_dancer_receiver) = unbounded();
        let (save_sender, save_receiver) = unbounded();
        let (cancel_sender, cancel_receiver) = unbounded();
        let (swap_sender, swap_receiver) = unbounded();
        let (selection_sender, selection_receiver) = unbounded();
        let (swap_selection_sender, swap_selection_receiver) = unbounded();
        let (select_role_sender, select_role_receiver) = unbounded();
        let (update_details_sender, update_details_receiver) = unbounded();
        let (update_icon_sender, update_icon_receiver) = unbounded();

        let behaviors: Vec<Box<dyn crate::behavior::Behavior<DancerSettingsViewModel>>> = vec![
            Box::new(LoadDancerSettingsBehavior::new(
                deps.global_state.clone(),
                reload_receiver,
                selection_sender.clone(),
                swap_selection_sender.clone(),
            )),
            Box::new(AddDancerBehavior::new(
                add_dancer_receiver,
                selection_sender.clone(),
                swap_selection_sender.clone(),
            )),
            Box::new(DeleteDancerBehavior::new(
                delete_dancer_receiver,
                selection_sender.clone(),
                swap_selection_sender.clone(),
            )),
            Box::new(SelectedDancerStateBehavior::new(selection_receiver)),
            Box::new(UpdateDancerDetailsBehavior::new(update_details_receiver)),
            Box::new(SelectedIconBehavior::new(update_icon_receiver)),
            Box::new(SelectedRoleBehavior::new(select_role_receiver)),
            Box::new(SwapDancerSelectionBehavior::new(swap_selection_receiver)),
            Box::new(SwapDancersBehavior::new(
                show_dialog_sender.clone(),
                swap_receiver,
            )),
            Box::new(HideDancerDialogBehavior::new(close_dialog_receiver)),
            Box::new(ShowDancerDialogBehavior::new(show_dialog_receiver)),
            Box::new(CancelDancerSettingsBehavior::new(cancel_receiver)),
            Box::new(SaveDancerSettingsBehavior::new(
                deps.global_state.clone(),
                save_receiver,
            )),
        ];

        let dancer_settings_view_model = Rc::new(RefCell::new(DancerSettingsViewModel::new(
            deps.haptic_feedback,
        )));
        dancer_settings_view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&dancer_settings_view_model));
        DancerSettingsViewModel::activate(&dancer_settings_view_model, behaviors);

        let selection_sender_for_select = selection_sender.clone();
        let swap_selection_sender_for_select = swap_selection_sender.clone();
        let add_dancer_sender_for_action = add_dancer_sender.clone();
        let delete_dancer_sender_for_action = delete_dancer_sender.clone();
        let select_role_sender_for_action = select_role_sender.clone();
        let update_details_sender_for_name = update_details_sender.clone();
        let update_details_sender_for_shortcut = update_details_sender.clone();
        let update_details_sender_for_color = update_details_sender.clone();
        let update_icon_sender_for_action = update_icon_sender.clone();
        let swap_sender_for_action = swap_sender.clone();
        let save_sender_for_action = save_sender.clone();
        let cancel_sender_for_action = cancel_sender.clone();

        let actions = DancerSettingsViewModelActions {
            select_dancer: Some(Rc::new(move |_view_model, index| {
                let _ = selection_sender_for_select.send(DancerSelectionCommand::Select(index));
                let _ = swap_selection_sender_for_select.send(UpdateSwapSelectionCommand);
            })),
            add_dancer: Some(Rc::new(move |_view_model| {
                let _ = add_dancer_sender_for_action.send(AddDancerCommand);
            })),
            delete_dancer: Some(Rc::new(move |_view_model| {
                let _ = delete_dancer_sender_for_action.send(DeleteDancerCommand);
            })),
            select_role: Some(Rc::new(move |_view_model, index| {
                let _ = select_role_sender_for_action.send(SelectRoleCommand { index });
            })),
            update_dancer_name: Some(Rc::new(move |_view_model, value| {
                let _ = update_details_sender_for_name.send(UpdateDancerDetailsCommand::Name(value));
            })),
            update_dancer_shortcut: Some(Rc::new(move |_view_model, value| {
                let _ = update_details_sender_for_shortcut
                    .send(UpdateDancerDetailsCommand::Shortcut(value));
            })),
            update_dancer_color: Some(Rc::new(move |_view_model, value| {
                let _ = update_details_sender_for_color.send(UpdateDancerDetailsCommand::Color(value));
            })),
            update_dancer_icon: Some(Rc::new(move |_view_model, value| {
                let _ = update_icon_sender_for_action.send(UpdateDancerIconCommand { value });
            })),
            swap_dancers: Some(Rc::new(move |_view_model| {
                let _ = swap_sender_for_action.send(SwapDancersCommand);
            })),
            save: Some(Rc::new(move |_view_model| {
                let _ = save_sender_for_action.send(SaveDancerSettingsCommand);
            })),
            cancel: Some(Rc::new(move |_view_model| {
                let _ = cancel_sender_for_action.send(CancelDancerSettingsCommand);
            })),
        };

        dancer_settings_view_model.borrow_mut().set_actions(actions);

        Self {
            dancer_settings_view_model,
            show_dialog_sender,
            close_dialog_sender,
            reload_sender,
        }
    }

    pub fn dancer_settings_view_model(&self) -> Rc<RefCell<DancerSettingsViewModel>> {
        Rc::clone(&self.dancer_settings_view_model)
    }

    pub fn show_dialog_sender(&self) -> Sender<ShowDancerDialogCommand> {
        self.show_dialog_sender.clone()
    }

    pub fn close_dialog_sender(&self) -> Sender<CloseDancerDialogCommand> {
        self.close_dialog_sender.clone()
    }

    pub fn reload(&self) {
        let _ = self.reload_sender.send(ReloadDancerSettingsCommand);
    }
}
