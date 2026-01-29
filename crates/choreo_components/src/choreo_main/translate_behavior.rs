use crate::behavior::{Behavior, CompositeDisposable};
use choreo_i18n::translation_with_fallback;
use nject::injectable;

use super::main_view_model::MainViewModel;

#[injectable]
#[inject(|locale: String| Self::new(locale))]
pub struct MainTranslateBehavior {
    locale: String,
}

impl MainTranslateBehavior {
    pub fn new(locale: impl Into<String>) -> Self {
        Self {
            locale: locale.into(),
        }
    }

    pub fn apply(&self, view_model: &mut MainViewModel) {
        view_model.toggle_nav_tooltip =
            t(&self.locale, "MainToggleNavTooltip", "Toggle navigation");
        view_model.open_settings_tooltip =
            t(&self.locale, "MainOpenSettingsTooltip", "Choreography settings");
        view_model.open_image_tooltip =
            t(&self.locale, "MainOpenFloorPlanTooltip", "Open floor plan");
        view_model.open_audio_tooltip =
            t(&self.locale, "MainOpenAudioTooltip", "Open audio file");
    }
}

impl Behavior<MainViewModel> for MainTranslateBehavior {
    fn activate(&self, view_model: &mut MainViewModel, _disposables: &mut CompositeDisposable) {
        self.apply(view_model);
    }
}

fn t(locale: &str, key: &str, fallback: &str) -> String {
    translation_with_fallback(locale, key)
        .unwrap_or(fallback)
        .to_string()
}
