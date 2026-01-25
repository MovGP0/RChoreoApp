use crate::behavior::{Behavior, CompositeDisposable};

use choreo_i18n::translation_with_fallback;

use super::audio_player_view_model::AudioPlayerViewModel;

pub struct AudioPlayerTranslateBehavior {
    locale: String,
}

impl AudioPlayerTranslateBehavior {
    pub fn new(locale: impl Into<String>) -> Self {
        Self {
            locale: locale.into(),
        }
    }

    pub fn apply(&self, view_model: &mut AudioPlayerViewModel) {
        if view_model.title.trim().is_empty() || view_model.title == "Audio" {
            view_model.title = t(&self.locale, "AudioPlayerTitle", "Audio");
        }

        view_model.update_speed_label();
        view_model.update_duration_label();
    }
}

impl Behavior<AudioPlayerViewModel> for AudioPlayerTranslateBehavior {
    fn activate(&self, view_model: &mut AudioPlayerViewModel, _disposables: &mut CompositeDisposable) {
        self.apply(view_model);
    }
}

fn t(locale: &str, key: &str, fallback: &str) -> String {
    translation_with_fallback(locale, key)
        .unwrap_or(fallback)
        .to_string()
}
