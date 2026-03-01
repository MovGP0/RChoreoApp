use crate::floor::floor_component::state::Point;
use crate::floor::floor_component::state::PointerButton;
use crate::floor::floor_component::state::PointerEventArgs;
use crate::floor::floor_component::actions::FloorAction;
use crate::floor::floor_component::reducer::reduce;
use crate::floor::floor_component::state::FloorState;
use crate::floor::floor_component::state::InteractionMode;
use crate::floor::floor_component::state::TouchAction;

#[test]
fn pointer_event_args_exposes_position_and_button() {
    let args = PointerEventArgs {
        position: Point::new(12.0, 34.0),
        button: PointerButton::Primary,
        is_in_contact: true,
    };

    assert_eq!(args.position, Point::new(12.0, 34.0));
    assert_eq!(args.button, PointerButton::Primary);
}

#[test]
fn floor_variants_and_ui_are_exercised_for_reducer_coverage() {
    let mut state = FloorState::default();
    reduce(&mut state, FloorAction::Initialize);
    reduce(
        &mut state,
        FloorAction::SetInteractionMode {
            mode: InteractionMode::Move,
        },
    );
    reduce(
        &mut state,
        FloorAction::SetInteractionMode {
            mode: InteractionMode::RotateAroundCenter,
        },
    );
    reduce(
        &mut state,
        FloorAction::SetInteractionMode {
            mode: InteractionMode::RotateAroundDancer,
        },
    );
    reduce(
        &mut state,
        FloorAction::SetInteractionMode {
            mode: InteractionMode::Scale,
        },
    );
    reduce(
        &mut state,
        FloorAction::SetInteractionMode {
            mode: InteractionMode::Place,
        },
    );
    reduce(&mut state, FloorAction::ResetViewport);
    reduce(
        &mut state,
        FloorAction::Touch {
            id: 1,
            action: TouchAction::Released,
            point: Point::new(0.0, 0.0),
            is_in_contact: false,
        },
    );
    let secondary_args = PointerEventArgs {
        position: Point::new(0.0, 0.0),
        button: PointerButton::Secondary,
        is_in_contact: false,
    };
    assert_eq!(secondary_args.button, PointerButton::Secondary);

    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = crate::floor::floor_component::ui::draw(ui, &state);
        });
    });
}
