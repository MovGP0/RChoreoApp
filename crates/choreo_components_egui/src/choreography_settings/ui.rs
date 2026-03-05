use egui::Color32;
use egui::DragValue;
use egui::RichText;
use egui::Ui;
use egui_material3::MaterialSelect;
use egui_material3::MaterialSlider;
use egui_material3::MaterialSwitch;

use choreo_master_mobile_json::Color;
use crate::choreo_info::messages::ChoreoInfoAction;
use crate::choreo_info::state::ChoreoDate;
use crate::choreo_info::state::ChoreoInfoState;
use crate::choreo_info::ui::ChoreoInfoLabels;

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

    ui.heading(ChoreographySettingsTranslations::title(locale));
    ui.separator();

    draw_choreography_section(ui, state, locale, &mut actions);
    ui.separator();
    draw_floor_section(ui, state, locale, &mut actions);
    ui.separator();
    draw_display_section(ui, state, locale, &mut actions);
    ui.separator();
    draw_selected_scene_section(ui, state, locale, &mut actions);

    actions
}

#[must_use]
pub fn selected_scene_controls_enabled(state: &ChoreographySettingsState) -> bool {
    state.has_selected_scene
}

#[must_use]
pub fn scene_timestamp_controls_enabled(state: &ChoreographySettingsState) -> bool {
    state.has_selected_scene && state.scene_has_timestamp
}

fn draw_choreography_section(
    ui: &mut Ui,
    state: &ChoreographySettingsState,
    locale: &str,
    actions: &mut Vec<ChoreographySettingsAction>,
) {
    ui.label(RichText::new(ChoreographySettingsTranslations::choreography(locale)).strong());
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
    ui.label(RichText::new(ChoreographySettingsTranslations::floor(locale)).strong());
    ui.horizontal(|ui| {
        ui.label(ChoreographySettingsTranslations::floor_front(locale));
        let mut front = state.floor_front;
        if ui.add(DragValue::new(&mut front).range(1..=100)).changed() {
            actions.push(ChoreographySettingsAction::UpdateFloorFront(front));
        }
        ui.label(ChoreographySettingsTranslations::floor_back(locale));
        let mut back = state.floor_back;
        if ui.add(DragValue::new(&mut back).range(1..=100)).changed() {
            actions.push(ChoreographySettingsAction::UpdateFloorBack(back));
        }
        ui.label(ChoreographySettingsTranslations::floor_left(locale));
        let mut left = state.floor_left;
        if ui.add(DragValue::new(&mut left).range(1..=100)).changed() {
            actions.push(ChoreographySettingsAction::UpdateFloorLeft(left));
        }
        ui.label(ChoreographySettingsTranslations::floor_right(locale));
        let mut right = state.floor_right;
        if ui.add(DragValue::new(&mut right).range(1..=100)).changed() {
            actions.push(ChoreographySettingsAction::UpdateFloorRight(right));
        }
    });

    ui.label(ChoreographySettingsTranslations::floor_color(locale));
    let mut floor_color = to_color32(&state.floor_color);
    if ui.color_edit_button_srgba(&mut floor_color).changed() {
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
    ui.label(RichText::new(ChoreographySettingsTranslations::display(locale)).strong());

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

    let mut transparency = state.transparency as f32;
    ui.label(format!(
        "{}: {}",
        ChoreographySettingsTranslations::transparency(locale),
        transparency_percentage_text(f64::from(transparency)).replace("Transparency: ", ""),
    ));
    if ui
        .add(
            MaterialSlider::new(&mut transparency, 0.0..=1.0)
                .show_value(false)
                .width(240.0),
        )
        .changed()
    {
        actions.push(ChoreographySettingsAction::UpdateTransparency(f64::from(
            transparency,
        )));
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
        state.show_legend,
        ChoreographySettingsTranslations::show_legend(locale),
        ChoreographySettingsAction::UpdateShowLegend,
        actions,
    );
    render_toggle_switch(
        ui,
        state.snap_to_grid,
        ChoreographySettingsTranslations::snap_to_grid(locale),
        ChoreographySettingsAction::UpdateSnapToGrid,
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
}

fn draw_selected_scene_section(
    ui: &mut Ui,
    state: &ChoreographySettingsState,
    locale: &str,
    actions: &mut Vec<ChoreographySettingsAction>,
) {
    ui.label(RichText::new(ChoreographySettingsTranslations::selected_scene(locale)).strong());
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
            let mut timestamp_parts_changed = false;
            ui.horizontal(|ui| {
                ui.label(ChoreographySettingsTranslations::timestamp_minutes(locale));
                timestamp_parts_changed |= ui
                    .add(DragValue::new(&mut minutes).range(0..=1440))
                    .changed();
                ui.label(ChoreographySettingsTranslations::timestamp_seconds(locale));
                timestamp_parts_changed |=
                    ui.add(DragValue::new(&mut seconds).range(0..=59)).changed();
                ui.label(ChoreographySettingsTranslations::timestamp_millis(locale));
                timestamp_parts_changed |=
                    ui.add(DragValue::new(&mut millis).range(0..=999)).changed();
            });
            if timestamp_parts_changed {
                let millis = (millis / 10) * 10;
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
        let mut scene_color = to_color32(&state.scene_color);
        if ui.color_edit_button_srgba(&mut scene_color).changed() {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneColor(from_color32(scene_color)),
            ));
        }
    });
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
        ChoreoInfoAction::UpdateSubtitle(value) => ChoreographySettingsAction::UpdateSubtitle(value),
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
    }
}
