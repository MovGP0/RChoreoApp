use egui::Color32;
use egui::CornerRadius;
use egui::Frame;
use egui::Margin;
use egui::Stroke;
use egui::Ui;
use egui_material3::MaterialSelect;
use egui_material3::MaterialSwitch;

use crate::choreo_info::messages::ChoreoInfoAction;
use crate::choreo_info::state::ChoreoDate;
use crate::choreo_info::state::ChoreoInfoState;
use crate::choreo_info::ui::ChoreoInfoLabels;
use crate::color_picker::state::ColorPickerState;
use crate::color_picker::ui as color_picker_ui;
use crate::material::components::MaterialScrollArea;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography as typography;
use crate::material::styling::material_typography::TypographyRole;
use crate::number_picker::ui::NumberPickerUiState;
use choreo_master_mobile_json::Color;

use super::actions::ChoreographySettingsAction;
use super::actions::UpdateSelectedSceneAction;
use super::state::ChoreographySettingsState;
use super::translations::ChoreographySettingsTranslations;

const DEFAULT_LOCALE: &str = "en";

#[must_use]
pub fn choreo_date_text(year: i32, month: u8, day: u8) -> String {
    crate::choreo_info::ui::choreo_date_text(year, month, day)
}

#[must_use]
pub fn transparency_percentage_text(transparency: f64) -> String {
    crate::choreo_info::ui::transparency_percentage_text(transparency)
}

pub fn draw(ui: &mut Ui, state: &ChoreographySettingsState) -> Vec<ChoreographySettingsAction> {
    let mut actions: Vec<ChoreographySettingsAction> = Vec::new();
    let locale = DEFAULT_LOCALE;
    let section_titles = settings_section_titles(locale);

    MaterialScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            draw_settings_card(ui, &section_titles[0], |ui| {
                draw_selected_scene_section(ui, state, locale, &mut actions);
            });
            ui.add_space(card_spacing_token());
            draw_settings_card(ui, &section_titles[1], |ui| {
                draw_display_section(ui, state, locale, &mut actions);
            });
            ui.add_space(card_spacing_token());
            draw_settings_card(ui, &section_titles[2], |ui| {
                draw_choreography_section(ui, state, locale, &mut actions);
            });
            ui.add_space(card_spacing_token());
            draw_settings_card(ui, &section_titles[3], |ui| {
                draw_floor_section(ui, state, locale, &mut actions);
            });
        });

    actions
}

#[must_use]
pub const fn uses_vertical_scroll_container() -> bool {
    true
}

#[must_use]
pub fn settings_section_titles(locale: &str) -> [String; 4] {
    [
        ChoreographySettingsTranslations::selected_scene(locale),
        ChoreographySettingsTranslations::display(locale),
        ChoreographySettingsTranslations::choreography(locale),
        ChoreographySettingsTranslations::floor(locale),
    ]
}

#[must_use]
pub fn selected_scene_controls_enabled(state: &ChoreographySettingsState) -> bool {
    state.has_selected_scene
}

#[must_use]
pub fn scene_timestamp_controls_enabled(state: &ChoreographySettingsState) -> bool {
    state.has_selected_scene && state.scene_has_timestamp
}

#[must_use]
pub fn floor_size_maximum(state: &ChoreographySettingsState) -> i32 {
    state.floor_size_options.last().copied().unwrap_or(100)
}

#[must_use]
pub fn floor_color_picker_state(state: &ChoreographySettingsState) -> ColorPickerState {
    color_picker_ui::state_for_color(to_color32(&state.floor_color))
}

#[must_use]
pub fn selected_scene_color_picker_state(state: &ChoreographySettingsState) -> ColorPickerState {
    color_picker_ui::state_for_color(to_color32(&state.scene_color))
}

