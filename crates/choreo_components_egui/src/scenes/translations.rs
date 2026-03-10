use crate::i18n::t;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenesTranslations {
    pub search_placeholder: String,
    pub add_before: String,
    pub add_after: String,
    pub delete_scene_title: String,
    pub open: String,
    pub save: String,
    pub settings: String,
    pub dancers: String,
    pub delete_scene_dialog_title: String,
    pub delete_scene_dialog_message: String,
    pub delete_scene_dialog_default_name: String,
    pub delete_scene_dialog_yes: String,
    pub delete_scene_dialog_no: String,
    pub copy_scene_positions_dialog_title: String,
    pub copy_scene_positions_dialog_message: String,
    pub copy_scene_positions_dialog_confirm: String,
    pub copy_scene_positions_dialog_cancel: String,
    pub common_cancel: String,
}

#[must_use]
pub fn scenes_translations(locale: &str) -> ScenesTranslations {
    ScenesTranslations {
        search_placeholder: t(locale, "SearchPlaceholder"),
        add_before: t(locale, "ScenesAddBefore"),
        add_after: t(locale, "ScenesAddAfter"),
        delete_scene_title: t(locale, "DeleteSceneDialogTitle"),
        open: t(locale, "ScenesOpen"),
        save: t(locale, "ScenesSave"),
        settings: t(locale, "SettingsTitle"),
        dancers: t(locale, "DancersTitle"),
        delete_scene_dialog_title: t(locale, "DeleteSceneDialogTitle"),
        delete_scene_dialog_message: t(locale, "DeleteSceneDialogMessage"),
        delete_scene_dialog_default_name: t(locale, "DeleteSceneDialogDefaultName"),
        delete_scene_dialog_yes: t(locale, "DeleteSceneDialogYes"),
        delete_scene_dialog_no: t(locale, "DeleteSceneDialogNo"),
        copy_scene_positions_dialog_title: t(locale, "CopyScenePositionsDialogTitle"),
        copy_scene_positions_dialog_message: t(locale, "CopyScenePositionsDialogMessage"),
        copy_scene_positions_dialog_confirm: t(locale, "CopyScenePositionsDialogConfirm"),
        copy_scene_positions_dialog_cancel: t(locale, "CopyScenePositionsDialogCancel"),
        common_cancel: t(locale, "CommonCancel"),
    }
}
