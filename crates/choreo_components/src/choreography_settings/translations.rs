use crate::i18n;

pub struct ChoreographySettingsTranslations;

impl ChoreographySettingsTranslations {
    pub fn comment(locale: &str) -> String {
        i18n::t(locale, "ChoreographyCommentPlaceholder")
    }

    pub fn name(locale: &str) -> String {
        i18n::t(locale, "ChoreographyNamePlaceholder")
    }

    pub fn subtitle(locale: &str) -> String {
        i18n::t(locale, "ChoreographySubtitlePlaceholder")
    }

    pub fn date(locale: &str) -> String {
        i18n::t(locale, "ChoreographyDateLabel")
    }

    pub fn variation(locale: &str) -> String {
        i18n::t(locale, "ChoreographyVariationPlaceholder")
    }

    pub fn author(locale: &str) -> String {
        i18n::t(locale, "ChoreographyAuthorPlaceholder")
    }

    pub fn description(locale: &str) -> String {
        i18n::t(locale, "ChoreographyDescriptionPlaceholder")
    }

    pub fn title(locale: &str) -> String {
        i18n::t(locale, "ChoreographyDisplayTitle")
    }

    pub fn choreography(locale: &str) -> String {
        i18n::t(locale, "ChoreographySectionTitle")
    }

    pub fn selected_scene(locale: &str) -> String {
        i18n::t(locale, "SceneSectionTitle")
    }

    pub fn floor(locale: &str) -> String {
        i18n::t(locale, "ChoreographyFloorTitle")
    }

    pub fn display(locale: &str) -> String {
        i18n::t(locale, "ChoreographyDisplayTitle")
    }

    pub fn scene_name(locale: &str) -> String {
        i18n::t(locale, "SceneNameLabel")
    }

    pub fn scene_text(locale: &str) -> String {
        i18n::t(locale, "SceneTextLabel")
    }

    pub fn floor_front(locale: &str) -> String {
        i18n::t(locale, "ChoreographyFloorFrontLabel")
    }

    pub fn floor_back(locale: &str) -> String {
        i18n::t(locale, "ChoreographyFloorBackLabel")
    }

    pub fn floor_left(locale: &str) -> String {
        i18n::t(locale, "ChoreographyFloorLeftLabel")
    }

    pub fn floor_right(locale: &str) -> String {
        i18n::t(locale, "ChoreographyFloorRightLabel")
    }

    pub fn floor_color(locale: &str) -> String {
        i18n::t(locale, "ChoreographyFloorColorLabel")
    }

    pub fn grid_resolution(locale: &str) -> String {
        i18n::t(locale, "ChoreographyGridSizeLabel")
    }

    pub fn grid_lines(locale: &str) -> String {
        i18n::t(locale, "ChoreographyGridLinesLabel")
    }

    pub fn show_legend(locale: &str) -> String {
        i18n::t(locale, "ChoreographyShowLegendLabel")
    }

    pub fn snap_to_grid(locale: &str) -> String {
        i18n::t(locale, "ChoreographySnapToGridLabel")
    }

    pub fn show_timestamps(locale: &str) -> String {
        i18n::t(locale, "ChoreographyShowTimestampsLabel")
    }

    pub fn positions_at_side(locale: &str) -> String {
        i18n::t(locale, "ChoreographyPositionsAtSideLabel")
    }

    pub fn draw_path_from(locale: &str) -> String {
        i18n::t(locale, "ChoreographyDrawPathFromLabel")
    }

    pub fn draw_path_to(locale: &str) -> String {
        i18n::t(locale, "ChoreographyDrawPathToLabel")
    }

    pub fn transparency(locale: &str) -> String {
        i18n::t(locale, "ChoreographyTransparencyLabel")
    }

    pub fn scene_fixed_positions(locale: &str) -> String {
        i18n::t(locale, "SceneFixedPositionsLabel")
    }

    pub fn scene_has_timestamp(locale: &str) -> String {
        i18n::t(locale, "SceneTimestampLabel")
    }

    pub fn timestamp_minutes(locale: &str) -> String {
        i18n::t(locale, "SceneTimestampMinutesLabel")
    }

    pub fn timestamp_seconds(locale: &str) -> String {
        i18n::t(locale, "SceneTimestampSecondsLabel")
    }

    pub fn timestamp_millis(locale: &str) -> String {
        i18n::t(locale, "SceneTimestampMillisecondsLabel")
    }

    pub fn scene_color(locale: &str) -> String {
        i18n::t(locale, "SceneColorLabel")
    }
}
