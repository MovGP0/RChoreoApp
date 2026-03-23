pub(super) struct FloorTranslations {
    pub placement_title: &'static str,
    pub placement_hint: &'static str,
    pub placement_remaining_prefix: &'static str,
}

#[must_use]
pub(super) fn floor_translations(locale: &str) -> FloorTranslations {
    match locale {
        "de" => FloorTranslations {
            placement_title: "Positionieren",
            placement_hint: "Tippen, um eine Position zu setzen",
            placement_remaining_prefix: "Verbleibend: ",
        },
        _ => FloorTranslations {
            placement_title: "Placement",
            placement_hint: "Tap to place a position",
            placement_remaining_prefix: "Remaining: ",
        },
    }
}
