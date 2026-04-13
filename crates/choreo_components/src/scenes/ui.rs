use egui::Button;
use egui::CornerRadius;
use egui::Frame;
use egui::Image;
use egui::Margin;
use egui::Response;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::vec2;

use crate::delete_scene_dialog::ui::DeleteSceneDialogAction;
use crate::delete_scene_dialog::ui::draw_delete_scene_dialog;
use crate::material::components::MaterialScrollArea;
use crate::material::components::centered_icon_rect;
use crate::material::components::paint_icon;
use crate::material::components::top_bar_icon::top_bar_icon_button_enabled;
use crate::material::styling::material_palette::MaterialPalette;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::scene_list_item;
use crate::scene_list_item::SceneItemState;

use super::actions::ScenesAction;
use super::state::ScenesState;
use super::translations::scenes_translations;
use super::ui_icons;
use super::ui_icons::UiIconKey;

const DEFAULT_LOCALE: &str = "en";
const SEARCH_BAR_HEIGHT_PX: f32 = 44.0;
const SEARCH_BAR_ICON_BUTTON_SIZE_PX: f32 = 24.0;
const TOOLBAR_ROW_HEIGHT_PX: f32 = 48.0;
const TOOLBAR_ICON_GLYPH_SIZE_PX: f32 = 24.0;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SceneSearchBarViewModel {
    pub placeholder_text: String,
    pub clear_tooltip: String,
    pub show_clear_button: bool,
}

pub use crate::scene_list_item::layout_for_row_rect as scene_list_item_layout;
pub use crate::scene_list_item::row_height_px as scene_row_height_px;
pub use crate::scene_list_item::timestamp_role as scene_timestamp_role;
pub use crate::scene_list_item::title_role as scene_title_role;

#[must_use]
pub fn scene_list_item_colors(
    palette: MaterialPalette,
    is_selected: bool,
) -> crate::scene_list_item::SceneListItemColors {
    crate::scene_list_item::colors_for_selection(palette, is_selected)
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
    let panel_width = ui.available_width().max(0.0);
    let palette = material_palette_for_visuals(ui.visuals());

    ui.spacing_mut().item_spacing = vec2(metrics.spacings.spacing_12, metrics.spacings.spacing_12);
    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        draw_fixed_height_section(ui, panel_width, TOOLBAR_ROW_HEIGHT_PX, |ui| {
            draw_navigation_toolbar_row(ui, state, &mut actions);
        });
        draw_fixed_height_section(ui, panel_width, TOOLBAR_ROW_HEIGHT_PX, |ui| {
            draw_edit_toolbar_row(ui, state, &mut actions);
        });
        draw_fixed_height_section(ui, panel_width, SEARCH_BAR_HEIGHT_PX, |ui| {
            draw_search_bar(ui, panel_width, state, &mut actions, DEFAULT_LOCALE);
        });

        ui.allocate_ui_with_layout(
            vec2(panel_width, ui.available_height().max(0.0)),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                ui.set_min_width(panel_width);
                Frame::new()
                    .fill(palette.surface_container_low)
                    .stroke(Stroke::new(
                        metrics.strokes.outline,
                        palette.outline_variant,
                    ))
                    .corner_radius(CornerRadius::same(
                        metrics.corner_radii.border_radius_12 as u8,
                    ))
                    .inner_margin(Margin::same(metrics.paddings.padding_12 as i8))
                    .show(ui, |ui| {
                        ui.set_min_height(ui.available_height());
                        MaterialScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                for (index, scene) in state.visible_scenes.iter().enumerate() {
                                    if scene_list_item::draw(ui, scene, state.show_timestamps)
                                        .clicked()
                                    {
                                        actions.push(ScenesAction::SelectScene { index });
                                    }
                                }
                            });
                    });
            },
        );
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
                .filter(|name: &&str| !name.trim().is_empty())
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
pub const fn scene_toolbar_button_stroke_width_px() -> f32 {
    0.0
}

#[must_use]
pub fn scene_pane_controls_height(spacing_px: f32, toolbar_row_height_px: f32) -> f32 {
    SEARCH_BAR_HEIGHT_PX + (toolbar_row_height_px * 2.0) + (spacing_px * 3.0)
}

