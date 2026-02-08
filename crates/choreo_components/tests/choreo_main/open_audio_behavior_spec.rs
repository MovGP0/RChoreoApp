use std::time::Duration;

use crate::choreo_main;

use choreo_components::audio_player::OpenAudioFileCommand;
use choreo_components::behavior::Behavior;
use choreo_components::choreo_main::OpenAudioBehavior;
use choreo_components::choreo_main::OpenAudioRequested;
use crossbeam_channel::unbounded;
use choreo_main::Report;

#[test]
#[serial_test::serial]
fn open_audio_behavior_spec() {
    let suite = rspec::describe("open audio behavior", (), |spec| {
        spec.it("forwards requested file path to audio command", |_| {
            let (open_audio_sender, open_audio_receiver) = unbounded::<OpenAudioFileCommand>();
            let (sender, receiver) = unbounded::<OpenAudioRequested>();
            let behavior = OpenAudioBehavior::new(open_audio_sender, receiver);
            let context = choreo_main::ChoreoMainTestContext::new(vec![
                Box::new(behavior) as Box<dyn Behavior<_>>,
            ]);

            sender
                .send(OpenAudioRequested {
                    file_path: "C:/music.mp3".to_string(),
                })
                .expect("send should succeed");

            let forwarded = context.wait_until(Duration::from_secs(1), || open_audio_receiver.try_recv().is_ok());
            assert!(forwarded);
        });
    });

    let report = choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
