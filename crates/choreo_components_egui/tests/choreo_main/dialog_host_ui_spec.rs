use egui::Rect;
use egui::pos2;

#[test]
fn dialog_host_panel_respects_24px_margin() {
    let bounds = Rect::from_min_max(pos2(0.0, 0.0), pos2(480.0, 320.0));

    let panel = crate::dialog_host::ui::dialog_panel_rect(bounds, 24.0);

    assert_eq!(panel.left(), 24.0);
    assert_eq!(panel.top(), 24.0);
    assert_eq!(panel.right(), 456.0);
    assert_eq!(panel.bottom(), 296.0);
}
