use egui::CornerRadius;
use egui::Frame;
use egui::Sense;
use egui::Ui;
use egui::vec2;
use egui_material3::MaterialButton;

use crate::scenes::state::SceneItemState;
use crate::scenes::translations::scenes_translations;

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteSceneDialogViewModel {
    pub title_text: String,
    pub scene_name: String,
    pub message_text: String,
    pub confirm_text: String,
    pub cancel_text: String,
    pub scene_color: egui::Color32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteSceneDialogAction {
    ConfirmDelete,
    Cancel,
}

#[must_use]
pub fn build_delete_scene_dialog_view_model(
    selected_scene: &SceneItemState,
    locale: &str,
) -> DeleteSceneDialogViewModel {
    let strings = scenes_translations(locale);
    let scene_name = if selected_scene.name.trim().is_empty() {
        strings.delete_scene_dialog_default_name
    } else {
        selected_scene.name.clone()
    };
    let message_text = strings
        .delete_scene_dialog_message
        .replace("{0}", &scene_name);

    DeleteSceneDialogViewModel {
        title_text: strings.delete_scene_dialog_title,
        scene_name,
        message_text,
        confirm_text: strings.delete_scene_dialog_yes,
        cancel_text: strings.delete_scene_dialog_no,
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
    Frame::group(ui.style()).show(ui, |ui| {
        ui.heading(view_model.title_text);
        ui.horizontal(|ui| {
            let swatch_size = vec2(12.0, 12.0);
            let (rect, _) = ui.allocate_exact_size(swatch_size, Sense::hover());
            ui.painter()
                .rect_filled(rect, CornerRadius::same(6), view_model.scene_color);
            ui.label(view_model.scene_name);
        });
        ui.label(view_model.message_text);
        ui.horizontal(|ui| {
            if ui
                .add(MaterialButton::new(view_model.cancel_text.as_str()))
                .clicked()
            {
                action = Some(DeleteSceneDialogAction::Cancel);
            }
            if ui
                .add(MaterialButton::new(view_model.confirm_text.as_str()))
                .clicked()
            {
                action = Some(DeleteSceneDialogAction::ConfirmDelete);
            }
        });
    });

    action
}
