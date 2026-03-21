use choreo_master_mobile_json::Color;
use egui::Align;
use egui::Area;
use egui::Color32;
use egui::CornerRadius;
use egui::Frame;
use egui::Id;
use egui::Layout;
use egui::Margin;
use egui::Order;
use egui::Rect;
use egui::RichText;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::UiBuilder;
use egui::pos2;
use egui::vec2;
use egui_material3::MaterialButton;

use crate::color_picker::ui as color_picker_ui;
use crate::dancers_pane_view::ui as dancers_pane_view;
use crate::dancers_pane_view::ui::DancersPaneViewAction;
use crate::dialog_host::ui::DialogHostProps;
use crate::dialog_host::ui::dialog_metrics_tokens;
use crate::dialog_host::ui::draw_dialog_host_with_panel;
use crate::drawer_host::actions::DrawerHostAction;
use crate::drawer_host::state::DrawerHostOpenMode;
use crate::drawer_host::state::DrawerHostState;
use crate::drawer_host::ui::draw_with_slots_in_rect;
use crate::i18n::t;
use crate::material::components;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography as typography;
use crate::material::styling::material_typography::TypographyRole;
use crate::nav_bar::hamburger_toggle_button;

use super::action::DancerSettingsPageAction;
use super::action::SwapDialogAction;
use super::reducer::map_swap_dialog_action;
use super::state::DancerSettingsPageState;
use super::state::build_swap_dialog_view_model;
use super::state::dancer_option_labels;
use super::state::icon_option_labels;
use super::state::role_option_labels;
use super::state::selected_dancer_color_picker_state;
use super::state::selected_dancer_index;
use super::state::selected_icon_index;
use super::state::selected_role_index;

const TOP_BAR_HEIGHT_PX: f32 = 64.0;
// Slint source control uses 420px for the drawer width.
const LIST_DRAWER_WIDTH_PX: f32 = 420.0;
const CONTENT_MAX_WIDTH_PX: f32 = 720.0;
const CONTENT_OUTER_MARGIN_PX: f32 = 16.0;
const FOOTER_HEIGHT_PX: f32 = 56.0;
const FOOTER_PADDING_PX: i8 = 8;
const SWAP_DANCERS_DIALOG_ID: &str = "swap_dancers";

#[must_use]
pub const fn content_spacing_token() -> f32 {
    material_style_metrics().spacings.spacing_12
}

#[must_use]
pub const fn card_corner_radius_token() -> f32 {
    material_style_metrics().corner_radii.border_radius_12
}

#[must_use]
pub const fn top_bar_height_token() -> f32 {
    TOP_BAR_HEIGHT_PX
}

#[must_use]
pub const fn content_max_width_token() -> f32 {
    CONTENT_MAX_WIDTH_PX
}

#[must_use]
pub const fn content_outer_margin_token() -> f32 {
    CONTENT_OUTER_MARGIN_PX
}

#[must_use]
pub const fn footer_height_token() -> f32 {
    FOOTER_HEIGHT_PX
}

#[must_use]
pub const fn dropdown_height_token() -> f32 {
    60.0
}

#[must_use]
pub const fn uses_scrollable_content_shell() -> bool {
    true
}

