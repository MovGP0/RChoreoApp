use egui::Color32;
use egui::Rect;
use egui::pos2;
use egui::vec2;

use crate::dancers;
use crate::dancers::Report;
use choreo_components_egui::material::styling::material_typography::TypographyRole;

#[test]
fn dancer_list_item_view_spec() {
    let suite = rspec::describe("dancer list item view", (), |spec| {
        spec.it("formats subtitle text to match slint parity", |_| {
            let mut lead_role = dancers::role("Lead");
            lead_role.z_index = 2;
            let dancer = dancers::dancer(3, lead_role, "Alice", "A", Some("IconCircle"));

            let details = dancers::dancer_list_item_view::role_details_text(&dancer);

            assert_eq!(details, "Lead (2)  [A]");
        });

        spec.it("uses body-medium typography for the primary label", |_| {
            assert_eq!(
                dancers::dancer_list_item_view::title_role(),
                TypographyRole::BodyMedium
            );
            assert_eq!(
                dancers::dancer_list_item_view::subtitle_role(),
                TypographyRole::BodySmall
            );
        });

        spec.it("computes row geometry using slint offsets", |_| {
            let row_rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(240.0, 56.0));

            let layout = dancers::dancer_list_item_view::layout_for_row_rect(row_rect);

            assert_eq!(layout.content_rect.min.x, 0.0);
            assert_eq!(layout.content_rect.min.y, 3.0);
            assert_eq!(layout.content_rect.max.x, 240.0);
            assert_eq!(layout.content_rect.max.y, 53.0);
            assert_eq!(layout.swatch_rect.min.x, 10.0);
            assert_eq!(layout.swatch_rect.min.y, 14.0);
            assert_eq!(layout.swatch_rect.width(), 28.0);
            assert_eq!(layout.swatch_rect.height(), 28.0);
            assert_eq!(layout.title_position.x, 46.0);
            assert_eq!(layout.title_position.y, 11.0);
            assert_eq!(layout.subtitle_position.x, 46.0);
            assert_eq!(layout.subtitle_position.y, 31.0);
        });

        spec.it("maps selected state to selection colors", |_| {
            let visuals = egui::Visuals::dark();

            let selected = dancers::dancer_list_item_view::colors_for_selection(&visuals, true);
            let unselected = dancers::dancer_list_item_view::colors_for_selection(&visuals, false);

            assert_eq!(selected.background, visuals.selection.bg_fill);
            assert_eq!(selected.border, visuals.selection.stroke.color);
            assert_eq!(selected.title, visuals.strong_text_color());
            assert_eq!(selected.subtitle, visuals.weak_text_color());
            assert_eq!(unselected.background, visuals.extreme_bg_color);
            assert_eq!(
                unselected.border,
                visuals.widgets.noninteractive.bg_stroke.color
            );
            assert_eq!(unselected.title, visuals.text_color());
            assert_eq!(unselected.subtitle, visuals.weak_text_color());
            assert_ne!(selected.background, Color32::TRANSPARENT);
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
