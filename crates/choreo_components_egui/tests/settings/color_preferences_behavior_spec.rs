use crate::settings::Report;
use crate::settings::actions::SettingsAction;
use crate::settings::reducer::reduce;
use crate::settings::state::PRIMARY_COLOR_KEY;
use crate::settings::state::SECONDARY_COLOR_KEY;
use crate::settings::state::SettingsState;
use crate::settings::state::TERTIARY_COLOR_KEY;

#[test]
fn color_preferences_behavior_spec() {
    let suite = rspec::describe("color preferences reducer behavior", (), |spec| {
        spec.it("enforces primary-secondary-tertiary enablement hierarchy", |_| {
            let mut state = SettingsState::default();
            reduce(
                &mut state,
                SettingsAction::UpdateUseSecondaryColor { enabled: true },
            );
            assert!(!state.use_secondary_color);

            reduce(
                &mut state,
                SettingsAction::UpdateUsePrimaryColor { enabled: true },
            );
            reduce(
                &mut state,
                SettingsAction::UpdateUseSecondaryColor { enabled: true },
            );
            reduce(
                &mut state,
                SettingsAction::UpdateUseTertiaryColor { enabled: true },
            );
            assert!(state.use_primary_color);
            assert!(state.use_secondary_color);
            assert!(state.use_tertiary_color);

            reduce(
                &mut state,
                SettingsAction::UpdateUsePrimaryColor { enabled: false },
            );
            assert!(!state.use_primary_color);
            assert!(!state.use_secondary_color);
            assert!(!state.use_tertiary_color);
            assert!(!state.preferences.contains_key(PRIMARY_COLOR_KEY));
            assert!(!state.preferences.contains_key(SECONDARY_COLOR_KEY));
            assert!(!state.preferences.contains_key(TERTIARY_COLOR_KEY));
        });

        spec.it("stores color hex only when color channel is enabled", |_| {
            let mut state = SettingsState::default();
            reduce(
                &mut state,
                SettingsAction::UpdatePrimaryColorHex {
                    value: "#FF336699".to_string(),
                },
            );
            assert!(!state.preferences.contains_key(PRIMARY_COLOR_KEY));

            reduce(
                &mut state,
                SettingsAction::UpdateUsePrimaryColor { enabled: true },
            );
            reduce(
                &mut state,
                SettingsAction::UpdatePrimaryColorHex {
                    value: "#FF336699".to_string(),
                },
            );
            reduce(
                &mut state,
                SettingsAction::UpdateSecondaryColorHex {
                    value: "#FF223344".to_string(),
                },
            );
            reduce(
                &mut state,
                SettingsAction::UpdateTertiaryColorHex {
                    value: "#FF112233".to_string(),
                },
            );
            assert_eq!(
                state.preferences.get(PRIMARY_COLOR_KEY).map(String::as_str),
                Some("#FF336699")
            );
            assert_eq!(state.primary_color_hex, "#FF336699");
        });
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
