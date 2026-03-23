use egui::Color32;
use egui::Ui;
use egui_material3::MaterialSlider;

#[derive(Debug, Clone, PartialEq)]
pub struct SliderWithTicksUiState<'a> {
    pub enabled: bool,
    pub minimum: f64,
    pub maximum: f64,
    pub value: f64,
    pub tick_values: &'a [f64],
    pub tick_color: Option<Color32>,
    pub width: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SliderWithTicksInteraction {
    DragStarted,
    ValueChanged { value: f64, is_dragging: bool },
    DragCompleted { value: f64 },
}

pub fn draw(ui: &mut Ui, state: SliderWithTicksUiState<'_>) -> Vec<SliderWithTicksInteraction> {
    let mut interactions = Vec::new();
    let span = state.maximum - state.minimum;
    let has_valid_range = span > 0.0;
    let slider_max = if has_valid_range {
        state.maximum as f32
    } else {
        (state.minimum + 1.0) as f32
    };
    let mut slider_value = state.value.clamp(state.minimum, state.maximum) as f32;

    let response = ui.add(
        MaterialSlider::new(&mut slider_value, state.minimum as f32..=slider_max)
            .enabled(state.enabled)
            .show_value(false)
            .width(state.width),
    );
    let mut was_dragging = ui
        .memory(|memory| memory.data.get_temp::<bool>(response.id))
        .unwrap_or(false);

    draw_tick_marks(
        ui,
        response.rect,
        state.minimum,
        state.maximum,
        state.tick_values,
        state
            .tick_color
            .unwrap_or(ui.visuals().widgets.noninteractive.fg_stroke.color),
    );

    let clamped_value = if has_valid_range {
        f64::from(slider_value).clamp(state.minimum, state.maximum)
    } else {
        state.minimum
    };

    if response.drag_started() {
        was_dragging = true;
        interactions.push(SliderWithTicksInteraction::DragStarted);
    }
    if response.changed() {
        interactions.push(SliderWithTicksInteraction::ValueChanged {
            value: clamped_value,
            is_dragging: slider_value_change_is_dragging(
                was_dragging,
                response.drag_started(),
                response.dragged(),
            ),
        });
    }
    if response.drag_stopped() {
        was_dragging = false;
        interactions.push(SliderWithTicksInteraction::DragCompleted {
            value: clamped_value,
        });
    }
    ui.memory_mut(|memory| {
        if was_dragging {
            memory.data.insert_temp(response.id, true);
        } else {
            memory.data.remove::<bool>(response.id);
        }
    });

    interactions
}

fn draw_tick_marks(
    ui: &Ui,
    rect: egui::Rect,
    minimum: f64,
    maximum: f64,
    tick_values: &[f64],
    color: Color32,
) {
    let fractions = visible_tick_fractions(minimum, maximum, tick_values);
    if fractions.is_empty() {
        return;
    }

    let painter = ui.painter();
    let tick_top = rect.bottom() + 2.0;
    let tick_bottom = tick_top + 8.0;
    for fraction in fractions {
        let x = egui::lerp(rect.x_range(), fraction);
        painter.line_segment(
            [egui::pos2(x, tick_top), egui::pos2(x, tick_bottom)],
            egui::Stroke::new(1.0, color),
        );
    }
}

#[must_use]
pub fn visible_tick_fractions(minimum: f64, maximum: f64, tick_values: &[f64]) -> Vec<f32> {
    let span = maximum - minimum;
    if span <= 0.0 {
        return Vec::new();
    }

    let mut fractions = Vec::new();
    for tick in tick_values {
        if *tick < minimum || *tick > maximum {
            continue;
        }
        fractions.push(((*tick - minimum) / span) as f32);
    }
    fractions
}

#[must_use]
pub fn slider_value_change_is_dragging(
    was_dragging: bool,
    drag_started: bool,
    is_currently_dragged: bool,
) -> bool {
    was_dragging || drag_started || is_currently_dragged
}
