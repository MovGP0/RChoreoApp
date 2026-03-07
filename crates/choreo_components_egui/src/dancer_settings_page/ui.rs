use choreo_master_mobile_json::Color;
use egui::Align;
use egui::Color32;
use egui::CornerRadius;
use egui::Frame;
use egui::Layout;
use egui::Margin;
use egui::Rect;
use egui::RichText;
use egui::ScrollArea;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::UiBuilder;
use egui::pos2;
use egui::vec2;
use egui_material3::MaterialButton;

use crate::dancers::actions::DancersAction;
use crate::dancers::state::DancerState;
use crate::dancers::state::DancersState;
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
use crate::nav_bar::hamburger_toggle_button;
use crate::ui_style::material_style_metrics::material_style_metrics;
use crate::ui_style::typography;
use crate::ui_style::typography::TypographyRole;

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
pub const fn uses_scrollable_content_shell() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwapDialogViewModel {
    pub title_text: String,
    pub first_dancer_name: String,
    pub second_dancer_name: String,
    pub first_dancer_color: Color32,
    pub second_dancer_color: Color32,
    pub message_text: String,
    pub cancel_text: String,
    pub confirm_text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapDialogAction {
    Cancel,
    Confirm,
}

#[must_use]
pub const fn map_swap_dialog_action(action: SwapDialogAction) -> DancersAction {
    match action {
        SwapDialogAction::Cancel => DancersAction::HideDialog,
        SwapDialogAction::Confirm => DancersAction::ConfirmSwapDancers,
    }
}

pub fn draw(ui: &mut Ui, state: &DancersState) -> Vec<DancersAction> {
    let mut page_actions: Vec<DancersAction> = Vec::new();
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
            let page_rect = ui.available_rect_before_wrap();
            let top_bar_rect =
                Rect::from_min_size(page_rect.min, vec2(page_rect.width(), top_bar_height_token()));
            let content_rect =
                Rect::from_min_max(pos2(page_rect.left(), top_bar_rect.bottom()), page_rect.max);

            let _ = ui.scope_builder(UiBuilder::new().max_rect(top_bar_rect), |ui| {
                draw_top_bar(ui, state, &mut page_actions, locale);
            });
            let _ = ui.scope_builder(UiBuilder::new().max_rect(content_rect), |ui| {
                draw_main_content(ui, content_rect, state, &mut page_actions, locale);
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
        page_actions.push(DancersAction::HideDialog);
    }

    page_actions
}

fn draw_dialog_panel(ui: &mut Ui, state: &DancersState, locale: &str) -> Option<SwapDialogAction> {
    if state.dialog_content.as_deref() == Some(SWAP_DANCERS_DIALOG_ID) {
        return draw_swap_dialog_panel(ui, state, locale);
    }

    ui.label(state.dialog_content.as_deref().unwrap_or_default());
    None
}

fn draw_main_content(
    ui: &mut Ui,
    content_rect: Rect,
    state: &DancersState,
    actions: &mut Vec<DancersAction>,
    locale: &str,
) {
    let drawer_state = drawer_host_state(
        state,
        ui.visuals().window_fill().linear_multiply(0.7),
        ui.visuals().panel_fill,
    );

    let mut content_actions: Vec<DancersAction> = Vec::new();
    let mut pane_actions: Vec<DancersAction> = Vec::new();
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
            actions.push(DancersAction::CloseDancerList);
        }
    }
}

