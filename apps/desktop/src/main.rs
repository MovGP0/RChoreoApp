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
use rfd::FileDialog;
use slint::ComponentHandle;

use choreo_components::audio_player::AudioPlayerViewModel;
use choreo_components::audio_player::CloseAudioFileCommand;
use choreo_components::audio_player::PlatformHapticFeedback;
use choreo_components::choreo_main::MainPageActionHandlers;
use choreo_components::choreo_main::MainPageBinding;
use choreo_components::choreo_main::MainPageDependencies;
use choreo_components::global::GlobalStateModel;
use choreo_components::i18n;
use choreo_components::preferences::PlatformPreferences;
use choreo_components::shell;
use choreo_state_machine::ApplicationStateMachine;
use choreo_i18n::detect_locale;

fn main() -> Result<(), slint::PlatformError> {
    let ui = shell::create_shell_host()?;
    let global_state = Rc::new(RefCell::new(GlobalStateModel::default()));
    let state_machine = Rc::new(RefCell::new(
        ApplicationStateMachine::with_default_transitions(Box::new(
            GlobalStateModel::default(),
        )),
    ));
    let locale = detect_locale();
    i18n::apply_translations(&ui, &locale);
    let audio_player = AudioPlayerViewModel::new(None);
    let preferences = Rc::new(PlatformPreferences::new("ChoreoApp"));
    let (open_audio_sender, _open_audio_receiver) = unbounded();
    let (close_audio_sender, _close_audio_receiver) = unbounded::<CloseAudioFileCommand>();
    let (_audio_position_sender, audio_position_receiver) = unbounded();
    let (open_svg_sender, open_svg_receiver) = unbounded();
    let (show_dialog_sender, show_dialog_receiver) = unbounded();
    let (close_dialog_sender, close_dialog_receiver) = unbounded();
    let (scenes_show_dialog_sender, _scenes_show_dialog_receiver) = unbounded();
    let (scenes_close_dialog_sender, _scenes_close_dialog_receiver) = unbounded();
    let (_redraw_floor_sender, redraw_floor_receiver) = unbounded();

    let actions = MainPageActionHandlers {
        pick_audio_path: Some(Rc::new(pick_audio_path)),
        pick_image_path: Some(Rc::new(pick_image_path)),
        pick_choreo_path: Some(Rc::new(pick_choreo_path)),
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
            open_svg_sender,
            open_svg_receiver,
            show_dialog_sender,
            show_dialog_receiver,
            close_dialog_sender,
            close_dialog_receiver,
            scenes_show_dialog_sender,
            scenes_close_dialog_sender,
            redraw_floor_receiver,
            preferences,
            actions,
        },
    );
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

fn pick_choreo_path() -> Option<String> {
    FileDialog::new()
        .set_title("Open choreography file")
        .add_filter("Choreo", &["choreo"])
        .add_filter("All files", &["*"])
        .pick_file()
        .map(|path| path.to_string_lossy().into_owned())
}
