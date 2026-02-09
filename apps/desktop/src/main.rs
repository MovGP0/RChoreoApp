#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use crossbeam_channel::{Sender, bounded, unbounded};
use i_slint_backend_winit::EventResult;
use i_slint_backend_winit::WinitWindowAccessor;
use i_slint_backend_winit::winit;
use rfd::FileDialog;
use slint::ComponentHandle;

use choreo_components::audio_player::{
    AudioPlayerBehaviorDependencies, AudioPlayerViewModel, CloseAudioFileCommand,
    LinkSceneToPositionCommand, PlatformHapticFeedback, build_audio_player_behaviors,
};
use choreo_components::choreo_main::MainPageActionHandlers;
use choreo_components::choreo_main::MainPageBinding;
use choreo_components::choreo_main::MainPageDependencies;
use choreo_components::global::GlobalProvider;
use choreo_components::i18n;
use choreo_components::preferences::{PlatformPreferences, Preferences};
use choreo_components::scenes::OpenChoreoRequested;
use choreo_components::shell;
use choreo_i18n::detect_locale;

mod observability;

fn main() -> Result<(), slint::PlatformError> {
    let ui_thread = std::thread::Builder::new()
        .name("ui".to_string())
        .stack_size(8 * 1024 * 1024)
        .spawn(run_ui)
        .map_err(|err| slint::PlatformError::from(format!("Failed to spawn UI thread: {err}")))?;
    match ui_thread.join() {
        Ok(result) => result,
        Err(_) => Err(slint::PlatformError::from("UI thread panicked")),
    }
}

fn run_ui() -> Result<(), slint::PlatformError> {
    let _otel_guard = observability::init_debug_otel();
    let _ = observability::capture_trace_context("ui.start");
    let ui = shell::create_shell_host()?;
    let global_provider = GlobalProvider::new();
    let global_state = global_provider.global_state();
    let global_state_store = global_provider.global_state_store();
    let state_machine = global_provider.state_machine();
    let locale = detect_locale();
    i18n::apply_translations(&ui, &locale);
    let preferences: Rc<dyn Preferences> = Rc::new(PlatformPreferences::new("ChoreoApp"));
    const AUDIO_CHANNEL_BUFFER: usize = 1;
    let (open_audio_sender, open_audio_receiver) = bounded(AUDIO_CHANNEL_BUFFER);
    let (close_audio_sender, close_audio_receiver) =
        bounded::<CloseAudioFileCommand>(AUDIO_CHANNEL_BUFFER);
    let (audio_position_sender_for_scenes, audio_position_receiver_for_scenes) =
        bounded(AUDIO_CHANNEL_BUFFER);
    let (audio_position_sender_for_floor, audio_position_receiver_for_floor) =
        bounded(AUDIO_CHANNEL_BUFFER);
    let (link_scene_sender, link_scene_receiver) =
        bounded::<LinkSceneToPositionCommand>(AUDIO_CHANNEL_BUFFER);
    let audio_player_behaviors = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
        global_state_store: Rc::clone(&global_state_store),
        open_audio_receiver,
        close_audio_receiver,
        position_changed_senders: vec![
            audio_position_sender_for_scenes,
            audio_position_sender_for_floor,
        ],
        link_scene_receiver,
        preferences: Rc::clone(&preferences),
    });
    let audio_player = Rc::new(RefCell::new(AudioPlayerViewModel::new(
        None,
        link_scene_sender,
        audio_player_behaviors,
    )));
    let (scenes_show_dialog_sender, _scenes_show_dialog_receiver) = unbounded();
    let (scenes_close_dialog_sender, _scenes_close_dialog_receiver) = unbounded();
    let (redraw_floor_sender, redraw_floor_receiver) = unbounded();

    let actions = MainPageActionHandlers {
        pick_audio_path: Some(Rc::new(pick_audio_path)),
        pick_image_path: Some(Rc::new(pick_image_path)),
        request_open_choreo: Some(Rc::new(request_open_choreo)),
        request_open_audio: None,
        request_open_image: None,
    };

    let binding = MainPageBinding::new(
        ui,
        MainPageDependencies {
            global_state,
            global_state_store,
            state_machine,
            audio_player,
            haptic_feedback: Some(Box::new(PlatformHapticFeedback::new())),
            open_audio_sender,
            close_audio_sender,
            audio_position_receiver_for_scenes,
            audio_position_receiver_for_floor,
            scenes_show_dialog_sender,
            scenes_close_dialog_sender,
            redraw_floor_sender,
            redraw_floor_receiver,
            preferences,
            actions,
        },
    );
    install_desktop_drop_handler(&binding);
    binding.view().run()
}

