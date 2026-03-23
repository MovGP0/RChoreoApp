use crate::dancers;
use crate::dancers::Report;
use choreo_components::material::icons::UiIconKey;
use choreo_components::material::styling::material_typography::TypographyRole;

#[test]
fn dancers_pane_view_ui_spec() {
    let suite = rspec::describe("dancers pane view ui", (), |spec| {
        spec.it(
            "formats supporting text like the slint row subtitle",
            |_| {
                let lead = dancers::state::RoleState {
                    name: "Lead".to_string(),
                    color: dancers::state::transparent_color(),
                    z_index: 2,
                };
                let dancer = dancers::dancer(1, lead, "Alice", "A", None);

                let supporting = dancers::dancer_list_item_view::supporting_text(&dancer);
                assert_eq!(supporting, "Lead (2)  [A]");
            },
        );

        spec.it("maps selected dancer id to selected row index", |_| {
            let lead = dancers::role("Lead");
            let follow = dancers::role("Follow");
            let state = dancers::state::DancersState {
                dancers: vec![
                    dancers::dancer(3, lead, "Alice", "A", None),
                    dancers::dancer(7, follow, "Bob", "B", None),
                ],
                selected_dancer: Some(dancers::dancer(
                    7,
                    dancers::role("Follow"),
                    "Bob",
                    "B",
                    None,
                )),
                ..dancers::state::DancersState::default()
            };

            let selected_index = dancers::ui::selected_dancer_index(&state);
            assert_eq!(selected_index, Some(1));
        });

        spec.it("uses title-medium typography for the pane title", |_| {
            assert_eq!(
                dancers::dancers_pane_view::ui::title_role(),
                TypographyRole::TitleMedium
            );
        });

        spec.it(
            "uses plus for add and minus for remove button icons",
            |_| {
                assert_eq!(
                    dancers::dancers_pane_view::ui::add_button_icon_key(),
                    UiIconKey::DancersAdd
                );
                assert_eq!(
                    dancers::dancers_pane_view::ui::remove_button_icon_key(),
                    UiIconKey::DancersRemove
                );
            },
        );

        spec.it(
            "reserves footer space so the add and delete buttons stay visible",
            |_| {
                assert_eq!(
                    dancers::dancers_pane_view::ui::pane_button_row_height_token(),
                    48.0
                );

                let list_height = dancers::dancers_pane_view::ui::pane_list_height(360.0);
                assert_eq!(list_height, 300.0);
            },
        );

        spec.it("maps pane select action to dancers select action", |_| {
            let action = dancers::ui::map_pane_action(
                dancers::dancers_pane_view::ui::DancersPaneViewAction::SelectDancer { index: 3 },
            );
            assert_eq!(
                action,
                dancers::actions::DancersAction::SelectDancer { index: 3 }
            );
        });

        spec.it("maps pane add action to dancers add action", |_| {
            let action = dancers::ui::map_pane_action(
                dancers::dancers_pane_view::ui::DancersPaneViewAction::AddDancer,
            );
            assert_eq!(action, dancers::actions::DancersAction::AddDancer);
        });

        spec.it("maps pane delete action to dancers delete action", |_| {
            let action = dancers::ui::map_pane_action(
                dancers::dancers_pane_view::ui::DancersPaneViewAction::DeleteDancer,
            );
            assert_eq!(
                action,
                dancers::actions::DancersAction::DeleteSelectedDancer
            );
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
