use std::time::Duration;

use crate::choreo_main;

use choreo_components::behavior::Behavior;
use choreo_components::choreo_main::OpenImageBehavior;
use choreo_components::choreo_main::OpenImageRequested;
use choreo_components::choreo_main::OpenSvgFileCommand;
use choreo_main::Report;
use crossbeam_channel::unbounded;

#[test]
#[serial_test::serial]
fn open_image_behavior_spec() {
    let suite = rspec::describe("open image behavior", (), |spec| {
        spec.it("forwards requested image path as open-svg command", |_| {
            let (open_svg_sender, open_svg_receiver) = unbounded::<OpenSvgFileCommand>();
            let (sender, receiver) = unbounded::<OpenImageRequested>();
            let behavior = OpenImageBehavior::new(open_svg_sender, receiver);
            let context = choreo_main::ChoreoMainTestContext::new(vec![
                Box::new(behavior) as Box<dyn Behavior<_>>
            ]);

            sender
                .send(OpenImageRequested {
                    file_path: "C:/image.svg".to_string(),
                })
                .expect("send should succeed");

            let forwarded = context.wait_until(Duration::from_secs(1), || {
                open_svg_receiver.try_recv().is_ok()
            });
            assert!(forwarded);
        });
    });

    let report = choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
