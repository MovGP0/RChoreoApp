use egui::CornerRadius;
use egui::Frame;
use egui::Margin;
use egui::ScrollArea;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::pos2;
use egui::vec2;
use egui_material3::MaterialButton;

use crate::delete_scene_dialog::ui::DeleteSceneDialogAction;
use crate::delete_scene_dialog::ui::draw_delete_scene_dialog;
use crate::ui_style::material_style_metrics::material_style_metrics;
use crate::ui_style::typography;
use crate::ui_style::typography::TypographyRole;

use super::actions::ScenesAction;
use super::state::SceneItemState;
use super::state::ScenesState;
use super::state::format_seconds;
use super::translations::scenes_translations;

#[must_use]
pub fn scene_pane_action_flow(state: &ScenesState) -> Vec<ScenesAction> {
    let mut actions = vec![
        ScenesAction::InsertScene {
            insert_after: false,
        },
        ScenesAction::InsertScene { insert_after: true },
    ];
    if state.can_delete_scene {
        actions.push(ScenesAction::OpenDeleteSceneDialog);
    }
    actions.push(ScenesAction::RequestOpenChoreography);
    if state.can_save_choreo {
        actions.push(ScenesAction::RequestSaveChoreography);
    }
    if state.can_navigate_to_settings {
        actions.push(ScenesAction::NavigateToSettings);
    }
    if state.can_navigate_to_dancer_settings {
        actions.push(ScenesAction::NavigateToDancerSettings);
    }
    actions
}

pub fn draw(ui: &mut Ui, state: &ScenesState) -> Vec<ScenesAction> {
    let mut actions: Vec<ScenesAction> = Vec::new();
    let metrics = material_style_metrics();
    let strings = scenes_translations("en");

    ui.spacing_mut().item_spacing = vec2(metrics.spacings.spacing_12, metrics.spacings.spacing_12);

    Frame::new()
        .fill(ui.visuals().faint_bg_color)
        .stroke(Stroke::new(
            metrics.strokes.outline,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .corner_radius(CornerRadius::same(metrics.corner_radii.border_radius_12 as u8))
        .inner_margin(Margin::same(metrics.paddings.padding_12 as i8))
        .show(ui, |ui| {
            ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    for (index, scene) in state.visible_scenes.iter().enumerate() {
                        if draw_scene_list_item(ui, scene, state.show_timestamps).clicked() {
                            actions.push(ScenesAction::SelectScene { index });
                        }
                    }
                });
        });

    let mut search = state.search_text.clone();
    let search_box = egui::TextEdit::singleline(&mut search)
        .desired_width(ui.available_width())
        .hint_text(strings.search_placeholder.as_str());
    if ui.add(search_box).changed() {
        actions.push(ScenesAction::UpdateSearchText(search));
    }

    ui.horizontal(|ui| {
        if ui
            .add(MaterialButton::new(strings.add_before.as_str()))
            .clicked()
        {
            actions.push(ScenesAction::InsertScene {
                insert_after: false,
            });
        }
        if ui
            .add(MaterialButton::new(strings.add_after.as_str()))
            .clicked()
        {
            actions.push(ScenesAction::InsertScene { insert_after: true });
        }
        if ui
            .add_enabled(
                state.can_delete_scene,
                MaterialButton::new(strings.delete_scene_title.as_str()),
            )
            .clicked()
        {
            actions.push(ScenesAction::OpenDeleteSceneDialog);
        }
    });

    ui.horizontal(|ui| {
        if ui.add(MaterialButton::new(strings.open.as_str())).clicked() {
            actions.push(ScenesAction::RequestOpenChoreography);
        }
        if ui
            .add_enabled(
                state.can_save_choreo,
                MaterialButton::new(strings.save.as_str()),
            )
            .clicked()
        {
            actions.push(ScenesAction::RequestSaveChoreography);
        }
        if ui
            .add_enabled(
                state.can_navigate_to_settings,
                MaterialButton::new(strings.settings.as_str()),
            )
            .clicked()
        {
            actions.push(ScenesAction::NavigateToSettings);
        }
        if ui
            .add_enabled(
                state.can_navigate_to_dancer_settings,
                MaterialButton::new(strings.dancers.as_str()),
            )
            .clicked()
        {
            actions.push(ScenesAction::NavigateToDancerSettings);
        }
    });

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
        Frame::group(ui.style()).show(ui, |ui| {
            ui.heading(strings.copy_scene_positions_dialog_title.as_str());
            let scene_name = state
                .selected_scene
                .as_ref()
                .map(|scene| scene.name.as_str())
                .filter(|name| !name.trim().is_empty())
                .unwrap_or("this scene");
            ui.label(
                strings
                    .copy_scene_positions_dialog_message
                    .replace("{0}", scene_name),
            );
            ui.horizontal(|ui| {
                if ui
                    .button(strings.copy_scene_positions_dialog_confirm.as_str())
                    .clicked()
                {
                    actions.push(ScenesAction::ConfirmCopyScenePositionsDialog {
                        copy_positions: true,
                    });
                }
                if ui
                    .button(strings.copy_scene_positions_dialog_cancel.as_str())
                    .clicked()
                {
                    actions.push(ScenesAction::ConfirmCopyScenePositionsDialog {
                        copy_positions: false,
                    });
                }
                if ui.button(strings.common_cancel.as_str()).clicked() {
                    actions.push(ScenesAction::CancelCopyScenePositionsDialog);
                }
            });
        });
    }

    actions
}

