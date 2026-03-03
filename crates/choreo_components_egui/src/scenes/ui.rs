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
    ui.horizontal(|ui| {
        ui.heading("Scenes");
        ui.label(format!(
            "{} / {}",
            state.visible_scenes.len(),
            state.scenes.len()
        ));
    });

    ui.horizontal(|ui| {
        if ui.button("📂").clicked() {
            actions.push(ScenesAction::RequestOpenChoreography);
        }
        if ui
            .add_enabled(state.can_save_choreo, egui::Button::new("💾"))
            .clicked()
        {
            actions.push(ScenesAction::RequestSaveChoreography);
        }
        if ui
            .add_enabled(
                state.can_navigate_to_settings,
                egui::Button::new("⚙ Settings"),
            )
            .clicked()
        {
            actions.push(ScenesAction::NavigateToSettings);
        }
        if ui
            .add_enabled(
                state.can_navigate_to_dancer_settings,
                egui::Button::new("🕺 Dancers"),
            )
            .clicked()
        {
            actions.push(ScenesAction::NavigateToDancerSettings);
        }
    });

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
    ui.horizontal(|ui| {
        if ui
            .add_enabled(state.can_delete_scene, egui::Button::new("Delete"))
            .clicked()
        {
            actions.push(ScenesAction::OpenDeleteSceneDialog);
        }
        if ui
            .add_enabled(state.can_delete_scene, egui::Button::new("Copy Positions"))
            .clicked()
        {
            actions.push(ScenesAction::OpenCopyScenePositionsDialog);
        }
    });

    let mut show_timestamps = state.show_timestamps;
    if ui
        .checkbox(&mut show_timestamps, "Show scene timestamps")
        .changed()
    {
        actions.push(ScenesAction::UpdateShowTimestamps(show_timestamps));
    }

    egui::Frame::group(ui.style()).show(ui, |ui| {
        for (index, scene) in state.visible_scenes.iter().enumerate() {
            let fill = egui::Color32::from_rgba_unmultiplied(
                scene.color.r,
                scene.color.g,
                scene.color.b,
                scene.color.a,
            );
            let label = if scene.is_selected {
                format!("● {}{}", scene.name, scene.timestamp.map(|v| format!(" ({v:.1})")).unwrap_or_default())
            } else {
                format!("{}{}", scene.name, scene.timestamp.map(|v| format!(" ({v:.1})")).unwrap_or_default())
            };
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
                ui.painter().circle_filled(rect.center(), 5.0, fill);
                if ui.selectable_label(scene.is_selected, label).clicked() {
                    actions.push(ScenesAction::SelectScene { index });
                }
            });
        }
    });

    if state.has_selected_scene {
        ui.separator();
        ui.label(format!("Selected: {}", state.selected_scene_name));
        if !state.selected_scene_text.is_empty() {
            ui.label(state.selected_scene_text.clone());
        }
        if !state.selected_scene_timestamp_text.is_empty() {
            ui.label(format!("Timestamp: {}", state.selected_scene_timestamp_text));
        }
        ui.label(format!(
            "Fixed positions: {}",
            if state.selected_scene_fixed_positions { "Yes" } else { "No" }
        ));
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
    if state.show_copy_scene_positions_dialog {
        ui.separator();
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.label("Copy scene positions?");
            ui.horizontal(|ui| {
                if ui.button("Copy").clicked() {
                    actions.push(ScenesAction::ConfirmCopyScenePositionsDialog {
                        copy_positions: true,
                    });
                }
                if ui.button("Keep Empty").clicked() {
                    actions.push(ScenesAction::ConfirmCopyScenePositionsDialog {
                        copy_positions: false,
                    });
                }
                if ui.button("Cancel").clicked() {
                    actions.push(ScenesAction::CancelCopyScenePositionsDialog);
                }
            });
        });
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
