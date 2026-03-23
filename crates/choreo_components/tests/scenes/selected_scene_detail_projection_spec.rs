use choreo_master_mobile_json::Color;

use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

#[test]
fn selected_scene_detail_projection_tracks_selection_and_audio_selection() {
    let mut state = create_state();
    let mut first = scene_model(1, "Intro", Some("1.5"), vec![]);
    first.text = Some("Start".to_string());
    first.fixed_positions = true;
    first.color = Color {
        a: 255,
        r: 10,
        g: 20,
        b: 30,
    };

    let second = scene_model(2, "Verse", Some("3.0"), vec![]);

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography_with_scenes("Show", vec![first, second])),
        },
    );

    assert!(state.has_selected_scene);
    assert_eq!(state.selected_scene_name, "Intro");
    assert_eq!(state.selected_scene_text, "Start");
    assert!(state.selected_scene_fixed_positions);
    assert_eq!(state.selected_scene_timestamp_text, "1.5");
    assert!(state.selected_scene_changed);

    state.clear_ephemeral_outputs();
    reduce(
        &mut state,
        ScenesAction::SelectSceneFromAudioPosition {
            position_seconds: 2.0,
        },
    );

    assert_eq!(state.selected_scene_name, "Intro");
    assert!(!state.redraw_floor_requested);
}
