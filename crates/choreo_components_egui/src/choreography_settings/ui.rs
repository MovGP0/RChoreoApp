use egui::Color32;
use egui::DragValue;
use egui::RichText;
use egui::Ui;
use egui_material3::MaterialSelect;
use egui_material3::MaterialSlider;
use egui_material3::MaterialSwitch;

use choreo_master_mobile_json::Color;

use super::actions::ChoreographySettingsAction;
use super::actions::UpdateSelectedSceneAction;
use super::state::ChoreographySettingsState;
use super::translations::ChoreographySettingsTranslations;

const DEFAULT_LOCALE: &str = "en";

#[must_use]
pub fn choreo_date_text(year: i32, month: u8, day: u8) -> String {
    format!("{year:04}-{month:02}-{day:02}")
}

#[must_use]
pub fn transparency_percentage_text(transparency: f64) -> String {
    let percentage = (transparency.clamp(0.0, 1.0) * 100.0).round() as i32;
    format!("Transparency: {percentage}%")
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
    if state.has_selected_scene {
        ui.separator();
        draw_selected_scene_section(ui, state, locale, &mut actions);
    }

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

    ui.label(ChoreographySettingsTranslations::comment(locale));
    let mut comment = state.comment.clone();
    if ui.text_edit_singleline(&mut comment).changed() {
        actions.push(ChoreographySettingsAction::UpdateComment(comment));
    }
    ui.label(ChoreographySettingsTranslations::name(locale));
    let mut name = state.name.clone();
    if ui.text_edit_singleline(&mut name).changed() {
        actions.push(ChoreographySettingsAction::UpdateName(name));
    }
    ui.label(ChoreographySettingsTranslations::subtitle(locale));
    let mut subtitle = state.subtitle.clone();
    if ui.text_edit_singleline(&mut subtitle).changed() {
        actions.push(ChoreographySettingsAction::UpdateSubtitle(subtitle));
    }

    ui.label(ChoreographySettingsTranslations::date(locale));
    ui.label(choreo_date_text(
        state.date.year,
        state.date.month,
        state.date.day,
    ));
    let mut year = state.date.year;
    let mut month = i32::from(state.date.month);
    let mut day = i32::from(state.date.day);
    let mut date_changed = false;
    ui.horizontal(|ui| {
        date_changed |= ui.add(DragValue::new(&mut year).range(1..=9999)).changed();
        date_changed |= ui.add(DragValue::new(&mut month).range(1..=12)).changed();
        date_changed |= ui.add(DragValue::new(&mut day).range(1..=31)).changed();
    });
    if date_changed {
        actions.push(ChoreographySettingsAction::UpdateDate {
            year,
            month: month as u8,
            day: day as u8,
        });
    }

    ui.label(ChoreographySettingsTranslations::variation(locale));
    let mut variation = state.variation.clone();
    if ui.text_edit_singleline(&mut variation).changed() {
        actions.push(ChoreographySettingsAction::UpdateVariation(variation));
    }
    ui.label(ChoreographySettingsTranslations::author(locale));
    let mut author = state.author.clone();
    if ui.text_edit_singleline(&mut author).changed() {
        actions.push(ChoreographySettingsAction::UpdateAuthor(author));
    }
    ui.label(ChoreographySettingsTranslations::description(locale));
    let mut description = state.description.clone();
    if ui.text_edit_singleline(&mut description).changed() {
        actions.push(ChoreographySettingsAction::UpdateDescription(description));
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
