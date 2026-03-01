use super::actions::ScenesAction;
use super::build_position;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

#[test]
fn load_scenes_maps_models_and_selects_first_scene() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![
            scene_model(1, "Intro", Some("00:05"), vec![build_position(0.0, 0.0)]),
            scene_model(2, "Verse", None, vec![build_position(1.0, 1.0)]),
        ],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );

    assert_eq!(state.scenes.len(), 2);
    assert_eq!(state.selected_scene.as_ref().map(|scene| scene.name.as_str()), Some("Intro"));
    assert_eq!(state.selected_scene.as_ref().and_then(|scene| scene.timestamp), Some(5.0));
}

#[test]
fn reload_scenes_reloads_from_choreography() {
    let mut state = create_state();
    let choreography = choreography_with_scenes("Test", vec![scene_model(1, "First", None, vec![])]);

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );

    state.choreography.scenes.push(scene_model(2, "Second", Some("00:09"), vec![]));
    reduce(&mut state, ScenesAction::ReloadScenes);

    assert_eq!(state.scenes.len(), 2);
    assert_eq!(state.scenes[1].name, "Second");
    assert_eq!(state.scenes[1].timestamp, Some(9.0));
    assert!(state.reload_requested);
}
