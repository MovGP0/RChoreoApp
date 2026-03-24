use crate::main_page::Report;
use choreo_components::main_page::ui::audio_panel_layer_order;
use choreo_components::main_page::ui::top_bar_layer_order;
use material3::components::drawer_host::ui::content_layer_order;
use material3::components::drawer_host::ui::panel_layer_order;

#[test]
fn z_order_spec() {
    let suite = rspec::describe("main page z-order hierarchy", (), |spec| {
        spec.it(
            "renders navbar above audio, audio above drawers, and drawers above floor",
            |_| {
                assert!(top_bar_layer_order() > audio_panel_layer_order());
                assert!(audio_panel_layer_order() > panel_layer_order());
                assert_eq!(panel_layer_order(), panel_layer_order());
                assert!(panel_layer_order() > content_layer_order());
            },
        );
    });

    let report = crate::main_page::run_suite(&suite);
    assert!(report.is_success());
}
