use crate::behavior::{Behavior, CompositeDisposable};
use choreo_i18n::translation_with_fallback;
use nject::injectable;

use super::dancer_settings_view_model::DancerSettingsViewModel;

#[injectable]
#[inject(|locale: String| Self::new(locale))]
pub struct DancerSettingsTranslateBehavior {
    locale: String,
}

impl DancerSettingsTranslateBehavior {
    pub fn new(locale: impl Into<String>) -> Self {
        Self {
            locale: locale.into(),
        }
    }

    pub fn apply(&self, view_model: &mut DancerSettingsViewModel) {
        view_model.title_text = t(&self.locale, "DancersTitle", "Dancers");
        view_model.swap_title_text =
            t(&self.locale, "DancerSwapSectionTitle", "Swap dancers");
        view_model.save_text = t(&self.locale, "CommonOk", "OK");
        view_model.cancel_text = t(&self.locale, "CommonCancel", "Cancel");
    }
}

impl Behavior<DancerSettingsViewModel> for DancerSettingsTranslateBehavior {
    fn activate(
        &self,
        view_model: &mut DancerSettingsViewModel,
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
