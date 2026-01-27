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
    use std::cell::RefCell;
    use std::rc::Rc;

    use crossbeam_channel::unbounded;
    use slint::ComponentHandle;
    use choreo_components::audio_player::AudioPlayerViewModel;
    use choreo_components::audio_player::CloseAudioFileCommand;
    use choreo_components::choreo_main::MainPageActionHandlers;
    use choreo_components::choreo_main::MainPageBinding;
    use choreo_components::choreo_main::MainPageDependencies;
    use choreo_components::global::GlobalStateModel;
    use choreo_components::i18n;
    use choreo_components::preferences::InMemoryPreferences;
    use choreo_components::shell;
    use choreo_state_machine::ApplicationStateMachine;
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
    let global_state = Rc::new(RefCell::new(GlobalStateModel::default()));
    let state_machine = Rc::new(RefCell::new(
        ApplicationStateMachine::with_default_transitions(Box::new(
            GlobalStateModel::default(),
        )),
    ));
    let locale = detect_locale();
    i18n::apply_translations(&ui, &locale);
    let audio_player = AudioPlayerViewModel::new(None);
    let preferences = Rc::new(InMemoryPreferences::default());
    let (open_audio_sender, _open_audio_receiver) = unbounded();
    let (close_audio_sender, _close_audio_receiver) = unbounded::<CloseAudioFileCommand>();
    let (open_svg_sender, open_svg_receiver) = unbounded();
    let (show_dialog_sender, show_dialog_receiver) = unbounded();
    let (close_dialog_sender, close_dialog_receiver) = unbounded();
    let (scenes_show_dialog_sender, _scenes_show_dialog_receiver) = unbounded();
    let (scenes_close_dialog_sender, _scenes_close_dialog_receiver) = unbounded();

    let actions = MainPageActionHandlers {
        pick_audio_path: None,
        pick_image_path: None,
        pick_choreo_path: None,
    };

    let binding = MainPageBinding::new(
        ui,
        MainPageDependencies {
            global_state,
            state_machine,
            audio_player,
            locale,
            haptic_feedback: None,
            open_audio_sender,
            close_audio_sender,
            open_svg_sender,
            open_svg_receiver,
            show_dialog_sender,
            show_dialog_receiver,
            close_dialog_sender,
            close_dialog_receiver,
            scenes_show_dialog_sender,
            scenes_close_dialog_sender,
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
