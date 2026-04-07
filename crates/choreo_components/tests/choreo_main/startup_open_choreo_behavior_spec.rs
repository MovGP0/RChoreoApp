use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use choreo_components::choreo_main::ChoreoMainBehaviorDependencies;
use choreo_components::choreo_main::MainPageBinding;
use choreo_components::choreo_main::MainPageDependencies;
use choreo_components::choreo_main::actions::ChoreoMainAction;
use choreo_components::choreo_main::actions::OpenChoreoRequested;
use choreo_components::preferences::InMemoryPreferences;
use choreo_components::preferences::Preferences;
use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::SceneId;
use choreo_master_mobile_json::export;
use choreo_models::ChoreographyModel;
use choreo_models::ChoreographyModelMapper;
use choreo_models::SceneModel;
use choreo_models::SettingsPreferenceKeys;

use crate::choreo_main::Report;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn startup_open_choreo_behavior_spec() {
    let suite = rspec::describe("startup open choreo behavior", (), |spec| {
        spec.it(
            "loads the remembered choreography file on startup when preferences point to an existing file",
            |_| {
                let temp_file = write_choreo_file("Remembered choreo", "Remembered Intro");
                let file_path = temp_file.to_string_lossy().into_owned();
                let preferences: Rc<dyn Preferences> = Rc::new(InMemoryPreferences::new());
                preferences.set_string(
                    SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE,
                    file_path.clone(),
                );

                let binding = MainPageBinding::new(MainPageDependencies {
                    behavior_dependencies: ChoreoMainBehaviorDependencies {
                        preferences: Some(Rc::clone(&preferences)),
                        ..ChoreoMainBehaviorDependencies::default()
                    },
                    ..MainPageDependencies::default()
                });

                binding.dispatch(ChoreoMainAction::Initialize);

                let state = binding.state();
                let state = state.borrow();
                let mut errors = Vec::new();

                check_eq!(
                    errors,
                    state.choreography_settings_state.name,
                    "Remembered choreo"
                );
                check_eq!(errors, state.scenes.len(), 1);
                check_eq!(errors, state.scenes[0].name, "Remembered Intro");
                check_eq!(
                    errors,
                    state.last_opened_choreo_file.as_deref(),
                    Some(file_path.as_str())
                );

                assert_no_errors(errors);

                let _ = fs::remove_file(temp_file);
            },
        );

        spec.it(
            "falls back to the bundled default choreography when no valid remembered file exists",
            |_| {
                let missing_file = unique_temp_file("choreo");
                let preferences: Rc<dyn Preferences> = Rc::new(InMemoryPreferences::new());
                preferences.set_string(
                    SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE,
                    missing_file.to_string_lossy().into_owned(),
                );

                let binding = MainPageBinding::new(MainPageDependencies {
                    behavior_dependencies: ChoreoMainBehaviorDependencies {
                        preferences: Some(Rc::clone(&preferences)),
                        ..ChoreoMainBehaviorDependencies::default()
                    },
                    ..MainPageDependencies::default()
                });

                binding.dispatch(ChoreoMainAction::Initialize);

                let state = binding.state();
                let state = state.borrow();
                let mut errors = Vec::new();

                check_eq!(
                    errors,
                    state.choreography_settings_state.name,
                    "Choreo erstellt mit JDC ChoreoMaster"
                );
                check_eq!(errors, state.scenes.len(), 1);
                check_eq!(errors, state.scenes[0].name, "Bild");
                check_eq!(
                    errors,
                    preferences.get_string(SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE, ""),
                    ""
                );

                assert_no_errors(errors);
            },
        );

        spec.it(
            "persists file backed choreography opens so startup can restore them later",
            |_| {
                let temp_file = write_choreo_file("Persisted choreo", "Persisted Intro");
                let file_path = temp_file.to_string_lossy().into_owned();
                let file_name = temp_file
                    .file_name()
                    .and_then(|value| value.to_str())
                    .expect("temp choreography file should have a file name")
                    .to_string();
                let contents = fs::read_to_string(&temp_file)
                    .expect("temp choreography file should be readable");
                let preferences: Rc<dyn Preferences> = Rc::new(InMemoryPreferences::new());

                let binding = MainPageBinding::new(MainPageDependencies {
                    behavior_dependencies: ChoreoMainBehaviorDependencies {
                        preferences: Some(Rc::clone(&preferences)),
                        ..ChoreoMainBehaviorDependencies::default()
                    },
                    ..MainPageDependencies::default()
                });

                binding.dispatch(ChoreoMainAction::RequestOpenChoreo(OpenChoreoRequested {
                    file_path: Some(file_path.clone()),
                    file_name: Some(file_name),
                    contents,
                }));

                assert_eq!(
                    preferences.get_string(SettingsPreferenceKeys::LAST_OPENED_CHOREO_FILE, ""),
                    file_path
                );

                let _ = fs::remove_file(temp_file);
            },
        );
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}

fn write_choreo_file(name: &str, scene_name: &str) -> PathBuf {
    let path = unique_temp_file("choreo");
    let contents = serialize_choreo(name, scene_name);
    fs::write(&path, contents).expect("temp choreography file should be written");
    path
}

fn serialize_choreo(name: &str, scene_name: &str) -> String {
    let choreography = ChoreographyModel {
        name: name.to_string(),
        scenes: vec![SceneModel {
            scene_id: SceneId(1),
            positions: Vec::new(),
            name: scene_name.to_string(),
            text: None,
            fixed_positions: false,
            timestamp: None,
            variation_depth: 0,
            variations: Vec::new(),
            current_variation: Vec::new(),
            color: Color::transparent(),
        }],
        ..ChoreographyModel::default()
    };
    export(&ChoreographyModelMapper.map_to_json(&choreography))
        .expect("test choreography should serialize")
}

fn unique_temp_file(extension: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let mut path = std::env::temp_dir();
    path.push(format!("rchoreo_startup_{nanos}.{extension}"));
    path
}
