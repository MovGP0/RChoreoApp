use egui::CornerRadius;
use egui::Frame;
use egui::Image;
use egui::Margin;
use egui::Sense;
use egui::Stroke;
use egui::Ui;
use egui::UiBuilder;
use material3::styling::material_typography::TypographyRole;
use crate::dancers::dancer_list_item_view;
use crate::dancers_pane_view::geometry::pane_list_size;
use crate::dancers_pane_view::tokens::pane_corner_radius_token;
use crate::dancers_pane_view::tokens::pane_inner_padding_token;
use crate::dancers_pane_view::tokens::pane_spacing_token;
use crate::dancers_pane_view::ui::DancersPaneViewAction;
use crate::dancers_pane_view::ui::DancersPaneViewUiState;
use crate::material::icons as ui_icons;
use crate::material::icons::UiIconKey;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography as typography;

pub fn draw(ui: &mut Ui, state: DancersPaneViewUiState<'_>) -> Vec<DancersPaneViewAction> {
    let mut actions = Vec::new();
    let spacing = pane_spacing_token();

    ui.add_space(spacing);
    draw_title(ui, state.title_text);
    ui.add_space(spacing);

    draw_dancer_list(ui, &state, &mut actions);

    ui.add_space(spacing);
    draw_action_buttons(ui, state.can_delete_dancer, &mut actions);

    actions
}

fn draw_title(ui: &mut Ui, title_text: &str) {
    ui.label(typography::rich_text_for_role(title_text, TypographyRole::TitleMedium));
}

fn draw_dancer_list(
    ui: &mut Ui,
    state: &DancersPaneViewUiState<'_>,
    actions: &mut Vec<DancersPaneViewAction>,
) {
    let list_rect = allocate_list_rect(ui);
    let _ = ui.scope_builder(UiBuilder::new().max_rect(list_rect), |ui| {
        pane_list_frame(ui).show(ui, |ui| {
            ui.set_min_height(list_rect.height());
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    draw_dancer_rows(ui, state, actions);
                });
        });
    });
}

fn allocate_list_rect(ui: &mut Ui) -> egui::Rect {
    let list_size = pane_list_size(ui.available_width(), ui.available_height());
    let (list_rect, _) = ui.allocate_exact_size(list_size, Sense::hover());
    list_rect
}

fn pane_list_frame(ui: &Ui) -> Frame {
    Frame::new()
        .fill(ui.visuals().faint_bg_color)
        .stroke(Stroke::new(
            material_style_metrics().strokes.outline,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .corner_radius(CornerRadius::same(pane_corner_radius_token() as u8))
        .inner_margin(Margin::same(pane_inner_padding_token()))
}

fn draw_dancer_rows(
    ui: &mut Ui,
    state: &DancersPaneViewUiState<'_>,
    actions: &mut Vec<DancersPaneViewAction>,
) {
    for (index, dancer) in state.dancer_items.iter().enumerate() {
        let is_selected = Some(index) == state.selected_dancer_index;
        let response = dancer_list_item_view::draw(ui, dancer, is_selected);
        if response.clicked() {
            actions.push(DancersPaneViewAction::SelectDancer { index });
        }
    }
}

fn draw_action_buttons(
    ui: &mut Ui,
    can_delete_dancer: bool,
    actions: &mut Vec<DancersPaneViewAction>,
) {
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), 48.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = material_style_metrics().spacings.spacing_8;

            draw_action_button(
                ui,
                PaneActionButton {
                    icon_key: UiIconKey::DancersAdd,
                    action: DancersPaneViewAction::AddDancer,
                    enabled: true,
                },
                actions,
            );
            draw_action_button(
                ui,
                PaneActionButton {
                    icon_key: UiIconKey::DancersRemove,
                    action: DancersPaneViewAction::DeleteDancer,
                    enabled: can_delete_dancer,
                },
                actions,
            );
        },
    );
}

fn draw_action_button(
    ui: &mut Ui,
    button: PaneActionButton,
    actions: &mut Vec<DancersPaneViewAction>,
) {
    let response = crate::material::components::top_bar_icon::top_bar_icon_button_enabled(
        ui,
        pane_icon_image(button.icon_key),
        false,
        button.enabled,
    );
    if response.clicked() {
        actions.push(button.action);
    }
}

fn pane_icon_image(icon_key: UiIconKey) -> Image<'static> {
    let icon = ui_icons::icon(icon_key);
    Image::from_bytes(pane_icon_uri(icon.token), icon.svg.as_bytes())
}

fn pane_icon_uri(token: &str) -> String {
    format!("bytes://dancers_pane/{token}.svg")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PaneActionButton {
    icon_key: UiIconKey,
    action: DancersPaneViewAction,
    enabled: bool,
}
