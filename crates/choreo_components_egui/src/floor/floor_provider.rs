use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::sync_channel;

use crate::behavior::Behavior;

use super::actions::FloorAction;
use super::floor_adapter::FloorAdapter;
use super::floor_adapter::FloorAdapterInput;
use super::floor_view_model::FloorCanvasViewModel;
use super::floor_view_model::FloorPointerEventSenders;
use super::messages::DrawFloorCommand;
use super::reducer::reduce;
use super::state::FloorState;
use super::types::FloorRenderGate;
use super::types::FloorRenderGateImpl;

pub struct FloorProviderDependencies {
    pub state: FloorState,
    pub floor_adapter: FloorAdapter,
    pub floor_render_gate: Arc<dyn FloorRenderGate>,
    pub view_model_behaviors: Vec<Box<dyn Behavior<FloorCanvasViewModel>>>,
    pub floor_event_senders: FloorPointerEventSenders,
}

impl Default for FloorProviderDependencies {
    fn default() -> Self {
        Self {
            state: FloorState::default(),
            floor_adapter: FloorAdapter::new(),
            floor_render_gate: Arc::new(FloorRenderGateImpl::new()),
            view_model_behaviors: Vec::new(),
            floor_event_senders: FloorPointerEventSenders {
                pointer_pressed_senders: Vec::new(),
                pointer_moved_senders: Vec::new(),
                pointer_released_senders: Vec::new(),
                pointer_wheel_changed_senders: Vec::new(),
                touch_senders: Vec::new(),
            },
        }
    }
}

pub struct FloorProvider {
    state: Rc<RefCell<FloorState>>,
    floor_view_model: Rc<RefCell<FloorCanvasViewModel>>,
    floor_adapter: Rc<RefCell<FloorAdapter>>,
    draw_floor_sender: SyncSender<DrawFloorCommand>,
    draw_floor_receiver: Receiver<DrawFloorCommand>,
    floor_render_gate: Arc<dyn FloorRenderGate>,
    view_model_behaviors: Option<Vec<Box<dyn Behavior<FloorCanvasViewModel>>>>,
    activated: bool,
}

impl FloorProvider {
    #[must_use]
    pub fn new(dependencies: FloorProviderDependencies) -> Self {
        let (draw_floor_sender, draw_floor_receiver) = sync_channel(1);
        let floor_view_model = Rc::new(RefCell::new(FloorCanvasViewModel::new(
            draw_floor_sender.clone(),
            dependencies.floor_event_senders,
        )));
        floor_view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&floor_view_model));

        Self {
            state: Rc::new(RefCell::new(dependencies.state)),
            floor_view_model,
            floor_adapter: Rc::new(RefCell::new(dependencies.floor_adapter)),
            draw_floor_sender,
            draw_floor_receiver,
            floor_render_gate: dependencies.floor_render_gate,
            view_model_behaviors: Some(dependencies.view_model_behaviors),
            activated: false,
        }
    }

    pub fn activate(&mut self) {
        if self.activated {
            return;
        }

        if let Some(behaviors) = self.view_model_behaviors.take() {
            FloorCanvasViewModel::activate(&mut self.floor_view_model.borrow_mut(), behaviors);
        }

        {
            let state = Rc::clone(&self.state);
            let render_gate = Arc::clone(&self.floor_render_gate);
            self.floor_view_model
                .borrow_mut()
                .set_on_redraw(Some(Rc::new(move || {
                    reduce(&mut state.borrow_mut(), FloorAction::RedrawFloor);
                    render_gate.mark_rendered();
                })));
        }

        reduce(&mut self.state.borrow_mut(), FloorAction::Initialize);
        self.activated = true;
    }

    pub fn apply_adapter_input(&self, input: FloorAdapterInput) {
        let mut state = self.state.borrow_mut();
        let mut floor_view_model = self.floor_view_model.borrow_mut();
        self.floor_adapter
            .borrow()
            .apply(&mut state, &mut floor_view_model, input);
    }

    pub fn tick(&self) {
        loop {
            match self.draw_floor_receiver.try_recv() {
                Ok(DrawFloorCommand) => {
                    reduce(&mut self.state.borrow_mut(), FloorAction::DrawFloor);
                    self.floor_render_gate.mark_rendered();
                }
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
            }
        }
    }

    pub fn deactivate(&mut self) {
        self.floor_view_model.borrow_mut().set_on_redraw(None);
        self.activated = false;
    }

    #[must_use]
    pub fn floor_view_model(&self) -> Rc<RefCell<FloorCanvasViewModel>> {
        Rc::clone(&self.floor_view_model)
    }

    #[must_use]
    pub fn floor_adapter(&self) -> Rc<RefCell<FloorAdapter>> {
        Rc::clone(&self.floor_adapter)
    }

    #[must_use]
    pub fn draw_floor_sender(&self) -> SyncSender<DrawFloorCommand> {
        self.draw_floor_sender.clone()
    }

    #[must_use]
    pub fn floor_render_gate(&self) -> Arc<dyn FloorRenderGate> {
        Arc::clone(&self.floor_render_gate)
    }

    #[must_use]
    pub fn state(&self) -> Ref<'_, FloorState> {
        self.state.borrow()
    }

    #[must_use]
    pub fn state_mut(&self) -> RefMut<'_, FloorState> {
        self.state.borrow_mut()
    }
}