#[must_use]
pub const fn scene_title_role() -> TypographyRole {
    TypographyRole::TitleSmall
}

#[must_use]
pub const fn scene_timestamp_role() -> TypographyRole {
    TypographyRole::BodySmall
}

fn draw_scene_list_item(
    ui: &mut Ui,
    scene: &SceneItemState,
    show_timestamps: bool,
) -> egui::Response {
    // Source scene list rows use 50px/62px; keep these dimensions for parity.
    let row_height = if show_timestamps { 62.0 } else { 50.0 };
    let (row_rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), row_height), Sense::click());
    if !ui.is_rect_visible(row_rect) {
        return response;
    }

    let visuals = ui.style().visuals.clone();
    let fill_color = if scene.is_selected {
        visuals.selection.bg_fill
    } else {
        visuals.extreme_bg_color
    };
    let stroke_color = if scene.is_selected {
        visuals.selection.stroke.color
    } else {
        visuals.widgets.noninteractive.bg_stroke.color
    };
    let metrics = material_style_metrics();
    let stroke_width = if scene.is_selected {
        metrics.strokes.focus
    } else {
        metrics.strokes.outline
    };

    let card_rect = row_rect.shrink2(vec2(0.0, metrics.sizes.size_4));
    ui.painter().rect(
        card_rect,
        CornerRadius::same(metrics.sizes.size_6 as u8),
        fill_color,
        Stroke::new(stroke_width, stroke_color),
        egui::StrokeKind::Middle,
    );

    if scene.is_selected {
        let accent = egui::Rect::from_min_size(card_rect.min, vec2(metrics.sizes.size_4, card_rect.height()));
        ui.painter().rect_filled(
            accent,
            CornerRadius::same(metrics.sizes.size_6 as u8),
            visuals.selection.stroke.color,
        );
    }

    let color_rect = egui::Rect::from_min_size(
        pos2(
            card_rect.left() + metrics.spacings.spacing_8,
            card_rect.top() + metrics.spacings.spacing_8,
        ),
        vec2(metrics.spacings.spacing_12, metrics.spacings.spacing_12),
    );
    ui.painter().rect_filled(
        color_rect,
        CornerRadius::same(3),
        egui::Color32::from_rgba_unmultiplied(
            scene.color.r,
            scene.color.g,
            scene.color.b,
            scene.color.a,
        ),
    );

    let title_color = if scene.is_selected {
        visuals.strong_text_color()
    } else {
        visuals.text_color()
    };
    ui.painter().text(
        pos2(
            card_rect.left() + metrics.sizes.size_24 + metrics.sizes.size_2,
            card_rect.top() + metrics.spacings.spacing_8,
        ),
        egui::Align2::LEFT_TOP,
        scene.name.as_str(),
        typography::font_id_for_role(scene_title_role()),
        title_color,
    );

    if show_timestamps {
        let timestamp_text = scene.timestamp.map(format_seconds).unwrap_or_default();
        ui.painter().text(
            pos2(
                card_rect.left() + metrics.spacings.spacing_8,
                card_rect.top() + metrics.sizes.size_30,
            ),
            egui::Align2::LEFT_TOP,
            timestamp_text,
            typography::font_id_for_role(scene_timestamp_role()),
            visuals.weak_text_color(),
        );
    }

    response
}
