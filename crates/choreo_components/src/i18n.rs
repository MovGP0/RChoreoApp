use choreo_i18n::translation_with_fallback;
use slint::ComponentHandle;
use slint::SharedString;

use crate::{ShellHost, Translations};

pub fn apply_translations(view: &ShellHost, locale: &str)
{
    let translations = view.global::<Translations<'_>>();

    translations.set_settings_title(t(locale, "SettingsTitle"));
    translations.set_dark_mode_label(t(locale, "DarkModeLabel"));
    translations.set_app_title(t(locale, "AppTitle"));
    translations.set_choreography_author_placeholder(t(locale, "ChoreographyAuthorPlaceholder"));
    translations.set_choreography_comment_placeholder(t(locale, "ChoreographyCommentPlaceholder"));
    translations.set_choreography_description_placeholder(t(locale, "ChoreographyDescriptionPlaceholder"));
    translations.set_choreography_display_title(t(locale, "ChoreographyDisplayTitle"));
    translations.set_choreography_draw_path_from_label(t(locale, "ChoreographyDrawPathFromLabel"));
    translations.set_choreography_draw_path_to_label(t(locale, "ChoreographyDrawPathToLabel"));
    translations.set_choreography_floor_back_label(t(locale, "ChoreographyFloorBackLabel"));
    translations.set_choreography_floor_color_label(t(locale, "ChoreographyFloorColorLabel"));
    translations.set_choreography_floor_front_label(t(locale, "ChoreographyFloorFrontLabel"));
    translations.set_choreography_floor_left_label(t(locale, "ChoreographyFloorLeftLabel"));
    translations.set_choreography_floor_right_label(t(locale, "ChoreographyFloorRightLabel"));
    translations.set_choreography_floor_title(t(locale, "ChoreographyFloorTitle"));
    translations.set_choreography_grid_lines_label(t(locale, "ChoreographyGridLinesLabel"));
    translations.set_choreography_snap_to_grid_label(t(locale, "ChoreographySnapToGridLabel"));
    translations.set_choreography_grid_size_label(t(locale, "ChoreographyGridSizeLabel"));
    translations.set_choreography_name_placeholder(t(locale, "ChoreographyNamePlaceholder"));
    translations.set_choreography_positions_at_side_label(t(locale, "ChoreographyPositionsAtSideLabel"));
    translations.set_choreography_section_title(t(locale, "ChoreographySectionTitle"));
    translations.set_choreography_show_timestamps_label(t(locale, "ChoreographyShowTimestampsLabel"));
    translations.set_choreography_show_legend_label(t(locale, "ChoreographyShowLegendLabel"));
    translations.set_choreography_subtitle_placeholder(t(locale, "ChoreographySubtitlePlaceholder"));
    translations.set_choreography_transparency_label(t(locale, "ChoreographyTransparencyLabel"));
    translations.set_delete_scene_dialog_title(t(locale, "DeleteSceneDialogTitle"));
    translations.set_delete_scene_dialog_message(t(locale, "DeleteSceneDialogMessage"));
    translations.set_delete_scene_dialog_default_name(t(locale, "DeleteSceneDialogDefaultName"));
    translations.set_delete_scene_dialog_yes(t(locale, "DeleteSceneDialogYes"));
    translations.set_delete_scene_dialog_no(t(locale, "DeleteSceneDialogNo"));
    translations.set_copy_scene_positions_dialog_title(t(locale, "CopyScenePositionsDialogTitle"));
    translations.set_copy_scene_positions_dialog_message(t(locale, "CopyScenePositionsDialogMessage"));
    translations.set_copy_scene_positions_dialog_confirm(t(locale, "CopyScenePositionsDialogConfirm"));
    translations.set_copy_scene_positions_dialog_cancel(t(locale, "CopyScenePositionsDialogCancel"));
    translations.set_choreography_variation_placeholder(t(locale, "ChoreographyVariationPlaceholder"));
    translations.set_scene_section_title(t(locale, "SceneSectionTitle"));
    translations.set_scene_name_label(t(locale, "SceneNameLabel"));
    translations.set_scene_text_label(t(locale, "SceneTextLabel"));
    translations.set_scene_fixed_positions_label(t(locale, "SceneFixedPositionsLabel"));
    translations.set_scene_timestamp_label(t(locale, "SceneTimestampLabel"));
    translations.set_scene_color_label(t(locale, "SceneColorLabel"));
    translations.set_mode_label(t(locale, "ModeLabel"));
    translations.set_mode_view(t(locale, "ModeView"));
    translations.set_mode_move(t(locale, "ModeMove"));
    translations.set_mode_rotate_around_center(t(locale, "ModeRotateAroundCenter"));
    translations.set_mode_rotate_around_dancer(t(locale, "ModeRotateAroundDancer"));
    translations.set_mode_scale(t(locale, "ModeScale"));
    translations.set_mode_line_of_sight(t(locale, "ModeLineOfSight"));
    translations.set_search_placeholder(t(locale, "SearchPlaceholder"));
    translations.set_scene_timestamp_minutes_label(t(locale, "SceneTimestampMinutesLabel"));
    translations.set_scene_timestamp_seconds_label(t(locale, "SceneTimestampSecondsLabel"));
    translations.set_scene_timestamp_milliseconds_label(t(locale, "SceneTimestampMillisecondsLabel"));
    translations.set_common_cancel(t(locale, "CommonCancel"));
    translations.set_common_ok(t(locale, "CommonOk"));
    translations.set_dancer_color_label(t(locale, "DancerColorLabel"));
    translations.set_dancer_icon_label(t(locale, "DancerIconLabel"));
    translations.set_dancer_name_label(t(locale, "DancerNameLabel"));
    translations.set_dancer_name_placeholder(t(locale, "DancerNamePlaceholder"));
    translations.set_dancer_preview_label(t(locale, "DancerPreviewLabel"));
    translations.set_dancer_role_label(t(locale, "DancerRoleLabel"));
    translations.set_dancer_shortcut_label(t(locale, "DancerShortcutLabel"));
    translations.set_dancer_shortcut_placeholder(t(locale, "DancerShortcutPlaceholder"));
    translations.set_dancer_title(t(locale, "DancerTitle"));
    translations.set_dancer_swap_section_title(t(locale, "DancerSwapSectionTitle"));
    translations.set_dancer_swap_from_label(t(locale, "DancerSwapFromLabel"));
    translations.set_dancer_swap_to_label(t(locale, "DancerSwapToLabel"));
    translations.set_dancer_swap_button(t(locale, "DancerSwapButton"));
    translations.set_dancer_swap_dialog_title(t(locale, "DancerSwapDialogTitle"));
    translations.set_dancer_swap_dialog_message(t(locale, "DancerSwapDialogMessage"));
    translations.set_dancer_swap_dialog_confirm(t(locale, "DancerSwapDialogConfirm"));
    translations.set_dancer_swap_dialog_cancel(t(locale, "DancerSwapDialogCancel"));
    translations.set_dancers_title(t(locale, "DancersTitle"));
    translations.set_color_amber(t(locale, "ColorAmber"));
    translations.set_color_blue(t(locale, "ColorBlue"));
    translations.set_color_blue_grey(t(locale, "ColorBlueGrey"));
    translations.set_color_brown(t(locale, "ColorBrown"));
    translations.set_color_cyan(t(locale, "ColorCyan"));
    translations.set_color_deep_orange(t(locale, "ColorDeepOrange"));
    translations.set_color_deep_purple(t(locale, "ColorDeepPurple"));
    translations.set_color_gray(t(locale, "ColorGray"));
    translations.set_color_green(t(locale, "ColorGreen"));
    translations.set_color_indigo(t(locale, "ColorIndigo"));
    translations.set_color_light_green(t(locale, "ColorLightGreen"));
    translations.set_color_lime(t(locale, "ColorLime"));
    translations.set_color_orange(t(locale, "ColorOrange"));
    translations.set_color_pink(t(locale, "ColorPink"));
    translations.set_color_purple(t(locale, "ColorPurple"));
    translations.set_color_red(t(locale, "ColorRed"));
    translations.set_color_teal(t(locale, "ColorTeal"));
    translations.set_color_yellow(t(locale, "ColorYellow"));
}

fn t(locale: &str, key: &str) -> SharedString
{
    translation_with_fallback(locale, key)
        .unwrap_or(key)
        .into()
}