#[must_use]
pub fn drawer_host_state(
    state: &DancersState,
    overlay_color: Color32,
    drawer_background: Color32,
) -> DrawerHostState {
    DrawerHostState {
        left_drawer_width: LIST_DRAWER_WIDTH_PX,
        responsive_breakpoint: 900.0,
        open_mode: DrawerHostOpenMode::Modal,
        top_inset: 0.0,
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

fn draw_top_bar(ui: &mut Ui, state: &DancersState, actions: &mut Vec<DancersAction>, locale: &str) {
    let title = t(locale, "DancersTitle", "Dancers");
    Frame::new()
        .fill(ui.visuals().faint_bg_color)
        .stroke(Stroke::new(
            material_style_metrics().strokes.outline,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .inner_margin(Margin::same(content_spacing_token() as i8))
        .show(ui, |ui| {
            ui.set_min_height(TOP_BAR_HEIGHT_PX);
            ui.horizontal(|ui| {
                let response = hamburger_toggle_button::draw(
                    ui,
                    state.is_dancer_list_open,
                    true,
                    &title,
                    Some(vec2(48.0, 48.0)),
                );
                if response.clicked() {
                    actions.push(DancersAction::ToggleDancerList);
                }
                ui.label(typography::rich_text_for_role(title, top_bar_title_role()));
            });
        });
}

#[must_use]
pub const fn top_bar_title_role() -> TypographyRole {
    TypographyRole::TitleLarge
}

#[must_use]
pub fn build_swap_dialog_view_model(
    state: &DancersState,
    locale: &str,
) -> Option<SwapDialogViewModel> {
    if !state.is_dialog_open || state.dialog_content.as_deref() != Some(SWAP_DANCERS_DIALOG_ID) {
        return None;
    }

    let first_name = swap_dialog_dancer_name(state.swap_from_dancer.as_ref());
    let second_name = swap_dialog_dancer_name(state.swap_to_dancer.as_ref());
    let message_template = t(
        locale,
        "DancerSwapDialogMessage",
        "Swap dancers \"{0}\" and \"{1}\"?",
    );

    Some(SwapDialogViewModel {
        title_text: t(locale, "DancerSwapDialogTitle", "Swap dancers"),
        first_dancer_name: first_name.clone(),
        second_dancer_name: second_name.clone(),
        first_dancer_color: state
            .swap_from_dancer
            .as_ref()
            .map(|dancer| color_to_egui(&dancer.color))
            .unwrap_or(Color32::TRANSPARENT),
        second_dancer_color: state
            .swap_to_dancer
            .as_ref()
            .map(|dancer| color_to_egui(&dancer.color))
            .unwrap_or(Color32::TRANSPARENT),
        message_text: message_template
            .replace("{0}", &first_name)
            .replace("{1}", &second_name),
        cancel_text: t(locale, "DancerSwapDialogCancel", "Cancel"),
        confirm_text: t(locale, "DancerSwapDialogConfirm", "Swap"),
    })
}

fn draw_dancers_pane(
    ui: &mut Ui,
    state: &DancersState,
    actions: &mut Vec<DancersAction>,
    locale: &str,
) {
    let pane_title = t(locale, "DancersTitle", "Dancers");
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

fn draw_content(ui: &mut Ui, state: &DancersState, actions: &mut Vec<DancersAction>, locale: &str) {
    let surface_rect = ui.max_rect();
    ui.painter()
        .rect_filled(surface_rect, CornerRadius::ZERO, ui.visuals().faint_bg_color);

    let footer_rect = Rect::from_min_max(
        pos2(surface_rect.left(), surface_rect.bottom() - footer_height_token()),
        surface_rect.right_bottom(),
    );
    let scroll_rect = Rect::from_min_max(
        pos2(
            surface_rect.left() + content_outer_margin_token(),
            surface_rect.top() + content_outer_margin_token(),
        ),
        pos2(
            (surface_rect.left() + content_outer_margin_token() + content_max_width_token())
                .min(surface_rect.right() - content_outer_margin_token()),
            footer_rect.top() - content_outer_margin_token(),
        ),
    );

    let _ = ui.scope_builder(UiBuilder::new().max_rect(scroll_rect), |ui| {
        ScrollArea::vertical()
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
        Frame::new()
            .fill(ui.visuals().window_fill)
            .stroke(Stroke::new(
                material_style_metrics().strokes.outline,
                ui.visuals().widgets.noninteractive.bg_stroke.color,
            ))
            .inner_margin(Margin::same(FOOTER_PADDING_PX))
            .show(ui, |ui| {
                ui.set_min_height(footer_height_token());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui
                        .add(MaterialButton::new(t(locale, "CommonOk", "OK")))
                        .clicked()
                    {
                        actions.push(DancersAction::SaveToGlobal);
                    }
                    if ui
                        .add(MaterialButton::new(t(locale, "CommonCancel", "Cancel")))
                        .clicked()
                    {
                        actions.push(DancersAction::Cancel);
                    }
                });
            });
    });
}

fn draw_dancer_card(
    ui: &mut Ui,
    state: &DancersState,
    actions: &mut Vec<DancersAction>,
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
            ui.heading(t(locale, "DancerTitle", "Dancer"));
            ui.label(t(locale, "DancerRoleLabel", "Role"));

            let selected_role = selected_role_index(state).unwrap_or(0);
            let mut selected_role_mut = selected_role;
            egui::ComboBox::from_id_salt("dancer_settings_role")
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
                    egui::TextEdit::singleline(&mut name).hint_text(t(
                        locale,
                        "DancerNameLabel",
                        "Name",
                    )),
                )
                .changed()
            {
                actions.push(DancersAction::UpdateDancerName { value: name });
            }

            let mut shortcut = state
                .selected_dancer
                .as_ref()
                .map(|dancer| dancer.shortcut.clone())
                .unwrap_or_default();
            if ui
                .add_enabled(
                    state.has_selected_dancer,
                    egui::TextEdit::singleline(&mut shortcut).hint_text(t(
                        locale,
                        "DancerShortcutLabel",
                        "Shortcut",
                    )),
                )
                .changed()
            {
                actions.push(DancersAction::UpdateDancerShortcut { value: shortcut });
            }

            ui.label(t(locale, "DancerIconLabel", "Icon"));
            let mut selected_icon = selected_icon_index(state).unwrap_or(0);
            let selected_icon_label = state
                .icon_options
                .get(selected_icon)
                .map(|option| option.display_name.clone())
                .unwrap_or_default();
            egui::ComboBox::from_id_salt("dancer_settings_icon")
                .selected_text(selected_icon_label)
                .show_ui(ui, |ui| {
                    for (index, option) in state.icon_options.iter().enumerate() {
                        ui.selectable_value(
                            &mut selected_icon,
                            index,
                            option.display_name.as_str(),
                        );
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
                .unwrap_or_else(crate::dancers::state::transparent_color);
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
        });
}

fn draw_swap_card(
    ui: &mut Ui,
    state: &DancersState,
    actions: &mut Vec<DancersAction>,
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
            ui.heading(t(locale, "DancerSwapSectionTitle", "Swap dancers"));
            ui.label(t(locale, "DancerSwapFromLabel", "Swap from"));

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
            egui::ComboBox::from_id_salt("dancer_settings_swap_from")
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

            ui.label(t(locale, "DancerSwapToLabel", "Swap to"));
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
            egui::ComboBox::from_id_salt("dancer_settings_swap_to")
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
        });
}

pub fn draw_swap_dialog_panel(
    ui: &mut Ui,
    state: &DancersState,
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

fn dancer_name_for_index(state: &DancersState, index: usize) -> String {
    state
        .dancers
        .get(index)
        .map(|dancer| dancer.name.clone())
        .unwrap_or_default()
}

fn swap_dialog_dancer_name(dancer: Option<&DancerState>) -> String {
    let Some(dancer) = dancer else {
        return "?".to_string();
    };

    if !dancer.name.trim().is_empty() {
        return dancer.name.clone();
    }
    if !dancer.shortcut.trim().is_empty() {
        return dancer.shortcut.clone();
    }

    format!("#{}", dancer.dancer_id)
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

#[must_use]
pub fn map_pane_action(action: DancersPaneViewAction) -> DancersAction {
    match action {
        DancersPaneViewAction::SelectDancer { index } => DancersAction::SelectDancer { index },
        DancersPaneViewAction::AddDancer => DancersAction::AddDancer,
        DancersPaneViewAction::DeleteDancer => DancersAction::DeleteSelectedDancer,
    }
}

fn color_to_egui(color: &Color) -> Color32 {
    Color32::from_rgba_unmultiplied(color.r, color.g, color.b, color.a)
}
