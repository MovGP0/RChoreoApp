use choreo_components_egui::ui_icons;
use choreo_components_egui::ui_icons::UiIconKey;

#[test]
fn key_component_icon_mappings_match_expected_tokens() {
    let floor_reset = ui_icons::icon(UiIconKey::FloorResetViewport);
    assert_eq!(floor_reset.token, "home");
    assert_eq!(floor_reset.slint_name, "Home");

    let nav_open = ui_icons::icon(UiIconKey::NavOpen);
    assert_eq!(nav_open.token, "menu");
    assert_eq!(nav_open.slint_name, "Menu");

    let nav_close = ui_icons::icon(UiIconKey::NavClose);
    assert_eq!(nav_close.token, "close");
    assert_eq!(nav_close.slint_name, "Close");

    let audio_link = ui_icons::icon(UiIconKey::AudioLink);
    assert_eq!(audio_link.token, "link");
    assert_eq!(audio_link.slint_name, "Link");

    let scenes_open = ui_icons::icon(UiIconKey::ScenesOpenChoreography);
    assert_eq!(scenes_open.token, "folder_open");
    assert_eq!(scenes_open.slint_name, "FolderOpen");

    let settings_back = ui_icons::icon(UiIconKey::SettingsNavigateBack);
    assert_eq!(settings_back.token, "arrow_back");
    assert_eq!(settings_back.slint_name, "ArrowLeft");
}

#[test]
fn component_icon_svgs_resolve_from_slint_catalog_assets() {
    let keys = [
        UiIconKey::FloorResetViewport,
        UiIconKey::FloorOpenSvgOverlay,
        UiIconKey::NavOpen,
        UiIconKey::NavClose,
        UiIconKey::NavSettings,
        UiIconKey::AudioOpenPanel,
        UiIconKey::AudioPlay,
        UiIconKey::AudioPause,
        UiIconKey::AudioLink,
        UiIconKey::ScenesOpenChoreography,
        UiIconKey::ScenesSaveChoreography,
        UiIconKey::ScenesNavigateSettings,
        UiIconKey::ScenesNavigateDancers,
        UiIconKey::SettingsNavigateBack,
        UiIconKey::DancersAdd,
        UiIconKey::DancersRemove,
        UiIconKey::NumberPickerDecrement,
        UiIconKey::NumberPickerIncrement,
    ];

    for key in keys {
        let mapping = ui_icons::icon(key);
        assert!(!mapping.token.is_empty());
        assert!(!mapping.slint_name.is_empty());
        assert!(
            mapping.svg.contains("<svg"),
            "missing svg for key {key:?} ({})",
            mapping.slint_name
        );
    }
}

#[test]
fn slint_name_resolver_returns_catalog_entries() {
    let resolved = ui_icons::from_slint_name("PlayCircle").expect("PlayCircle should resolve");
    assert_eq!(resolved.token, "play_circle");

    let resolved = ui_icons::from_slint_name("AccountMultipleRemove")
        .expect("AccountMultipleRemove should resolve");
    assert_eq!(resolved.token, "group_remove");

    assert!(ui_icons::from_slint_name("DoesNotExist").is_none());
}
