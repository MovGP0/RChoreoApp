use crate::i18n;

pub struct ChoreographySettingsTranslations;

impl ChoreographySettingsTranslations {
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
}
