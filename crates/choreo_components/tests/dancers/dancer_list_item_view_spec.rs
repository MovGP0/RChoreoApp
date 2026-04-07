use egui::Color32;
use egui::Rect;
use egui::pos2;
use egui::vec2;

use crate::dancers;
use crate::dancers::Report;
use choreo_components::material::styling::material_typography::TypographyRole;

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

macro_rules! check_ne {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left == $right {
            $errors.push(format!(
                "{} == {} (left = {:?}, right = {:?})",
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
            let mut errors = Vec::new();

            check_eq!(
                errors,
                dancers::dancer_list_item_view::title_role(),
                TypographyRole::BodyMedium
            );
            check_eq!(
                errors,
                dancers::dancer_list_item_view::subtitle_role(),
                TypographyRole::BodySmall
            );

            assert_no_errors(errors);
        });

        spec.it("computes row geometry using slint offsets", |_| {
            let row_rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(240.0, 56.0));

            let layout = dancers::dancer_list_item_view::layout_for_row_rect(row_rect);

            let mut errors = Vec::new();

            check_eq!(errors, layout.content_rect.min.x, 0.0);
            check_eq!(errors, layout.content_rect.min.y, 3.0);
            check_eq!(errors, layout.content_rect.max.x, 240.0);
            check_eq!(errors, layout.content_rect.max.y, 53.0);
            check_eq!(errors, layout.swatch_rect.min.x, 10.0);
            check_eq!(errors, layout.swatch_rect.min.y, 14.0);
            check_eq!(errors, layout.swatch_rect.width(), 28.0);
            check_eq!(errors, layout.swatch_rect.height(), 28.0);
            check_eq!(errors, layout.title_position.x, 46.0);
            check_eq!(errors, layout.title_position.y, 11.0);
            check_eq!(errors, layout.subtitle_position.x, 46.0);
            check_eq!(errors, layout.subtitle_position.y, 31.0);

            assert_no_errors(errors);
        });

        spec.it("maps selected state to selection colors", |_| {
            let visuals = egui::Visuals::dark();

            let selected = dancers::dancer_list_item_view::colors_for_selection(&visuals, true);
            let unselected = dancers::dancer_list_item_view::colors_for_selection(&visuals, false);

            let mut errors = Vec::new();

            check_eq!(errors, selected.background, visuals.selection.bg_fill);
            check_eq!(errors, selected.border, visuals.selection.stroke.color);
            check_eq!(errors, selected.title, visuals.strong_text_color());
            check_eq!(errors, selected.subtitle, visuals.weak_text_color());
            check_eq!(errors, unselected.background, visuals.extreme_bg_color);
            check_eq!(
                errors,
                unselected.border,
                visuals.widgets.noninteractive.bg_stroke.color
            );
            check_eq!(errors, unselected.title, visuals.text_color());
            check_eq!(errors, unselected.subtitle, visuals.weak_text_color());
            check_ne!(errors, selected.background, Color32::TRANSPARENT);

            assert_no_errors(errors);
        });
    });

    let report = dancers::run_suite(&suite);
    assert!(report.is_success());
}
