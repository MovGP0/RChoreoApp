use super::actions::ScenesAction;
use super::choreography_with_scenes;
use super::create_state;
use super::reducer::reduce;
use super::scene_model;

#[test]
fn filter_scenes_by_search_text() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![
            scene_model(1, "Opening", None, vec![]),
            scene_model(2, "Chorus", None, vec![]),
            scene_model(3, "Closing", None, vec![]),
        ],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    reduce(&mut state, ScenesAction::UpdateSearchText("clo".to_string()));

    assert_eq!(state.visible_scenes.len(), 1);
    assert_eq!(state.visible_scenes[0].name, "Closing");
}

#[test]
fn filter_scenes_clears_and_restores_all() {
    let mut state = create_state();
    let choreography = choreography_with_scenes(
        "Test",
        vec![scene_model(1, "Opening", None, vec![]), scene_model(2, "Chorus", None, vec![])],
    );

    reduce(
        &mut state,
        ScenesAction::LoadScenes {
            choreography: Box::new(choreography),
        },
    );
    reduce(&mut state, ScenesAction::UpdateSearchText("open".to_string()));
    assert_eq!(state.visible_scenes.len(), 1);

    reduce(&mut state, ScenesAction::UpdateSearchText(String::new()));
    assert_eq!(state.visible_scenes.len(), 2);
}
