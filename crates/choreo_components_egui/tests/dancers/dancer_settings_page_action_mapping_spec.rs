use crate::dancers;
use crate::dancers::Report;
use choreo_components_egui::dancer_settings_page::action::DancerSettingsPageAction;
use choreo_components_egui::dancer_settings_page::reducer::map_action;

#[test]
fn dancer_settings_page_action_mapping_spec() {
    let suite = rspec::describe("dancer settings page action mapping", (), |spec| {
        spec.it(
            "maps page-local shell and dialog actions into dancers actions",
            |_| {
                assert_eq!(
                    map_action(DancerSettingsPageAction::ToggleDancerList),
                    vec![dancers::actions::DancersAction::ToggleDancerList]
                );
                assert_eq!(
                    map_action(DancerSettingsPageAction::DismissDialog),
                    vec![dancers::actions::DancersAction::HideDialog]
                );
                assert_eq!(
                    map_action(DancerSettingsPageAction::ConfirmSwapDialog),
                    vec![dancers::actions::DancersAction::ConfirmSwapDancers]
                );
                assert_eq!(
                    map_action(DancerSettingsPageAction::CancelPage),
                    vec![dancers::actions::DancersAction::Cancel]
                );
                assert_eq!(
                    map_action(DancerSettingsPageAction::SavePage),
                    vec![dancers::actions::DancersAction::SaveToGlobal]
                );
            },
        );

        spec.it(
            "maps page-local form actions into dancers reducer actions",
            |_| {
                assert_eq!(
                    map_action(DancerSettingsPageAction::SelectDancer { index: 2 }),
                    vec![dancers::actions::DancersAction::SelectDancer { index: 2 }]
                );
                assert_eq!(
                    map_action(DancerSettingsPageAction::UpdateDancerName {
                        value: "Alex".to_string(),
                    }),
                    vec![dancers::actions::DancersAction::UpdateDancerName {
                        value: "Alex".to_string(),
                    }]
                );
                assert_eq!(
                    map_action(DancerSettingsPageAction::RequestSwapDancers),
                    vec![dancers::actions::DancersAction::RequestSwapDancers]
                );
            },
        );
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
