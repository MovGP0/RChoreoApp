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

        macro_rules! assert_no_errors {
            ($errors:expr) => {
                assert!($errors.is_empty(), "{:?}", $errors);
            };
        }

        spec.it(
            "enforces primary-secondary-tertiary enablement hierarchy",
            |_| {
                let mut state = SettingsState::default();
                let mut errors = Vec::new();

                reduce(
                    &mut state,
                    SettingsAction::UpdateUseSecondaryColor { enabled: true },
                );
                check!(&mut errors, !state.use_secondary_color);

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
                check!(&mut errors, state.use_primary_color);
                check!(&mut errors, state.use_secondary_color);
                check!(&mut errors, state.use_tertiary_color);

                reduce(
                    &mut state,
                    SettingsAction::UpdateUsePrimaryColor { enabled: false },
                );
                check!(&mut errors, !state.use_primary_color);
                check!(&mut errors, !state.use_secondary_color);
                check!(&mut errors, !state.use_tertiary_color);
                check!(&mut errors, !state.preferences.contains_key(PRIMARY_COLOR_KEY));
                check!(&mut errors, !state.preferences.contains_key(SECONDARY_COLOR_KEY));
                check!(&mut errors, !state.preferences.contains_key(TERTIARY_COLOR_KEY));

                assert_no_errors!(&errors);
            },
        );

        spec.it(
            "stores color hex only when color channel is enabled",
            |_| {
                let mut state = SettingsState::default();
                let mut errors = Vec::new();

                reduce(
                    &mut state,
                    SettingsAction::UpdatePrimaryColorHex {
                        value: "#FF336699".to_string(),
                    },
                );
                check!(
                    &mut errors,
                    !state.preferences.contains_key(PRIMARY_COLOR_KEY)
                );

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
                check_eq!(
                    &mut errors,
                    state.preferences.get(PRIMARY_COLOR_KEY).map(String::as_str),
                    Some("#FF336699")
                );
                check_eq!(&mut errors, state.primary_color_hex, "#FF336699");

                assert_no_errors!(&errors);
            },
        );
    });

    let report = crate::settings::run_suite(&suite);
    assert!(report.is_success());
}
