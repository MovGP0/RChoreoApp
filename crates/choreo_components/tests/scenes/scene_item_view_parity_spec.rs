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
use crate::scenes::Report;

#[test]
fn scene_item_view_parity_spec() {
    let suite = rspec::describe("scene item parity", (), |spec| {
        spec.it(
            "matches source row heights for timestamp visibility",
            |_| {
                assert_eq!(scene_row_height_px(false), 50.0);
                assert_eq!(scene_row_height_px(true), 62.0);
            },
        );

        spec.it(
            "uses body and label typography roles from the source item view",
            |_| {
                assert_eq!(format!("{:?}", scene_title_role()), "BodyMedium");
                assert_eq!(format!("{:?}", scene_timestamp_role()), "LabelMedium");
            },
        );

        spec.it("maps row geometry to the source offsets", |_| {
            let row_rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(240.0, 62.0));

            let layout = scene_list_item_layout(row_rect, true);

            assert_eq!(layout.content_rect.min.x, 0.0);
            assert_eq!(layout.content_rect.min.y, 4.0);
            assert_eq!(layout.content_rect.max.x, 240.0);
            assert_eq!(layout.content_rect.max.y, 58.0);
            assert_eq!(layout.accent_rect.width(), 4.0);
            assert_eq!(layout.swatch_rect.min.x, 8.0);
            assert_eq!(layout.swatch_rect.min.y, 12.0);
            assert_eq!(layout.swatch_rect.width(), 12.0);
            assert_eq!(layout.swatch_rect.height(), 12.0);
            assert_eq!(layout.title_position.x, 26.0);
            assert_eq!(layout.title_position.y, 12.0);
            assert_eq!(layout.timestamp_position.x, 26.0);
            assert_eq!(layout.timestamp_position.y, 34.0);
        });

        spec.it(
            "maps selected state to source-aligned selection colors",
            |_| {
                let visuals = egui::Visuals::dark();
                let palette =
                    crate::material::styling::material_palette::MaterialPalette::from_visuals(
                        &visuals,
                    );

                let (
                    selected_background,
                    selected_border,
                    selected_title,
                    selected_timestamp,
                    selected_accent,
                    selected_border_width,
                ) = scene_list_item_colors(&visuals, true);
                let (
                    unselected_background,
                    unselected_border,
                    unselected_title,
                    unselected_timestamp,
                    _unselected_accent,
                    unselected_border_width,
                ) = scene_list_item_colors(&visuals, false);

                assert_eq!(selected_background, palette.secondary_container);
                assert_eq!(selected_border, palette.on_secondary_container);
                assert_eq!(selected_title, palette.on_secondary_container);
                assert_eq!(selected_timestamp, palette.on_secondary_container);
                assert_eq!(selected_accent, palette.on_secondary_container);
                assert_eq!(unselected_background, palette.surface_container_highest);
                assert_eq!(unselected_border, palette.outline_variant);
                assert_eq!(unselected_title, palette.on_surface);
                assert_eq!(unselected_timestamp, palette.on_surface_variant);
                assert_eq!(selected_border_width, 3.0);
                assert_eq!(unselected_border_width, 1.0);
                assert_ne!(selected_background, Color32::TRANSPARENT);
            },
        );

        spec.it(
            "shows translated placeholder and clear affordance only when needed",
            |_| {
                let empty = build_scene_search_bar_view_model("", "en");
                let populated = build_scene_search_bar_view_model("Intro", "en");

                assert_eq!(empty.placeholder_text, "Search");
                assert_eq!(empty.clear_tooltip, "Cancel");
                assert!(!empty.show_clear_button);
                assert!(populated.show_clear_button);
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
