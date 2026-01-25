#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

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

fn main() -> Result<(), slint::PlatformError> {
    let ui = shell::create_shell_host()?;
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
        pick_audio_path: Some(Rc::new(pick_audio_path)),
        pick_image_path: Some(Rc::new(pick_image_path)),
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
    binding.view().run()
}

fn pick_audio_path() -> Option<String> {
    rfd::FileDialog::new()
        .set_title("Open audio file")
        .add_filter("Audio", &["mp3"])
        .pick_file()
        .map(|path| path.to_string_lossy().into_owned())
}

fn pick_image_path() -> Option<String> {
    rfd::FileDialog::new()
        .set_title("Open floor plan")
        .add_filter("SVG", &["svg"])
        .pick_file()
        .map(|path| path.to_string_lossy().into_owned())
}
