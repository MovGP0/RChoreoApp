use crate::settings::Report;
use crate::settings::actions::SettingsAction;
use crate::settings::reducer::reduce;
use crate::settings::state::SettingsState;

#[test]
fn material_theme_application_spec() {
    let suite = rspec::describe("material theme reducer integration", (), |spec| {
        spec.it("bumps material update counter when theme inputs change", |_| {
            let mut state = SettingsState::default();
            let baseline = state.material_update_count;
            reduce(
                &mut state,
                SettingsAction::UpdateUseSystemTheme { enabled: false },
            );
            reduce(
                &mut state,
                SettingsAction::UpdateIsDarkMode { enabled: true },
            );

            assert!(state.material_update_count >= baseline + 2);
        });
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
