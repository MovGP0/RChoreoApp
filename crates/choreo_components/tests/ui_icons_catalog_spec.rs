use choreo_components::material::icons;
use choreo_components::material::icons::UiIconKey;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn key_component_icon_mappings_match_expected_tokens() {
    let mut errors = Vec::new();

    let floor_reset = icons::icon(UiIconKey::FloorResetViewport);
    check_eq!(errors, floor_reset.token, "home");
    check_eq!(errors, floor_reset.slint_name, "Home");

    let nav_open = icons::icon(UiIconKey::NavOpen);
    check_eq!(errors, nav_open.token, "menu");
    check_eq!(errors, nav_open.slint_name, "Menu");

    let nav_close = icons::icon(UiIconKey::NavClose);
    check_eq!(errors, nav_close.token, "close");
    check_eq!(errors, nav_close.slint_name, "Close");

    let audio_link = icons::icon(UiIconKey::AudioLink);
    check_eq!(errors, audio_link.token, "link");
    check_eq!(errors, audio_link.slint_name, "Link");

    let scenes_add_before = icons::icon(UiIconKey::ScenesAddBefore);
    check_eq!(errors, scenes_add_before.token, "add_row_above");
    check_eq!(errors, scenes_add_before.slint_name, "TableRowPlusBefore");

    let scenes_add_after = icons::icon(UiIconKey::ScenesAddAfter);
    check_eq!(errors, scenes_add_after.token, "add_row_below");
    check_eq!(errors, scenes_add_after.slint_name, "TableRowPlusAfter");

    let scenes_delete = icons::icon(UiIconKey::ScenesDelete);
    check_eq!(errors, scenes_delete.token, "delete");
    check_eq!(errors, scenes_delete.slint_name, "Delete");

    let scenes_open = icons::icon(UiIconKey::ScenesOpenChoreography);
    check_eq!(errors, scenes_open.token, "folder_open");
    check_eq!(errors, scenes_open.slint_name, "FolderOpen");

    let scenes_dancers = icons::icon(UiIconKey::ScenesNavigateDancers);
    check_eq!(errors, scenes_dancers.token, "groups");
    check_eq!(errors, scenes_dancers.slint_name, "AccountGroup");

    let settings_back = icons::icon(UiIconKey::SettingsNavigateBack);
    check_eq!(errors, settings_back.token, "arrow_back");
    check_eq!(errors, settings_back.slint_name, "ArrowLeft");

    let dancers_add = icons::icon(UiIconKey::DancersAdd);
    check_eq!(errors, dancers_add.token, "group_add");
    check_eq!(errors, dancers_add.slint_name, "AccountMultiplePlus");

    let dancers_remove = icons::icon(UiIconKey::DancersRemove);
    check_eq!(errors, dancers_remove.token, "group_remove");
    check_eq!(errors, dancers_remove.slint_name, "AccountMultipleMinus");

    assert_no_errors(errors);
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

    let mut errors = Vec::new();

    for key in keys {
        let mapping = icons::icon(key);
        check!(errors, !mapping.token.is_empty());
        check!(errors, !mapping.slint_name.is_empty());
        check!(errors, mapping.svg.contains("<svg"));
    }

    assert_no_errors(errors);
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

    let resolved = icons::from_slint_name("AccountMultipleMinus")
        .expect("AccountMultipleMinus should resolve");
    assert_eq!(resolved.token, "group_remove");

    assert!(icons::from_slint_name("DoesNotExist").is_none());
}