#[must_use]
pub fn scene_list_panel_height(
    available_height: f32,
    spacing_px: f32,
    toolbar_row_height_px: f32,
) -> f32 {
    (available_height - scene_pane_controls_height(spacing_px, toolbar_row_height_px)).max(0.0)
}

#[must_use]
pub fn scene_search_bar_text_edit_width(
    available_width: f32,
    spacing_px: f32,
    icon_slot_width_px: f32,
) -> f32 {
    (available_width - (icon_slot_width_px * 2.0) - (spacing_px * 2.0)).max(0.0)
}

#[must_use]
pub fn scene_search_bar_content_width(
    panel_width: f32,
    left_padding_px: f32,
    right_padding_px: f32,
) -> f32 {
    (panel_width - left_padding_px - right_padding_px).max(0.0)
}

fn draw_search_bar(
    ui: &mut Ui,
    available_width: f32,
    state: &ScenesState,
    actions: &mut Vec<ScenesAction>,
    locale: &str,
) {
    let metrics = material_style_metrics();
    let horizontal_spacing = metrics.spacings.spacing_12;
    let left_padding = metrics.paddings.padding_10;
    let right_padding = metrics.paddings.padding_4;
    let palette = material_palette_for_visuals(ui.visuals());
    let view_model = build_scene_search_bar_view_model(&state.search_text, locale);
    let clear_icon = clear_search_icon();
    let content_width =
        scene_search_bar_content_width(available_width, left_padding, right_padding);

    // Keep 44px to match the original Material search bar control geometry.
    Frame::new()
        .fill(palette.surface_container)
        .stroke(Stroke::new(
            metrics.strokes.outline,
            palette.outline_variant,
        ))
        .corner_radius(CornerRadius::same(
            metrics.corner_radii.border_radius_12 as u8,
        ))
        .inner_margin(Margin {
            left: left_padding as i8,
            right: right_padding as i8,
            top: 0,
            bottom: 0,
        })
        .show(ui, |ui| {
            let text_edit_width = scene_search_bar_text_edit_width(
                content_width,
                horizontal_spacing,
                SEARCH_BAR_ICON_BUTTON_SIZE_PX,
            );
            ui.spacing_mut().item_spacing.x = horizontal_spacing;
            ui.allocate_ui_with_layout(
                vec2(content_width, SEARCH_BAR_HEIGHT_PX),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.set_min_width(content_width);
                    ui.set_min_height(SEARCH_BAR_HEIGHT_PX);
                    let _ = ui.add_sized(
                        vec2(
                            SEARCH_BAR_ICON_BUTTON_SIZE_PX,
                            SEARCH_BAR_ICON_BUTTON_SIZE_PX,
                        ),
                        search_icon_image(),
                    );

                    let mut search = state.search_text.clone();
                    let changed = ui
                        .add_sized(
                            vec2(text_edit_width, ui.spacing().interact_size.y),
                            egui::TextEdit::singleline(&mut search)
                                .frame(false)
                                .hint_text(view_model.placeholder_text.as_str()),
                        )
                        .changed();
                    if changed {
                        actions.push(ScenesAction::UpdateSearchText(search));
                    }

                    if view_model.show_clear_button {
                        if add_search_bar_clear_button(ui, clear_icon.0, clear_icon.1)
                            .on_hover_text(view_model.clear_tooltip)
                            .clicked()
                        {
                            actions.push(ScenesAction::UpdateSearchText(String::new()));
                        }
                    } else {
                        let _ = ui.allocate_exact_size(
                            vec2(
                                SEARCH_BAR_ICON_BUTTON_SIZE_PX,
                                SEARCH_BAR_ICON_BUTTON_SIZE_PX,
                            ),
                            Sense::hover(),
                        );
                    }
                },
            );
        });
}

fn draw_edit_toolbar_row(ui: &mut Ui, state: &ScenesState, actions: &mut Vec<ScenesAction>) {
    let strings = scenes_translations(DEFAULT_LOCALE);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = material_style_metrics().spacings.spacing_12;
        let add_before = scene_add_before_icon();
        if add_scene_icon_button(ui, add_before, true)
            .on_hover_text(strings.add_before.as_str())
            .clicked()
        {
            actions.push(ScenesAction::InsertScene {
                insert_after: false,
            });
        }
        let add_after = scene_add_after_icon();
        if add_scene_icon_button(ui, add_after, true)
            .on_hover_text(strings.add_after.as_str())
            .clicked()
        {
            actions.push(ScenesAction::InsertScene { insert_after: true });
        }
        let delete_scene = scene_delete_icon();
        if add_scene_icon_button(ui, delete_scene, state.can_delete_scene)
            .on_hover_text(strings.delete_scene_title.as_str())
            .clicked()
        {
            actions.push(ScenesAction::OpenDeleteSceneDialog);
        }
    });
}

