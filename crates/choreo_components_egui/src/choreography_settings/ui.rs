use egui::Color32;
use egui::DragValue;
use egui::RichText;
use egui::Ui;
use egui_material3::MaterialButton;

use choreo_master_mobile_json::Color;

use super::actions::ChoreographySettingsAction;
use super::actions::UpdateSelectedSceneAction;
use super::state::ChoreographySettingsState;

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

    ui.heading(t("choreo.settings.title", "Choreography Settings"));
    ui.separator();

    draw_choreography_section(ui, state, &mut actions);
    ui.separator();
    draw_floor_section(ui, state, &mut actions);
    ui.separator();
    draw_display_section(ui, state, &mut actions);
    if state.has_selected_scene {
        ui.separator();
        draw_selected_scene_section(ui, state, &mut actions);
    }

    actions
}

fn draw_choreography_section(
    ui: &mut Ui,
    state: &ChoreographySettingsState,
    actions: &mut Vec<ChoreographySettingsAction>,
) {
    ui.label(RichText::new(t("choreo.settings.choreography", "Choreography")).strong());

    let mut comment = state.comment.clone();
    if ui.text_edit_singleline(&mut comment).changed() {
        actions.push(ChoreographySettingsAction::UpdateComment(comment));
    }
    let mut name = state.name.clone();
    if ui.text_edit_singleline(&mut name).changed() {
        actions.push(ChoreographySettingsAction::UpdateName(name));
    }
    let mut subtitle = state.subtitle.clone();
    if ui.text_edit_singleline(&mut subtitle).changed() {
        actions.push(ChoreographySettingsAction::UpdateSubtitle(subtitle));
    }

    ui.label(choreo_date_text(state.date.year, state.date.month, state.date.day));
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

    let mut variation = state.variation.clone();
    if ui.text_edit_singleline(&mut variation).changed() {
        actions.push(ChoreographySettingsAction::UpdateVariation(variation));
    }
    let mut author = state.author.clone();
    if ui.text_edit_singleline(&mut author).changed() {
        actions.push(ChoreographySettingsAction::UpdateAuthor(author));
    }
    let mut description = state.description.clone();
    if ui.text_edit_singleline(&mut description).changed() {
        actions.push(ChoreographySettingsAction::UpdateDescription(description));
    }
}

fn draw_floor_section(
    ui: &mut Ui,
    state: &ChoreographySettingsState,
    actions: &mut Vec<ChoreographySettingsAction>,
) {
    ui.label(RichText::new(t("choreo.settings.floor", "Floor")).strong());
    ui.horizontal(|ui| {
        let mut front = state.floor_front;
        if ui.add(DragValue::new(&mut front).range(1..=100)).changed() {
            actions.push(ChoreographySettingsAction::UpdateFloorFront(front));
        }
        let mut back = state.floor_back;
        if ui.add(DragValue::new(&mut back).range(1..=100)).changed() {
            actions.push(ChoreographySettingsAction::UpdateFloorBack(back));
        }
        let mut left = state.floor_left;
        if ui.add(DragValue::new(&mut left).range(1..=100)).changed() {
            actions.push(ChoreographySettingsAction::UpdateFloorLeft(left));
        }
        let mut right = state.floor_right;
        if ui.add(DragValue::new(&mut right).range(1..=100)).changed() {
            actions.push(ChoreographySettingsAction::UpdateFloorRight(right));
        }
    });

    let mut floor_color = to_color32(&state.floor_color);
    if ui.color_edit_button_srgba(&mut floor_color).changed() {
        actions.push(ChoreographySettingsAction::UpdateFloorColor(from_color32(floor_color)));
    }
}

