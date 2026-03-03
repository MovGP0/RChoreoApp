use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::choreo_main::Report;
use choreo_components_egui::choreo_main::MainPageActionHandlers;
use choreo_components_egui::choreo_main::MainPageBinding;
use choreo_components_egui::choreo_main::MainPageDependencies;
use choreo_components_egui::choreo_main::OpenAudioRequested;
use choreo_components_egui::choreo_main::actions::OpenChoreoRequested;

#[test]
fn external_file_routing_spec() {
    let suite = rspec::describe("external file routing", (), |spec| {
        spec.it(
            "routes choreo files to typed open-choreo handler with file metadata",
            |_| {
                let routed_choreo: Rc<RefCell<Vec<OpenChoreoRequested>>> =
                    Rc::new(RefCell::new(Vec::new()));
                let routed_choreo_for_handler = Rc::clone(&routed_choreo);

                let binding = MainPageBinding::new(MainPageDependencies {
                    action_handlers: MainPageActionHandlers {
                        request_open_choreo: Some(Rc::new(move |request| {
                            routed_choreo_for_handler.borrow_mut().push(request);
                        })),
                        ..MainPageActionHandlers::default()
                    },
                    ..MainPageDependencies::default()
                });

                let choreo_file = unique_temp_file("choreo");
                let contents = "demo choreography";
                fs::write(&choreo_file, contents).expect("test should write .choreo file");

                binding.route_external_file_path(&choreo_file.to_string_lossy());

                let routed = routed_choreo.borrow();
                assert_eq!(routed.len(), 1);
                assert_eq!(
                    routed[0].file_path.as_deref(),
                    Some(choreo_file.to_string_lossy().as_ref())
                );
                assert_eq!(
                    routed[0].file_name.as_deref(),
                    choreo_file.file_name().and_then(|value| value.to_str())
                );
                assert_eq!(routed[0].contents, contents);

                let _ = fs::remove_file(choreo_file);
            },
        );

        spec.it("routes svg and mp3 files to image/audio handlers", |_| {
            let routed_images: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
            let routed_audio: Rc<RefCell<Vec<OpenAudioRequested>>> =
                Rc::new(RefCell::new(Vec::new()));
            let routed_images_for_handler = Rc::clone(&routed_images);
            let routed_audio_for_handler = Rc::clone(&routed_audio);

            let binding = MainPageBinding::new(MainPageDependencies {
                action_handlers: MainPageActionHandlers {
                    request_open_image: Some(Rc::new(move |path| {
                        routed_images_for_handler.borrow_mut().push(path);
                    })),
                    request_open_audio: Some(Rc::new(move |request| {
                        routed_audio_for_handler.borrow_mut().push(request);
                    })),
                    ..MainPageActionHandlers::default()
                },
                ..MainPageDependencies::default()
            });

            binding.route_external_file_path("C:/floor.svg");
            binding.route_external_file_path("C:/track.mp3");
            binding.route_external_file_path("C:/ignored.txt");

            let routed_images = routed_images.borrow();
            let routed_audio = routed_audio.borrow();
            assert_eq!(routed_images.as_slice(), ["C:/floor.svg"]);
            assert_eq!(routed_audio.len(), 1);
            assert_eq!(routed_audio[0].file_path, "C:/track.mp3");
            assert_eq!(routed_audio[0].trace_context, None);
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}

fn unique_temp_file(extension: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let mut path = std::env::temp_dir();
    path.push(format!("rchoreo_{nanos}.{extension}"));
    path
}