fn draw_navigation_toolbar_row(ui: &mut Ui, state: &ScenesState, actions: &mut Vec<ScenesAction>) {
    let strings = scenes_translations(DEFAULT_LOCALE);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = material_style_metrics().spacings.spacing_12;
        let open = open_choreography_icon();
        if add_scene_icon_button(ui, open, true)
            .on_hover_text(strings.open.as_str())
            .clicked()
        {
            actions.push(ScenesAction::RequestOpenChoreography);
        }
        let save = save_choreography_icon();
        if add_scene_icon_button(ui, save, state.can_save_choreo)
            .on_hover_text(strings.save.as_str())
            .clicked()
        {
            actions.push(ScenesAction::RequestSaveChoreography);
        }
        let settings = navigate_settings_icon();
        if add_scene_icon_button(ui, settings, state.can_navigate_to_settings)
            .on_hover_text(strings.settings.as_str())
            .clicked()
        {
            actions.push(ScenesAction::NavigateToSettings);
        }
        let dancers = navigate_dancers_icon();
        if add_scene_icon_button(ui, dancers, state.can_navigate_to_dancer_settings)
            .on_hover_text(strings.dancers.as_str())
            .clicked()
        {
            actions.push(ScenesAction::NavigateToDancerSettings);
        }
    });
}

fn scene_search_icon() -> (&'static str, &'static str) {
    ("search", include_str!("../../assets/icons/Magnify.svg"))
}

fn clear_search_icon() -> (&'static str, &'static str) {
    ("close", include_str!("../../assets/icons/Close.svg"))
}

fn add_scene_icon_button(ui: &mut Ui, icon: ui_icons::UiIconSpec, enabled: bool) -> Response {
    let image = Image::from_bytes(scene_icon_uri(icon.token), icon.svg.as_bytes());
    top_bar_icon_button_enabled(ui, image, false, enabled)
}

fn search_icon_image() -> Image<'static> {
    let search_icon = scene_search_icon();
    Image::from_bytes(scene_icon_uri(search_icon.0), search_icon.1.as_bytes())
        .fit_to_exact_size(vec2(TOOLBAR_ICON_GLYPH_SIZE_PX, TOOLBAR_ICON_GLYPH_SIZE_PX))
}

fn search_bar_clear_button(token: &'static str, svg: &'static str) -> Button<'static> {
    let _ = (token, svg);
    Button::new("").frame(false).min_size(vec2(
        SEARCH_BAR_ICON_BUTTON_SIZE_PX,
        SEARCH_BAR_ICON_BUTTON_SIZE_PX,
    ))
}

fn add_search_bar_clear_button(ui: &mut Ui, token: &'static str, svg: &'static str) -> Response {
    let image = Image::from_bytes(scene_icon_uri(token), svg.as_bytes());
    let response = ui.add(search_bar_clear_button(token, svg));
    let palette = material_palette_for_visuals(ui.visuals());
    let tint = if response.hovered() || response.has_focus() {
        palette.on_surface
    } else {
        palette.on_surface_variant
    };
    paint_icon(
        ui,
        &image,
        centered_icon_rect(
            response.rect,
            vec2(TOOLBAR_ICON_GLYPH_SIZE_PX, TOOLBAR_ICON_GLYPH_SIZE_PX),
        ),
        tint,
    );
    response
}

fn scene_icon_uri(token: &str) -> String {
    format!("bytes://scenes/{token}.svg")
}

fn draw_fixed_height_section(
    ui: &mut Ui,
    width_px: f32,
    height_px: f32,
    add_contents: impl FnOnce(&mut Ui),
) {
    ui.allocate_ui_with_layout(
        vec2(width_px, height_px),
        egui::Layout::top_down(egui::Align::LEFT),
        |ui| {
            ui.set_min_width(width_px);
            add_contents(ui);
        },
    );
}
