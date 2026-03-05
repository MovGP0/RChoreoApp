use egui::CornerRadius;
use egui::Frame;
use egui::Margin;
use egui::Pos2;
use egui::Rect;
use egui::Response;
use egui::ScrollArea;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::pos2;
use egui::vec2;
use egui_material3::MaterialIconButton;

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
use super::ui_icons;
use super::ui_icons::UiIconKey;

const DEFAULT_LOCALE: &str = "en";
const SEARCH_BAR_HEIGHT_PX: f32 = 44.0;
const SCENE_ROW_HEIGHT_PX: f32 = 50.0;
const SCENE_ROW_HEIGHT_WITH_TIMESTAMPS_PX: f32 = 62.0;
const SCENE_ROW_VERTICAL_GAP_PX: f32 = 4.0;
const SCENE_ROW_SWATCH_X_PX: f32 = 8.0;
const SCENE_ROW_SWATCH_Y_PX: f32 = 8.0;
const SCENE_ROW_SWATCH_SIZE_PX: f32 = 12.0;
const SCENE_ROW_SWATCH_CORNER_RADIUS_PX: u8 = 3;
const SCENE_ROW_TEXT_LEFT_PX: f32 = 26.0;
const SCENE_ROW_TITLE_Y_PX: f32 = 8.0;
const SCENE_ROW_TIMESTAMP_Y_PX: f32 = 30.0;
const SCENE_ROW_ACCENT_WIDTH_PX: f32 = 4.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SceneSearchBarViewModel {
    pub placeholder_text: String,
    pub clear_tooltip: String,
    pub show_clear_button: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SceneListItemLayout {
    pub content_rect: Rect,
    pub accent_rect: Rect,
    pub swatch_rect: Rect,
    pub title_position: Pos2,
    pub timestamp_position: Pos2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SceneListItemColors {
    pub background: egui::Color32,
    pub border: egui::Color32,
    pub title: egui::Color32,
    pub timestamp: egui::Color32,
    pub accent: egui::Color32,
    pub border_width: f32,
}

#[must_use]
pub fn delete_scene_dialog_scene(state: &ScenesState) -> Option<&SceneItemState> {
    if state.show_delete_scene_dialog {
        state.delete_scene_dialog_scene.as_ref()
    } else {
        None
    }
}

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
    let strings = scenes_translations(DEFAULT_LOCALE);

    ui.spacing_mut().item_spacing = vec2(metrics.spacings.spacing_12, metrics.spacings.spacing_12);

    Frame::new()
        .fill(ui.visuals().faint_bg_color)
        .stroke(Stroke::new(
            metrics.strokes.outline,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .corner_radius(CornerRadius::same(
            metrics.corner_radii.border_radius_12 as u8,
        ))
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

    draw_search_bar(ui, state, &mut actions, DEFAULT_LOCALE);

    ui.horizontal(|ui| {
        let add_before = scene_add_before_icon();
        if ui
            .add(MaterialIconButton::standard(add_before.token).svg_data(add_before.svg))
            .on_hover_text(strings.add_before.as_str())
            .clicked()
        {
            actions.push(ScenesAction::InsertScene {
                insert_after: false,
            });
        }
        let add_after = scene_add_after_icon();
        if ui
            .add(MaterialIconButton::standard(add_after.token).svg_data(add_after.svg))
            .on_hover_text(strings.add_after.as_str())
            .clicked()
        {
            actions.push(ScenesAction::InsertScene { insert_after: true });
        }
        let delete_scene = scene_delete_icon();
        if ui
            .add_enabled(
                state.can_delete_scene,
                MaterialIconButton::standard(delete_scene.token).svg_data(delete_scene.svg),
            )
            .on_hover_text(strings.delete_scene_title.as_str())
            .clicked()
        {
            actions.push(ScenesAction::OpenDeleteSceneDialog);
        }
    });

    ui.horizontal(|ui| {
        let open = open_choreography_icon();
        if ui
            .add(MaterialIconButton::standard(open.token).svg_data(open.svg))
            .on_hover_text(strings.open.as_str())
            .clicked()
        {
            actions.push(ScenesAction::RequestOpenChoreography);
        }
        let save = save_choreography_icon();
        if ui
            .add_enabled(
                state.can_save_choreo,
                MaterialIconButton::standard(save.token).svg_data(save.svg),
            )
            .on_hover_text(strings.save.as_str())
            .clicked()
        {
            actions.push(ScenesAction::RequestSaveChoreography);
        }
        let settings = navigate_settings_icon();
        if ui
            .add_enabled(
                state.can_navigate_to_settings,
                MaterialIconButton::standard(settings.token).svg_data(settings.svg),
            )
            .on_hover_text(strings.settings.as_str())
            .clicked()
        {
            actions.push(ScenesAction::NavigateToSettings);
        }
        let dancers = navigate_dancers_icon();
        if ui
            .add_enabled(
                state.can_navigate_to_dancer_settings,
                MaterialIconButton::standard(dancers.token).svg_data(dancers.svg),
            )
            .on_hover_text(strings.dancers.as_str())
            .clicked()
        {
            actions.push(ScenesAction::NavigateToDancerSettings);
        }
    });

    if let Some(dialog_scene) = delete_scene_dialog_scene(state) {
        match draw_delete_scene_dialog(ui, dialog_scene, DEFAULT_LOCALE) {
            Some(DeleteSceneDialogAction::Cancel) => {
                actions.push(ScenesAction::CancelDeleteSceneDialog);
            }
            Some(DeleteSceneDialogAction::ConfirmDelete) => {
                actions.push(ScenesAction::ConfirmDeleteSceneDialog);
            }
            None => {}
        }
    } else if state.show_delete_scene_dialog {
        actions.push(ScenesAction::CancelDeleteSceneDialog);
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
                .unwrap_or(strings.delete_scene_dialog_default_name.as_str());
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
    TypographyRole::BodyMedium
}

#[must_use]
pub const fn scene_timestamp_role() -> TypographyRole {
    TypographyRole::LabelMedium
}

#[must_use]
pub fn scene_add_before_icon() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::ScenesAddBefore)
}

#[must_use]
pub fn scene_add_after_icon() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::ScenesAddAfter)
}

