#![allow(dead_code)]

use choreo_components::behavior::Behavior;
use choreo_components::choreography_settings::RedrawFloorCommand;
use choreo_components::floor::{
    CanvasViewHandle,
    DrawFloorCommand,
    FloorCanvasViewModel,
    FloorPointerEventSenders,
    GestureHandlingBehavior,
    Matrix,
    MovePositionsBehavior,
    PlacePositionBehavior,
    Point,
    PointerButton,
    PointerEventArgs,
    PointerMovedCommand,
    PointerPressedCommand,
    PointerReleasedCommand,
    PointerWheelChangedCommand,
    Rect,
    RedrawFloorBehavior,
    RotateAroundCenterBehavior,
    ScaleAroundDancerBehavior,
    ScalePositionsBehavior,
    Size,
    TouchAction,
    TouchCommand,
    TouchDeviceType,
    TouchEventArgs,
};
use choreo_components::global::{GlobalStateActor, GlobalStateModel};
use choreo_components::scenes::SceneViewModel;
use choreo_models::{
    ChoreographyModel,
    DancerModel,
    FloorModel,
    PositionModel,
    RoleModel,
    SceneModel,
    SettingsModel,
};
use choreo_state_machine::ApplicationStateMachine;
use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
pub use rspec::report::Report;
use rspec::{ConfigurationBuilder, Logger, Runner};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

const POINTER_EVENT_BUFFER: usize = 256;

pub fn run_suite<T>(suite: &rspec::block::Suite<T>) -> rspec::report::SuiteReport
where
    T: Clone + Send + Sync + std::fmt::Debug,
{
    let configuration = ConfigurationBuilder::default()
        .exit_on_failure(false)
        .build()
        .expect("rspec configuration should build");
    let logger = Arc::new(Logger::new(io::stdout()));
    let runner = Runner::new(configuration, vec![logger]);
    runner.run(suite)
}

pub struct FloorTestContext {
    pub global_state_store: Rc<GlobalStateActor>,
    pub state_machine: Rc<RefCell<ApplicationStateMachine>>,
    pub view_model: Rc<RefCell<FloorCanvasViewModel>>,
    pub draw_floor_receiver: Receiver<DrawFloorCommand>,
    redraw_floor_sender: Sender<RedrawFloorCommand>,
}

