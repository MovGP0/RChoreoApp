#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

use choreo_components_egui::AppShellViewModel;

#[derive(Default)]
struct DesktopEguiApp {
    shell: AppShellViewModel,
}

impl DesktopEguiApp {
    fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            shell: AppShellViewModel::new("ChoreoApp egui"),
        }
    }
}

impl eframe::App for DesktopEguiApp {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        self.shell.ui(context);
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "ChoreoApp egui",
        native_options,
        Box::new(|creation_context| Ok(Box::new(DesktopEguiApp::new(creation_context)))),
    )
}
