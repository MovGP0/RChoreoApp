use std::cell::Cell;
use std::rc::Rc;

use choreo_components_egui::global::GlobalProvider;
use choreo_components_egui::global::GlobalStateActor;
use choreo_components_egui::global::GlobalStateModel;
use choreo_components_egui::global::InteractionMode;
use choreo_components_egui::global::SceneViewModel;
use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::SceneId;

#[test]
fn dispatch_queues_mutations_until_drain_then_notifies_subscribers_once() {
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

    let is_place_mode_before_drain = actor
        .try_with_state(|state| state.is_place_mode)
        .expect("state should be readable");

    assert!(!is_place_mode_before_drain);
    assert_eq!(notifications.get(), 0);

    actor.drain();

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
fn drain_applies_all_queued_commands_before_notifying_subscribers() {
    let actor = GlobalStateActor::new();
    let notifications = Rc::new(Cell::new(0usize));
    let notifications_for_handler = Rc::clone(&notifications);
    let handler: Rc<dyn Fn()> = Rc::new(move || {
        notifications_for_handler.set(notifications_for_handler.get() + 1);
    });
    actor.subscribe(Rc::clone(&handler));

    actor.dispatch(|state| {
        state.redraw_floor = true;
    });
    actor.dispatch(|state| {
        state.is_rendering_floor = true;
    });

    actor.drain();

    let (redraw_floor, is_rendering_floor) = actor
        .try_with_state(|state| (state.redraw_floor, state.is_rendering_floor))
        .expect("state should be readable");

    assert!(redraw_floor);
    assert!(is_rendering_floor);
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
    actor.drain();

    assert!(global_state.borrow().redraw_floor);
    assert_eq!(
        state_machine.borrow().state().kind(),
        choreo_state_machine::StateKind::ViewSceneState
    );
}

#[test]
fn global_state_defaults_and_scene_view_fields_match_original_responsibilities() {
    let state = GlobalStateModel::default();

    assert!(state.scenes.is_empty());
    assert!(state.selected_scene.is_none());
    assert!(state.selected_scene_model.is_none());
    assert!(state.main_canvas_view.is_none());
    assert!(!state.redraw_floor);
    assert_eq!(state.interaction_mode, InteractionMode::View);
}

#[test]
fn actor_stores_scene_view_models_separately_from_selected_scene_model() {
    let actor = GlobalStateActor::new();

    assert!(actor.try_update(|state| {
        let mut scene = SceneViewModel::new(SceneId(7), "Bridge", Color::transparent());
        scene.timestamp = Some(12.3);
        state.scenes = vec![scene.clone()];
        state.selected_scene = Some(scene);
        state.selected_scene_model = Some(choreo_models::SceneModel {
            scene_id: SceneId(7),
            positions: Vec::new(),
            name: "Bridge".to_string(),
            text: Some("raw".to_string()),
            fixed_positions: false,
            timestamp: Some("12.3".to_string()),
            variation_depth: 0,
            variations: Vec::new(),
            current_variation: Vec::new(),
            color: Color::transparent(),
        });
    }));

    let snapshot = actor
        .try_with_state(|state| {
            (
                state.scenes[0].timestamp,
                state
                    .selected_scene
                    .as_ref()
                    .and_then(|scene| scene.timestamp),
                state
                    .selected_scene_model
                    .as_ref()
                    .and_then(|scene| scene.timestamp.as_deref().map(str::to_string)),
            )
        })
        .expect("state should be readable");

    assert_eq!(snapshot.0, Some(12.3));
    assert_eq!(snapshot.1, Some(12.3));
    assert_eq!(snapshot.2.as_deref(), Some("12.3"));
}
