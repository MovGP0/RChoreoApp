use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;

#[test]
fn open_image_behavior_spec() {
    let suite = rspec::describe("open image reducer behavior", (), |spec| {
        spec.it("maps image request into open-svg command", |_| {
            let mut state = ChoreoMainState::default();
            reduce(
                &mut state,
                ChoreoMainAction::RequestOpenImage {
                    file_path: "C:/image.svg".to_string(),
                },
            );

            assert_eq!(state.outgoing_open_svg_commands.len(), 1);
            assert_eq!(state.outgoing_open_svg_commands[0].file_path, "C:/image.svg");
        });
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}
