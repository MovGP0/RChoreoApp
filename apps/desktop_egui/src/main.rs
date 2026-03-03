#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use choreo_components_egui::AppShellViewModel;
use choreo_components_egui::shell;
use std::sync::Once;

const APP_ID: &str = "rchoreo_desktop_egui";
const APP_TITLE: &str = "ChoreoApp";

#[derive(Default)]
struct DesktopEguiApp {
    shell: AppShellViewModel,
}

impl DesktopEguiApp {
    fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        apply_desktop_theme(&_creation_context.egui_ctx);
        Self {
            shell: shell::create_shell_host(),
        }
    }
}

impl eframe::App for DesktopEguiApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        self.shell.ui(context);
    }
}

fn main() -> eframe::Result<()> {
    init_logging();
    log::info!("starting {APP_ID}");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id(APP_ID)
            .with_title(APP_TITLE),
        ..Default::default()
    };

    eframe::run_native(
        APP_TITLE,
        native_options,
        Box::new(|creation_context| Ok(Box::new(DesktopEguiApp::new(creation_context)))),
    )
}

fn apply_desktop_theme(context: &egui::Context) {
    context.set_visuals(egui::Visuals::light());
}

fn init_logging() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .try_init();
    });
}
