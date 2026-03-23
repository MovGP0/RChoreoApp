use crate::drawer_host::state::DrawerHostState;
use crate::drawer_host::ui::draw;

#[test]
fn drawer_host_ui_draw_executes_without_panicking() {
    let state = DrawerHostState {
        is_left_open: true,
        inline_left: true,
        ..DrawerHostState::default()
    };

    let context = egui::Context::default();
    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = draw(ui, &state);
        });
    });
}
