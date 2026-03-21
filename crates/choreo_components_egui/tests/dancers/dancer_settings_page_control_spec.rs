use crate::dancers;
use crate::dancers::Report;
use choreo_components_egui::drawer_host::state::DrawerHostOpenMode;
use egui::Color32;

#[test]
fn dancer_settings_page_control_spec() {
    let suite = rspec::describe("dancer settings page control", (), |spec| {
        spec.it("keeps dancer list open by default", |_| {
            let state = dancers::state::DancersState::default();
            assert!(state.is_dancer_list_open);
        });

        spec.it("updates drawer visibility from reducer actions", |_| {
            let mut state = dancers::state::DancersState::default();

            dancers::reducer::reduce(
                &mut state,
                dancers::actions::DancersAction::ToggleDancerList,
            );
            assert!(!state.is_dancer_list_open);

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::OpenDancerList);
            assert!(state.is_dancer_list_open);

            dancers::reducer::reduce(&mut state, dancers::actions::DancersAction::CloseDancerList);
            assert!(!state.is_dancer_list_open);
        });

        spec.it("projects a left-only generic drawer host state with the drawer panel below the nav bar", |_| {
            let state = dancers::state::DancersState::default();
            let drawer_state = dancers::ui::drawer_host_state(
                &state,
                Color32::from_black_alpha(179),
                Color32::from_gray(240),
            );

            assert_eq!(drawer_state.left_drawer_width, 420.0);
            assert_eq!(drawer_state.responsive_breakpoint, 900.0);
            assert_eq!(drawer_state.open_mode, DrawerHostOpenMode::Modal);
            assert_eq!(drawer_state.top_inset, 64.0);
            assert!(drawer_state.is_left_open);
            assert!(!drawer_state.is_right_open);
            assert!(!drawer_state.is_top_open);
            assert!(!drawer_state.is_bottom_open);
        });

        spec.it(
            "exposes slint shell metrics for top bar content and footer",
            |_| {
                assert_eq!(dancers::ui::top_bar_height_token(), 64.0);
                assert_eq!(dancers::ui::content_max_width_token(), 720.0);
                assert_eq!(dancers::ui::content_outer_margin_token(), 16.0);
                assert_eq!(dancers::ui::footer_height_token(), 56.0);
                assert!(dancers::ui::uses_scrollable_content_shell());
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

                assert_eq!(observed_shell, Some(scoped_rect));
                assert!(
                    observed_available
                        .is_some_and(|rect| rect.top() > scoped_rect.top())
                );
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

                assert_eq!(top_bar.top(), 96.0);
                assert_eq!(top_bar.bottom(), 160.0);
                assert_eq!(content.top(), 160.0);
                assert_eq!(content.bottom(), 780.0);
                assert_eq!(footer.bottom(), 780.0);
                assert_eq!(footer.top(), 724.0);
                assert_eq!(scroll.left(), 136.0);
                assert_eq!(scroll.right(), 856.0);
                assert_eq!(scroll.top(), 176.0);
                assert_eq!(scroll.bottom(), 708.0);
            },
        );
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
