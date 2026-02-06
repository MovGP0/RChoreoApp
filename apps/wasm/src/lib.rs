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

#[cfg(target_arch = "wasm32")]
use choreo_components::choreo_main::{OpenAudioRequested, OpenImageRequested};

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

    let file_picker_actor = create_web_file_picker_actor();
    let actions = MainPageActionHandlers {
        pick_audio_path: None,
        pick_image_path: None,
        request_open_choreo: Some(Rc::new({
            let file_picker_actor = file_picker_actor.clone();
            move |sender| {
                if let Some(actor) = file_picker_actor.as_ref() {
                    actor.request_open_choreo(sender);
                }
            }
        })),
        request_open_audio: Some(Rc::new({
            let file_picker_actor = file_picker_actor.clone();
            move |sender| {
                if let Some(actor) = file_picker_actor.as_ref() {
                    actor.request_open_audio(sender);
                }
            }
        })),
        request_open_image: Some(Rc::new({
            let file_picker_actor = file_picker_actor.clone();
            move |sender| {
                if let Some(actor) = file_picker_actor.as_ref() {
                    actor.request_open_image(sender);
                }
            }
        })),
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
fn create_web_file_picker_actor() -> Option<Rc<WebFilePickerActor>> {
    WebFilePickerActor::new().map(Rc::new)
}

#[cfg(not(target_arch = "wasm32"))]
fn create_web_file_picker_actor() -> Option<Rc<WebFilePickerActor>> {
    None
}

#[cfg(target_arch = "wasm32")]
struct WebFilePickerActor {
    choreo_input: web_sys::HtmlInputElement,
    audio_input: web_sys::HtmlInputElement,
    image_input: web_sys::HtmlInputElement,
    _drop_overlay: web_sys::HtmlElement,
    choreo_sender: Rc<RefCell<Option<Sender<OpenChoreoRequested>>>>,
    audio_sender: Rc<RefCell<Option<Sender<OpenAudioRequested>>>>,
    image_sender: Rc<RefCell<Option<Sender<OpenImageRequested>>>>,
    _choreo_onchange: Closure<dyn FnMut(web_sys::Event)>,
    _audio_onchange: Closure<dyn FnMut(web_sys::Event)>,
    _image_onchange: Closure<dyn FnMut(web_sys::Event)>,
    _dragenter: Closure<dyn FnMut(web_sys::Event)>,
    _dragover: Closure<dyn FnMut(web_sys::Event)>,
    _dragleave: Closure<dyn FnMut(web_sys::Event)>,
    _drop: Closure<dyn FnMut(web_sys::Event)>,
}

#[cfg(not(target_arch = "wasm32"))]
struct WebFilePickerActor;

#[cfg(not(target_arch = "wasm32"))]
impl WebFilePickerActor {
    fn request_open_choreo(&self, _sender: Sender<OpenChoreoRequested>) {}

    fn request_open_audio(
        &self,
        _sender: Sender<choreo_components::choreo_main::OpenAudioRequested>,
    ) {
    }

    fn request_open_image(
        &self,
        _sender: Sender<choreo_components::choreo_main::OpenImageRequested>,
    ) {
    }
}

#[cfg(target_arch = "wasm32")]
impl WebFilePickerActor {
    fn new() -> Option<Self> {
        let window = web_sys::window()?;
        let document = window.document()?;

        let choreo_input = Self::create_hidden_file_input(
            &document,
            "wasm-choreo-picker",
            ".choreo,application/json",
        )?;
        let audio_input =
            Self::create_hidden_file_input(&document, "wasm-audio-picker", "audio/*")?;
        let image_input = Self::create_hidden_file_input(
            &document,
            "wasm-image-picker",
            ".svg,image/svg+xml",
        )?;
        let drop_overlay = Self::create_drop_overlay(&document)?;

        let choreo_sender = Rc::new(RefCell::new(None));
        let audio_sender = Rc::new(RefCell::new(None));
        let image_sender = Rc::new(RefCell::new(None));

        let choreo_onchange = {
            let choreo_sender = Rc::clone(&choreo_sender);
            Closure::wrap(Box::new(move |event: web_sys::Event| {
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
                Self::handle_dropped_file(
                    file,
                    Rc::clone(&choreo_sender),
                    Rc::new(RefCell::new(None)),
                    Rc::new(RefCell::new(None)),
                );
            }) as Box<dyn FnMut(_)>)
        };
        choreo_input.set_onchange(Some(choreo_onchange.as_ref().unchecked_ref()));

        let audio_onchange = {
            let audio_sender = Rc::clone(&audio_sender);
            Closure::wrap(Box::new(move |event: web_sys::Event| {
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
                Self::handle_dropped_file(
                    file,
                    Rc::new(RefCell::new(None)),
                    Rc::clone(&audio_sender),
                    Rc::new(RefCell::new(None)),
                );
            }) as Box<dyn FnMut(_)>)
        };
        audio_input.set_onchange(Some(audio_onchange.as_ref().unchecked_ref()));

        let image_onchange = {
            let image_sender = Rc::clone(&image_sender);
            Closure::wrap(Box::new(move |event: web_sys::Event| {
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
                Self::handle_dropped_file(
                    file,
                    Rc::new(RefCell::new(None)),
                    Rc::new(RefCell::new(None)),
                    Rc::clone(&image_sender),
                );
            }) as Box<dyn FnMut(_)>)
        };
        image_input.set_onchange(Some(image_onchange.as_ref().unchecked_ref()));

        let dragenter = {
            let drop_overlay = drop_overlay.clone();
            Closure::wrap(Box::new(move |event: web_sys::Event| {
                event.prevent_default();
                Self::set_overlay_visible(&drop_overlay, true);
            }) as Box<dyn FnMut(_)>)
        };
        let dragover = {
            let drop_overlay = drop_overlay.clone();
            Closure::wrap(Box::new(move |event: web_sys::Event| {
                event.prevent_default();
                Self::set_overlay_visible(&drop_overlay, true);
            }) as Box<dyn FnMut(_)>)
        };
        let dragleave = {
            let drop_overlay = drop_overlay.clone();
            Closure::wrap(Box::new(move |event: web_sys::Event| {
                event.prevent_default();
                Self::set_overlay_visible(&drop_overlay, false);
            }) as Box<dyn FnMut(_)>)
        };
        let drop = {
            let drop_overlay = drop_overlay.clone();
            let choreo_sender = Rc::clone(&choreo_sender);
            let audio_sender = Rc::clone(&audio_sender);
            let image_sender = Rc::clone(&image_sender);
            Closure::wrap(Box::new(move |event: web_sys::Event| {
                event.prevent_default();
                Self::set_overlay_visible(&drop_overlay, false);

                let Some(drag_event) = event.dyn_ref::<web_sys::DragEvent>() else {
                    return;
                };
                let Some(data_transfer) = drag_event.data_transfer() else {
                    return;
                };
                let Some(files) = data_transfer.files() else {
                    return;
                };

                let length = files.length();
                for index in 0..length {
                    let Some(file) = files.get(index) else {
                        continue;
                    };
                    Self::handle_dropped_file(
                        file,
                        Rc::clone(&choreo_sender),
                        Rc::clone(&audio_sender),
                        Rc::clone(&image_sender),
                    );
                }
            }) as Box<dyn FnMut(_)>)
        };

        let body = document.body()?;
        let _ = body.add_event_listener_with_callback(
            "dragenter",
            dragenter.as_ref().unchecked_ref(),
        );
        let _ = body.add_event_listener_with_callback(
            "dragover",
            dragover.as_ref().unchecked_ref(),
        );
        let _ = body.add_event_listener_with_callback(
            "dragleave",
            dragleave.as_ref().unchecked_ref(),
        );
        let _ = body.add_event_listener_with_callback(
            "drop",
            drop.as_ref().unchecked_ref(),
        );

        Some(Self {
            choreo_input,
            audio_input,
            image_input,
            _drop_overlay: drop_overlay,
            choreo_sender,
            audio_sender,
            image_sender,
            _choreo_onchange: choreo_onchange,
            _audio_onchange: audio_onchange,
            _image_onchange: image_onchange,
            _dragenter: dragenter,
            _dragover: dragover,
            _dragleave: dragleave,
            _drop: drop,
        })
    }

    fn create_hidden_file_input(
        document: &web_sys::Document,
        id: &str,
        accept: &str,
    ) -> Option<web_sys::HtmlInputElement> {
        let Ok(element) = document.create_element("input") else {
            return None;
        };
        let Ok(input) = element.dyn_into::<web_sys::HtmlInputElement>() else {
            return None;
        };
        input.set_type("file");
        input.set_id(id);
        input.set_accept(accept);
        let _ = input.set_attribute("class", "wasm-file-picker");
        let _ = input.set_attribute("style", "display:none");
        let _ = input.set_attribute("aria-hidden", "true");
        if let Some(body) = document.body() {
            let _ = body.append_child(&input);
            return Some(input);
        }
        None
    }

    fn create_drop_overlay(document: &web_sys::Document) -> Option<web_sys::HtmlElement> {
        let Ok(element) = document.create_element("div") else {
            return None;
        };
        let Ok(overlay) = element.dyn_into::<web_sys::HtmlElement>() else {
            return None;
        };
        overlay.set_id("wasm-drop-overlay");
        overlay.set_inner_text("Drop .choreo, .svg or .mp3 files");
        let _ = overlay.set_attribute(
            "style",
            "position:fixed;inset:0;display:none;align-items:center;justify-content:center;\
             background:rgba(0,0,0,0.35);color:white;font:600 24px sans-serif;z-index:2147483647;",
        );
        if let Some(body) = document.body() {
            let _ = body.append_child(&overlay);
            return Some(overlay);
        }
        None
    }

    fn set_overlay_visible(overlay: &web_sys::HtmlElement, visible: bool) {
        let style = if visible {
            "position:fixed;inset:0;display:flex;align-items:center;justify-content:center;\
             background:rgba(0,0,0,0.35);color:white;font:600 24px sans-serif;z-index:2147483647;"
        } else {
            "position:fixed;inset:0;display:none;align-items:center;justify-content:center;\
             background:rgba(0,0,0,0.35);color:white;font:600 24px sans-serif;z-index:2147483647;"
        };
        let _ = overlay.set_attribute("style", style);
    }

    fn handle_dropped_file(
        file: web_sys::File,
        choreo_sender: Rc<RefCell<Option<Sender<OpenChoreoRequested>>>>,
        audio_sender: Rc<RefCell<Option<Sender<OpenAudioRequested>>>>,
        image_sender: Rc<RefCell<Option<Sender<OpenImageRequested>>>>,
    ) {
        let file_name = file.name();
        let extension = file_name
            .rsplit('.')
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();

        if extension == "choreo" {
            let Some(sender) = choreo_sender.borrow().as_ref().cloned() else {
                return;
            };
            web_sys::console::log_1(&"drop: handling .choreo".into());
            let Ok(reader) = web_sys::FileReader::new() else {
                return;
            };
            let reader_for_callback = reader.clone();
            let onloadend = Closure::wrap(Box::new(move |_event: web_sys::ProgressEvent| {
                let Ok(result) = reader_for_callback.result() else {
                    return;
                };
                let Some(contents) = result.as_string() else {
                    return;
                };
                let _ = sender.send(OpenChoreoRequested {
                    file_path: None,
                    file_name: Some(file_name.clone()),
                    contents,
                });
            }) as Box<dyn FnMut(_)>);
            reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
            let _ = reader.read_as_text(&file);
            onloadend.forget();
            return;
        }

        if extension == "svg" {
            let Some(sender) = image_sender.borrow().as_ref().cloned() else {
                return;
            };
            web_sys::console::log_1(&"drop: handling .svg".into());
            let object_url = match web_sys::Url::create_object_url_with_blob(&file) {
                Ok(url) => url,
                Err(_) => return,
            };
            let _ = sender.send(OpenImageRequested {
                file_path: object_url,
            });
            return;
        }

        if extension == "mp3" {
            let Some(sender) = audio_sender.borrow().as_ref().cloned() else {
                return;
            };
            web_sys::console::log_1(&"drop: handling .mp3".into());
            let object_url = match web_sys::Url::create_object_url_with_blob(&file) {
                Ok(url) => url,
                Err(_) => return,
            };
            let _ = sender.send(OpenAudioRequested {
                file_path: object_url,
            });
            return;
        }

        web_sys::console::log_1(&format!("drop: unsupported file type ({file_name})").into());
    }

    fn request_open_choreo(&self, sender: Sender<OpenChoreoRequested>) {
        self.choreo_sender.replace(Some(sender));
        self.choreo_input.set_value("");
        self.choreo_input.click();
    }

    fn request_open_audio(&self, sender: Sender<OpenAudioRequested>) {
        self.audio_sender.replace(Some(sender));
        self.audio_input.set_value("");
        self.audio_input.click();
    }

    fn request_open_image(&self, sender: Sender<OpenImageRequested>) {
        self.image_sender.replace(Some(sender));
        self.image_input.set_value("");
        self.image_input.click();
    }
}
