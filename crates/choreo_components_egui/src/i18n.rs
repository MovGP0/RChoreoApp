use choreo_i18n::translation_with_fallback;

#[must_use]
pub fn t(locale: &str, key: &str, fallback: &'static str) -> String {
    translation_with_fallback(locale, key)
        .unwrap_or(fallback)
        .to_string()
}