impl FloorTestContext {
    pub fn new() -> Self {
        let global_state_store = GlobalStateActor::new();
        let state_machine = Rc::new(RefCell::new(
            ApplicationStateMachine::with_default_transitions(Box::new(GlobalStateModel::default())),
        ));

        let (draw_floor_sender, draw_floor_receiver) = unbounded();
        let (redraw_floor_sender, redraw_floor_receiver) = unbounded();

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

        let view_model = Rc::new(RefCell::new(FloorCanvasViewModel::new(
            draw_floor_sender,
            floor_event_senders,
        )));
        view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&view_model));

        let floor_behaviors: Vec<Box<dyn Behavior<FloorCanvasViewModel>>> = vec![
            Box::new(RedrawFloorBehavior::new(redraw_floor_receiver)),
            Box::new(GestureHandlingBehavior::new(
                Rc::clone(&state_machine),
                gesture_pressed_receiver,
                gesture_moved_receiver,
                gesture_released_receiver,
                gesture_wheel_receiver,
                gesture_touch_receiver,
            )),
            Box::new(PlacePositionBehavior::new(
                Rc::clone(&global_state_store),
                Rc::clone(&state_machine),
                place_pressed_receiver,
                place_moved_receiver,
                place_released_receiver,
            )),
            Box::new(MovePositionsBehavior::new(
                Rc::clone(&global_state_store),
                Rc::clone(&state_machine),
                move_pressed_receiver,
                move_moved_receiver,
                move_released_receiver,
            )),
            Box::new(RotateAroundCenterBehavior::new(
                Rc::clone(&global_state_store),
                Rc::clone(&state_machine),
                rotate_pressed_receiver,
                rotate_moved_receiver,
                rotate_released_receiver,
            )),
            Box::new(ScalePositionsBehavior::new(
                Rc::clone(&global_state_store),
                Rc::clone(&state_machine),
                scale_pressed_receiver,
                scale_moved_receiver,
                scale_released_receiver,
            )),
            Box::new(ScaleAroundDancerBehavior::new(
                Rc::clone(&global_state_store),
                Rc::clone(&state_machine),
                scale_dancer_pressed_receiver,
                scale_dancer_moved_receiver,
                scale_dancer_released_receiver,
            )),
        ];

        FloorCanvasViewModel::activate(&view_model, floor_behaviors);

        let context = Self {
            global_state_store,
            state_machine,
            view_model,
            draw_floor_receiver,
            redraw_floor_sender,
        };

        context.pump_events();
        context
    }

    pub fn configure_canvas(&self) {
        let mut view_model = self.view_model.borrow_mut();
        view_model.set_floor_bounds(Rect::new(0.0, 0.0, 100.0, 100.0));
        view_model.set_canvas_size(Size::new(100.0, 100.0));
        view_model.set_transformation_matrix(Matrix::identity());
    }

    pub fn set_transformation_matrix(&self, matrix: Matrix) {
        self.view_model.borrow_mut().set_transformation_matrix(matrix);
    }

    pub fn update_global_state(&self, update: impl FnOnce(&mut GlobalStateModel)) {
        let updated = self.global_state_store.try_update(update);
        assert!(updated, "failed to update global state in test context");
    }

    pub fn read_global_state<T>(&self, read: impl FnOnce(&GlobalStateModel) -> T) -> T {
        self.global_state_store
            .try_with_state(read)
            .expect("failed to read global state in test context")
    }

    pub fn update_state_machine(&self, update: impl FnOnce(&mut ApplicationStateMachine)) {
        update(&mut self.state_machine.borrow_mut());
    }

    pub fn send_pointer_pressed(&self, point: Point) {
        self.view_model.borrow().pointer_pressed(pointer_pressed(point));
        self.pump_events();
    }

    pub fn send_pointer_moved(&self, point: Point) {
        self.view_model.borrow().pointer_moved(pointer_moved(point));
        self.pump_events();
    }

    pub fn send_pointer_released(&self, point: Point) {
        self.view_model.borrow().pointer_released(pointer_released(point));
        self.pump_events();
    }

    pub fn send_pointer_wheel_changed(&self, delta: f64, position: Option<Point>) {
        self.view_model
            .borrow()
            .pointer_wheel_changed(pointer_wheel_changed(delta, position));
        self.pump_events();
    }

    pub fn send_touch(&self, id: i64, action: TouchAction, point: Point, in_contact: bool) {
        self.view_model
            .borrow()
            .touch(touch_command(id, action, point, in_contact));
        self.pump_events();
    }

    pub fn send_redraw_command(&self) {
        self.redraw_floor_sender
            .send(RedrawFloorCommand)
            .expect("redraw command send should succeed");
        self.pump_events();
    }

    pub fn state_kind(&self) -> choreo_state_machine::StateKind {
        self.state_machine.borrow().state().kind()
    }

    pub fn draw_count(&self) -> usize {
        self.draw_floor_receiver.try_iter().count()
    }

    pub fn wait_until(&self, timeout: Duration, mut predicate: impl FnMut() -> bool) -> bool {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            if predicate() {
                return true;
            }
            self.pump_events();
            thread::sleep(Duration::from_millis(10));
        }
        predicate()
    }

    pub fn pump_events(&self) {
        for _ in 0..3 {
            slint::platform::update_timers_and_animations();
            thread::sleep(Duration::from_millis(20));
        }
        slint::platform::update_timers_and_animations();
    }
}

