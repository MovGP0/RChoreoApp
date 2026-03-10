use choreo_i18n::translation;

#[must_use]
pub fn t(locale: &str, key: &str) -> String {
    translation(locale, key)
        .unwrap_or_else(|| panic!("Missing translation for locale '{locale}' and key '{key}'"))
        .to_string()
}

#[must_use]
pub fn slint_key_to_i18n_key(key: &str) -> String {
    let mut normalized = String::with_capacity(key.len());
    for segment in key.split('_').filter(|segment| !segment.is_empty()) {
        let mut chars = segment.chars();
        if let Some(first) = chars.next() {
            normalized.push(first.to_ascii_uppercase());
            normalized.extend(chars);
        }
    }

    normalized
}

#[must_use]
pub fn translate_slint(locale: &str, key: &str) -> Option<&'static str> {
    let normalized_key = slint_key_to_i18n_key(key);
    translation(locale, normalized_key.as_str())
}

#[must_use]
pub fn t_slint(locale: &str, key: &str) -> String {
    let normalized_key = slint_key_to_i18n_key(key);
    t(locale, normalized_key.as_str())
}
