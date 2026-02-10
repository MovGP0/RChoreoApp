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
        spec.it(
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

                let file_path = unique_temp_file("open-audio")
                    .to_string_lossy()
                    .into_owned();
                sender
                    .send(OpenAudioFileCommand {
                        file_path: file_path.clone(),
                        trace_context: None,
                    })
                    .expect("send should succeed");
                context.pump_events();

                let stored =
                    preferences.get_string(SettingsPreferenceKeys::LAST_OPENED_AUDIO_FILE, "");
                assert_eq!(stored, file_path);
                assert!(context.view_model.borrow().stream_factory.is_some());
            },
        );

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

        spec.it("keeps player disabled when the selected mp3 file is invalid", |_| {
            let preferences = Rc::new(choreo_components::preferences::InMemoryPreferences::new());
            let (sender, receiver) = unbounded::<OpenAudioFileCommand>();
            let behavior =
                OpenAudioFileBehavior::new(receiver, preferences.clone() as Rc<dyn Preferences>);
            let context = audio_player::AudioPlayerTestContext::with_dependencies(
                vec![Box::new(behavior) as Box<dyn Behavior<_>>],
                choreo_components::global::GlobalStateActor::new(),
                preferences.clone(),
            );

            let file_path = unique_temp_file("invalid-audio");
            fs::write(&file_path, b"not-an-mp3").expect("invalid temp file should be written");

            sender
                .send(OpenAudioFileCommand {
                    file_path: file_path.to_string_lossy().into_owned(),
                    trace_context: None,
                })
                .expect("send should succeed");

            let settled = context.wait_until(Duration::from_secs(1), || {
                let view_model = context.view_model.borrow();
                view_model.stream_factory.is_some() && view_model.player.is_some()
            });
            assert!(settled);

            context.view_model.borrow_mut().toggle_play_pause();
            let not_playing = context.wait_until(Duration::from_secs(1), || {
                let view_model = context.view_model.borrow();
                view_model
                    .player
                    .as_ref()
                    .map(|player| !player.is_playing())
                    .unwrap_or(true)
            });
            assert!(not_playing);

            let stored =
                preferences.get_string(SettingsPreferenceKeys::LAST_OPENED_AUDIO_FILE, "");
            assert_eq!(stored, file_path.to_string_lossy());

            fs::remove_file(file_path).expect("invalid temp file should be removed");
        });
    });

    let report = audio_player::run_suite(&suite);
    assert!(report.is_success());
}
