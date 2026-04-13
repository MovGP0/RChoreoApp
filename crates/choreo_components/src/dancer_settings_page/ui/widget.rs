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
use egui::vec2;
use egui_material3::MaterialButton;

use crate::dancers_pane_view::ui as dancers_pane_view;
use crate::dancers_pane_view::ui::DancersPaneViewAction;
use crate::i18n::t;
use crate::material::components;
use crate::material::components::color_picker::ui as color_picker_ui;
use crate::material::components::dialog_host::DialogHostProps;
use crate::material::components::dialog_host::dialog_metrics_tokens;
use crate::material::components::dialog_host::draw_dialog_host_with_panel;
use crate::material::components::drawer_host::actions::DrawerHostAction;
use crate::material::components::drawer_host::state::DrawerHostOpenMode;
use crate::material::components::drawer_host::state::DrawerHostState;
use crate::material::components::drawer_host::ui::draw_with_slots_in_rect;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography as typography;
use crate::material::styling::material_typography::TypographyRole;
use crate::nav_bar::hamburger_toggle_button;

use super::layout::footer_rect;
use super::layout::main_content_rect;
use super::layout::scroll_rect;
use super::layout::shell_rect;
use super::layout::top_bar_rect;
use super::tokens::LIST_DRAWER_WIDTH_PX;
use super::tokens::SWAP_DANCERS_DIALOG_ID;
use super::tokens::card_corner_radius_token;
use super::tokens::content_spacing_token;
use super::tokens::dropdown_height_token;
use super::tokens::footer_content_padding_token;
use super::tokens::footer_height_token;
use super::tokens::top_bar_height_token;
use super::tokens::top_bar_title_role;

use crate::dancer_settings_page::action::DancerSettingsPageAction;
use crate::dancer_settings_page::action::SwapDialogAction;
use crate::dancer_settings_page::reducer::map_swap_dialog_action;
use crate::dancer_settings_page::state::DancerSettingsPageState;
use crate::dancer_settings_page::state::build_swap_dialog_view_model;
use crate::dancer_settings_page::state::dancer_option_labels;
use crate::dancer_settings_page::state::icon_option_labels;
use crate::dancer_settings_page::state::role_option_labels;
use crate::dancer_settings_page::state::selected_dancer_color_picker_state;
use crate::dancer_settings_page::state::selected_dancer_index;
use crate::dancer_settings_page::state::selected_icon_index;
use crate::dancer_settings_page::state::selected_role_index;

pub fn draw(ui: &mut Ui, state: &DancerSettingsPageState) -> Vec<DancerSettingsPageAction> {
    let mut page_actions: Vec<DancerSettingsPageAction> = Vec::new();
    let mut dialog_action = None;
    let locale = "en";
    let dialog_metrics = dialog_metrics_tokens();
    let palette = material_palette_for_visuals(ui.visuals());

    let close_requested = draw_dialog_host_with_panel(
        ui,
        &DialogHostProps {
            id_source: "dancer_settings_page_dialog_host",
            is_open: state.is_dialog_open,
            close_on_click_away: true,
            overlay_color: palette.background_modal,
            dialog_background: palette.surface_container_low,
            dialog_text_color: palette.on_surface,
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
                        .rect_filled(local_rect, 0.0, palette.background);
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
    let palette = material_palette_for_visuals(ui.visuals());
    let drawer_state = drawer_host_state(
        state,
        palette.background_modal,
        palette.background,
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

fn draw_top_bar(
    ui: &mut Ui,
    state: &DancerSettingsPageState,
    actions: &mut Vec<DancerSettingsPageAction>,
    locale: &str,
) {
    let title = t(locale, "DancersTitle");
    ui.set_min_height(top_bar_height_token());
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

fn draw_footer(ui: &mut Ui, actions: &mut Vec<DancerSettingsPageAction>, locale: &str) {
    let palette = material_palette_for_visuals(ui.visuals());
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, palette.surface_container);
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
    let palette = material_palette_for_visuals(ui.visuals());
    let surface_rect = main_content_rect(ui.max_rect());
    ui.painter()
        .rect_filled(surface_rect, CornerRadius::ZERO, palette.surface_container_low);

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
    settings_card_frame(ui).show(ui, |ui| {
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
    settings_card_frame(ui).show(ui, |ui| {
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

fn settings_card_frame(ui: &Ui) -> Frame {
    let palette = material_palette_for_visuals(ui.visuals());
    Frame::new()
        .fill(palette.surface_container)
        .stroke(Stroke::new(
            material_style_metrics().strokes.outline,
            palette.outline_variant,
        ))
        .corner_radius(CornerRadius::same(card_corner_radius_token() as u8))
        .inner_margin(Margin::same(content_spacing_token() as i8))
}
