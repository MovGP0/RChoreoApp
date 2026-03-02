use crate::main_page_drawer_host::state::MainPageDrawerHostState;
use crate::main_page_drawer_host::ui::draw;

#[test]
fn main_page_drawer_host_ui_draw_executes_without_panicking() {
    let state = MainPageDrawerHostState::default();
    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = draw(ui, &state);
        });
    });
}
