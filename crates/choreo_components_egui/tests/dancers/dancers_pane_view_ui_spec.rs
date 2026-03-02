use crate::dancers;
use crate::dancers::Report;

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

                let supporting = dancers::ui::dancer_supporting_text(&dancer);
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
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
