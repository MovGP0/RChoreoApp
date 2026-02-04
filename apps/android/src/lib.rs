#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

#[allow(unsafe_code)]
#[unsafe(no_mangle)]
#[cfg(target_os = "android")]
fn android_main(app: slint::android::AndroidApp) {
    // Android UI must run on the main (Looper) thread. The main thread stack
    // size is controlled by the OS and is not adjustable from Rust. Keep stack
    // usage low and offload heavy work to background threads when needed.
    use std::cell::RefCell;
    use std::rc::Rc;

    use crossbeam_channel::unbounded;
    use slint::ComponentHandle;
    use choreo_components::audio_player::{
        build_audio_player_behaviors,
        AudioPlayerBehaviorDependencies,
        AudioPlayerViewModel,
        CloseAudioFileCommand,
        LinkSceneToPositionCommand,
        PlatformHapticFeedback,
    };
    use choreo_components::choreo_main::MainPageActionHandlers;
    use choreo_components::choreo_main::MainPageBinding;
    use choreo_components::choreo_main::MainPageDependencies;
    use choreo_components::global::GlobalProvider;
    use choreo_components::i18n;
    use choreo_components::preferences::{PlatformPreferences, Preferences};
    use choreo_components::shell;
    use choreo_i18n::detect_locale;

    if let Err(err) = slint::android::init(app) {
        eprintln!("failed to init Slint Android backend: {err}");
        return;
    }
    let ui = match shell::create_shell_host() {
        Ok(ui) => ui,
        Err(err) => {
            eprintln!("failed to create UI: {err}");
            return;
        }
    };
    let global_provider = GlobalProvider::new();
    let global_state = global_provider.global_state();
    let state_machine = global_provider.state_machine();
    let locale = detect_locale();
    i18n::apply_translations(&ui, &locale);
    let preferences: Rc<dyn Preferences> = Rc::new(PlatformPreferences::new("ChoreoApp"));
    let (open_audio_sender, open_audio_receiver) = unbounded();
    let (close_audio_sender, close_audio_receiver) = unbounded::<CloseAudioFileCommand>();
    let (audio_position_sender, audio_position_receiver) = unbounded();
    let (link_scene_sender, link_scene_receiver) = unbounded::<LinkSceneToPositionCommand>();
    let audio_player_behaviors = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
        global_state: Rc::clone(&global_state),
        open_audio_receiver,
        close_audio_receiver,
        position_changed_sender: audio_position_sender,
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
    let (_redraw_floor_sender, redraw_floor_receiver) = unbounded();

    let actions = MainPageActionHandlers {
        pick_audio_path: None,
        pick_image_path: None,
        request_open_choreo: None,
    };

    let binding = MainPageBinding::new(
        ui,
        MainPageDependencies {
            global_state,
            state_machine,
            audio_player,
            haptic_feedback: Some(Box::new(PlatformHapticFeedback::new())),
            open_audio_sender,
            close_audio_sender,
            audio_position_receiver,
            scenes_show_dialog_sender,
            scenes_close_dialog_sender,
            redraw_floor_receiver,
            preferences,
            actions,
        },
    );
    if let Err(err) = binding.view().run() {
        eprintln!("failed to run UI: {err}");
    }
}

#[cfg(not(target_os = "android"))]
#[allow(dead_code)]
fn android_main(_: ()) {
    // Non-Android builds should not attempt to use the Android backend.
}
