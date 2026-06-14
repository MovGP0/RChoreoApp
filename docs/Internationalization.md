# Internationalization

This project uses `choreo_i18n` TOML catalogs as the source of truth for user-visible strings.
Every string that appears in egui must be resolved from the catalog through the egui translation
module for that feature.

## Core Rules

- Do not put user-visible strings in ViewModels, reducers, behaviors, or widget logic.
- Do not use English fallback strings in feature translation modules or UI call sites.
- Do not add English text to non-English locale catalogs as a placeholder.
- Missing translations must fail fast. Use `crates/choreo_components/src/i18n.rs::t(locale, key)`
  for direct lookups so missing keys panic during tests instead of silently showing fallback text.
- Translation keys must use the same `PascalCase` names as the `choreo_i18n` TOML catalog.
- Keep key normalization logic in `crates/choreo_components/src/i18n.rs`; do not duplicate
  snake-case to PascalCase conversion in feature modules.

## Adding A New UI String

1. Choose a descriptive `PascalCase` key, for example `SettingsThemeVariantLabel`.
2. Add that key to every file in `crates/choreo_i18n/assets/i18n/`.
3. Translate every non-English locale into the regional language for that catalog.
4. Add the new field to the feature translation struct, for example
   `crates/choreo_components/src/settings/translations.rs`.
5. Bind the UI label from that translation struct. The widget should receive translated text, not
   catalog keys.
6. Add or update a spec that resolves the translated value through the egui translation module.
7. Run the focused test target that parses and uses the catalog.

## Regional Translation Requirements

Translations must be useful regional-language strings, not copied English text. This applies to
headers, labels, menu items, dropdown entries, tooltips, dialog text, validation messages, and any
other text that users can see.

Use English only for the English catalog. A non-English catalog may contain a word that looks like
English only when that is the accepted regional spelling or loanword for that UI concept.

When adding many keys, update all locale files in the same change. Partial catalog updates are not
acceptable because the lookup layer is fail-fast and because mixed-language UI is a regression.

## Dropdowns And Enum Labels

For enum-backed controls, every visible option needs its own catalog key. Do not generate labels
from enum names at runtime. For example, a theme variant dropdown should have keys such as:

```toml
SettingsThemeVariantLabel = "Theme variant"
MaterialThemeVariantContent = "Content"
MaterialThemeVariantTonalSpot = "Tonal spot"
```

Each locale must translate the header and every entry. The UI should map enum variants to translated
strings in one feature-owned helper, then use those strings in the dropdown.

## Validation

For i18n-only changes, run:

```sh
cargo build -p choreo_i18n
cargo test -p choreo_i18n
```

For egui feature translation changes, also run the focused component tests for that feature, for
example:

```sh
cargo test -p choreo_components --test settings_tests
cargo clippy -p choreo_components --test settings_tests --all-features -- -D warnings
```

If `cargo-nextest` is available and the relevant test target supports it, prefer the repository's
standard `cargo nextest run -p PROJECTNAME` workflow.
