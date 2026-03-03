use choreo_master_mobile_json::Color;
use egui::Align2;
use egui::Color32;
use egui::CornerRadius;
use egui::FontId;
use egui::Frame;
use egui::Margin;
use egui::RichText;
use egui::ScrollArea;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::pos2;
use egui::vec2;
use egui_material3::MaterialButton;

use super::actions::DancersAction;
use super::state::DancerState;
use super::state::DancersState;
use crate::i18n::t;

pub fn draw(ui: &mut Ui, state: &DancersState) -> Vec<DancersAction> {
    let mut actions: Vec<DancersAction> = Vec::new();
    const GRID: f32 = 12.0;
    let locale = "en";

    ui.heading(t(locale, "DancersTitle", "Dancers"));
    ui.horizontal(|ui| {
        if ui
            .add_sized(
                [3.0 * GRID, 3.0 * GRID],
                MaterialButton::new(RichText::new("+").size(18.0)),
            )
            .on_hover_text(t(locale, "DancersAddTooltip", "+"))
            .clicked()
        {
            actions.push(DancersAction::AddDancer);
        }
        if ui
            .add_enabled(
                state.can_delete_dancer,
                MaterialButton::new(RichText::new("-").size(18.0)),
            )
            .on_hover_text(t(locale, "DancersDeleteTooltip", "-"))
            .clicked()
        {
            actions.push(DancersAction::DeleteSelectedDancer);
        }
    });

    ui.columns(2, |columns| {
        Frame::new()
            .fill(columns[0].visuals().faint_bg_color)
            .stroke(Stroke::new(
                1.0,
                columns[0].visuals().widgets.noninteractive.bg_stroke.color,
            ))
            .corner_radius(CornerRadius::same(12))
            .inner_margin(Margin::same(8))
            .show(&mut columns[0], |ui| {
                ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (index, dancer) in state.dancers.iter().enumerate() {
                            let is_selected = selected_dancer_index(state) == Some(index);
                            if draw_dancer_list_item_view(ui, dancer, is_selected).clicked() {
                                actions.push(DancersAction::SelectDancer { index });
                            }
                        }
                    });
            });

        draw_settings_panel(&mut columns[1], state, &mut actions, locale);
    });

    draw_swap_dialog(ui, state, &mut actions, locale);

    actions
}