fn draw_choreography_section(
    ui: &mut Ui,
    state: &ChoreographySettingsState,
    locale: &str,
    actions: &mut Vec<ChoreographySettingsAction>,
) {
    let component_state = to_choreo_info_state(state);
    let labels = ChoreoInfoLabels {
        comment: ChoreographySettingsTranslations::comment(locale),
        name: ChoreographySettingsTranslations::name(locale),
        subtitle: ChoreographySettingsTranslations::subtitle(locale),
        date: ChoreographySettingsTranslations::date(locale),
        variation: ChoreographySettingsTranslations::variation(locale),
        author: ChoreographySettingsTranslations::author(locale),
        description: ChoreographySettingsTranslations::description(locale),
    };
    let info_actions = crate::choreo_info::ui::draw(ui, &component_state, &labels);
    for action in info_actions {
        actions.push(map_choreo_info_action(action));
    }
}

fn draw_floor_section(
    ui: &mut Ui,
    state: &ChoreographySettingsState,
    locale: &str,
    actions: &mut Vec<ChoreographySettingsAction>,
) {
    let floor_maximum = floor_size_maximum(state);
    if let Some(front) = crate::number_picker::ui::draw(
        ui,
        NumberPickerUiState {
            label: &ChoreographySettingsTranslations::floor_front(locale),
            value: state.floor_front,
            minimum: 1,
            maximum: floor_maximum,
            step: 1,
            enabled: true,
        },
    ) {
        actions.push(ChoreographySettingsAction::UpdateFloorFront(front));
    }
    if let Some(back) = crate::number_picker::ui::draw(
        ui,
        NumberPickerUiState {
            label: &ChoreographySettingsTranslations::floor_back(locale),
            value: state.floor_back,
            minimum: 1,
            maximum: floor_maximum,
            step: 1,
            enabled: true,
        },
    ) {
        actions.push(ChoreographySettingsAction::UpdateFloorBack(back));
    }
    if let Some(left) = crate::number_picker::ui::draw(
        ui,
        NumberPickerUiState {
            label: &ChoreographySettingsTranslations::floor_left(locale),
            value: state.floor_left,
            minimum: 1,
            maximum: floor_maximum,
            step: 1,
            enabled: true,
        },
    ) {
        actions.push(ChoreographySettingsAction::UpdateFloorLeft(left));
    }
    if let Some(right) = crate::number_picker::ui::draw(
        ui,
        NumberPickerUiState {
            label: &ChoreographySettingsTranslations::floor_right(locale),
            value: state.floor_right,
            minimum: 1,
            maximum: floor_maximum,
            step: 1,
            enabled: true,
        },
    ) {
        actions.push(ChoreographySettingsAction::UpdateFloorRight(right));
    }

    let mut selected_grid_index = state
        .grid_size_options
        .iter()
        .position(|option| option.value == state.selected_grid_size_option.value);
    let mut grid_resolution_select = MaterialSelect::new(&mut selected_grid_index)
        .label(ChoreographySettingsTranslations::grid_resolution(locale));
    for (index, option) in state.grid_size_options.iter().enumerate() {
        grid_resolution_select = grid_resolution_select.option(index, option.display.as_str());
    }
    let grid_resolution_response =
        ui.add(grid_resolution_select.enabled(!state.grid_size_options.is_empty()));
    if grid_resolution_response.changed()
        && let Some(index) = selected_grid_index
        && let Some(option) = state.grid_size_options.get(index)
    {
        actions.push(ChoreographySettingsAction::UpdateGridResolution(
            option.value,
        ));
    }

    render_toggle_switch(
        ui,
        state.grid_lines,
        ChoreographySettingsTranslations::grid_lines(locale),
        ChoreographySettingsAction::UpdateGridLines,
        actions,
    );
    render_toggle_switch(
        ui,
        state.snap_to_grid,
        ChoreographySettingsTranslations::snap_to_grid(locale),
        ChoreographySettingsAction::UpdateSnapToGrid,
        actions,
    );

    if let Some(action) = crate::choreo_info::ui::draw_transparency(
        ui,
        state.transparency,
        &ChoreographySettingsTranslations::transparency(locale),
    ) {
        actions.push(map_choreo_info_action(action));
    }

    ui.label(ChoreographySettingsTranslations::floor_color(locale));
    if let Some(floor_color) = color_picker_ui::draw_bound(ui, floor_color_picker_state(state)) {
        actions.push(ChoreographySettingsAction::UpdateFloorColor(from_color32(
            floor_color,
        )));
    }
}

