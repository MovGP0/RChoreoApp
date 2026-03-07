use std::cell::RefCell;
use std::rc::Rc;

use choreo_master_mobile_json::export;
use choreo_components_egui::choreo_main::MainPageActionHandlers;
use choreo_components_egui::choreo_main::MainPageBinding;
use choreo_components_egui::choreo_main::MainPageDependencies;
use choreo_components_egui::choreo_main::OpenAudioRequested;
use choreo_components_egui::choreo_main::actions::OpenChoreoRequested;
use choreo_components_egui::choreo_main::OpenImageRequested;
use choreo_components_egui::choreo_main::ShowDialogCommand;
use choreo_components_egui::choreo_main::actions::ChoreoMainAction;
use choreo_components_egui::choreo_main::state::InteractionMode;
use choreo_components_egui::observability::TraceContext;
use choreo_models::ChoreographyModel;
use choreo_models::ChoreographyModelMapper;

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

#[test]
fn binding_uses_pick_choreo_handler_to_load_selected_file() {
    let mut choreography = ChoreographyModel {
        name: "Loaded choreo".to_string(),
        ..ChoreographyModel::default()
    };
    choreography.scenes = vec![choreo_models::SceneModel {
        scene_id: choreo_master_mobile_json::SceneId(1),
        positions: Vec::new(),
        name: "Intro".to_string(),
        text: None,
        fixed_positions: false,
        timestamp: Some("00:01.500".to_string()),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: choreo_master_mobile_json::Color::transparent(),
    }];
    let mapper = ChoreographyModelMapper;
    let contents = export(&mapper.map_to_json(&choreography))
        .expect("test choreography should serialize to json");

    let binding = MainPageBinding::new(MainPageDependencies {
        action_handlers: MainPageActionHandlers {
            pick_choreo_file: Some(Rc::new(move || {
                Some(OpenChoreoRequested {
                    file_path: Some("C:/demo.choreo".to_string()),
                    file_name: Some("demo.choreo".to_string()),
                    contents: contents.clone(),
                })
            })),
            ..MainPageActionHandlers::default()
        },
        ..MainPageDependencies::default()
    });

    binding.dispatch(ChoreoMainAction::RequestOpenChoreo(OpenChoreoRequested {
        file_path: None,
        file_name: None,
        contents: String::new(),
    }));

    let view_model = binding.view_model();
    let state = view_model.borrow();
    assert_eq!(state.state().scenes.len(), 1);
    assert_eq!(state.state().scenes[0].name, "Intro");
    assert_eq!(state.state().selected_scene_index, Some(0));
    assert_eq!(state.state().floor_scene_name.as_deref(), Some("Intro"));
    assert_eq!(state.state().choreography_settings_state.name, "Loaded choreo");
}
