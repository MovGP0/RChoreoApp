#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use choreo_components_egui::AppShellViewModel;
use choreo_components_egui::material;
use choreo_components_egui::shell;
use std::env;
use std::sync::Once;

mod app_icon;

const APP_ID: &str = "rchoreo_desktop_egui";
const APP_TITLE: &str = "ChoreoApp";

#[derive(Default)]
struct DesktopEguiApp {
    shell: AppShellViewModel,
}

impl DesktopEguiApp {
    fn new(creation_context: &eframe::CreationContext<'_>) -> Self {
        material::install_image_loaders(&creation_context.egui_ctx);
        apply_desktop_theme(&creation_context.egui_ctx);
        Self {
            shell: shell::create_shell_host(),
        }
    }
}

impl eframe::App for DesktopEguiApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        let dropped_paths = context.input(|input| {
            input
                .raw
                .dropped_files
                .iter()
                .filter_map(|file| file.path.as_ref())
                .map(|path| path.to_string_lossy().into_owned())
                .collect::<Vec<_>>()
        });
        for file_path in dropped_paths {
            self.shell.route_external_file_path(&file_path);
        }
        self.shell.ui(context);
    }
}

fn main() -> eframe::Result<()> {
    init_logging();
    log::info!("starting {APP_ID}");

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id(APP_ID)
            .with_title(APP_TITLE)
            .with_icon(app_icon::load_window_icon().unwrap_or_default()),
        ..Default::default()
    };

    let external_paths = env::args().skip(1).collect::<Vec<_>>();

    eframe::run_native(
        APP_TITLE,
        native_options,
        Box::new(move |creation_context| {
            let app = DesktopEguiApp::new(creation_context);
            for file_path in &external_paths {
                app.shell.route_external_file_path(file_path);
            }
            Ok(Box::new(app))
        }),
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
