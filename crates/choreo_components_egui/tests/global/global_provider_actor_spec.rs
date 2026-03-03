use std::cell::Cell;
use std::rc::Rc;

use choreo_components_egui::global::GlobalProvider;
use choreo_components_egui::global::GlobalStateActor;
use choreo_components_egui::global::InteractionMode;

#[test]
fn dispatch_updates_state_and_notifies_subscribers() {
    let actor = GlobalStateActor::new();
    let notifications = Rc::new(Cell::new(0usize));
    let notifications_for_handler = Rc::clone(&notifications);
    let handler: Rc<dyn Fn()> = Rc::new(move || {
        notifications_for_handler.set(notifications_for_handler.get() + 1);
    });
    actor.subscribe(Rc::clone(&handler));

    actor.dispatch(|state| {
        state.is_place_mode = true;
        state.interaction_mode = InteractionMode::Move;
    });

    let is_place_mode = actor
        .try_with_state(|state| state.is_place_mode)
        .expect("state should be readable");
    let interaction_mode = actor
        .try_with_state(|state| state.interaction_mode)
        .expect("state should be readable");

    assert!(is_place_mode);
    assert_eq!(interaction_mode, InteractionMode::Move);
    assert_eq!(notifications.get(), 1);
}

#[test]
fn provider_exposes_shared_actor_state_and_state_machine() {
    let provider = GlobalProvider::new();
    let actor = provider.global_state_store();
    let global_state = provider.global_state();
    let state_machine = provider.state_machine();

    actor.dispatch(|state| {
        state.redraw_floor = true;
    });

    assert!(global_state.borrow().redraw_floor);
    assert_eq!(
        state_machine.borrow().state().kind(),
        choreo_state_machine::StateKind::ViewSceneState
    );
}
