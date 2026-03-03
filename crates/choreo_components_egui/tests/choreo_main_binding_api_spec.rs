use std::cell::RefCell;
use std::rc::Rc;

use choreo_components_egui::choreo_main::MainPageActionHandlers;
use choreo_components_egui::choreo_main::MainPageBinding;
use choreo_components_egui::choreo_main::MainPageDependencies;
use choreo_components_egui::choreo_main::OpenAudioRequested;
use choreo_components_egui::choreo_main::OpenImageRequested;
use choreo_components_egui::choreo_main::ShowDialogCommand;
use choreo_components_egui::choreo_main::actions::ChoreoMainAction;
use choreo_components_egui::choreo_main::state::InteractionMode;
use choreo_components_egui::observability::TraceContext;

#[test]
fn binding_forwards_open_audio_request_with_typed_trace_context() {
    let routed_requests: Rc<RefCell<Vec<OpenAudioRequested>>> = Rc::new(RefCell::new(Vec::new()));
    let routed_requests_for_handler = Rc::clone(&routed_requests);

    let binding = MainPageBinding::new(MainPageDependencies {
        action_handlers: MainPageActionHandlers {
            request_open_audio: Some(Rc::new(move |request| {
                routed_requests_for_handler.borrow_mut().push(request);
            })),
            ..MainPageActionHandlers::default()
        },
        ..MainPageDependencies::default()
    });

    let trace_context = TraceContext {
        trace_id_hex: Some("abc123".to_string()),
        span_id_hex: Some("def456".to_string()),
    };
    binding.request_open_audio(OpenAudioRequested {
        file_path: "C:/track.mp3".to_string(),
        trace_context: Some(trace_context.clone()),
    });

    let routed_requests = routed_requests.borrow();
    assert_eq!(routed_requests.len(), 1);
    assert_eq!(routed_requests[0].file_path, "C:/track.mp3");
    assert_eq!(routed_requests[0].trace_context, Some(trace_context));
}

#[test]
fn binding_routes_open_image_request_to_host_handler() {
    let routed_images: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let routed_images_for_handler = Rc::clone(&routed_images);
    let binding = MainPageBinding::new(MainPageDependencies {
        action_handlers: MainPageActionHandlers {
            request_open_image: Some(Rc::new(move |path| {
                routed_images_for_handler.borrow_mut().push(path);
            })),
            ..MainPageActionHandlers::default()
        },
        ..MainPageDependencies::default()
    });

    binding.request_open_image(OpenImageRequested {
        file_path: "C:/floor.svg".to_string(),
    });

    let routed_images = routed_images.borrow();
    assert_eq!(routed_images.as_slice(), ["C:/floor.svg"]);
}

#[test]
fn binding_updates_dialog_and_interaction_mode_state() {
    let binding = MainPageBinding::new(MainPageDependencies::default());

    binding.show_dialog(ShowDialogCommand {
        content: Some("Confirm".to_string()),
    });
    binding.dispatch(ChoreoMainAction::SelectMode { index: 5 });

    let view_model = binding.view_model();
    let state = view_model.borrow();
    assert!(state.state().is_dialog_open);
    assert_eq!(state.state().dialog_content.as_deref(), Some("Confirm"));
    assert_eq!(state.state().interaction_mode, InteractionMode::LineOfSight);
}