#[must_use]
pub fn scene_delete_icon() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::ScenesDelete)
}

#[must_use]
pub fn open_choreography_icon() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::ScenesOpenChoreography)
}

#[must_use]
pub fn save_choreography_icon() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::ScenesSaveChoreography)
}

#[must_use]
pub fn navigate_settings_icon() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::ScenesNavigateSettings)
}

#[must_use]
pub fn navigate_dancers_icon() -> ui_icons::UiIconSpec {
    ui_icons::icon(UiIconKey::ScenesNavigateDancers)
}

#[must_use]
pub fn build_scene_search_bar_view_model(
    search_text: &str,
    locale: &str,
) -> SceneSearchBarViewModel {
    let strings = scenes_translations(locale);
    SceneSearchBarViewModel {
        placeholder_text: strings.search_placeholder,
        clear_tooltip: strings.common_cancel,
        show_clear_button: !search_text.is_empty(),
    }
}

#[must_use]
pub fn scene_row_height_px(show_timestamps: bool) -> f32 {
    if show_timestamps {
        SCENE_ROW_HEIGHT_WITH_TIMESTAMPS_PX
    } else {
        SCENE_ROW_HEIGHT_PX
    }
}

#[must_use]
pub fn scene_list_item_layout(row_rect: Rect, show_timestamps: bool) -> SceneListItemLayout {
    let content_rect = row_rect.shrink2(vec2(0.0, SCENE_ROW_VERTICAL_GAP_PX));
    let swatch_rect = Rect::from_min_size(
        pos2(
            content_rect.left() + SCENE_ROW_SWATCH_X_PX,
            content_rect.top() + SCENE_ROW_SWATCH_Y_PX,
        ),
        vec2(SCENE_ROW_SWATCH_SIZE_PX, SCENE_ROW_SWATCH_SIZE_PX),
    );
    let accent_rect = Rect::from_min_size(
        content_rect.min,
        vec2(SCENE_ROW_ACCENT_WIDTH_PX, content_rect.height()),
    );
    let text_left = content_rect.left() + SCENE_ROW_TEXT_LEFT_PX;
    let timestamp_y = if show_timestamps {
        content_rect.top() + SCENE_ROW_TIMESTAMP_Y_PX
    } else {
        content_rect.bottom()
    };

    SceneListItemLayout {
        content_rect,
        accent_rect,
        swatch_rect,
        title_position: pos2(text_left, content_rect.top() + SCENE_ROW_TITLE_Y_PX),
        timestamp_position: pos2(text_left, timestamp_y),
    }
}

#[must_use]
pub fn scene_list_item_colors(visuals: &egui::Visuals, is_selected: bool) -> SceneListItemColors {
    let metrics = material_style_metrics();
    let (background, border, title, timestamp, border_width) = if is_selected {
        (
            visuals.selection.bg_fill,
            visuals.selection.stroke.color,
            visuals.strong_text_color(),
            visuals.selection.stroke.color,
            metrics.strokes.focus,
        )
    } else {
        (
            visuals.extreme_bg_color,
            visuals.widgets.noninteractive.bg_stroke.color,
            visuals.text_color(),
            visuals.weak_text_color(),
            metrics.strokes.outline,
        )
    };

    SceneListItemColors {
        background,
        border,
        title,
        timestamp,
        accent: visuals.selection.stroke.color,
        border_width,
    }
}