pub fn draw(ui: &mut Ui, state: &DancerSettingsPageState) -> Vec<DancerSettingsPageAction> {
    let mut page_actions: Vec<DancerSettingsPageAction> = Vec::new();
    let mut dialog_action = None;
    let locale = "en";
    let dialog_metrics = dialog_metrics_tokens();

    let close_requested = draw_dialog_host_with_panel(
        ui,
        &DialogHostProps {
            id_source: "dancer_settings_page_dialog_host",
            is_open: state.is_dialog_open,
            close_on_click_away: true,
            overlay_color: ui.visuals().window_fill().linear_multiply(0.7),
            dialog_background: ui.visuals().widgets.noninteractive.bg_fill,
            dialog_text_color: ui.visuals().text_color(),
            dialog_padding: dialog_metrics.dialog_padding,
            dialog_margin: dialog_metrics.dialog_margin,
            dialog_corner_radius: dialog_metrics.dialog_corner_radius,
            dialog_content: state.dialog_content.as_deref().unwrap_or_default(),
        },
        |ui| {
            let page_rect = shell_rect(ui);
            let top_bar_rect = top_bar_rect(page_rect);

            Area::new(Id::new("dancer_settings_page_top_bar"))
                .order(Order::Foreground)
                .fixed_pos(top_bar_rect.min)
                .show(ui.ctx(), |ui| {
                    let local_rect = Rect::from_min_size(egui::Pos2::ZERO, top_bar_rect.size());
                    ui.painter()
                        .rect_filled(local_rect, 0.0, ui.visuals().panel_fill);
                    let _ = ui.scope_builder(UiBuilder::new().max_rect(local_rect), |ui| {
                        draw_top_bar(ui, state, &mut page_actions, locale);
                    });
                });

            let _ = ui.scope_builder(UiBuilder::new().max_rect(page_rect), |ui| {
                draw_main_content(ui, page_rect, state, &mut page_actions, locale);
            });
            let _ = ui.allocate_rect(page_rect, Sense::hover());
        },
        |ui| {
            dialog_action = draw_dialog_panel(ui, state, locale);
        },
    );

    if let Some(dialog_action) = dialog_action {
        page_actions.push(map_swap_dialog_action(dialog_action));
    }

    if close_requested {
        page_actions.push(DancerSettingsPageAction::DismissDialog);
    }

    page_actions
}

#[must_use]
pub fn shell_rect(ui: &Ui) -> Rect {
    ui.max_rect()
}

#[must_use]
pub fn top_bar_rect(page_rect: Rect) -> Rect {
    Rect::from_min_size(
        page_rect.min,
        vec2(page_rect.width(), top_bar_height_token()),
    )
}

#[must_use]
pub fn main_content_rect(page_rect: Rect) -> Rect {
    Rect::from_min_max(
        pos2(page_rect.left(), top_bar_rect(page_rect).bottom()),
        page_rect.right_bottom(),
    )
}

fn draw_dialog_panel(
    ui: &mut Ui,
    state: &DancerSettingsPageState,
    locale: &str,
) -> Option<SwapDialogAction> {
    if state.dialog_content.as_deref() == Some(SWAP_DANCERS_DIALOG_ID) {
        return draw_swap_dialog_panel(ui, state, locale);
    }

    ui.label(state.dialog_content.as_deref().unwrap_or_default());
    None
}

fn draw_main_content(
    ui: &mut Ui,
    content_rect: Rect,
    state: &DancerSettingsPageState,
    actions: &mut Vec<DancerSettingsPageAction>,
    locale: &str,
) {
    let drawer_state = drawer_host_state(
        state,
        ui.visuals().window_fill().linear_multiply(0.7),
        ui.visuals().panel_fill,
    );

    let mut content_actions: Vec<DancerSettingsPageAction> = Vec::new();
    let mut pane_actions: Vec<DancerSettingsPageAction> = Vec::new();
    let drawer_actions = draw_with_slots_in_rect(
        ui.ctx(),
        content_rect,
        "dancer_settings_page",
        &drawer_state,
        |ui| draw_content(ui, state, &mut content_actions, locale),
        |ui| draw_dancers_pane(ui, state, &mut pane_actions, locale),
        |_| {},
        |_| {},
        |_| {},
    );
    let _ = ui.allocate_rect(content_rect, Sense::hover());
    actions.extend(pane_actions);
    actions.extend(content_actions);

    for drawer_action in drawer_actions {
        let DrawerHostAction::OverlayClicked { close_left, .. } = drawer_action;
        if close_left {
            actions.push(DancerSettingsPageAction::CloseDancerList);
        }
    }
}

#[must_use]
pub fn drawer_host_state(
    state: &DancerSettingsPageState,
    overlay_color: Color32,
    drawer_background: Color32,
) -> DrawerHostState {
    DrawerHostState {
        left_drawer_width: LIST_DRAWER_WIDTH_PX,
        responsive_breakpoint: 900.0,
        open_mode: DrawerHostOpenMode::Modal,
        top_inset: top_bar_height_token(),
        inline_left: false,
        is_left_open: state.is_dancer_list_open,
        is_right_open: false,
        is_top_open: false,
        is_bottom_open: false,
        left_close_on_click_away: true,
        right_close_on_click_away: true,
        top_close_on_click_away: true,
        bottom_close_on_click_away: true,
        overlay_color,
        drawer_background,
        ..DrawerHostState::default()
    }
}

