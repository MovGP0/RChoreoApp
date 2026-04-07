use crate::drawer_host::Report;
use crate::drawer_host::state::DrawerHostOpenMode;
use crate::drawer_host::state::DrawerHostState;
use crate::drawer_host::ui::compute_layout;
use egui::Rect;
use egui::pos2;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn layout_state_spec() {
    let suite = rspec::describe("drawer host layout state", (), |spec| {
        spec.it(
            "keeps overlay bounds on full host while panel area uses top inset",
            |_| {
                let state = DrawerHostState {
                    top_inset: 84.0,
                    ..DrawerHostState::default()
                };
                let host_rect = Rect::from_min_max(pos2(20.0, 30.0), pos2(1620.0, 930.0));
                let layout = compute_layout(host_rect, &state);

                let mut errors = Vec::new();

                check_eq!(errors, layout.overlay_rect, host_rect);
                check_eq!(errors, layout.panel_rect.min.y, 114.0);
                check_eq!(errors, layout.panel_rect.height(), 816.0);

                assert_no_errors(errors);
            },
        );

        spec.it(
            "positions closed drawers off-screen from the panel area",
            |_| {
                let state = DrawerHostState {
                    left_drawer_width: 320.0,
                    right_drawer_width: 480.0,
                    top_drawer_height: 240.0,
                    bottom_drawer_height: 300.0,
                    top_inset: 36.0,
                    ..DrawerHostState::default()
                };
                let host_rect = Rect::from_min_max(pos2(20.0, 30.0), pos2(1220.0, 930.0));
                let layout = compute_layout(host_rect, &state);

                let mut errors = Vec::new();

                check_eq!(errors, layout.left_panel_rect.left(), -300.0);
                check_eq!(errors, layout.right_panel_rect.left(), 1220.0);
                check_eq!(errors, layout.top_panel_rect.top(), -174.0);
                check_eq!(errors, layout.bottom_panel_rect.top(), 930.0);

                assert_no_errors(errors);
            },
        );

        spec.it(
            "uses responsive inline-left layout in standard mode above the breakpoint",
            |_| {
                let state = DrawerHostState {
                    open_mode: DrawerHostOpenMode::Standard,
                    is_left_open: true,
                    left_drawer_width: 320.0,
                    ..DrawerHostState::default()
                };
                let host_rect = Rect::from_min_max(pos2(20.0, 30.0), pos2(1220.0, 930.0));
                let layout = compute_layout(host_rect, &state);

                let mut errors = Vec::new();

                check_eq!(errors, layout.left_panel_rect.left(), 20.0);
                check_eq!(errors, layout.content_rect.left(), 340.0);
                check_eq!(errors, layout.overlay_rect, host_rect);

                assert_no_errors(errors);
            },
        );

        spec.it(
            "keeps the left drawer closed off-screen in standard mode until it is opened",
            |_| {
                let state = DrawerHostState {
                    open_mode: DrawerHostOpenMode::Standard,
                    is_left_open: false,
                    left_drawer_width: 320.0,
                    ..DrawerHostState::default()
                };
                let host_rect = Rect::from_min_max(pos2(20.0, 30.0), pos2(1220.0, 930.0));
                let layout = compute_layout(host_rect, &state);

                let mut errors = Vec::new();

                check_eq!(errors, layout.left_panel_rect.right(), host_rect.left());
                check_eq!(errors, layout.content_rect.left(), host_rect.left());

                assert_no_errors(errors);
            },
        );

        spec.it(
            "anchors right drawer to the host right edge while keeping top and bottom spans host-wide",
            |_| {
                let state = DrawerHostState {
                    open_mode: DrawerHostOpenMode::Standard,
                    is_left_open: true,
                    is_right_open: true,
                    is_top_open: true,
                    is_bottom_open: true,
                    left_drawer_width: 324.0,
                    right_drawer_width: 480.0,
                    top_drawer_height: 240.0,
                    bottom_drawer_height: 300.0,
                    ..DrawerHostState::default()
                };
                let host_rect = Rect::from_min_max(pos2(120.0, 84.0), pos2(1320.0, 780.0));
                let layout = compute_layout(host_rect, &state);

                let mut errors = Vec::new();

                check_eq!(errors, layout.content_rect.left(), 444.0);
                check_eq!(errors, layout.content_rect.right(), 1320.0);
                check_eq!(errors, layout.right_panel_rect.right(), host_rect.right());
                check_eq!(errors, layout.right_panel_rect.left(), 840.0);
                check_eq!(errors, layout.top_panel_rect.left(), host_rect.left());
                check_eq!(errors, layout.top_panel_rect.right(), host_rect.right());
                check_eq!(errors, layout.bottom_panel_rect.left(), host_rect.left());
                check_eq!(errors, layout.bottom_panel_rect.right(), host_rect.right());

                assert_no_errors(errors);
            },
        );
    });

    let report = crate::drawer_host::run_suite(&suite);
    assert!(report.is_success());
}
