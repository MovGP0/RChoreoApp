use crate::dancers;
use crate::dancers::Report;
use choreo_components::dancer_settings_page::action::DancerSettingsPageAction;
use choreo_components::dancer_settings_page::reducer::map_action;

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
fn dancer_settings_page_action_mapping_spec() {
    let suite = rspec::describe("dancer settings page action mapping", (), |spec| {
        spec.it(
            "maps page-local shell and dialog actions into dancers actions",
            |_| {
                let mut errors = Vec::new();

                check_eq!(
                    errors,
                    map_action(DancerSettingsPageAction::ToggleDancerList),
                    vec![dancers::actions::DancersAction::ToggleDancerList]
                );
                check_eq!(
                    errors,
                    map_action(DancerSettingsPageAction::DismissDialog),
                    vec![dancers::actions::DancersAction::HideDialog]
                );
                check_eq!(
                    errors,
                    map_action(DancerSettingsPageAction::ConfirmSwapDialog),
                    vec![dancers::actions::DancersAction::ConfirmSwapDancers]
                );
                check_eq!(
                    errors,
                    map_action(DancerSettingsPageAction::CancelPage),
                    vec![dancers::actions::DancersAction::Cancel]
                );
                check_eq!(
                    errors,
                    map_action(DancerSettingsPageAction::SavePage),
                    vec![dancers::actions::DancersAction::SaveToGlobal]
                );

                assert_no_errors(errors);
            },
        );

        spec.it(
            "maps page-local form actions into dancers reducer actions",
            |_| {
                let mut errors = Vec::new();

                check_eq!(
                    errors,
                    map_action(DancerSettingsPageAction::SelectDancer { index: 2 }),
                    vec![dancers::actions::DancersAction::SelectDancer { index: 2 }]
                );
                check_eq!(
                    errors,
                    map_action(DancerSettingsPageAction::UpdateDancerName {
                        value: "Alex".to_string(),
                    }),
                    vec![dancers::actions::DancersAction::UpdateDancerName {
                        value: "Alex".to_string(),
                    }]
                );
                check_eq!(
                    errors,
                    map_action(DancerSettingsPageAction::RequestSwapDancers),
                    vec![dancers::actions::DancersAction::RequestSwapDancers]
                );

                assert_no_errors(errors);
            },
        );
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
