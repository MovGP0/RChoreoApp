use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use crate::scenes;

use choreo_components::behavior::Behavior;
use choreo_components::preferences::Preferences;
use choreo_components::scenes::SaveChoreoBehavior;
use choreo_models::SettingsPreferenceKeys;
use scenes::Report;

fn unique_temp_dir(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("current time should be after unix epoch")
        .as_nanos();
    path.push(format!("rchoreo-{name}-{nanos}"));
    fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

#[test]
#[serial_test::serial]
fn save_choreo_behavior_spec() {
    let suite = rspec::describe("save choreo behavior", (), |spec| {
        spec.it(
            "saves current scenes to the last opened choreography file",
            |_| {
                let context = scenes::ScenesTestContext::new();
                let temp_dir = unique_temp_dir("save-choreo");
                let choreo_path = temp_dir.join("saved.choreo");
                fs::write(&choreo_path, "{}").expect("choreo file should exist");

                context.preferences.set_string(
                    SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE,
                    choreo_path.to_string_lossy().into_owned(),
                );

                let source_scene = scenes::build_scene_model(
                    1,
                    "Intro",
                    Some("00:12"),
                    vec![scenes::build_position(1.0, 2.0)],
                );
                let source_scene_vm = scenes::map_scene_view_model(&source_scene);
                context.update_global_state(|state| {
                    state.choreography.name = "My Choreo".to_string();
                    state.choreography.scenes = vec![source_scene.clone()];
                    state.scenes = vec![source_scene_vm.clone()];
                    state.selected_scene = Some(source_scene_vm);
                });

                let preferences_dyn: Rc<dyn Preferences> = context.preferences.clone();
                let behavior =
                    SaveChoreoBehavior::new(context.global_state_store.clone(), preferences_dyn);
                context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

                context.view_model.borrow_mut().save_choreo();
                context.pump_events();

                let saved_json =
                    fs::read_to_string(&choreo_path).expect("saved file should be readable");
                let saved = choreo_master_mobile_json::import(&saved_json)
                    .expect("saved file should be valid choreography json");
                assert_eq!(saved.name, "My Choreo");
                assert_eq!(saved.scenes.len(), 1);
                assert_eq!(saved.scenes[0].name, "Intro");
                assert_eq!(saved.scenes[0].timestamp.as_deref(), Some("12"));

                fs::remove_dir_all(temp_dir).expect("temp dir should be removed");
            },
        );

        spec.it("does nothing when last opened file path is missing", |_| {
            let context = scenes::ScenesTestContext::new();

            let original_name = "Before".to_string();
            context.update_global_state(|state| {
                state.choreography.name = original_name.clone();
            });

            let preferences_dyn: Rc<dyn Preferences> = context.preferences.clone();
            let behavior =
                SaveChoreoBehavior::new(context.global_state_store.clone(), preferences_dyn);
            context.activate_behaviors(vec![Box::new(behavior) as Box<dyn Behavior<_>>]);

            context.view_model.borrow_mut().save_choreo();
            context.pump_events();

            assert_eq!(
                context.read_global_state(|state| state.choreography.name.clone()),
                original_name
            );
        });
    });

    let report = scenes::run_suite(&suite);
    assert!(report.is_success());
}