fn draw_search_bar(
    ui: &mut Ui,
    state: &ScenesState,
    actions: &mut Vec<ScenesAction>,
    locale: &str,
) {
    let metrics = material_style_metrics();
    let view_model = build_scene_search_bar_view_model(&state.search_text, locale);
    let search_icon = scene_search_icon();
    let clear_icon = clear_search_icon();

    // Keep 44px to match the original Material search bar control geometry.
    Frame::new()
        .fill(ui.visuals().widgets.inactive.weak_bg_fill)
        .stroke(Stroke::new(
            metrics.strokes.outline,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .corner_radius(CornerRadius::same(
            metrics.corner_radii.border_radius_12 as u8,
        ))
        .inner_margin(Margin {
            left: metrics.paddings.padding_10 as i8,
            right: metrics.paddings.padding_4 as i8,
            top: 0,
            bottom: 0,
        })
        .show(ui, |ui| {
            ui.set_min_height(SEARCH_BAR_HEIGHT_PX);
            ui.horizontal(|ui| {
                let _ = ui.add_enabled(
                    false,
                    MaterialIconButton::standard(search_icon.0).svg_data(search_icon.1),
                );

                let mut search = state.search_text.clone();
                let text_edit_width = if view_model.show_clear_button {
                    (ui.available_width() - metrics.sizes.size_40).max(0.0)
                } else {
                    ui.available_width()
                };
                let changed = ui
                    .add_sized(
                        vec2(text_edit_width, SEARCH_BAR_HEIGHT_PX - metrics.sizes.size_2),
                        egui::TextEdit::singleline(&mut search)
                            .frame(false)
                            .hint_text(view_model.placeholder_text.as_str()),
                    )
                    .changed();
                if changed {
                    actions.push(ScenesAction::UpdateSearchText(search));
                }

                if view_model.show_clear_button
                    && ui
                        .add(MaterialIconButton::standard(clear_icon.0).svg_data(clear_icon.1))
                        .on_hover_text(view_model.clear_tooltip)
                        .clicked()
                {
                    actions.push(ScenesAction::UpdateSearchText(String::new()));
                }
            });
        });
}

fn draw_scene_list_item(ui: &mut Ui, scene: &SceneItemState, show_timestamps: bool) -> Response {
    let (row_rect, response) = ui.allocate_exact_size(
        vec2(ui.available_width(), scene_row_height_px(show_timestamps)),
        Sense::click(),
    );
    if !ui.is_rect_visible(row_rect) {
        return response;
    }

    let visuals = ui.style().visuals.clone();
    let layout = scene_list_item_layout(row_rect, show_timestamps);
    let colors = scene_list_item_colors(&visuals, scene.is_selected);

    ui.painter().rect(
        layout.content_rect,
        CornerRadius::same(material_style_metrics().sizes.size_6 as u8),
        colors.background,
        Stroke::new(colors.border_width, colors.border),
        egui::StrokeKind::Middle,
    );

    if scene.is_selected {
        ui.painter().rect_filled(
            layout.accent_rect,
            CornerRadius::same(material_style_metrics().sizes.size_6 as u8),
            colors.accent,
        );
    }

    ui.painter().rect_filled(
        layout.swatch_rect,
        CornerRadius::same(SCENE_ROW_SWATCH_CORNER_RADIUS_PX),
        egui::Color32::from_rgba_unmultiplied(
            scene.color.r,
            scene.color.g,
            scene.color.b,
            scene.color.a,
        ),
    );

    ui.painter().text(
        layout.title_position,
        egui::Align2::LEFT_TOP,
        scene.name.as_str(),
        typography::font_id_for_role(scene_title_role()),
        colors.title,
    );

    if show_timestamps {
        let timestamp_text = scene.timestamp.map(format_seconds).unwrap_or_default();
        ui.painter().text(
            layout.timestamp_position,
            egui::Align2::LEFT_TOP,
            timestamp_text,
            typography::font_id_for_role(scene_timestamp_role()),
            colors.timestamp,
        );
    }

    response
}

fn scene_search_icon() -> (&'static str, &'static str) {
    (
        "search",
        include_str!("../../../choreo_components/ui/icons/Magnify.svg"),
    )
}

fn clear_search_icon() -> (&'static str, &'static str) {
    (
        "close",
        include_str!("../../../choreo_components/ui/icons/Close.svg"),
    )
}
