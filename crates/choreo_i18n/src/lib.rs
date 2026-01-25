#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(rust_2018_idioms)]
#![deny(unused_must_use)]
#![deny(unreachable_pub)]
#![deny(elided_lifetimes_in_paths)]
#![deny(clippy::all)]

mod generated;

use once_cell::sync::Lazy;
use std::collections::HashMap;

static TRANSLATIONS: Lazy<HashMap<&'static str, HashMap<&'static str, &'static str>>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        for &locale in generated::LOCALES {
            if let Some(content) = generated::locale_toml(locale) {
                map.insert(locale, parse_locale(locale, content));
            }
        }
        map
    });

fn parse_locale(locale: &str, content: &str) -> HashMap<&'static str, &'static str> {
    let table: toml::Table = toml::from_str(content).unwrap_or_else(|err| {
        panic!("Failed to parse i18n TOML for locale {locale}: {err}")
    });

    let mut map = HashMap::with_capacity(table.len());
    for (key, value) in table {
        let value_str = value.as_str().unwrap_or_else(|| {
            panic!("i18n TOML value for key {key} in locale {locale} must be a string")
        });
        let key_static: &'static str = Box::leak(key.clone().into_boxed_str());
        let value_static: &'static str = Box::leak(value_str.to_string().into_boxed_str());
        map.insert(key_static, value_static);
    }
    map
}

pub fn translation(locale: &str, key: &str) -> Option<&'static str> {
    TRANSLATIONS
        .get(locale)
        .and_then(|table| table.get(key).copied())
}

pub fn translation_with_fallback(locale: &str, key: &str) -> Option<&'static str> {
    translation(locale, key).or_else(|| translation("en", key))
}

pub fn icon_bytes(name: &str) -> Option<&'static [u8]> {
    generated::icon_bytes(name)
}

pub fn locales() -> &'static [&'static str] {
    generated::LOCALES
}

pub fn keys() -> &'static [&'static str] {
    generated::KEYS
}

pub fn icon_names() -> &'static [&'static str] {
    generated::ICON_NAMES
}
