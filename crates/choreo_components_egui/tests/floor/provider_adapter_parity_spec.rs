use crate::floor::floor_component::floor_adapter::AudioInterpolationInput;
use crate::floor::floor_component::floor_adapter::FloorAdapter;
use crate::floor::floor_component::floor_adapter::FloorAdapterInput;
use crate::floor::floor_component::floor_provider::FloorBehavior;
use crate::floor::floor_component::floor_provider::FloorBehaviorFactory;
use crate::floor::floor_component::floor_provider::FloorBehaviorKind;
use crate::floor::floor_component::floor_provider::FloorProvider;
use crate::floor::floor_component::floor_provider::FloorProviderDependencies;
use crate::floor::floor_component::floor_provider::TickRedrawBehaviorFactory;
use crate::floor::floor_component::state::FloorPosition;
use crate::floor::floor_component::state::FloorState;

#[test]
fn adapter_maps_scene_overlay_and_audio_interpolation_into_state() {
    let adapter = FloorAdapter::new();
    let mut state = FloorState::default();

    adapter.apply(
        &mut state,
        FloorAdapterInput {
            scene_positions: vec![FloorPosition::new(0.0, 0.0), FloorPosition::new(24.0, 36.0)],
            axis_x_label: "X Axis".to_string(),
            axis_y_label: "Y Axis".to_string(),
            legend_entries: vec![("Lead".to_string(), [255, 64, 64, 255])],
            svg_path: Some("floor.svg".to_string()),
            placement_remaining: Some(3),
            interpolation: Some(AudioInterpolationInput {
                from: vec![FloorPosition::new(0.0, 0.0)],
                to: vec![FloorPosition::new(12.0, 0.0)],
                progress: 0.5,
            }),
        },
    );

    assert_eq!(state.positions.len(), 2);
    assert_eq!(state.axis_labels.len(), 2);
    assert_eq!(state.legend_entries.len(), 1);
    assert_eq!(state.placement_remaining, Some(3));
    assert_eq!(state.svg_path.as_deref(), Some("floor.svg"));
    assert_eq!(state.interpolated_positions.len(), 1);
    assert!((state.interpolated_positions[0].x - 6.0).abs() < 0.001);
}

#[derive(Default)]
struct ProbeBehavior {
    pub activated: bool,
    pub deactivated: bool,
    pub ticks: usize,
}

impl FloorBehavior for ProbeBehavior {
    fn kind(&self) -> FloorBehaviorKind {
        FloorBehaviorKind::Gesture
    }

    fn activate(&mut self, _state: &mut FloorState) {
        self.activated = true;
    }

    fn on_tick(&mut self, _state: &mut FloorState) {
        self.ticks += 1;
    }

    fn deactivate(&mut self, _state: &mut FloorState) {
        self.deactivated = true;
    }
}

struct ProbeFactory;

impl FloorBehaviorFactory for ProbeFactory {
    fn create(&self) -> Box<dyn FloorBehavior> {
        Box::<ProbeBehavior>::default()
    }
}

#[test]
fn provider_activates_behaviors_in_order_and_ticks_redraw_pipeline() {
    let _all_kinds = [
        FloorBehaviorKind::Draw,
        FloorBehaviorKind::Redraw,
        FloorBehaviorKind::Gesture,
        FloorBehaviorKind::Move,
        FloorBehaviorKind::Place,
        FloorBehaviorKind::Rotate,
        FloorBehaviorKind::Scale,
    ];

    let mut provider = FloorProvider::new(
        FloorState::default(),
        FloorProviderDependencies {
            behavior_factories: vec![Box::new(ProbeFactory), Box::new(TickRedrawBehaviorFactory)],
        },
    );

    provider.activate();
    provider.state_mut().zoom = 1.25;
    provider.tick();

    assert_eq!(provider.activation_order[0], FloorBehaviorKind::Gesture);
    assert_eq!(provider.activation_order[1], FloorBehaviorKind::Redraw);
    assert!(provider.state().draw_count >= 2);

    provider.deactivate();
}
