use egui::Ui;

use choreo_i18n::translation_with_fallback;

use super::actions::ScenesAction;
use super::state::SceneItemState;
use super::state::ScenesState;

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteSceneDialogViewModel {
    pub title_text: String,
    pub scene_name: String,
    pub message_text: String,
    pub confirm_text: String,
    pub cancel_text: String,
    pub scene_color: egui::Color32,
}

pub fn draw(ui: &mut Ui, state: &ScenesState) -> Vec<ScenesAction> {
    let mut actions: Vec<ScenesAction> = Vec::new();
    ui.heading("Scenes");
    ui.label(format!(
        "Visible scenes: {} / {}",
        state.visible_scenes.len(),
        state.scenes.len()
    ));

    let mut search = state.search_text.clone();
    if ui.text_edit_singleline(&mut search).changed() {
        actions.push(ScenesAction::UpdateSearchText(search));
    }

    if ui.button("Insert Before").clicked() {
        actions.push(ScenesAction::InsertScene {
            insert_after: false,
        });
    }
    if ui.button("Insert After").clicked() {
        actions.push(ScenesAction::InsertScene { insert_after: true });
    }
    if ui
        .add_enabled(
            state.selected_scene.is_some(),
            egui::Button::new("Delete Scene"),
        )
        .clicked()
    {
        actions.push(ScenesAction::OpenDeleteSceneDialog);
    }

    let mut show_timestamps = state.show_timestamps;
    if ui
        .checkbox(&mut show_timestamps, "Show scene timestamps")
        .changed()
    {
        actions.push(ScenesAction::UpdateShowTimestamps(show_timestamps));
    }

    for (index, scene) in state.visible_scenes.iter().enumerate() {
        let label = if scene.is_selected {
            format!("* {}", scene.name)
        } else {
            scene.name.clone()
        };
        if ui.button(label).clicked() {
            actions.push(ScenesAction::SelectScene { index });
        }
    }

    if state.show_delete_scene_dialog {
        if let Some(selected_scene) = state.selected_scene.as_ref() {
            match draw_delete_scene_dialog(ui, selected_scene, "en") {
                Some(DeleteSceneDialogAction::Cancel) => {
                    actions.push(ScenesAction::CancelDeleteSceneDialog);
                }
                Some(DeleteSceneDialogAction::ConfirmDelete) => {
                    actions.push(ScenesAction::ConfirmDeleteSceneDialog);
                }
                None => {}
            }
        } else {
            actions.push(ScenesAction::CancelDeleteSceneDialog);
        }
    }

    actions
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteSceneDialogAction {
    ConfirmDelete,
    Cancel,
}

pub fn build_delete_scene_dialog_view_model(
    selected_scene: &SceneItemState,
    locale: &str,
) -> DeleteSceneDialogViewModel {
    let title_text = t(locale, "DeleteSceneDialogTitle");
    let default_name = t(locale, "DeleteSceneDialogDefaultName");
    let scene_name = if selected_scene.name.trim().is_empty() {
        default_name
    } else {
        selected_scene.name.clone()
    };
    let message_text = t(locale, "DeleteSceneDialogMessage").replace("{0}", &scene_name);
    let confirm_text = t(locale, "DeleteSceneDialogYes");
    let cancel_text = t(locale, "DeleteSceneDialogNo");

    DeleteSceneDialogViewModel {
        title_text,
        scene_name,
        message_text,
        confirm_text,
        cancel_text,
        scene_color: egui::Color32::from_rgba_unmultiplied(
            selected_scene.color.r,
            selected_scene.color.g,
            selected_scene.color.b,
            selected_scene.color.a,
        ),
    }
}

pub fn draw_delete_scene_dialog(
    ui: &mut Ui,
    selected_scene: &SceneItemState,
    locale: &str,
) -> Option<DeleteSceneDialogAction> {
    let view_model = build_delete_scene_dialog_view_model(selected_scene, locale);
    let mut action = None;

    ui.separator();
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.heading(view_model.title_text);
        ui.horizontal(|ui| {
            let swatch_size = egui::vec2(12.0, 12.0);
            let (rect, _) = ui.allocate_exact_size(swatch_size, egui::Sense::hover());
            ui.painter()
                .circle_filled(rect.center(), 6.0, view_model.scene_color);
            ui.label(view_model.scene_name);
        });
        ui.label(view_model.message_text);
        ui.horizontal(|ui| {
            if ui.button(view_model.cancel_text).clicked() {
                action = Some(DeleteSceneDialogAction::Cancel);
            }
            if ui.button(view_model.confirm_text).clicked() {
                action = Some(DeleteSceneDialogAction::ConfirmDelete);
            }
        });
    });

    action
}

fn t(locale: &str, key: &str) -> String {
    translation_with_fallback(locale, key)
        .unwrap_or(key)
        .to_string()
}
