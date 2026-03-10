use egui::CornerRadius;
use egui::Frame;
use egui::Margin;
use egui::Stroke;
use egui::Ui;
use egui_material3::MaterialIconButton;

use crate::dancers::dancer_list_item_view;
use crate::dancers::state::DancerState;
use crate::material::components::MaterialScrollArea;
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
pub const fn title_role() -> TypographyRole {
    TypographyRole::TitleMedium
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

    Frame::new()
        .fill(ui.visuals().faint_bg_color)
        .stroke(Stroke::new(
            material_style_metrics().strokes.outline,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .corner_radius(CornerRadius::same(pane_corner_radius_token() as u8))
        .inner_margin(Margin::same(pane_inner_padding_token()))
        .show(ui, |ui| {
            MaterialScrollArea::vertical()
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

    ui.add_space(spacing);
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = pane_icon_gap_token();

        let add_icon = ui_icons::icon(UiIconKey::DancersAdd);
        let add_response =
            ui.add(MaterialIconButton::standard(add_icon.token).svg_data(add_icon.svg));
        if add_response.clicked() {
            actions.push(DancersPaneViewAction::AddDancer);
        }
        let _ = add_response.on_hover_text("+");

        let remove_icon = ui_icons::icon(UiIconKey::DancersRemove);
        let delete_response = ui.add_enabled(
            state.can_delete_dancer,
            MaterialIconButton::standard(remove_icon.token).svg_data(remove_icon.svg),
        );
        if delete_response.clicked() {
            actions.push(DancersPaneViewAction::DeleteDancer);
        }
        let _ = delete_response.on_hover_text("-");
    });

    actions
}