fn draw_top_bar(
    ui: &mut Ui,
    state: &DancerSettingsPageState,
    actions: &mut Vec<DancerSettingsPageAction>,
    locale: &str,
) {
    let title = t(locale, "DancersTitle");
    ui.set_min_height(TOP_BAR_HEIGHT_PX);
    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
        ui.add_space(content_spacing_token());
        let response = hamburger_toggle_button::draw(
            ui,
            state.is_dancer_list_open,
            true,
            &title,
            Some(vec2(48.0, 48.0)),
        );
        if response.clicked() {
            actions.push(DancerSettingsPageAction::ToggleDancerList);
        }
        ui.add_space(content_spacing_token());
        ui.label(typography::rich_text_for_role(title, top_bar_title_role()));
    });
}

#[must_use]
pub fn content_top_inset_token() -> f32 {
    content_outer_margin_token()
}

#[must_use]
pub fn footer_content_padding_token() -> f32 {
    f32::from(FOOTER_PADDING_PX)
}

#[must_use]
pub fn footer_inner_height_token() -> f32 {
    footer_height_token() - (footer_content_padding_token() * 2.0)
}

#[must_use]
pub fn content_column_width(surface_rect: Rect) -> f32 {
    let max_content_width = (surface_rect.width() - (content_outer_margin_token() * 2.0)).max(0.0);
    content_max_width_token().min(max_content_width)
}

#[must_use]
pub fn content_column_left(surface_rect: Rect) -> f32 {
    surface_rect.left() + content_outer_margin_token()
}

#[must_use]
pub fn content_column_right(surface_rect: Rect) -> f32 {
    content_column_left(surface_rect) + content_column_width(surface_rect)
}

fn draw_footer(ui: &mut Ui, actions: &mut Vec<DancerSettingsPageAction>, locale: &str) {
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, ui.visuals().window_fill);
    ui.set_width(ui.max_rect().width());
    ui.set_min_height(footer_height_token());
    ui.spacing_mut().item_spacing = vec2(
        footer_content_padding_token(),
        footer_content_padding_token(),
    );
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        ui.add_space(footer_content_padding_token());
        if ui.add(MaterialButton::new(t(locale, "CommonOk"))).clicked() {
            actions.push(DancerSettingsPageAction::SavePage);
        }
        if ui
            .add(MaterialButton::new(t(locale, "CommonCancel")))
            .clicked()
        {
            actions.push(DancerSettingsPageAction::CancelPage);
        }
        ui.add_space(footer_content_padding_token());
    });
}

fn draw_content(
    ui: &mut Ui,
    state: &DancerSettingsPageState,
    actions: &mut Vec<DancerSettingsPageAction>,
    locale: &str,
) {
    let surface_rect = main_content_rect(ui.max_rect());
    ui.painter().rect_filled(
        surface_rect,
        CornerRadius::ZERO,
        ui.visuals().faint_bg_color,
    );

    let footer_rect = footer_rect(surface_rect);
    let scroll_rect = scroll_rect(surface_rect);

    let _ = ui.scope_builder(UiBuilder::new().max_rect(scroll_rect), |ui| {
        egui::ScrollArea::vertical()
            .id_salt("dancer_settings_page_scroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_width(scroll_rect.width());
                draw_dancer_card(ui, state, actions, locale);
                ui.add_space(content_spacing_token());
                draw_swap_card(ui, state, actions, locale);
            });
    });

    let _ = ui.scope_builder(UiBuilder::new().max_rect(footer_rect), |ui| {
        draw_footer(ui, actions, locale);
    });
}

#[must_use]
pub fn footer_rect(surface_rect: Rect) -> Rect {
    Rect::from_min_max(
        pos2(
            surface_rect.left(),
            surface_rect.bottom() - footer_height_token(),
        ),
        surface_rect.right_bottom(),
    )
}

#[must_use]
pub fn scroll_rect(surface_rect: Rect) -> Rect {
    let footer_rect = footer_rect(surface_rect);
    Rect::from_min_max(
        pos2(
            content_column_left(surface_rect),
            surface_rect.top() + content_top_inset_token(),
        ),
        pos2(
            content_column_right(surface_rect),
            footer_rect.top() - content_outer_margin_token(),
        ),
    )
}

#[must_use]
pub const fn top_bar_title_role() -> TypographyRole {
    TypographyRole::TitleLarge
}