pub fn floor_to_view_point(context: &FloorTestContext, floor_point: Point) -> Point {
    let (floor_width, floor_height) = context.read_global_state(|state| {
        (
            (state.choreography.floor.size_left + state.choreography.floor.size_right) as f64,
            (state.choreography.floor.size_front + state.choreography.floor.size_back) as f64,
        )
    });

    let view_model = context.view_model.borrow();
    let floor_bounds = view_model.floor_bounds();
    let width = floor_bounds.width() as f64;
    let height = floor_bounds.height() as f64;
    let scale = (width / floor_width).min(height / floor_height);
    let center_x = floor_bounds.left as f64 + width / 2.0;
    let center_y = floor_bounds.top as f64 + height / 2.0;
    let canvas_x = center_x + floor_point.x * scale;
    let canvas_y = center_y - floor_point.y * scale;

    view_model
        .transformation_matrix
        .map_point(Point::new(canvas_x, canvas_y))
}

pub fn assert_close(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "expected {expected} +/- {epsilon}, got {actual}"
    );
}

pub fn pointer_pressed(point: Point) -> PointerPressedCommand {
    PointerPressedCommand {
        canvas_view: CanvasViewHandle,
        event_args: PointerEventArgs {
            position: point,
            button: PointerButton::Primary,
            is_in_contact: true,
        },
    }
}

pub fn pointer_moved(point: Point) -> PointerMovedCommand {
    PointerMovedCommand {
        canvas_view: CanvasViewHandle,
        event_args: PointerEventArgs {
            position: point,
            button: PointerButton::Primary,
            is_in_contact: true,
        },
    }
}

pub fn pointer_released(point: Point) -> PointerReleasedCommand {
    PointerReleasedCommand {
        canvas_view: CanvasViewHandle,
        event_args: PointerEventArgs {
            position: point,
            button: PointerButton::Primary,
            is_in_contact: false,
        },
    }
}

pub fn pointer_wheel_changed(delta: f64, position: Option<Point>) -> PointerWheelChangedCommand {
    PointerWheelChangedCommand {
        canvas_view: CanvasViewHandle,
        delta,
        position,
    }
}

pub fn touch_command(id: i64, action: TouchAction, point: Point, in_contact: bool) -> TouchCommand {
    TouchCommand {
        canvas_view: CanvasViewHandle,
        event_args: TouchEventArgs {
            id,
            action,
            device_type: TouchDeviceType::Touch,
            location: point,
            in_contact,
        },
    }
}

