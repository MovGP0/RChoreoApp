use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender, bounded};
use slint::{ComponentHandle, Timer, TimerMode};

use choreo_state_machine::ApplicationStateMachine;

use crate::ShellHost;
use crate::FloorMetricsInfo;
use crate::audio_player::AudioPlayerPositionChangedEvent;
use crate::behavior::Behavior;
use crate::global::{GlobalStateActor, GlobalStateModel};
use crate::observability::start_internal_span;
use crate::preferences::Preferences;

use super::draw_floor_behavior::DrawFloorBehavior;
use super::floor_adapter::FloorAdapter;
use super::floor_view_model::{FloorCanvasViewModel, FloorPointerEventSenders};
use super::gesture_handling_behavior::GestureHandlingBehavior;
use super::messages::DrawFloorCommand;
use super::move_positions_behavior::MovePositionsBehavior;
use super::place_position_behavior::PlacePositionBehavior;
use super::redraw_floor_behavior::RedrawFloorBehavior;
use super::rotate_around_center_behavior::RotateAroundCenterBehavior;
use super::scale_around_dancer_behavior::ScaleAroundDancerBehavior;
use super::scale_positions_behavior::ScalePositionsBehavior;

pub struct FloorProviderDependencies {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub global_state_store: Rc<GlobalStateActor>,
    pub state_machine: Rc<RefCell<ApplicationStateMachine>>,
    pub preferences: Rc<dyn Preferences>,
    pub audio_position_receiver: Receiver<AudioPlayerPositionChangedEvent>,
    pub redraw_floor_receiver: Receiver<crate::choreography_settings::RedrawFloorCommand>,
}

pub struct FloorProvider {
    floor_view_model: Rc<RefCell<FloorCanvasViewModel>>,
    floor_adapter: Rc<RefCell<FloorAdapter>>,
    draw_floor_sender: Sender<DrawFloorCommand>,
    floor_audio_timer: Timer,
    floor_layout_timer: Timer,
}

