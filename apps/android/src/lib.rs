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
    use choreo_components::choreo_main::MainPageActionHandlers;
    use choreo_components::choreo_main::MainPageBinding;
    use choreo_components::choreo_main::MainPageDependencies;
    use choreo_components::global::GlobalStateModel;
    use choreo_components::preferences::InMemoryPreferences;
    use choreo_components::shell;
    use choreo_state_machine::ApplicationStateMachine;

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
    let audio_player = AudioPlayerViewModel::new(None);
    let preferences = InMemoryPreferences::default();
    let (open_audio_sender, _open_audio_receiver) = unbounded();
    let (open_svg_sender, open_svg_receiver) = unbounded();
    let (_show_dialog_sender, show_dialog_receiver) = unbounded();
    let (_close_dialog_sender, close_dialog_receiver) = unbounded();

    let actions = MainPageActionHandlers {
        pick_audio_path: None,
        pick_image_path: None,
    };

    let binding = MainPageBinding::new(
        ui,
        MainPageDependencies {
            global_state,
            state_machine,
            audio_player,
            haptic_feedback: None,
            open_audio_sender,
            open_svg_sender,
            open_svg_receiver,
            show_dialog_receiver,
            close_dialog_receiver,
            preferences,
            actions,
        },
    );
    binding.view().set_title_text(shell::app_title().into());
    if let Err(err) = binding.view().run() {
        eprintln!("failed to run UI: {err}");
    }
}

#[cfg(not(target_os = "android"))]
#[allow(dead_code)]
fn android_main(_: ()) {
    // Non-Android builds should not attempt to use the Android backend.
}