fn pick_audio_path() -> Option<String> {
    FileDialog::new()
        .set_title("Open audio file")
        .add_filter("Audio", &["mp3"])
        .add_filter("All files", &["*"])
        .pick_file()
        .map(|path| path.to_string_lossy().into_owned())
}

fn pick_image_path() -> Option<String> {
    FileDialog::new()
        .set_title("Open floor plan")
        .add_filter("SVG", &["svg"])
        .add_filter("All files", &["*"])
        .pick_file()
        .map(|path| path.to_string_lossy().into_owned())
}

fn request_open_choreo(sender: Sender<OpenChoreoRequested>) {
    let path = FileDialog::new()
        .set_title("Open choreography file")
        .add_filter("Choreo", &["choreo"])
        .add_filter("All files", &["*"])
        .pick_file();

    let Some(path) = path else {
        return;
    };

    let contents = match std::fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(_) => return,
    };

    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned());
    let file_path = Some(path.to_string_lossy().into_owned());
    let _ = sender.send(OpenChoreoRequested {
        file_path,
        file_name,
        contents,
    });
}

fn install_desktop_drop_handler(binding: &MainPageBinding) {
    let open_choreo_sender = binding.open_choreo_sender();
    let open_audio_request_sender = binding.open_audio_request_sender();
    let open_image_request_sender = binding.open_image_request_sender();

    binding
        .view()
        .window()
        .on_winit_window_event(move |_window, event| {
            let winit::event::WindowEvent::DroppedFile(path) = event else {
                return EventResult::Propagate;
            };

            route_dropped_file(
                path,
                &open_choreo_sender,
                &open_audio_request_sender,
                &open_image_request_sender,
            );

            EventResult::PreventDefault
        });
}

fn route_dropped_file(
    path: &PathBuf,
    open_choreo_sender: &Sender<OpenChoreoRequested>,
    open_audio_request_sender: &Sender<choreo_components::choreo_main::OpenAudioRequested>,
    open_image_request_sender: &Sender<choreo_components::choreo_main::OpenImageRequested>,
) {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();

    if extension == "choreo" {
        let contents = match std::fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(_) => {
                eprintln!("desktop drop: failed to read .choreo file");
                return;
            }
        };
        let file_name = path
            .file_name()
            .map(|value| value.to_string_lossy().into_owned());
        let file_path = Some(path.to_string_lossy().into_owned());
        let _ = open_choreo_sender.try_send(OpenChoreoRequested {
            file_path,
            file_name,
            contents,
        });
        eprintln!("desktop drop: loaded .choreo file");
        return;
    }

    if extension == "svg" {
        let _ = open_image_request_sender.try_send(
            choreo_components::choreo_main::OpenImageRequested {
                file_path: path.to_string_lossy().into_owned(),
            },
        );
        eprintln!("desktop drop: loaded .svg file");
        return;
    }

    if extension == "mp3" {
        let trace_context = observability::capture_trace_context("desktop.drop_open_audio");
        let _ = open_audio_request_sender.try_send(
            choreo_components::choreo_main::OpenAudioRequested {
                file_path: path.to_string_lossy().into_owned(),
                trace_context,
            },
        );
        eprintln!("desktop drop: loaded .mp3 file");
        return;
    }

    eprintln!("desktop drop: unsupported file type");
}
