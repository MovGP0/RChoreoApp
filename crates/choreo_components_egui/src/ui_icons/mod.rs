#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UiIconKey {
    FloorResetViewport,
    FloorOpenSvgOverlay,
    NavOpen,
    NavClose,
    NavSettings,
    AudioOpenPanel,
    AudioPlay,
    AudioPause,
    AudioLink,
    ScenesOpenChoreography,
    ScenesSaveChoreography,
    ScenesNavigateSettings,
    ScenesNavigateDancers,
    SettingsNavigateBack,
    DancersAdd,
    DancersRemove,
    NumberPickerDecrement,
    NumberPickerIncrement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UiIconSpec {
    pub token: &'static str,
    pub slint_name: &'static str,
    pub svg: &'static str,
}

#[must_use]
pub fn icon(key: UiIconKey) -> UiIconSpec {
    match key {
        UiIconKey::FloorResetViewport => UiIconSpec {
            token: "home",
            slint_name: "Home",
            svg: include_str!("../../../choreo_components/ui/icons/Home.svg"),
        },
        UiIconKey::FloorOpenSvgOverlay => UiIconSpec {
            token: "image",
            slint_name: "Svg",
            svg: include_str!("../../../choreo_components/ui/icons/Svg.svg"),
        },
        UiIconKey::NavOpen => UiIconSpec {
            token: "menu",
            slint_name: "Menu",
            svg: include_str!("../../../choreo_components/ui/icons/Menu.svg"),
        },
        UiIconKey::NavClose => UiIconSpec {
            token: "close",
            slint_name: "Close",
            svg: include_str!("../../../choreo_components/ui/icons/Close.svg"),
        },
        UiIconKey::NavSettings => UiIconSpec {
            token: "edit",
            slint_name: "Pen",
            svg: include_str!("../../../choreo_components/ui/icons/Pen.svg"),
        },
        UiIconKey::AudioOpenPanel => UiIconSpec {
            token: "play_circle",
            slint_name: "PlayCircle",
            svg: include_str!("../../../choreo_components/ui/icons/PlayCircle.svg"),
        },
        UiIconKey::AudioPlay => UiIconSpec {
            token: "play_arrow",
            slint_name: "Play",
            svg: include_str!("../../../choreo_components/ui/icons/Play.svg"),
        },
        UiIconKey::AudioPause => UiIconSpec {
            token: "pause",
            slint_name: "Pause",
            svg: include_str!("../../../choreo_components/ui/icons/Pause.svg"),
        },
        UiIconKey::AudioLink => UiIconSpec {
            token: "link",
            slint_name: "Link",
            svg: include_str!("../../../choreo_components/ui/icons/Link.svg"),
        },
        UiIconKey::ScenesOpenChoreography => UiIconSpec {
            token: "folder_open",
            slint_name: "FolderOpen",
            svg: include_str!("../../../choreo_components/ui/icons/FolderOpen.svg"),
        },
        UiIconKey::ScenesSaveChoreography => UiIconSpec {
            token: "save",
            slint_name: "ContentSave",
            svg: include_str!("../../../choreo_components/ui/icons/ContentSave.svg"),
        },
        UiIconKey::ScenesNavigateSettings => UiIconSpec {
            token: "settings",
            slint_name: "Cog",
            svg: include_str!("../../../choreo_components/ui/icons/Cog.svg"),
        },
        UiIconKey::ScenesNavigateDancers => UiIconSpec {
            token: "group",
            slint_name: "AccountMultiple",
            svg: include_str!("../../../choreo_components/ui/icons/AccountMultiple.svg"),
        },
        UiIconKey::SettingsNavigateBack => UiIconSpec {
            token: "arrow_back",
            slint_name: "ArrowLeft",
            svg: include_str!("../../../choreo_components/ui/icons/ArrowLeft.svg"),
        },
        UiIconKey::DancersAdd => UiIconSpec {
            token: "group_add",
            slint_name: "AccountMultiplePlus",
            svg: include_str!("../../../choreo_components/ui/icons/AccountMultiplePlus.svg"),
        },
        UiIconKey::DancersRemove => UiIconSpec {
            token: "group_remove",
            slint_name: "AccountMultipleRemove",
            svg: include_str!("../../../choreo_components/ui/icons/AccountMultipleRemove.svg"),
        },
        UiIconKey::NumberPickerDecrement => UiIconSpec {
            token: "remove",
            slint_name: "Minus",
            svg: include_str!("../../../choreo_components/ui/icons/Minus.svg"),
        },
        UiIconKey::NumberPickerIncrement => UiIconSpec {
            token: "add",
            slint_name: "Plus",
            svg: include_str!("../../../choreo_components/ui/icons/Plus.svg"),
        },
    }
}

#[must_use]
pub fn from_slint_name(slint_name: &str) -> Option<UiIconSpec> {
    let key = match slint_name {
        "Home" => UiIconKey::FloorResetViewport,
        "Svg" => UiIconKey::FloorOpenSvgOverlay,
        "Menu" => UiIconKey::NavOpen,
        "Close" => UiIconKey::NavClose,
        "Pen" => UiIconKey::NavSettings,
        "PlayCircle" => UiIconKey::AudioOpenPanel,
        "Play" => UiIconKey::AudioPlay,
        "Pause" => UiIconKey::AudioPause,
        "Link" => UiIconKey::AudioLink,
        "FolderOpen" => UiIconKey::ScenesOpenChoreography,
        "ContentSave" => UiIconKey::ScenesSaveChoreography,
        "Cog" => UiIconKey::ScenesNavigateSettings,
        "AccountMultiple" => UiIconKey::ScenesNavigateDancers,
        "ArrowLeft" => UiIconKey::SettingsNavigateBack,
        "AccountMultiplePlus" => UiIconKey::DancersAdd,
        "AccountMultipleRemove" => UiIconKey::DancersRemove,
        "Minus" => UiIconKey::NumberPickerDecrement,
        "Plus" => UiIconKey::NumberPickerIncrement,
        _ => return None,
    };
    Some(icon(key))
}
