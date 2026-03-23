#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use choreo_components::AppShellViewModel;
use choreo_components::choreo_main::MainPageActionHandlers;
use choreo_components::choreo_main::MainPageDependencies;
use choreo_components::choreo_main::actions::OpenChoreoRequested;
use choreo_components::material;
use choreo_components::shell;
use rfd::FileDialog;
use std::env;
use std::path::Path;
use std::rc::Rc;
use std::sync::Once;

mod app_icon;

const APP_ID: &str = "rchoreo_desktop";
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
            shell: create_desktop_shell_host(),
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

fn create_desktop_shell_host() -> AppShellViewModel {
    shell::create_shell_host_with_dependencies(desktop_main_page_dependencies())
}

fn desktop_main_page_dependencies() -> MainPageDependencies {
    MainPageDependencies {
        action_handlers: MainPageActionHandlers {
            pick_choreo_file: Some(Rc::new(pick_choreo_file)),
            pick_audio_path: Some(Rc::new(pick_audio_path)),
            pick_image_path: Some(Rc::new(pick_image_path)),
            ..MainPageActionHandlers::default()
        },
        ..MainPageDependencies::default()
    }
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

fn pick_choreo_file() -> Option<OpenChoreoRequested> {
    let path = FileDialog::new()
        .set_title("Open choreography file")
        .add_filter("Choreo", &["choreo"])
        .add_filter("All files", &["*"])
        .pick_file()?;
    load_open_choreo_request_from_path(&path)
}

fn load_open_choreo_request_from_path(path: &Path) -> Option<OpenChoreoRequested> {
    let contents = std::fs::read_to_string(path).ok()?;
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned());
    let file_path = Some(path.to_string_lossy().into_owned());
    Some(OpenChoreoRequested {
        file_path,
        file_name,
        contents,
    })
}

fn init_logging() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .try_init();
    });
}

#[cfg(test)]
mod desktop_open_choreo_spec {
    use std::fs;
    use std::path::PathBuf;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use super::desktop_main_page_dependencies;
    use super::load_open_choreo_request_from_path;

    #[test]
    fn desktop_host_wires_open_choreo_handler() {
        let dependencies = desktop_main_page_dependencies();
        assert!(dependencies.action_handlers.pick_choreo_file.is_some());
    }

    #[test]
    fn load_open_choreo_request_reads_contents_and_metadata() {
        let path = unique_temp_file("choreo");
        fs::write(&path, "demo choreography").expect("test should write .choreo file");

        let request = load_open_choreo_request_from_path(&path)
            .expect("existing .choreo file should load into open request");

        assert_eq!(
            request.file_path.as_deref(),
            Some(path.to_string_lossy().as_ref())
        );
        assert_eq!(
            request.file_name.as_deref(),
            path.file_name().and_then(|value| value.to_str())
        );
        assert_eq!(request.contents, "demo choreography");

        let _ = fs::remove_file(path);
    }

    fn unique_temp_file(extension: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let mut path = std::env::temp_dir();
        path.push(format!("rchoreo_desktop_{nanos}.{extension}"));
        path
    }
}
