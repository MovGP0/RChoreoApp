#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

#[cfg(target_arch = "wasm32")]
use choreo_components::AppShellStore;
#[cfg(target_arch = "wasm32")]
use choreo_components::material;
#[cfg(target_arch = "wasm32")]
use choreo_components::shell;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

#[cfg(target_arch = "wasm32")]
const CANVAS_ID: &str = "rchoreo-wasm-egui-canvas";

#[cfg(target_arch = "wasm32")]
#[derive(Default)]
struct WasmEguiApp {
    shell: AppShellStore,
}

#[cfg(target_arch = "wasm32")]
impl WasmEguiApp {
    fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        material::install_image_loaders(&creation_context.egui_ctx);
        Self {
            shell: shell::create_shell_host(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl eframe::App for WasmEguiApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        self.shell.ui(context);
    }
}

#[cfg(target_arch = "wasm32")]
fn ensure_canvas_exists(canvas_id: &str) -> Result<(), String> {
    let window = web_sys::window().ok_or_else(|| "window is unavailable".to_owned())?;
    let document = window
        .document()
        .ok_or_else(|| "document is unavailable".to_owned())?;
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| format!("canvas with id '{canvas_id}' was not found"))?;
    canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| format!("element with id '{canvas_id}' is not a canvas"))?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn init_browser_diagnostics() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let _ = console_log::init_with_level(log::Level::Info);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        init_browser_diagnostics();
        if let Err(error) = ensure_canvas_exists(CANVAS_ID) {
            web_sys::console::error_1(&error.into());
            return;
        }

        spawn_local(async move {
            let runner = eframe::WebRunner::new();
            let web_options = eframe::WebOptions::default();
            let result = runner
                .start(
                    CANVAS_ID,
                    web_options,
                    Box::new(|creation_context| Ok(Box::new(WasmEguiApp::new(creation_context)))),
                )
                .await;

            if let Err(error) = result {
                web_sys::console::error_1(
                    &format!("failed to start wasm egui app: {error}").into(),
                );
            }
        });
    }
}