fn draw_display_section(
    ui: &mut Ui,
    state: &ChoreographySettingsState,
    locale: &str,
    actions: &mut Vec<ChoreographySettingsAction>,
) {
    render_toggle_switch(
        ui,
        state.draw_path_from,
        ChoreographySettingsTranslations::draw_path_from(locale),
        ChoreographySettingsAction::UpdateDrawPathFrom,
        actions,
    );
    render_toggle_switch(
        ui,
        state.draw_path_to,
        ChoreographySettingsTranslations::draw_path_to(locale),
        ChoreographySettingsAction::UpdateDrawPathTo,
        actions,
    );
    render_toggle_switch(
        ui,
        state.show_timestamps,
        ChoreographySettingsTranslations::show_timestamps(locale),
        ChoreographySettingsAction::UpdateShowTimestamps,
        actions,
    );
    render_toggle_switch(
        ui,
        state.positions_at_side,
        ChoreographySettingsTranslations::positions_at_side(locale),
        ChoreographySettingsAction::UpdatePositionsAtSide,
        actions,
    );
    render_toggle_switch(
        ui,
        state.show_legend,
        ChoreographySettingsTranslations::show_legend(locale),
        ChoreographySettingsAction::UpdateShowLegend,
        actions,
    );
}

fn draw_selected_scene_section(
    ui: &mut Ui,
    state: &ChoreographySettingsState,
    locale: &str,
    actions: &mut Vec<ChoreographySettingsAction>,
) {
    ui.add_enabled_ui(selected_scene_controls_enabled(state), |ui| {
        ui.label(ChoreographySettingsTranslations::scene_name(locale));
        let mut scene_name = state.scene_name.clone();
        if ui.text_edit_singleline(&mut scene_name).changed() {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneName(scene_name),
            ));
        }

        ui.label(ChoreographySettingsTranslations::scene_text(locale));
        let mut scene_text = state.scene_text.clone();
        if ui.text_edit_singleline(&mut scene_text).changed() {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneText(scene_text),
            ));
        }

        let mut scene_fixed_positions = state.scene_fixed_positions;
        let fixed_positions_response =
            ui.add(MaterialSwitch::new(&mut scene_fixed_positions).text(
                ChoreographySettingsTranslations::scene_fixed_positions(locale),
            ));
        if fixed_positions_response.changed() {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneFixedPositions(scene_fixed_positions),
            ));
        }

        let mut scene_has_timestamp = state.scene_has_timestamp;
        let has_timestamp_response = ui.add(MaterialSwitch::new(&mut scene_has_timestamp).text(
            ChoreographySettingsTranslations::scene_has_timestamp(locale),
        ));
        if has_timestamp_response.changed() {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneTimestamp {
                    has_timestamp: scene_has_timestamp,
                    seconds: state.scene_timestamp_seconds,
                },
            ));
        }

        ui.add_enabled_ui(scene_timestamp_controls_enabled(state), |ui| {
            let mut minutes = state.scene_timestamp_minutes;
            let mut seconds = state.scene_timestamp_seconds_part;
            let mut millis = state.scene_timestamp_millis;

            if let Some(next_minutes) = crate::number_picker::ui::draw(
                ui,
                NumberPickerUiState {
                    label: &ChoreographySettingsTranslations::timestamp_minutes(locale),
                    value: minutes,
                    minimum: 0,
                    maximum: 1440,
                    step: 1,
                    enabled: true,
                },
            ) {
                minutes = next_minutes;
            }
            if let Some(next_seconds) = crate::number_picker::ui::draw(
                ui,
                NumberPickerUiState {
                    label: &ChoreographySettingsTranslations::timestamp_seconds(locale),
                    value: seconds,
                    minimum: 0,
                    maximum: 59,
                    step: 1,
                    enabled: true,
                },
            ) {
                seconds = next_seconds;
            }
            if let Some(next_millis) = crate::number_picker::ui::draw(
                ui,
                NumberPickerUiState {
                    label: &ChoreographySettingsTranslations::timestamp_millis(locale),
                    value: millis,
                    minimum: 0,
                    maximum: 999,
                    step: 10,
                    enabled: true,
                },
            ) {
                millis = next_millis;
            }

            if minutes != state.scene_timestamp_minutes
                || seconds != state.scene_timestamp_seconds_part
                || millis != state.scene_timestamp_millis
            {
                let total = (minutes as f64 * 60.0) + (seconds as f64) + (millis as f64 / 1000.0);
                actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                    UpdateSelectedSceneAction::SceneTimestamp {
                        has_timestamp: scene_has_timestamp,
                        seconds: total,
                    },
                ));
            }
        });

        ui.label(ChoreographySettingsTranslations::scene_color(locale));
        if let Some(scene_color) =
            color_picker_ui::draw_bound(ui, selected_scene_color_picker_state(state))
        {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneColor(from_color32(scene_color)),
            ));
        }
    });
}

