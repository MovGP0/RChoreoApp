use std::fs;
use std::rc::Rc;
use std::time::Duration;

use crate::audio_player;

use audio_player::Report;
use choreo_components::audio_player::OpenAudioFileBehavior;
use choreo_components::audio_player::OpenAudioFileCommand;
use choreo_components::behavior::Behavior;
use choreo_components::preferences::Preferences;
use choreo_models::SettingsPreferenceKeys;
use crossbeam_channel::unbounded;

fn unique_temp_file(name: &str) -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("current time should be after unix epoch")
        .as_nanos();
    path.push(format!("rchoreo-{name}-{nanos}.mp3"));
    path
}

#[test]
#[serial_test::serial]
fn open_audio_file_behavior_spec() {
    let suite = rspec::describe("open audio file behavior", (), |spec| {
/*        spec.it(
            "stores stream factory and persists last opened audio path",
            |_| {
                let preferences =
                    Rc::new(choreo_components::preferences::InMemoryPreferences::new());
                let (sender, receiver) = unbounded::<OpenAudioFileCommand>();
                let behavior = OpenAudioFileBehavior::new(
                    receiver,
                    preferences.clone() as Rc<dyn Preferences>,
                );
                let context = audio_player::AudioPlayerTestContext::with_dependencies(
                    vec![Box::new(behavior) as Box<dyn Behavior<_>>],
                    choreo_components::global::GlobalStateActor::new(),
                    preferences.clone(),
                );

                let file_path = unique_temp_file("open-audio");
                fs::write(&file_path, b"audio").expect("audio temp file should be written");
                sender
                    .send(OpenAudioFileCommand {
                        file_path: file_path.to_string_lossy().into_owned(),
                        trace_context: None,
                    })
                    .expect("send should succeed");

                let updated = context.wait_until(Duration::from_secs(1), || {
                    context.view_model.borrow().stream_factory.is_some()
                });
                assert!(updated);

                let stored =
                    preferences.get_string(SettingsPreferenceKeys::LAST_OPENED_AUDIO_FILE, "");
                assert_eq!(stored, file_path.to_string_lossy());

                fs::remove_file(file_path).expect("audio temp file should be removed");
            },
        );*/

        spec.it("ignores empty file paths", |_| {
            let preferences = Rc::new(choreo_components::preferences::InMemoryPreferences::new());
            let (sender, receiver) = unbounded::<OpenAudioFileCommand>();
            let behavior =
                OpenAudioFileBehavior::new(receiver, preferences.clone() as Rc<dyn Preferences>);
            let context = audio_player::AudioPlayerTestContext::with_dependencies(
                vec![Box::new(behavior) as Box<dyn Behavior<_>>],
                choreo_components::global::GlobalStateActor::new(),
                preferences,
            );

            sender
                .send(OpenAudioFileCommand {
                    file_path: "   ".to_string(),
                    trace_context: None,
                })
                .expect("send should succeed");
            context.pump_events();

            assert!(context.view_model.borrow().stream_factory.is_none());
        });
    });

    let report = audio_player::run_suite(&suite);
    assert!(report.is_success());
}