fn draw_display_section(
    ui: &mut Ui,
    state: &ChoreographySettingsState,
    actions: &mut Vec<ChoreographySettingsAction>,
) {
    ui.label(RichText::new(t("choreo.settings.display", "Display")).strong());
    egui::ComboBox::from_id_salt("grid_resolution")
        .selected_text(state.selected_grid_size_option.display.as_str())
        .show_ui(ui, |ui| {
            for option in &state.grid_size_options {
                if ui
                    .selectable_label(
                        option.value == state.selected_grid_size_option.value,
                        option.display.as_str(),
                    )
                    .clicked()
                {
                    actions.push(ChoreographySettingsAction::UpdateGridResolution(option.value));
                }
            }
        });

    let mut transparency = state.transparency;
    ui.label(transparency_percentage_text(transparency));
    if ui
        .add(egui::Slider::new(&mut transparency, 0.0..=1.0).show_value(false))
        .changed()
    {
        actions.push(ChoreographySettingsAction::UpdateTransparency(transparency));
    }

    render_toggle_button(
        ui,
        state.grid_lines,
        "Grid lines",
        ChoreographySettingsAction::UpdateGridLines,
        actions,
    );
    render_toggle_button(
        ui,
        state.show_legend,
        "Show legend",
        ChoreographySettingsAction::UpdateShowLegend,
        actions,
    );
    render_toggle_button(
        ui,
        state.snap_to_grid,
        "Snap to grid",
        ChoreographySettingsAction::UpdateSnapToGrid,
        actions,
    );
    render_toggle_button(
        ui,
        state.show_timestamps,
        "Show timestamps",
        ChoreographySettingsAction::UpdateShowTimestamps,
        actions,
    );
    render_toggle_button(
        ui,
        state.positions_at_side,
        "Positions at side",
        ChoreographySettingsAction::UpdatePositionsAtSide,
        actions,
    );
    render_toggle_button(
        ui,
        state.draw_path_from,
        "Draw path from",
        ChoreographySettingsAction::UpdateDrawPathFrom,
        actions,
    );
    render_toggle_button(
        ui,
        state.draw_path_to,
        "Draw path to",
        ChoreographySettingsAction::UpdateDrawPathTo,
        actions,
    );
}

fn draw_selected_scene_section(
    ui: &mut Ui,
    state: &ChoreographySettingsState,
    actions: &mut Vec<ChoreographySettingsAction>,
) {
    ui.label(RichText::new(t("choreo.settings.selected_scene", "Selected Scene")).strong());
    ui.add_enabled_ui(state.has_selected_scene, |ui| {
        let mut scene_name = state.scene_name.clone();
        if ui.text_edit_singleline(&mut scene_name).changed() {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneName(scene_name),
            ));
        }

        let mut scene_text = state.scene_text.clone();
        if ui.text_edit_singleline(&mut scene_text).changed() {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneText(scene_text),
            ));
        }

        let mut scene_fixed_positions = state.scene_fixed_positions;
        if ui.checkbox(&mut scene_fixed_positions, "Fixed positions").changed() {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneFixedPositions(scene_fixed_positions),
            ));
        }

        let mut scene_has_timestamp = state.scene_has_timestamp;
        if ui.checkbox(&mut scene_has_timestamp, "Has timestamp").changed() {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneTimestamp {
                    has_timestamp: scene_has_timestamp,
                    seconds: state.scene_timestamp_seconds,
                },
            ));
        }

        let mut minutes = state.scene_timestamp_minutes;
        let mut seconds = state.scene_timestamp_seconds_part;
        let mut millis = state.scene_timestamp_millis;
        let mut timestamp_parts_changed = false;
        ui.horizontal(|ui| {
            timestamp_parts_changed |= ui.add(DragValue::new(&mut minutes).range(0..=1440)).changed();
            timestamp_parts_changed |= ui.add(DragValue::new(&mut seconds).range(0..=59)).changed();
            timestamp_parts_changed |= ui.add(DragValue::new(&mut millis).range(0..=999)).changed();
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

        let mut scene_color = to_color32(&state.scene_color);
        if ui.color_edit_button_srgba(&mut scene_color).changed() {
            actions.push(ChoreographySettingsAction::UpdateSelectedScene(
                UpdateSelectedSceneAction::SceneColor(from_color32(scene_color)),
            ));
        }
    });
}

fn render_toggle_button<F>(
    ui: &mut Ui,
    current_value: bool,
    label: &str,
    ctor: F,
    actions: &mut Vec<ChoreographySettingsAction>,
) where
    F: Fn(bool) -> ChoreographySettingsAction,
{
    let button_label = if current_value {
        format!("{label}: On")
    } else {
        format!("{label}: Off")
    };
    if ui.add(MaterialButton::new(button_label)).clicked() {
        actions.push(ctor(!current_value));
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

fn t(key: &str, fallback: &'static str) -> String {
    choreo_i18n::translation_with_fallback(DEFAULT_LOCALE, key)
        .unwrap_or(fallback)
        .to_string()
}
