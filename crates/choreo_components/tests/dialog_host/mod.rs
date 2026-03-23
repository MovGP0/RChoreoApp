use std::io;
use std::sync::Arc;

use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;

#[path = "../../src/dialog_host/ui.rs"]
pub mod ui;

pub use rspec::report::Report;

fn click_events(position: egui::Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(position),
        egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::NONE,
        },
        egui::Event::PointerButton {
            pos: position,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::NONE,
        },
    ]
}

fn draw_close_requested(
    is_open: bool,
    close_on_click_away: bool,
    click_position: egui::Pos2,
) -> bool {
    draw_close_requested_in_bounds(
        egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(200.0, 200.0)),
        is_open,
        close_on_click_away,
        click_position,
    )
}

fn draw_close_requested_in_bounds(
    bounds: egui::Rect,
    is_open: bool,
    close_on_click_away: bool,
    click_position: egui::Pos2,
) -> bool {
    let context = egui::Context::default();
    let raw_input = egui::RawInput {
        screen_rect: Some(bounds),
        events: click_events(click_position),
        ..egui::RawInput::default()
    };

    let mut close_requested = false;
    let _ = context.run(raw_input, |ctx| {
        let response = egui::CentralPanel::default().show(ctx, |ui| {
            ui.set_min_size(egui::vec2(200.0, 200.0));
            ui::draw_dialog_host(
                ui,
                &ui::DialogHostProps {
                    id_source: "dialog-host-spec",
                    is_open,
                    close_on_click_away,
                    overlay_color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 128),
                    dialog_background: egui::Color32::from_gray(40),
                    dialog_text_color: egui::Color32::WHITE,
                    dialog_padding: 24,
                    dialog_margin: 24.0,
                    dialog_corner_radius: 12,
                    dialog_content: "Dialog content",
                },
                |_| {},
            )
        });
        close_requested = response.inner;
    });
    close_requested
}

pub fn run_suite<T>(suite: &rspec::block::Suite<T>) -> rspec::report::SuiteReport
where
    T: Clone + Send + Sync + std::fmt::Debug,
{
    let configuration = ConfigurationBuilder::default()
        .parallel(false)
        .exit_on_failure(false)
        .build()
        .expect("rspec configuration should build");
    let logger = Arc::new(Logger::new(io::stdout()));
    let runner = Runner::new(configuration, vec![logger]);
    runner.run(suite)
}

#[test]
fn dialog_host_ui_spec() {
    let suite = rspec::describe("dialog_host ui parity", (), |spec| {
        spec.it("uses a 24px margin around the dialog panel", |_| {
            let bounds = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(480.0, 320.0));
            let panel = ui::dialog_panel_rect(bounds, 24.0);

            assert_eq!(panel.left(), 24.0);
            assert_eq!(panel.top(), 24.0);
            assert_eq!(panel.right(), 456.0);
            assert_eq!(panel.bottom(), 296.0);
        });

        spec.it(
            "clamps panel size to zero when margin exceeds bounds",
            |_| {
                let bounds =
                    egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(40.0, 40.0));
                let panel = ui::dialog_panel_rect(bounds, 24.0);

                assert_eq!(panel.width(), 0.0);
                assert_eq!(panel.height(), 0.0);
            },
        );

        spec.it(
            "returns close requested when overlay is clicked away and click-away is enabled",
            |_| {
                let close_requested = draw_close_requested(true, true, egui::pos2(8.0, 8.0));
                assert!(close_requested);
            },
        );

        spec.it("does not close when click-away is disabled", |_| {
            let close_requested = draw_close_requested(true, false, egui::pos2(8.0, 8.0));
            assert!(!close_requested);
        });

        spec.it(
            "does not close when clicking inside the dialog panel",
            |_| {
                let close_requested = draw_close_requested(true, true, egui::pos2(100.0, 100.0));
                assert!(!close_requested);
            },
        );

        spec.it(
            "closes on click-away when the dialog host is rendered at a non-zero origin",
            |_| {
                let bounds =
                    egui::Rect::from_min_size(egui::pos2(120.0, 80.0), egui::vec2(200.0, 200.0));
                let close_requested =
                    draw_close_requested_in_bounds(bounds, true, true, egui::pos2(128.0, 88.0));

                assert!(close_requested);
            },
        );
    });

    let report = run_suite(&suite);
    assert!(report.is_success());
}
