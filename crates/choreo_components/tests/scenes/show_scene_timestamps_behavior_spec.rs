use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;

#[test]
fn show_scene_timestamps_syncs_from_choreography() {
    let mut state = create_state();
    let mut choreography = choreography_with_scenes("Test", vec![]);
    choreography.settings.show_timestamps = true;

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    state.show_timestamps = false;

    reduce(&mut state, ScenesAction::SyncShowTimestampsFromChoreography);

    assert!(state.show_timestamps);
}

#[test]
fn show_scene_timestamps_updates_view_and_model() {
    let mut state = create_state();
    let mut choreography = choreography_with_scenes("Test", vec![]);
    choreography.settings.show_timestamps = false;

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );

    reduce(&mut state, ScenesAction::UpdateShowTimestamps(true));

    assert!(state.show_timestamps);
    assert!(state.choreography.settings.show_timestamps);
}