pub fn build_three_position_choreography() -> (ChoreographyModel, SceneModel) {
    let role_lead = Rc::new(RoleModel {
        z_index: 0,
        name: "Lead".to_string(),
        color: choreo_models::Colors::red(),
    });
    let role_follow = Rc::new(RoleModel {
        z_index: 0,
        name: "Follow".to_string(),
        color: choreo_models::Colors::blue(),
    });

    let dancer_a = Rc::new(DancerModel {
        dancer_id: choreo_master_mobile_json::DancerId(1),
        name: "Alice".to_string(),
        shortcut: "A".to_string(),
        role: role_lead.clone(),
        color: choreo_models::Colors::red(),
        icon: None,
    });
    let dancer_b = Rc::new(DancerModel {
        dancer_id: choreo_master_mobile_json::DancerId(2),
        name: "Bob".to_string(),
        shortcut: "B".to_string(),
        role: role_follow.clone(),
        color: choreo_models::Colors::blue(),
        icon: None,
    });
    let dancer_c = Rc::new(DancerModel {
        dancer_id: choreo_master_mobile_json::DancerId(3),
        name: "Cory".to_string(),
        shortcut: "C".to_string(),
        role: role_lead.clone(),
        color: choreo_models::Colors::green(),
        icon: None,
    });

    let mut scene = SceneModel {
        scene_id: choreo_master_mobile_json::SceneId(1),
        positions: Vec::new(),
        name: "Scene 1".to_string(),
        text: None,
        fixed_positions: true,
        timestamp: None,
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: choreo_models::Colors::transparent(),
    };

    scene.positions.push(PositionModel {
        dancer: Some(dancer_a.clone()),
        orientation: None,
        x: -1.0,
        y: 1.0,
        curve1_x: None,
        curve1_y: None,
        curve2_x: None,
        curve2_y: None,
        movement1_x: None,
        movement1_y: None,
        movement2_x: None,
        movement2_y: None,
    });
    scene.positions.push(PositionModel {
        dancer: Some(dancer_b.clone()),
        orientation: None,
        x: 1.0,
        y: 1.0,
        curve1_x: None,
        curve1_y: None,
        curve2_x: None,
        curve2_y: None,
        movement1_x: None,
        movement1_y: None,
        movement2_x: None,
        movement2_y: None,
    });
    scene.positions.push(PositionModel {
        dancer: Some(dancer_c.clone()),
        orientation: None,
        x: 3.0,
        y: -2.0,
        curve1_x: None,
        curve1_y: None,
        curve2_x: None,
        curve2_y: None,
        movement1_x: None,
        movement1_y: None,
        movement2_x: None,
        movement2_y: None,
    });

    let choreography = ChoreographyModel {
        name: "Test".to_string(),
        floor: FloorModel {
            size_front: 5,
            size_back: 5,
            size_left: 5,
            size_right: 5,
        },
        settings: SettingsModel {
            dancer_size: 1.0,
            snap_to_grid: false,
            resolution: 0,
            ..SettingsModel::default()
        },
        roles: vec![role_lead, role_follow],
        dancers: vec![dancer_a, dancer_b, dancer_c],
        scenes: vec![scene.clone()],
        ..ChoreographyModel::default()
    };

    (choreography, scene)
}

pub fn build_empty_scene_choreography() -> (ChoreographyModel, SceneModel) {
    let role = Rc::new(RoleModel {
        z_index: 0,
        name: "Lead".to_string(),
        color: choreo_models::Colors::red(),
    });
    let dancer = Rc::new(DancerModel {
        dancer_id: choreo_master_mobile_json::DancerId(1),
        name: "Alex".to_string(),
        shortcut: "A".to_string(),
        role,
        color: choreo_models::Colors::red(),
        icon: None,
    });

    let scene = SceneModel {
        scene_id: choreo_master_mobile_json::SceneId(1),
        positions: Vec::new(),
        name: "Scene 1".to_string(),
        text: None,
        fixed_positions: true,
        timestamp: None,
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: choreo_models::Colors::transparent(),
    };

    let choreography = ChoreographyModel {
        name: "Test".to_string(),
        floor: FloorModel {
            size_front: 5,
            size_back: 5,
            size_left: 5,
            size_right: 5,
        },
        settings: SettingsModel {
            dancer_size: 1.0,
            snap_to_grid: false,
            resolution: 0,
            ..SettingsModel::default()
        },
        dancers: vec![dancer],
        scenes: vec![scene.clone()],
        ..ChoreographyModel::default()
    };

    (choreography, scene)
}

pub fn map_scene_view_model(scene: &SceneModel) -> SceneViewModel {
    SceneViewModel {
        scene_id: scene.scene_id,
        name: scene.name.clone(),
        text: scene.text.clone().unwrap_or_default(),
        fixed_positions: scene.fixed_positions,
        timestamp: scene
            .timestamp
            .as_ref()
            .and_then(|value| value.parse::<f64>().ok()),
        is_selected: false,
        positions: scene.positions.clone(),
        variation_depth: scene.variation_depth,
        variations: scene.variations.clone(),
        current_variation: scene.current_variation.clone(),
        color: scene.color.clone(),
    }
}

pub fn sleep_ms(duration_ms: u64) {
    thread::sleep(Duration::from_millis(duration_ms));
}
