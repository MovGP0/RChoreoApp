use choreo_i18n::translation_with_fallback;

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
        search_placeholder: t(locale, "SearchPlaceholder", "Search"),
        add_before: t(locale, "ScenesAddBefore", "Add before"),
        add_after: t(locale, "ScenesAddAfter", "Add after"),
        delete_scene_title: t(locale, "DeleteSceneDialogTitle", "Delete scene"),
        open: t(locale, "ScenesOpen", "Open"),
        save: t(locale, "ScenesSave", "Save"),
        settings: t(locale, "SettingsTitle", "Settings"),
        dancers: t(locale, "DancersTitle", "Dancers"),
        delete_scene_dialog_title: t(locale, "DeleteSceneDialogTitle", "Delete scene"),
        delete_scene_dialog_message: t(locale, "DeleteSceneDialogMessage", "Delete scene \"{0}\"?"),
        delete_scene_dialog_default_name: t(locale, "DeleteSceneDialogDefaultName", "this scene"),
        delete_scene_dialog_yes: t(locale, "DeleteSceneDialogYes", "Yes"),
        delete_scene_dialog_no: t(locale, "DeleteSceneDialogNo", "No"),
        copy_scene_positions_dialog_title: t(
            locale,
            "CopyScenePositionsDialogTitle",
            "Copy positions",
        ),
        copy_scene_positions_dialog_message: t(
            locale,
            "CopyScenePositionsDialogMessage",
            "Copy dancer positions from \"{0}\" to the new scene?",
        ),
        copy_scene_positions_dialog_confirm: t(locale, "CopyScenePositionsDialogConfirm", "Copy"),
        copy_scene_positions_dialog_cancel: t(
            locale,
            "CopyScenePositionsDialogCancel",
            "Don't copy",
        ),
        common_cancel: t(locale, "CommonCancel", "Cancel"),
    }
}

fn t(locale: &str, key: &str, fallback: &'static str) -> String {
    translation_with_fallback(locale, key)
        .unwrap_or(fallback)
        .to_string()
}
