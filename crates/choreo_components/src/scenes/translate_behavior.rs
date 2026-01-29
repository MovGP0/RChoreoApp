use crate::behavior::{Behavior, CompositeDisposable};
use choreo_i18n::translation_with_fallback;
use nject::injectable;

use super::scenes_view_model::ScenesPaneViewModel;

#[injectable]
#[inject(|locale: String| Self::new(locale))]
pub struct ScenesTranslateBehavior {
    locale: String,
}

impl ScenesTranslateBehavior {
    pub fn new(locale: impl Into<String>) -> Self {
        Self {
            locale: locale.into(),
        }
    }

    pub fn apply(&self, view_model: &mut ScenesPaneViewModel) {
        view_model.add_before_text = t(&self.locale, "ScenesAddBefore", "Add before");
        view_model.add_after_text = t(&self.locale, "ScenesAddAfter", "Add after");
        view_model.open_text = t(&self.locale, "ScenesOpen", "Open");
        view_model.save_text = t(&self.locale, "ScenesSave", "Save");
    }
}

impl Behavior<ScenesPaneViewModel> for ScenesTranslateBehavior {
    fn activate(&self, view_model: &mut ScenesPaneViewModel, _disposables: &mut CompositeDisposable) {
        self.apply(view_model);
    }
}

fn t(locale: &str, key: &str, fallback: &str) -> String {
    translation_with_fallback(locale, key)
        .unwrap_or(fallback)
        .to_string()
}
