use crate::behavior::{Behavior, CompositeDisposable};
use choreo_i18n::translation_with_fallback;
use nject::injectable;

use super::choreography_settings_view_model::ChoreographySettingsViewModel;

#[injectable]
#[inject(|locale: String| Self::new(locale))]
pub struct ChoreographySettingsTranslateBehavior {
    locale: String,
}

impl ChoreographySettingsTranslateBehavior {
    pub fn new(locale: impl Into<String>) -> Self {
        Self {
            locale: locale.into(),
        }
    }

    pub fn apply(&self, view_model: &mut ChoreographySettingsViewModel) {
        view_model.date_label = t(&self.locale, "ChoreographyDateLabel", "Date");
        view_model.date_picker_title =
            t(&self.locale, "ChoreographyDatePickerTitle", "Date");
    }
}

impl Behavior<ChoreographySettingsViewModel> for ChoreographySettingsTranslateBehavior {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        self.apply(view_model);
    }
}

fn t(locale: &str, key: &str, fallback: &str) -> String {
    translation_with_fallback(locale, key)
        .unwrap_or(fallback)
        .to_string()
}