fn draw_dancers_pane(
    ui: &mut Ui,
    state: &DancerSettingsPageState,
    actions: &mut Vec<DancerSettingsPageAction>,
    locale: &str,
) {
    let pane_title = t(locale, "DancersTitle");
    let pane_actions = dancers_pane_view::draw(
        ui,
        dancers_pane_view::DancersPaneViewUiState {
            dancer_items: &state.dancers,
            selected_dancer_index: selected_dancer_index(state),
            can_delete_dancer: state.can_delete_dancer,
            title_text: pane_title.as_str(),
        },
    );
    actions.extend(pane_actions.into_iter().map(map_pane_action));
}

fn draw_dancer_card(
    ui: &mut Ui,
    state: &DancerSettingsPageState,
    actions: &mut Vec<DancerSettingsPageAction>,
    locale: &str,
) {
    Frame::new()
        .fill(ui.visuals().window_fill)
        .stroke(Stroke::new(
            material_style_metrics().strokes.outline,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .corner_radius(CornerRadius::same(card_corner_radius_token() as u8))
        .inner_margin(Margin::same(content_spacing_token() as i8))
        .show(ui, |ui| {
            ui.heading(t(locale, "DancerTitle"));
            ui.label(t(locale, "DancerRoleLabel"));

            let selected_role = selected_role_index(state).unwrap_or(0);
            let role_labels = role_option_labels(state);
            let role_label_refs = role_labels.iter().map(String::as_str).collect::<Vec<_>>();
            let role_response = components::mode_dropdown(
                ui,
                egui::Id::new("dancer_settings_role"),
                if state.roles.is_empty() {
                    None
                } else {
                    Some(selected_role)
                },
                role_label_refs.as_slice(),
                !state.roles.is_empty(),
                ui.available_width(),
                dropdown_height_token(),
            );
            if let Some(selected_role_mut) = role_response
                && selected_role_mut != selected_role
            {
                actions.push(DancerSettingsPageAction::SelectRole {
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
                    egui::TextEdit::singleline(&mut name).hint_text(t(locale, "DancerNameLabel")),
                )
                .changed()
            {
                actions.push(DancerSettingsPageAction::UpdateDancerName { value: name });
            }

            let mut shortcut = state
                .selected_dancer
                .as_ref()
                .map(|dancer| dancer.shortcut.clone())
                .unwrap_or_default();
            if ui
                .add_enabled(
                    state.has_selected_dancer,
                    egui::TextEdit::singleline(&mut shortcut)
                        .hint_text(t(locale, "DancerShortcutLabel")),
                )
                .changed()
            {
                actions.push(DancerSettingsPageAction::UpdateDancerShortcut { value: shortcut });
            }

            ui.label(t(locale, "DancerIconLabel"));
            let mut selected_icon = selected_icon_index(state).unwrap_or(0);
            let icon_labels = icon_option_labels(state);
            let icon_label_refs = icon_labels.iter().map(String::as_str).collect::<Vec<_>>();
            if let Some(next_selected_icon) = components::mode_dropdown(
                ui,
                egui::Id::new("dancer_settings_icon"),
                if state.icon_options.is_empty() {
                    None
                } else {
                    Some(selected_icon)
                },
                icon_label_refs.as_slice(),
                !state.icon_options.is_empty(),
                ui.available_width(),
                dropdown_height_token(),
            ) {
                selected_icon = next_selected_icon;
            }
            if let Some(option) = state.icon_options.get(selected_icon)
                && state
                    .selected_icon_option
                    .as_ref()
                    .map(|value| value.key.as_str())
                    != Some(option.key.as_str())
            {
                actions.push(DancerSettingsPageAction::UpdateDancerIcon {
                    value: option.icon_name.clone(),
                });
            }

            ui.label(t(locale, "DancerColorLabel"));
            ui.add_enabled_ui(state.has_selected_dancer, |ui| {
                if let Some(color32) =
                    color_picker_ui::draw_bound(ui, selected_dancer_color_picker_state(state))
                {
                    actions.push(DancerSettingsPageAction::UpdateDancerColor {
                        value: Color {
                            r: color32.r(),
                            g: color32.g(),
                            b: color32.b(),
                            a: color32.a(),
                        },
                    });
                }
            });
        });
}

fn draw_swap_card(
    ui: &mut Ui,
    state: &DancerSettingsPageState,
    actions: &mut Vec<DancerSettingsPageAction>,
    locale: &str,
) {
    Frame::new()
        .fill(ui.visuals().window_fill)
        .stroke(Stroke::new(
            material_style_metrics().strokes.outline,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .corner_radius(CornerRadius::same(card_corner_radius_token() as u8))
        .inner_margin(Margin::same(content_spacing_token() as i8))
        .show(ui, |ui| {
            ui.heading(t(locale, "DancerSwapSectionTitle"));
            ui.label(t(locale, "DancerSwapFromLabel"));

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
            let dancer_labels = dancer_option_labels(state);
            let dancer_label_refs = dancer_labels.iter().map(String::as_str).collect::<Vec<_>>();
            if let Some(from_index_mut) = components::mode_dropdown(
                ui,
                egui::Id::new("dancer_settings_swap_from"),
                if state.dancers.is_empty() {
                    None
                } else {
                    Some(from_index)
                },
                dancer_label_refs.as_slice(),
                !state.dancers.is_empty(),
                ui.available_width(),
                dropdown_height_token(),
            ) && from_index_mut != from_index
            {
                actions.push(DancerSettingsPageAction::UpdateSwapFrom {
                    index: from_index_mut,
                });
            }

            ui.label(t(locale, "DancerSwapToLabel"));
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
            if let Some(to_index_mut) = components::mode_dropdown(
                ui,
                egui::Id::new("dancer_settings_swap_to"),
                if state.dancers.is_empty() {
                    None
                } else {
                    Some(to_index)
                },
                dancer_label_refs.as_slice(),
                !state.dancers.is_empty(),
                ui.available_width(),
                dropdown_height_token(),
            ) && to_index_mut != to_index
            {
                actions.push(DancerSettingsPageAction::UpdateSwapTo {
                    index: to_index_mut,
                });
            }

            if ui
                .add_enabled(
                    state.can_swap_dancers,
                    MaterialButton::new(t(locale, "DancerSwapButton")),
                )
                .clicked()
            {
                actions.push(DancerSettingsPageAction::RequestSwapDancers);
            }
        });
}

pub fn draw_swap_dialog_panel(
    ui: &mut Ui,
    state: &DancerSettingsPageState,
    locale: &str,
) -> Option<SwapDialogAction> {
    let view_model = build_swap_dialog_view_model(state, locale)?;

    let mut action = None;
    ui.set_min_width(360.0);
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = vec2(content_spacing_token(), content_spacing_token());

        ui.label(typography::rich_text_for_role(
            view_model.title_text,
            TypographyRole::TitleLarge,
        ));
        draw_swap_dialog_dancer_row(
            ui,
            &view_model.first_dancer_name,
            view_model.first_dancer_color,
        );
        draw_swap_dialog_dancer_row(
            ui,
            &view_model.second_dancer_name,
            view_model.second_dancer_color,
        );
        ui.label(typography::rich_text_for_role(
            view_model.message_text,
            TypographyRole::BodyMedium,
        ));
        if ui
            .add(MaterialButton::new(view_model.cancel_text))
            .clicked()
        {
            action = Some(SwapDialogAction::Cancel);
        }
        if ui
            .add(MaterialButton::new(view_model.confirm_text))
            .clicked()
        {
            action = Some(SwapDialogAction::Confirm);
        }
    });

    action
}

fn draw_swap_dialog_dancer_row(ui: &mut Ui, dancer_name: &str, dancer_color: Color32) {
    ui.horizontal(|ui| {
        let (swatch_rect, _) = ui.allocate_exact_size(
            vec2(content_spacing_token(), content_spacing_token()),
            Sense::hover(),
        );
        ui.painter().rect_filled(
            swatch_rect,
            CornerRadius::same((content_spacing_token() / 2.0) as u8),
            dancer_color,
        );
        ui.label(
            RichText::new(dancer_name)
                .font(typography::font_id_for_role(TypographyRole::BodyMedium)),
        );
    });
}

#[must_use]
pub fn map_pane_action(action: DancersPaneViewAction) -> DancerSettingsPageAction {
    match action {
        DancersPaneViewAction::SelectDancer { index } => {
            DancerSettingsPageAction::SelectDancer { index }
        }
        DancersPaneViewAction::AddDancer => DancerSettingsPageAction::AddDancer,
        DancersPaneViewAction::DeleteDancer => DancerSettingsPageAction::DeleteSelectedDancer,
    }
}
