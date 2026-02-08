use std::time::Duration;

use crate::audio_player;

use audio_player::Report;
use choreo_components::audio_player::AudioPlayerTicksBehavior;
use choreo_components::behavior::Behavior;

#[test]
#[serial_test::serial]
fn audio_player_ticks_behavior_spec() {
    let suite = rspec::describe("audio player ticks behavior", (), |spec| {
        spec.it("updates tick values and can-link state from global scenes", |_| {
            let global_state = choreo_components::global::GlobalStateActor::new();
            let behavior = AudioPlayerTicksBehavior::new(global_state.clone());
            let context = audio_player::AudioPlayerTestContext::with_global_state(vec![
                Box::new(behavior) as Box<dyn Behavior<_>>,
            ], global_state);

            let first = audio_player::scene_view_model(1, "A", Some("00:01"));
            let second = audio_player::scene_view_model(2, "B", Some("00:04"));
            context.update_global_state(|state| {
                state.scenes = vec![first.clone(), second.clone()];
                state.selected_scene = Some(second);
            });
            context.view_model.borrow_mut().duration = 10.0;
            context.view_model.borrow_mut().position = 2.0;

            let updated = context.wait_until(Duration::from_secs(1), || {
                let vm = context.view_model.borrow();
                vm.tick_values.len() == 2 && vm.can_link_scene_to_position
            });
            assert!(updated);
        });
    });

    let report = audio_player::run_suite(&suite);
    assert!(report.is_success());
}
