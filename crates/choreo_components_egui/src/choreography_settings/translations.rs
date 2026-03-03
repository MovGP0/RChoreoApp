use crate::i18n;

pub struct ChoreographySettingsTranslations;

impl ChoreographySettingsTranslations {
    pub fn comment(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.comment", "Comment")
    }

    pub fn name(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.name", "Name")
    }

    pub fn subtitle(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.subtitle", "Subtitle")
    }

    pub fn date(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.date", "Date")
    }

    pub fn variation(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.variation", "Variation")
    }

    pub fn author(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.author", "Author")
    }

    pub fn description(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.description", "Description")
    }

    pub fn title(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.title", "Choreography Settings")
    }

    pub fn choreography(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.choreography", "Choreography")
    }

    pub fn selected_scene(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.selected_scene", "Selected Scene")
    }

    pub fn floor(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.floor", "Floor")
    }

    pub fn display(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.display", "Display")
    }

    pub fn scene_name(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.scene_name", "Scene name")
    }

    pub fn scene_text(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.scene_text", "Scene text")
    }

    pub fn floor_front(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.floor_front", "Front")
    }

    pub fn floor_back(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.floor_back", "Back")
    }

    pub fn floor_left(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.floor_left", "Left")
    }

    pub fn floor_right(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.floor_right", "Right")
    }

    pub fn floor_color(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.floor_color", "Floor color")
    }

    pub fn grid_resolution(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.grid_resolution", "Grid resolution")
    }

    pub fn grid_lines(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.grid_lines", "Grid lines")
    }

    pub fn show_legend(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.show_legend", "Show legend")
    }

    pub fn snap_to_grid(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.snap_to_grid", "Snap to grid")
    }

    pub fn show_timestamps(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.show_timestamps", "Show timestamps")
    }

    pub fn positions_at_side(locale: &str) -> String {
        i18n::t(
            locale,
            "choreo.settings.positions_at_side",
            "Positions at side",
        )
    }

    pub fn draw_path_from(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.draw_path_from", "Draw path from")
    }

    pub fn draw_path_to(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.draw_path_to", "Draw path to")
    }

    pub fn transparency(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.transparency", "Transparency")
    }

    pub fn scene_fixed_positions(locale: &str) -> String {
        i18n::t(
            locale,
            "choreo.settings.scene_fixed_positions",
            "Fixed positions",
        )
    }

    pub fn scene_has_timestamp(locale: &str) -> String {
        i18n::t(
            locale,
            "choreo.settings.scene_has_timestamp",
            "Has timestamp",
        )
    }

    pub fn timestamp_minutes(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.timestamp_minutes", "Minutes")
    }

    pub fn timestamp_seconds(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.timestamp_seconds", "Seconds")
    }

    pub fn timestamp_millis(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.timestamp_millis", "Milliseconds")
    }

    pub fn scene_color(locale: &str) -> String {
        i18n::t(locale, "choreo.settings.scene_color", "Scene color")
    }
}
