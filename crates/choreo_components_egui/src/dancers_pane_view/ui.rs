use egui::CornerRadius;
use egui::Frame;
use egui::Image;
use egui::Margin;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::UiBuilder;
use egui::vec2;

use crate::dancers::dancer_list_item_view;
use crate::dancers::state::DancerState;
use crate::material::icons as ui_icons;
use crate::material::icons::UiIconKey;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography as typography;
use crate::material::styling::material_typography::TypographyRole;

#[derive(Debug, Clone, PartialEq)]
pub struct DancersPaneViewUiState<'a> {
    pub dancer_items: &'a [DancerState],
    pub selected_dancer_index: Option<usize>,
    pub can_delete_dancer: bool,
    pub title_text: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DancersPaneViewAction {
    SelectDancer { index: usize },
    AddDancer,
    DeleteDancer,
}

#[must_use]
pub const fn pane_spacing_token() -> f32 {
    material_style_metrics().spacings.spacing_12
}

#[must_use]
pub const fn pane_corner_radius_token() -> f32 {
    material_style_metrics().corner_radii.border_radius_12
}

#[must_use]
pub const fn pane_inner_padding_token() -> i8 {
    material_style_metrics().paddings.padding_8 as i8
}

#[must_use]
pub const fn pane_icon_gap_token() -> f32 {
    material_style_metrics().spacings.spacing_8
}

#[must_use]
pub const fn pane_button_row_height_token() -> f32 {
    48.0
}

#[must_use]
pub fn pane_list_height(available_height: f32) -> f32 {
    (available_height - pane_spacing_token() - pane_button_row_height_token()).max(0.0)
}

#[must_use]
pub const fn title_role() -> TypographyRole {
    TypographyRole::TitleMedium
}

#[must_use]
pub const fn add_button_icon_key() -> UiIconKey {
    UiIconKey::DancersAdd
}

#[must_use]
pub const fn remove_button_icon_key() -> UiIconKey {
    UiIconKey::DancersRemove
}

pub fn draw(ui: &mut Ui, state: DancersPaneViewUiState<'_>) -> Vec<DancersPaneViewAction> {
    let mut actions = Vec::new();
    let spacing = pane_spacing_token();

    ui.add_space(spacing);
    ui.label(typography::rich_text_for_role(
        state.title_text,
        title_role(),
    ));
    ui.add_space(spacing);

    let list_height = pane_list_height(ui.available_height());
    let list_size = vec2(ui.available_width(), list_height);
    let (list_rect, _) = ui.allocate_exact_size(list_size, Sense::hover());
    let _ = ui.scope_builder(UiBuilder::new().max_rect(list_rect), |ui| {
        Frame::new()
            .fill(ui.visuals().faint_bg_color)
            .stroke(Stroke::new(
                material_style_metrics().strokes.outline,
                ui.visuals().widgets.noninteractive.bg_stroke.color,
            ))
            .corner_radius(CornerRadius::same(pane_corner_radius_token() as u8))
            .inner_margin(Margin::same(pane_inner_padding_token()))
            .show(ui, |ui| {
                ui.set_min_height(list_rect.height());
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (index, dancer) in state.dancer_items.iter().enumerate() {
                            let is_selected = Some(index) == state.selected_dancer_index;
                            let response = dancer_list_item_view::draw(ui, dancer, is_selected);
                            if response.clicked() {
                                actions.push(DancersPaneViewAction::SelectDancer { index });
                            }
                        }
                    });
            });
    });

    ui.add_space(spacing);
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), pane_button_row_height_token()),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
        ui.spacing_mut().item_spacing.x = pane_icon_gap_token();

        let add_icon = ui_icons::icon(add_button_icon_key());
        let add_image = Image::from_bytes(pane_icon_uri(add_icon.token), add_icon.svg.as_bytes());
        let add_response =
            crate::material::components::top_bar_icon::top_bar_icon_button_enabled(
                ui,
                add_image,
                false,
                true,
            );
        if add_response.clicked() {
            actions.push(DancersPaneViewAction::AddDancer);
        }

        let remove_icon = ui_icons::icon(remove_button_icon_key());
        let remove_image =
            Image::from_bytes(pane_icon_uri(remove_icon.token), remove_icon.svg.as_bytes());
        let delete_response =
            crate::material::components::top_bar_icon::top_bar_icon_button_enabled(
                ui,
                remove_image,
                false,
                state.can_delete_dancer,
            );
        if delete_response.clicked() {
            actions.push(DancersPaneViewAction::DeleteDancer);
        }
        },
    );

    actions
}

fn pane_icon_uri(token: &str) -> String {
    format!("bytes://dancers_pane/{token}.svg")
}
