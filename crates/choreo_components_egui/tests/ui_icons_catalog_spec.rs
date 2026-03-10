use choreo_components_egui::material::icons;
use choreo_components_egui::material::icons::UiIconKey;

#[test]
fn key_component_icon_mappings_match_expected_tokens() {
    let floor_reset = icons::icon(UiIconKey::FloorResetViewport);
    assert_eq!(floor_reset.token, "home");
    assert_eq!(floor_reset.slint_name, "Home");

    let nav_open = icons::icon(UiIconKey::NavOpen);
    assert_eq!(nav_open.token, "menu");
    assert_eq!(nav_open.slint_name, "Menu");

    let nav_close = icons::icon(UiIconKey::NavClose);
    assert_eq!(nav_close.token, "close");
    assert_eq!(nav_close.slint_name, "Close");

    let audio_link = icons::icon(UiIconKey::AudioLink);
    assert_eq!(audio_link.token, "link");
    assert_eq!(audio_link.slint_name, "Link");

    let scenes_add_before = icons::icon(UiIconKey::ScenesAddBefore);
    assert_eq!(scenes_add_before.token, "add_row_above");
    assert_eq!(scenes_add_before.slint_name, "TableRowPlusBefore");

    let scenes_add_after = icons::icon(UiIconKey::ScenesAddAfter);
    assert_eq!(scenes_add_after.token, "add_row_below");
    assert_eq!(scenes_add_after.slint_name, "TableRowPlusAfter");

    let scenes_delete = icons::icon(UiIconKey::ScenesDelete);
    assert_eq!(scenes_delete.token, "delete");
    assert_eq!(scenes_delete.slint_name, "Delete");

    let scenes_open = icons::icon(UiIconKey::ScenesOpenChoreography);
    assert_eq!(scenes_open.token, "folder_open");
    assert_eq!(scenes_open.slint_name, "FolderOpen");

    let scenes_dancers = icons::icon(UiIconKey::ScenesNavigateDancers);
    assert_eq!(scenes_dancers.token, "groups");
    assert_eq!(scenes_dancers.slint_name, "AccountGroup");

    let settings_back = icons::icon(UiIconKey::SettingsNavigateBack);
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
        UiIconKey::ScenesAddBefore,
        UiIconKey::ScenesAddAfter,
        UiIconKey::ScenesDelete,
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
        let mapping = icons::icon(key);
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
    let resolved = icons::from_slint_name("PlayCircle").expect("PlayCircle should resolve");
    assert_eq!(resolved.token, "play_circle");

    let resolved =
        icons::from_slint_name("TableRowPlusBefore").expect("TableRowPlusBefore should resolve");
    assert_eq!(resolved.token, "add_row_above");

    let resolved = icons::from_slint_name("AccountGroup").expect("AccountGroup should resolve");
    assert_eq!(resolved.token, "groups");

    assert!(icons::from_slint_name("DoesNotExist").is_none());
}