impl FloorProvider {
    pub fn new(deps: FloorProviderDependencies) -> Self {
        const POINTER_EVENT_BUFFER: usize = 256;
        let (draw_floor_sender, draw_floor_receiver) = bounded(1);

        let (gesture_pressed_sender, gesture_pressed_receiver) = bounded(POINTER_EVENT_BUFFER);
        let (gesture_moved_sender, gesture_moved_receiver) = bounded(POINTER_EVENT_BUFFER);
        let (gesture_released_sender, gesture_released_receiver) = bounded(POINTER_EVENT_BUFFER);
        let (gesture_wheel_sender, gesture_wheel_receiver) = bounded(POINTER_EVENT_BUFFER);
        let (gesture_touch_sender, gesture_touch_receiver) = bounded(POINTER_EVENT_BUFFER);

        let (place_pressed_sender, place_pressed_receiver) = bounded(POINTER_EVENT_BUFFER);
        let (place_moved_sender, place_moved_receiver) = bounded(POINTER_EVENT_BUFFER);
        let (place_released_sender, place_released_receiver) = bounded(POINTER_EVENT_BUFFER);

        let (move_pressed_sender, move_pressed_receiver) = bounded(POINTER_EVENT_BUFFER);
        let (move_moved_sender, move_moved_receiver) = bounded(POINTER_EVENT_BUFFER);
        let (move_released_sender, move_released_receiver) = bounded(POINTER_EVENT_BUFFER);

        let (rotate_pressed_sender, rotate_pressed_receiver) = bounded(POINTER_EVENT_BUFFER);
        let (rotate_moved_sender, rotate_moved_receiver) = bounded(POINTER_EVENT_BUFFER);
        let (rotate_released_sender, rotate_released_receiver) = bounded(POINTER_EVENT_BUFFER);

        let (scale_pressed_sender, scale_pressed_receiver) = bounded(POINTER_EVENT_BUFFER);
        let (scale_moved_sender, scale_moved_receiver) = bounded(POINTER_EVENT_BUFFER);
        let (scale_released_sender, scale_released_receiver) = bounded(POINTER_EVENT_BUFFER);

        let (scale_dancer_pressed_sender, scale_dancer_pressed_receiver) =
            bounded(POINTER_EVENT_BUFFER);
        let (scale_dancer_moved_sender, scale_dancer_moved_receiver) =
            bounded(POINTER_EVENT_BUFFER);
        let (scale_dancer_released_sender, scale_dancer_released_receiver) =
            bounded(POINTER_EVENT_BUFFER);

        let floor_event_senders = FloorPointerEventSenders {
            pointer_pressed_senders: vec![
                gesture_pressed_sender,
                place_pressed_sender,
                move_pressed_sender,
                rotate_pressed_sender,
                scale_pressed_sender,
                scale_dancer_pressed_sender,
            ],
            pointer_moved_senders: vec![
                gesture_moved_sender,
                place_moved_sender,
                move_moved_sender,
                rotate_moved_sender,
                scale_moved_sender,
                scale_dancer_moved_sender,
            ],
            pointer_released_senders: vec![
                gesture_released_sender,
                place_released_sender,
                move_released_sender,
                rotate_released_sender,
                scale_released_sender,
                scale_dancer_released_sender,
            ],
            pointer_wheel_changed_senders: vec![gesture_wheel_sender],
            touch_senders: vec![gesture_touch_sender],
        };

        let floor_view_model = Rc::new(RefCell::new(FloorCanvasViewModel::new(
            draw_floor_sender.clone(),
            floor_event_senders,
        )));
        floor_view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&floor_view_model));

        let floor_behaviors: Vec<Box<dyn Behavior<FloorCanvasViewModel>>> = vec![
            Box::new(DrawFloorBehavior::new(draw_floor_receiver, None)),
            Box::new(RedrawFloorBehavior::new(deps.redraw_floor_receiver)),
            Box::new(GestureHandlingBehavior::new(
                Rc::clone(&deps.state_machine),
                gesture_pressed_receiver,
                gesture_moved_receiver,
                gesture_released_receiver,
                gesture_wheel_receiver,
                gesture_touch_receiver,
            )),
            Box::new(PlacePositionBehavior::new(
                Rc::clone(&deps.global_state_store),
                Rc::clone(&deps.state_machine),
                place_pressed_receiver,
                place_moved_receiver,
                place_released_receiver,
            )),
            Box::new(MovePositionsBehavior::new(
                Rc::clone(&deps.global_state_store),
                Rc::clone(&deps.state_machine),
                move_pressed_receiver,
                move_moved_receiver,
                move_released_receiver,
            )),
            Box::new(RotateAroundCenterBehavior::new(
                Rc::clone(&deps.global_state_store),
                Rc::clone(&deps.state_machine),
                rotate_pressed_receiver,
                rotate_moved_receiver,
                rotate_released_receiver,
            )),
            Box::new(ScalePositionsBehavior::new(
                Rc::clone(&deps.global_state_store),
                Rc::clone(&deps.state_machine),
                scale_pressed_receiver,
                scale_moved_receiver,
                scale_released_receiver,
            )),
            Box::new(ScaleAroundDancerBehavior::new(
                Rc::clone(&deps.global_state_store),
                Rc::clone(&deps.state_machine),
                scale_dancer_pressed_receiver,
                scale_dancer_moved_receiver,
                scale_dancer_released_receiver,
            )),
        ];

        FloorCanvasViewModel::activate(&floor_view_model, floor_behaviors);

        let floor_adapter = Rc::new(RefCell::new(FloorAdapter::new(
            Rc::clone(&deps.global_state),
            Rc::clone(&deps.state_machine),
            Rc::clone(&deps.preferences),
            deps.audio_position_receiver,
        )));

        Self {
            floor_view_model,
            floor_adapter,
            draw_floor_sender,
            floor_audio_timer: Timer::default(),
            floor_layout_timer: Timer::default(),
        }
    }

    pub fn initialize_view(&mut self, view_weak: slint::Weak<ShellHost>) {
        {
            let floor_view_model_for_redraw = Rc::clone(&self.floor_view_model);
            let floor_adapter_for_redraw = Rc::clone(&self.floor_adapter);
            let view_weak = view_weak.clone();
            self.floor_view_model
                .borrow_mut()
                .set_on_redraw(Some(Rc::new(move || {
                    let floor_view_model = Rc::clone(&floor_view_model_for_redraw);
                    let floor_adapter = Rc::clone(&floor_adapter_for_redraw);
                    let view_weak = view_weak.clone();
                    slint::Timer::single_shot(Duration::from_millis(0), move || {
                        let mut span = start_internal_span("floor.render.refresh_event", None);
                        span.set_string_attribute(
                            "choreo.floor.refresh_source",
                            "view_model_redraw".to_string(),
                        );
                        if let Some(view) = view_weak.upgrade() {
                            Self::apply_floor_view(&view, &floor_adapter, &floor_view_model);
                        }
                    });
                })));
        }

        {
            let floor_adapter = Rc::clone(&self.floor_adapter);
            let floor_view_model = Rc::clone(&self.floor_view_model);
            let view_weak = view_weak.clone();
            self.floor_audio_timer.start(
                TimerMode::Repeated,
                Duration::from_millis(16),
                move || {
                    let mut adapter = floor_adapter.borrow_mut();
                    if !adapter.poll_audio_position() {
                        return;
                    }

                    let mut span = start_internal_span("floor.render.refresh_event", None);
                    span.set_string_attribute(
                        "choreo.floor.refresh_source",
                        "audio_position_timer".to_string(),
                    );
                    if let Some(view) = view_weak.upgrade() {
                        let mut floor_view_model = floor_view_model.borrow_mut();
                        adapter.apply(&view, &mut floor_view_model);
                    }
                },
            );
        }

        {
            let floor_view_model = Rc::clone(&self.floor_view_model);
            let floor_adapter = Rc::clone(&self.floor_adapter);
            let view_weak = view_weak.clone();
            slint::Timer::single_shot(Duration::from_millis(0), move || {
                let mut span = start_internal_span("floor.render.refresh_event", None);
                span.set_string_attribute(
                    "choreo.floor.refresh_source",
                    "initial_render".to_string(),
                );
                if let Some(view) = view_weak.upgrade() {
                    Self::apply_floor_view(&view, &floor_adapter, &floor_view_model);
                    floor_view_model.borrow_mut().draw_floor();
                }
            });
        }

        {
            let floor_view_model = Rc::clone(&self.floor_view_model);
            let floor_adapter = Rc::clone(&self.floor_adapter);
            let view_weak = view_weak.clone();
            let last_snapshot = Rc::new(RefCell::new(None::<FloorLayoutSnapshot>));
            self.floor_layout_timer.start(
                TimerMode::Repeated,
                Duration::from_millis(16),
                move || {
                    let Some(view) = view_weak.upgrade() else {
                        return;
                    };

                    let current = FloorLayoutSnapshot::from_view(&view);
                    let mut last = last_snapshot.borrow_mut();
                    if last.is_some_and(|previous| !previous.has_meaningful_change(current)) {
                        return;
                    }

                    *last = Some(current);
                    let mut span = start_internal_span("floor.render.refresh_event", None);
                    span.set_string_attribute(
                        "choreo.floor.refresh_source",
                        "layout_timer".to_string(),
                    );
                    Self::apply_floor_view(&view, &floor_adapter, &floor_view_model);
                },
            );
        }
    }

    pub fn apply_to_view(&self, view: &ShellHost) {
        Self::apply_floor_view(view, &self.floor_adapter, &self.floor_view_model);
    }

    pub fn floor_view_model(&self) -> Rc<RefCell<FloorCanvasViewModel>> {
        Rc::clone(&self.floor_view_model)
    }

    pub fn floor_adapter(&self) -> Rc<RefCell<FloorAdapter>> {
        Rc::clone(&self.floor_adapter)
    }

    pub fn draw_floor_sender(&self) -> Sender<DrawFloorCommand> {
        self.draw_floor_sender.clone()
    }

    fn apply_floor_view(
        view: &ShellHost,
        floor_adapter: &Rc<RefCell<FloorAdapter>>,
        floor_view_model: &Rc<RefCell<FloorCanvasViewModel>>,
    ) {
        let mut floor_view_model = floor_view_model.borrow_mut();
        floor_adapter
            .borrow_mut()
            .apply(view, &mut floor_view_model);
    }
}