fn draw_settings_panel(
    ui: &mut Ui,
    state: &DancersState,
    actions: &mut Vec<DancersAction>,
    locale: &str,
) {
    ui.heading(t(locale, "DancerTitle", "Dancer"));

    let selected_role = selected_role_index(state).unwrap_or(0);
    let mut selected_role_mut = selected_role;
    egui::ComboBox::from_label(t(locale, "DancerRoleLabel", "Role"))
        .selected_text(
            state
                .roles
                .get(selected_role_mut)
                .map(|role| role.name.clone())
                .unwrap_or_default(),
        )
        .show_ui(ui, |ui| {
            for (index, role) in state.roles.iter().enumerate() {
                ui.selectable_value(&mut selected_role_mut, index, role.name.as_str());
            }
        });
    if selected_role_mut != selected_role {
        actions.push(DancersAction::SelectRole {
            index: selected_role_mut,
        });
    }

    let mut name = state
        .selected_dancer
        .as_ref()
        .map(|dancer| dancer.name.clone())
        .unwrap_or_default();
    if ui
        .add_enabled(
            state.has_selected_dancer,
            egui::TextEdit::singleline(&mut name),
        )
        .changed()
    {
        actions.push(DancersAction::UpdateDancerName { value: name });
    }
    ui.label(t(locale, "DancerNameLabel", "Name"));

    let mut shortcut = state
        .selected_dancer
        .as_ref()
        .map(|dancer| dancer.shortcut.clone())
        .unwrap_or_default();
    if ui
        .add_enabled(
            state.has_selected_dancer,
            egui::TextEdit::singleline(&mut shortcut),
        )
        .changed()
    {
        actions.push(DancersAction::UpdateDancerShortcut { value: shortcut });
    }
    ui.label(t(locale, "DancerShortcutLabel", "Shortcut"));

    let mut selected_icon = selected_icon_index(state).unwrap_or(0);
    let selected_icon_label = state
        .icon_options
        .get(selected_icon)
        .map(|option| option.display_name.clone())
        .unwrap_or_default();
    egui::ComboBox::from_label(t(locale, "DancerIconLabel", "Icon"))
        .selected_text(selected_icon_label)
        .show_ui(ui, |ui| {
            for (index, option) in state.icon_options.iter().enumerate() {
                ui.selectable_value(&mut selected_icon, index, option.display_name.as_str());
            }
        });
    if let Some(option) = state.icon_options.get(selected_icon)
        && state
            .selected_icon_option
            .as_ref()
            .map(|value| value.key.as_str())
            != Some(option.key.as_str())
    {
        actions.push(DancersAction::UpdateDancerIcon {
            value: option.icon_name.clone(),
        });
    }

    ui.label(t(locale, "DancerColorLabel", "Color"));
    let mut color = state
        .selected_dancer
        .as_ref()
        .map(|dancer| dancer.color.clone())
        .unwrap_or_else(super::state::transparent_color);
    ui.add_enabled_ui(state.has_selected_dancer, |ui| {
        let mut color32 = color_to_egui(&color);
        if ui.color_edit_button_srgba(&mut color32).changed() {
            color = Color {
                r: color32.r(),
                g: color32.g(),
                b: color32.b(),
                a: color32.a(),
            };
            actions.push(DancersAction::UpdateDancerColor { value: color });
        }
    });

    let from_index = state
        .swap_from_dancer
        .as_ref()
        .and_then(|from| {
            state
                .dancers
                .iter()
                .position(|dancer| dancer.dancer_id == from.dancer_id)
        })
        .unwrap_or(0);
    let mut from_index_mut = from_index;
    egui::ComboBox::from_label(t(locale, "DancerSwapFromLabel", "Swap from"))
        .selected_text(dancer_name_for_index(state, from_index_mut))
        .show_ui(ui, |ui| {
            for (index, dancer) in state.dancers.iter().enumerate() {
                ui.selectable_value(&mut from_index_mut, index, dancer.name.as_str());
            }
        });
    if from_index_mut != from_index {
        actions.push(DancersAction::UpdateSwapFrom {
            index: from_index_mut,
        });
    }

    let to_index = state
        .swap_to_dancer
        .as_ref()
        .and_then(|to| {
            state
                .dancers
                .iter()
                .position(|dancer| dancer.dancer_id == to.dancer_id)
        })
        .unwrap_or(0);
    let mut to_index_mut = to_index;
    egui::ComboBox::from_label(t(locale, "DancerSwapToLabel", "Swap to"))
        .selected_text(dancer_name_for_index(state, to_index_mut))
        .show_ui(ui, |ui| {
            for (index, dancer) in state.dancers.iter().enumerate() {
                ui.selectable_value(&mut to_index_mut, index, dancer.name.as_str());
            }
        });
    if to_index_mut != to_index {
        actions.push(DancersAction::UpdateSwapTo {
            index: to_index_mut,
        });
    }

    if ui
        .add_enabled(
            state.can_swap_dancers,
            MaterialButton::new(t(locale, "DancerSwapButton", "Swap")),
        )
        .clicked()
    {
        actions.push(DancersAction::RequestSwapDancers);
    }

    ui.horizontal(|ui| {
        if ui
            .add(MaterialButton::new(t(locale, "CommonCancel", "Cancel")))
            .clicked()
        {
            actions.push(DancersAction::Cancel);
        }
        if ui
            .add(MaterialButton::new(t(locale, "CommonOk", "OK")))
            .clicked()
        {
            actions.push(DancersAction::SaveToGlobal);
        }
    });
}

