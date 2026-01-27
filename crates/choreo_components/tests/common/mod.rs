#![allow(dead_code)]

use choreo_components::floor::{
    CanvasViewHandle,
    DrawFloorCommand,
    FloorCanvasViewModel,
    Matrix,
    Point,
    PointerButton,
    PointerEventArgs,
    PointerMovedCommand,
    PointerPressedCommand,
    PointerReleasedCommand,
    PointerWheelChangedCommand,
    Rect,
    Size,
    TouchAction,
    TouchCommand,
    TouchDeviceType,
    TouchEventArgs,
};
use choreo_components::global::GlobalStateModel;
use choreo_components::scenes::SceneViewModel;
use choreo_models::{ChoreographyModel, DancerModel, FloorModel, PositionModel, RoleModel, SceneModel, SettingsModel};
use choreo_state_machine::ApplicationStateMachine;
use rspec::{ConfigurationBuilder, Logger, Runner};
pub use rspec::report::Report;
use std::io;
use std::rc::Rc;
use std::sync::Arc;

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
    pub global_state: GlobalStateModel,
    pub state_machine: ApplicationStateMachine,
    pub view_model: FloorCanvasViewModel,
    pub draw_floor_receiver: crossbeam_channel::Receiver<DrawFloorCommand>,
}

impl FloorTestContext {
    pub fn new() -> Self {
        let (sender, receiver) = crossbeam_channel::unbounded();
        let view_model = FloorCanvasViewModel::new(sender, Vec::new());
        let state_machine = ApplicationStateMachine::with_default_transitions(Box::new(GlobalStateModel::default()));

        Self {
            global_state: GlobalStateModel::default(),
            state_machine,
            view_model,
            draw_floor_receiver: receiver,
        }
    }

    pub fn configure_canvas(&mut self) {
        self.view_model.set_floor_bounds(Rect::new(0.0, 0.0, 100.0, 100.0));
        self.view_model.set_canvas_size(Size::new(100.0, 100.0));
        self.view_model.set_transformation_matrix(Matrix::identity());
    }
}

pub fn floor_to_view_point(
    view_model: &FloorCanvasViewModel,
    choreography: &ChoreographyModel,
    floor_point: Point,
) -> Point {
    let floor_bounds = view_model.floor_bounds();
    let width = floor_bounds.width() as f64;
    let height = floor_bounds.height() as f64;
    let floor_width = (choreography.floor.size_left + choreography.floor.size_right) as f64;
    let floor_height = (choreography.floor.size_front + choreography.floor.size_back) as f64;
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
        timestamp: scene.timestamp.as_ref().and_then(|value| value.parse::<f64>().ok()),
        is_selected: false,
        positions: scene.positions.clone(),
        variation_depth: scene.variation_depth,
        variations: scene.variations.clone(),
        current_variation: scene.current_variation.clone(),
        color: scene.color.clone(),
    }
}