#[derive(Clone, Copy)]
struct FloorLayoutSnapshot {
    floor_bounds_left: f32,
    floor_bounds_top: f32,
    floor_bounds_right: f32,
    floor_bounds_bottom: f32,
    floor_canvas_width: f32,
    floor_canvas_height: f32,
}

impl FloorLayoutSnapshot {
    fn has_meaningful_change(self, other: Self) -> bool {
        const EPSILON: f32 = 0.25;

        (self.floor_bounds_left - other.floor_bounds_left).abs() > EPSILON
            || (self.floor_bounds_top - other.floor_bounds_top).abs() > EPSILON
            || (self.floor_bounds_right - other.floor_bounds_right).abs() > EPSILON
            || (self.floor_bounds_bottom - other.floor_bounds_bottom).abs() > EPSILON
            || (self.floor_canvas_width - other.floor_canvas_width).abs() > EPSILON
            || (self.floor_canvas_height - other.floor_canvas_height).abs() > EPSILON
    }

    fn from_view(view: &ShellHost) -> Self {
        let floor_metrics_info = view.global::<FloorMetricsInfo<'_>>();
        Self {
            floor_bounds_left: floor_metrics_info.get_floor_bounds_left(),
            floor_bounds_top: floor_metrics_info.get_floor_bounds_top(),
            floor_bounds_right: floor_metrics_info.get_floor_bounds_right(),
            floor_bounds_bottom: floor_metrics_info.get_floor_bounds_bottom(),
            floor_canvas_width: floor_metrics_info.get_floor_canvas_width(),
            floor_canvas_height: floor_metrics_info.get_floor_canvas_height(),
        }
    }
}