fn draw_swap_dialog(
    ui: &mut Ui,
    state: &DancersState,
    actions: &mut Vec<DancersAction>,
    locale: &str,
) {
    if !state.is_dialog_open {
        return;
    }

    egui::Window::new(t(locale, "DancerSwapDialogTitle", "Swap dancers"))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(ui.ctx(), |ui| {
            ui.label(state.dialog_content.clone().unwrap_or_default());
            ui.horizontal(|ui| {
                if ui
                    .add(MaterialButton::new(t(
                        locale,
                        "DancerSwapDialogCancel",
                        "Cancel",
                    )))
                    .clicked()
                {
                    actions.push(DancersAction::Cancel);
                }
                if ui
                    .add(MaterialButton::new(t(
                        locale,
                        "DancerSwapDialogConfirm",
                        "Swap",
                    )))
                    .clicked()
                {
                    actions.push(DancersAction::ConfirmSwapDancers);
                }
            });
        });
}

fn dancer_name_for_index(state: &DancersState, index: usize) -> String {
    state
        .dancers
        .get(index)
        .map(|dancer| dancer.name.clone())
        .unwrap_or_default()
}

#[must_use]
pub fn selected_dancer_index(state: &DancersState) -> Option<usize> {
    let selected_dancer_id = state
        .selected_dancer
        .as_ref()
        .map(|dancer| dancer.dancer_id)?;
    state
        .dancers
        .iter()
        .position(|dancer| dancer.dancer_id == selected_dancer_id)
}

#[must_use]
pub fn selected_role_index(state: &DancersState) -> Option<usize> {
    let selected_role_name = state
        .selected_role
        .as_ref()
        .map(|role| role.name.as_str())?;
    state
        .roles
        .iter()
        .position(|role| role.name == selected_role_name)
}

#[must_use]
pub fn selected_icon_index(state: &DancersState) -> Option<usize> {
    let selected_key = state
        .selected_icon_option
        .as_ref()
        .map(|option| option.key.as_str())?;
    state
        .icon_options
        .iter()
        .position(|option| option.key == selected_key)
}

#[must_use]
pub fn dancer_supporting_text(dancer: &DancerState) -> String {
    format!(
        "{} ({})  [{}]",
        dancer.role.name, dancer.role.z_index, dancer.shortcut
    )
}

#[must_use]
pub fn dancer_role_details_text(dancer: &DancerState) -> String {
    dancer_supporting_text(dancer)
}

fn draw_dancer_list_item_view(
    ui: &mut Ui,
    dancer: &DancerState,
    is_selected: bool,
) -> egui::Response {
    // Slint row height is 56px; keep this for visual parity with the source list item view.
    let row_size = vec2(ui.available_width(), 56.0);
    let (row_rect, response) = ui.allocate_exact_size(row_size, Sense::click());
    if !ui.is_rect_visible(row_rect) {
        return response;
    }

    let visuals = ui.style().visuals.clone();
    let background = if is_selected {
        visuals.selection.bg_fill
    } else {
        visuals.extreme_bg_color
    };
    ui.painter()
        .rect_filled(row_rect.shrink2(vec2(0.0, 3.0)), 8.0, background);

    let content_rect = row_rect.shrink2(vec2(0.0, 3.0));
    let swatch_rect = egui::Rect::from_min_size(
        pos2(content_rect.left() + 10.0, content_rect.center().y - 14.0),
        vec2(28.0, 28.0),
    );
    ui.painter()
        .rect_filled(swatch_rect, 6.0, color_to_egui(&dancer.color));

    let title_color = if is_selected {
        visuals.selection.stroke.color
    } else {
        visuals.text_color()
    };
    let subtitle_color = visuals.weak_text_color();
    let text_left = content_rect.left() + 46.0;

    ui.painter().text(
        pos2(text_left, content_rect.top() + 8.0),
        Align2::LEFT_TOP,
        dancer.name.as_str(),
        FontId::proportional(14.0),
        title_color,
    );
    ui.painter().text(
        pos2(text_left, content_rect.top() + 28.0),
        Align2::LEFT_TOP,
        dancer_supporting_text(dancer),
        FontId::proportional(12.0),
        subtitle_color,
    );

    response
}

fn color_to_egui(color: &Color) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r, color.g, color.b, color.a)
}
