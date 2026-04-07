use crate::dancers;
use crate::dancers::Report;
use egui::Color32;
use material3::components::drawer_host::state::DrawerHostOpenMode;

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

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
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
fn dancer_settings_page_control_spec() {
    let suite = rspec::describe("dancer settings page control", (), |spec| {
        spec.it("keeps dancer list open by default", |_| {
            let state = dancers::state::DancersState::default();
            assert!(state.is_dancer_list_open);
        });

        spec.it("updates drawer visibility from reducer actions", |_| {
            let mut state = dancers::state::DancersState::default();
            let mut errors = Vec::new();

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::ToggleDancerList,
            );
            check_eq!(errors, state.is_dancer_list_open, false);

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::OpenDancerList);
            check_eq!(errors, state.is_dancer_list_open, true);

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::CloseDancerList);
            check_eq!(errors, state.is_dancer_list_open, false);

            assert_no_errors(errors);
        });

        spec.it("projects a left-only generic drawer host state with the drawer panel below the nav bar", |_| {
            let state = dancers::state::DancersState::default();
            let drawer_state = dancers::ui::drawer_host_state(
                &state,
                Color32::from_black_alpha(179),
                Color32::from_gray(240),
            );
            let mut errors = Vec::new();

            check_eq!(errors, drawer_state.left_drawer_width, 420.0);
            check_eq!(errors, drawer_state.responsive_breakpoint, 900.0);
            check_eq!(errors, drawer_state.open_mode, DrawerHostOpenMode::Modal);
            check_eq!(errors, drawer_state.top_inset, 64.0);
            check_eq!(errors, drawer_state.is_left_open, true);
            check_eq!(errors, drawer_state.is_right_open, false);
            check_eq!(errors, drawer_state.is_top_open, false);
            check_eq!(errors, drawer_state.is_bottom_open, false);

            assert_no_errors(errors);
        });

        spec.it(
            "exposes slint shell metrics for top bar content and footer",
            |_| {
                let mut errors = Vec::new();

                check_eq!(errors, dancers::ui::top_bar_height_token(), 64.0);
                check_eq!(errors, dancers::ui::content_max_width_token(), 720.0);
                check_eq!(errors, dancers::ui::content_outer_margin_token(), 16.0);
                check_eq!(errors, dancers::ui::footer_height_token(), 56.0);
                check_eq!(errors, dancers::ui::uses_scrollable_content_shell(), true);

                assert_no_errors(errors);
            },
        );

        spec.it(
            "anchors shell geometry to the current ui rect instead of the cursor-based available rect",
            |_| {
                let context = egui::Context::default();
                let scoped_rect =
                    egui::Rect::from_min_max(egui::pos2(120.0, 96.0), egui::pos2(1320.0, 780.0));
                let mut observed_shell = None;
                let mut observed_available = None;
                let mut errors = Vec::new();

                let _ = context.run(egui::RawInput::default(), |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        let _ =
                            ui.scope_builder(egui::UiBuilder::new().max_rect(scoped_rect), |ui| {
                                ui.allocate_space(egui::vec2(
                                    0.0,
                                    dancers::ui::top_bar_height_token(),
                                ));
                                observed_shell = Some(dancers::ui::shell_rect(ui));
                                observed_available = Some(ui.available_rect_before_wrap());
                            });
                    });
                });

                check_eq!(errors, observed_shell, Some(scoped_rect));
                check!(
                    errors,
                    observed_available.is_some_and(|rect| rect.top() > scoped_rect.top())
                );

                assert_no_errors(errors);
            },
        );

        spec.it(
            "positions the drawer host below the top bar and docks the footer to the bottom",
            |_| {
                let page_rect =
                    egui::Rect::from_min_max(egui::pos2(120.0, 96.0), egui::pos2(1320.0, 780.0));

                let top_bar = dancers::ui::top_bar_rect(page_rect);
                let content = dancers::ui::main_content_rect(page_rect);
                let footer = dancers::ui::footer_rect(content);
                let scroll = dancers::ui::scroll_rect(content);
                let mut errors = Vec::new();

                check_eq!(errors, top_bar.top(), 96.0);
                check_eq!(errors, top_bar.bottom(), 160.0);
                check_eq!(errors, content.top(), 160.0);
                check_eq!(errors, content.bottom(), 780.0);
                check_eq!(errors, footer.bottom(), 780.0);
                check_eq!(errors, footer.top(), 724.0);
                check_eq!(errors, scroll.left(), 136.0);
                check_eq!(errors, scroll.right(), 856.0);
                check_eq!(errors, scroll.top(), 176.0);
                check_eq!(errors, scroll.bottom(), 708.0);

                assert_no_errors(errors);
            },
        );
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
