use crate::shell::Report;
use crate::shell::actions::ShellAction;
use crate::shell::reducer::reduce;
use crate::shell::state::ShellState;

#[test]
fn material_scheme_applier_spec() {
    let suite = rspec::describe("shell material scheme application", (), |spec| {
        spec.it("applies dark and light scheme backgrounds based on theme", |_| {
            let mut state = ShellState::default();
            reduce(
                &mut state,
                ShellAction::ApplyMaterialSchemes {
                    light_background_hex: "#FFEEEEEE".to_string(),
                    dark_background_hex: "#FF222222".to_string(),
                },
            );
            assert_eq!(state.active_background_hex, "#FFEEEEEE");

            reduce(&mut state, ShellAction::SetThemeMode { is_dark: true });
            assert_eq!(state.active_background_hex, "#FF222222");
        });
    });

    let report = crate::shell::run_suite(&suite);
    assert!(report.is_success());
}
