use egui::Color32;
use egui::Rect;
use egui::pos2;
use egui::vec2;

use super::ui::build_scene_search_bar_view_model;
use super::ui::scene_list_item_colors;
use super::ui::scene_list_item_layout;
use super::ui::scene_row_height_px;
use super::ui::scene_timestamp_role;
use super::ui::scene_title_role;
use super::ui::scene_toolbar_button_stroke_width_px;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::scenes::Report;

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

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
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
fn scene_item_view_parity_spec() {
    let suite = rspec::describe("scene item parity", (), |spec| {
        spec.it(
            "matches source row heights for timestamp visibility",
            |_| {
                let mut errors = Vec::new();

                check_eq!(errors, scene_row_height_px(false), 50.0);
                check_eq!(errors, scene_row_height_px(true), 62.0);

                assert_no_errors(errors);
            },
        );

        spec.it(
            "uses body and label typography roles from the source item view",
            |_| {
                let mut errors = Vec::new();

                check_eq!(errors, format!("{:?}", scene_title_role()), "BodyMedium");
                check_eq!(
                    errors,
                    format!("{:?}", scene_timestamp_role()),
                    "LabelMedium"
                );

                assert_no_errors(errors);
            },
        );

        spec.it("maps row geometry to the source offsets", |_| {
            let row_rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(240.0, 62.0));

            let layout = scene_list_item_layout(row_rect, true);

            let mut errors = Vec::new();

            check_eq!(errors, layout.content_rect.min.x, 0.0);
            check_eq!(errors, layout.content_rect.min.y, 4.0);
            check_eq!(errors, layout.content_rect.max.x, 240.0);
            check_eq!(errors, layout.content_rect.max.y, 58.0);
            check_eq!(errors, layout.accent_rect.width(), 4.0);
            check_eq!(errors, layout.swatch_rect.min.x, 8.0);
            check_eq!(errors, layout.swatch_rect.min.y, 12.0);
            check_eq!(errors, layout.swatch_rect.width(), 12.0);
            check_eq!(errors, layout.swatch_rect.height(), 12.0);
            check_eq!(errors, layout.title_position.x, 26.0);
            check_eq!(errors, layout.title_position.y, 12.0);
            check_eq!(errors, layout.timestamp_position.x, 26.0);
            check_eq!(errors, layout.timestamp_position.y, 34.0);

            assert_no_errors(errors);
        });

        spec.it(
            "maps selected state to source-aligned selection colors",
            |_| {
                let visuals = egui::Visuals::dark();
                let palette =
                    crate::material::styling::material_palette::MaterialPalette::from_visuals(
                        &visuals,
                    );

                let selected = scene_list_item_colors(&visuals, true);
                let unselected = scene_list_item_colors(&visuals, false);

                let mut errors = Vec::new();

                check_eq!(errors, selected.background, palette.surface_container_high);
                check_eq!(errors, selected.border, palette.secondary);
                check_eq!(errors, selected.title, palette.on_surface);
                check_eq!(errors, selected.timestamp, palette.on_surface_variant);
                check_eq!(errors, selected.accent, palette.secondary);
                check_eq!(errors, unselected.background, palette.surface_container_low);
                check_eq!(errors, unselected.border, palette.outline_variant);
                check_eq!(errors, unselected.title, palette.on_surface);
                check_eq!(errors, unselected.timestamp, palette.on_surface_variant);
                check_eq!(errors, selected.border_width, material_style_metrics().strokes.focus);
                check_eq!(
                    errors,
                    unselected.border_width,
                    material_style_metrics().strokes.outline
                );
                check_ne!(errors, selected.background, Color32::TRANSPARENT);

                assert_no_errors(errors);
            },
        );

        spec.it(
            "shows translated placeholder and clear affordance only when needed",
            |_| {
                let empty = build_scene_search_bar_view_model("", "en");
                let populated = build_scene_search_bar_view_model("Intro", "en");

                let mut errors = Vec::new();

                check_eq!(errors, empty.placeholder_text, "Search");
                check_eq!(errors, empty.clear_tooltip, "Cancel");
                check!(errors, !empty.show_clear_button);
                check!(errors, populated.show_clear_button);

                assert_no_errors(errors);
            },
        );

        spec.it(
            "keeps scene toolbar buttons borderless so hover uses fill instead of an outline ring",
            |_| {
                assert_eq!(scene_toolbar_button_stroke_width_px(), 0.0);
            },
        );
    });

    let report = crate::scenes::run_suite(&suite);
    assert!(report.is_success());
}
