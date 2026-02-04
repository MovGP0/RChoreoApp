#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use std::cell::RefCell;
use std::rc::Rc;

use crossbeam_channel::{unbounded, Sender};
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
use choreo_components::scenes::OpenChoreoRequested;
use choreo_components::shell;
use choreo_i18n::detect_locale;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    // WASM runs on the browser main thread. There is no per-thread stack size
    // configuration at runtime. If stack usage becomes an issue, reduce stack
    // allocations (move large locals to heap) or adjust toolchain stack settings.
    let ui = match shell::create_shell_host() {
        Ok(ui) => ui,
        Err(err) => {
            eprintln!("failed to create UI: {err}");
            return;
        }
    };
    let global_provider = GlobalProvider::new();
    let global_state = global_provider.global_state();
    let global_state_store = global_provider.global_state_store();
    let state_machine = global_provider.state_machine();
    let locale = detect_locale();
    i18n::apply_translations(&ui, &locale);
    let preferences: Rc<dyn Preferences> = Rc::new(PlatformPreferences::new("ChoreoApp"));
    let (open_audio_sender, open_audio_receiver) = unbounded();
    let (close_audio_sender, close_audio_receiver) = unbounded::<CloseAudioFileCommand>();
    let (audio_position_sender, audio_position_receiver) = unbounded();
    let (link_scene_sender, link_scene_receiver) = unbounded::<LinkSceneToPositionCommand>();
    let audio_player_behaviors = build_audio_player_behaviors(AudioPlayerBehaviorDependencies {
        global_state_store: Rc::clone(&global_state_store),
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
        request_open_choreo: Some(Rc::new(request_open_choreo)),
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

#[cfg(target_arch = "wasm32")]
fn request_open_choreo(sender: Sender<OpenChoreoRequested>) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Ok(element) = document.create_element("input") else {
        return;
    };
    let Ok(input) = element.dyn_into::<web_sys::HtmlInputElement>() else {
        return;
    };
    input.set_type("file");
    input.set_accept(".choreo,application/json");

    let sender_for_change = sender.clone();
    let onchange = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let target = match event.target() {
            Some(target) => target,
            None => return,
        };
        let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() else {
            return;
        };
        let Some(files) = input.files() else {
            return;
        };
        let Some(file) = files.get(0) else {
            return;
        };
        let file_name = file.name();
        let Ok(reader) = web_sys::FileReader::new() else {
            return;
        };
        let sender_for_load = sender_for_change.clone();
        let onloadend = Closure::wrap(Box::new(move |_event: web_sys::ProgressEvent| {
            let Ok(result) = reader.result() else {
                return;
            };
            let Some(contents) = result.as_string() else {
                return;
            };
            let _ = sender_for_load.send(OpenChoreoRequested {
                file_path: None,
                file_name: Some(file_name.clone()),
                contents,
            });
        }) as Box<dyn FnMut(_)>);

        reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
        let _ = reader.read_as_text(&file);
        onloadend.forget();
    }) as Box<dyn FnMut(_)>);

    input.set_onchange(Some(onchange.as_ref().unchecked_ref()));
    if let Some(body) = document.body() {
        let _ = body.append_child(&input);
    }
    input.click();
    onchange.forget();
}
