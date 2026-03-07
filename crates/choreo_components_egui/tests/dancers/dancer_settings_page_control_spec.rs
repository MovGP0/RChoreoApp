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

        spec.it("projects a left-only generic drawer host state", |_| {
            let state = dancers::state::DancersState::default();
            let drawer_state = dancers::ui::drawer_host_state(
                &state,
                Color32::from_black_alpha(179),
                Color32::from_gray(240),
            );

            assert_eq!(drawer_state.left_drawer_width, 420.0);
            assert_eq!(drawer_state.responsive_breakpoint, 900.0);
            assert_eq!(drawer_state.open_mode, DrawerHostOpenMode::Modal);
            assert_eq!(drawer_state.top_inset, 0.0);
            assert!(drawer_state.is_left_open);
            assert!(!drawer_state.is_right_open);
            assert!(!drawer_state.is_top_open);
            assert!(!drawer_state.is_bottom_open);
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
