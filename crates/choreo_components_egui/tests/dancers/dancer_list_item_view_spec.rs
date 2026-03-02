use super::dancer;
use super::role;
use super::ui::dancer_role_details_text;

#[test]
fn dancer_role_details_text_matches_slint_format() {
    let mut lead_role = role("Lead");
    lead_role.z_index = 2;
    let dancer = dancer(3, lead_role, "Alice", "A", Some("IconCircle"));

    let details = dancer_role_details_text(&dancer);

    assert_eq!(details, "Lead (2)  [A]");
}