fn draw_settings_card(ui: &mut Ui, title: &str, add_contents: impl FnOnce(&mut Ui)) {
    let metrics = material_style_metrics();
    Frame::new()
        .fill(ui.visuals().window_fill)
        .stroke(Stroke::new(
            metrics.strokes.outline,
            ui.visuals().widgets.noninteractive.bg_stroke.color,
        ))
        .corner_radius(CornerRadius::same(
            metrics.corner_radii.border_radius_8 as u8,
        ))
        .inner_margin(Margin::same(metrics.paddings.padding_12 as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = metrics.spacings.spacing_8;
            ui.label(typography::rich_text_for_role(
                title,
                settings_card_title_role(),
            ));
            add_contents(ui);
        });
}

#[must_use]
pub fn card_spacing_token() -> f32 {
    material_style_metrics().spacings.spacing_12
}

#[must_use]
pub const fn settings_card_title_role() -> TypographyRole {
    TypographyRole::TitleLarge
}

fn render_toggle_switch<F>(
    ui: &mut Ui,
    current_value: bool,
    label: String,
    ctor: F,
    actions: &mut Vec<ChoreographySettingsAction>,
) where
    F: Fn(bool) -> ChoreographySettingsAction,
{
    let mut value = current_value;
    let response = ui.add(MaterialSwitch::new(&mut value).text(label));
    if response.changed() {
        actions.push(ctor(value));
    }
}

fn to_color32(color: &Color) -> Color32 {
    Color32::from_rgba_premultiplied(color.r, color.g, color.b, color.a)
}

fn from_color32(color: Color32) -> Color {
    Color {
        a: color.a(),
        r: color.r(),
        g: color.g(),
        b: color.b(),
    }
}

fn to_choreo_info_state(state: &ChoreographySettingsState) -> ChoreoInfoState {
    ChoreoInfoState {
        choreo_name: state.name.clone(),
        choreo_subtitle: state.subtitle.clone(),
        choreo_comment: state.comment.clone(),
        choreo_date: ChoreoDate {
            year: state.date.year,
            month: state.date.month,
            day: state.date.day,
        },
        choreo_variation: state.variation.clone(),
        choreo_author: state.author.clone(),
        choreo_description: state.description.clone(),
        choreo_transparency: state.transparency,
    }
}

fn map_choreo_info_action(action: ChoreoInfoAction) -> ChoreographySettingsAction {
    match action {
        ChoreoInfoAction::UpdateComment(value) => ChoreographySettingsAction::UpdateComment(value),
        ChoreoInfoAction::UpdateName(value) => ChoreographySettingsAction::UpdateName(value),
        ChoreoInfoAction::UpdateSubtitle(value) => {
            ChoreographySettingsAction::UpdateSubtitle(value)
        }
        ChoreoInfoAction::UpdateDate { year, month, day } => {
            ChoreographySettingsAction::UpdateDate { year, month, day }
        }
        ChoreoInfoAction::UpdateVariation(value) => {
            ChoreographySettingsAction::UpdateVariation(value)
        }
        ChoreoInfoAction::UpdateAuthor(value) => ChoreographySettingsAction::UpdateAuthor(value),
        ChoreoInfoAction::UpdateDescription(value) => {
            ChoreographySettingsAction::UpdateDescription(value)
        }
        ChoreoInfoAction::UpdateTransparency(value) => {
            ChoreographySettingsAction::UpdateTransparency(value)
        }
    }
}
