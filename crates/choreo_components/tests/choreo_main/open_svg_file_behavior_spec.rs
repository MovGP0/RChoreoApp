use std::fs;
use std::rc::Rc;
use std::time::Duration;
use std::cell::RefCell;

use crate::choreo_main;

use choreo_components::behavior::Behavior;
use choreo_components::choreo_main::OpenSvgFileBehavior;
use choreo_components::choreo_main::OpenSvgFileCommand;
use choreo_components::floor::DrawFloorCommand;
use choreo_components::preferences::InMemoryPreferences;
use choreo_components::preferences::Preferences;
use choreo_models::SettingsPreferenceKeys;
use crossbeam_channel::unbounded;
use choreo_main::Report;

fn unique_temp_file(name: &str) -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("current time should be after unix epoch")
        .as_nanos();
    path.push(format!("rchoreo-{name}-{nanos}.svg"));
    path
}

#[test]
#[serial_test::serial]
fn open_svg_file_behavior_spec() {
    let suite = rspec::describe("open svg file behavior", (), |spec| {
        spec.it("opens svg path, stores preference, and requests draw", |_| {
            let (sender, receiver) = unbounded::<OpenSvgFileCommand>();
            let (draw_sender, draw_receiver) = unbounded::<DrawFloorCommand>();
            let preferences = Rc::new(InMemoryPreferences::new()) as Rc<dyn Preferences>;
            let global_state_store = choreo_components::global::GlobalStateActor::new();
            let state_machine = Rc::new(RefCell::new(
                choreo_state_machine::ApplicationStateMachine::with_default_transitions(
                    Box::new(choreo_components::global::GlobalStateModel::default()),
                ),
            ));
            let behavior = OpenSvgFileBehavior::new(
                global_state_store.clone(),
                preferences.clone(),
                receiver,
                draw_sender,
            );
            let context = choreo_main::ChoreoMainTestContext::with_dependencies(
                vec![Box::new(behavior) as Box<dyn Behavior<_>>],
                global_state_store,
                state_machine,
            );

            let svg_path = unique_temp_file("open-svg");
            fs::write(&svg_path, "<svg/>").expect("temp svg should be written");
            sender
                .send(OpenSvgFileCommand {
                    file_path: svg_path.to_string_lossy().into_owned(),
                })
                .expect("send should succeed");

            let updated = context.wait_until(Duration::from_secs(1), || {
                context.read_global_state(|state| state.svg_file_path.as_deref() == Some(svg_path.to_string_lossy().as_ref()))
            });
            assert!(updated);
            assert!(draw_receiver.try_recv().is_ok());
            assert_eq!(
                preferences.get_string(SettingsPreferenceKeys::LAST_OPENED_SVG_FILE, ""),
                svg_path.to_string_lossy()
            );
            fs::remove_file(svg_path).expect("temp svg should be removed");
        });
    });

    let report = choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
