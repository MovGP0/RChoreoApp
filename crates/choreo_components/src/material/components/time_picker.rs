use std::borrow::Cow;

use egui::Button;
use egui::CornerRadius;
use egui::Id;
use egui::Response;
use egui::RichText;
use egui::Stroke;
use egui::Ui;
use egui::vec2;

use crate::material::components::filled_button::FilledButton;
use crate::material::components::material_text::material_text;
use crate::material::components::outline_button::OutlineButton;
use crate::material::styling::material_palette::material_palette_for_visuals;
use crate::material::styling::material_style_metrics::material_style_metrics;
use crate::material::styling::material_typography::MATERIAL_TYPOGRAPHY;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Time {
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

impl Default for Time {
    fn default() -> Self {
        Self {
            hour: 12,
            minute: 0,
            second: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimePickerPopupState {
    pub open: bool,
    pub use_24_hour_format: bool,
    pub current_time: Time,
    pub minutes_selected: bool,
    pub am_selected: bool,
}

impl Default for TimePickerPopupState {
    fn default() -> Self {
        Self {
            open: false,
            use_24_hour_format: true,
            current_time: Time::default(),
            minutes_selected: false,
            am_selected: true,
        }
    }
}

pub struct TimePickerPopupResponse {
    pub accepted: bool,
    pub cancelled: bool,
    pub popup_open: bool,
    pub current_time: Time,
}

pub struct TimePickerPopup<'a> {
    pub id: Id,
    pub title: Cow<'a, str>,
    pub cancel_text: Cow<'a, str>,
    pub ok_text: Cow<'a, str>,
    pub hour_label: Cow<'a, str>,
    pub minute_label: Cow<'a, str>,
    pub state: &'a mut TimePickerPopupState,
}

pub struct TimePickerInput<'a> {
    pub text: Cow<'a, str>,
    pub checked: bool,
}

impl<'a> TimePickerInput<'a> {
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = material_palette_for_visuals(ui.visuals());
        let metrics = material_style_metrics();
        let fill = if self.checked {
            palette.primary_container
        } else {
            palette.surface_container_highest
        };
        let text_color = if self.checked {
            palette.on_primary_container
        } else {
            palette.on_surface
        };
        let button = Button::new(
            RichText::new(self.text.as_ref())
                .size(MATERIAL_TYPOGRAPHY.display_large.font_size_px)
                .color(text_color),
        )
        .min_size(vec2(metrics.sizes.size_80, metrics.sizes.size_80))
        .fill(fill)
        .stroke(Stroke::NONE)
        .corner_radius(CornerRadius::same(
            metrics.corner_radii.border_radius_8.round() as u8,
        ));
        ui.add(button)
    }
}

impl<'a> TimePickerPopup<'a> {
    pub fn show(self, ui: &mut Ui) -> TimePickerPopupResponse {
        normalize_state(self.state);
        let mut accepted = false;
        let mut cancelled = false;

        if self.state.open {
            let mut open = self.state.open;
            egui::Window::new(self.title.as_ref())
                .id(self.id)
                .collapsible(false)
                .resizable(false)
                .open(&mut open)
                .show(ui.ctx(), |ui| {
                    let metrics = material_style_metrics();
                    ui.set_min_width(metrics.sizes.size_200 * 2.0);
                    let _ = material_text(ui, self.title.as_ref())
                        .text_style(MATERIAL_TYPOGRAPHY.title_large)
                        .show(ui);
                    ui.add_space(metrics.spacings.spacing_16);

                    ui.horizontal(|ui| {
                        let hour_response = TimePickerInput {
                            text: format!("{:02}", display_hour(self.state)).into(),
                            checked: !self.state.minutes_selected,
                        }
                        .show(ui);
                        if hour_response.clicked() {
                            self.state.minutes_selected = false;
                        }

                        let _ = material_text(ui, ":")
                            .text_style(MATERIAL_TYPOGRAPHY.display_large)
                            .show(ui);

                        let minute_response = TimePickerInput {
                            text: format!("{:02}", self.state.current_time.minute).into(),
                            checked: self.state.minutes_selected,
                        }
                        .show(ui);
                        if minute_response.clicked() {
                            self.state.minutes_selected = true;
                        }

                        if !self.state.use_24_hour_format {
                            ui.vertical(|ui| {
                                let am = ui.selectable_label(self.state.am_selected, "AM");
                                let pm = ui.selectable_label(!self.state.am_selected, "PM");
                                if am.clicked() {
                                    self.state.am_selected = true;
                                    apply_period(self.state);
                                }
                                if pm.clicked() {
                                    self.state.am_selected = false;
                                    apply_period(self.state);
                                }
                            });
                        }
                    });

                    ui.add_space(metrics.spacings.spacing_16);

                    let range = if self.state.minutes_selected {
                        0..=59
                    } else if self.state.use_24_hour_format {
                        0..=23
                    } else {
                        1..=12
                    };
                    let value = if self.state.minutes_selected {
                        &mut self.state.current_time.minute
                    } else {
                        &mut self.state.current_time.hour
                    };

                    let label = if self.state.minutes_selected {
                        self.minute_label.as_ref()
                    } else {
                        self.hour_label.as_ref()
                    };
                    let _ = material_text(ui, label)
                        .text_style(MATERIAL_TYPOGRAPHY.label_large)
                        .show(ui);
                    let _ = ui.add(egui::Slider::new(value, range).show_value(true));

                    if !self.state.use_24_hour_format && !self.state.minutes_selected {
                        apply_period(self.state);
                    }

                    ui.add_space(metrics.spacings.spacing_16);
                    ui.horizontal(|ui| {
                        if OutlineButton::new(self.cancel_text.as_ref())
                            .show(ui)
                            .clicked()
                        {
                            cancelled = true;
                            self.state.open = false;
                        }
                        if FilledButton::new(self.ok_text.as_ref()).show(ui).clicked() {
                            accepted = true;
                            self.state.open = false;
                        }
                    });
                });
            self.state.open = self.state.open && open;
        }

        TimePickerPopupResponse {
            accepted,
            cancelled,
            popup_open: self.state.open,
            current_time: self.state.current_time,
        }
    }
}

fn normalize_state(state: &mut TimePickerPopupState) {
    state.current_time.minute = state.current_time.minute.clamp(0, 59);
    state.current_time.second = state.current_time.second.clamp(0, 59);
    if state.use_24_hour_format {
        state.current_time.hour = state.current_time.hour.clamp(0, 23);
    } else {
        if state.current_time.hour <= 0 {
            state.current_time.hour = 12;
        }
        if state.current_time.hour > 23 {
            state.current_time.hour %= 24;
        }
        state.am_selected = state.current_time.hour < 12;
    }
}

fn display_hour(state: &TimePickerPopupState) -> i32 {
    if state.use_24_hour_format {
        state.current_time.hour
    } else {
        match state.current_time.hour % 12 {
            0 => 12,
            hour => hour,
        }
    }
}

fn apply_period(state: &mut TimePickerPopupState) {
    if state.use_24_hour_format {
        return;
    }
    let display = state.current_time.hour.clamp(1, 12);
    state.current_time.hour = match (state.am_selected, display) {
        (true, 12) => 0,
        (true, hour) => hour,
        (false, 12) => 12,
        (false, hour) => hour + 12,
    };
}

#[cfg(test)]
mod tests {
    use egui::Context;
    use egui::Id;

    use super::Time;
    use super::TimePickerPopup;
    use super::TimePickerPopupState;
    use super::display_hour;
    use super::normalize_state;

    #[test]
    fn time_picker_popup_renders_without_panicking() {
        let context = Context::default();
        let mut popup_open = false;
        let mut state = TimePickerPopupState {
            open: true,
            use_24_hour_format: false,
            current_time: Time {
                hour: 15,
                minute: 20,
                second: 0,
            },
            minutes_selected: false,
            am_selected: false,
        };
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                popup_open = TimePickerPopup {
                    id: Id::new("time-picker"),
                    title: "Select time".into(),
                    cancel_text: "Cancel".into(),
                    ok_text: "Ok".into(),
                    hour_label: "Hour".into(),
                    minute_label: "Minute".into(),
                    state: &mut state,
                }
                .show(ui)
                .popup_open;
            });
        });
        assert!(popup_open);
    }

    #[test]
    fn normalize_state_clamps_invalid_values() {
        let mut state = TimePickerPopupState {
            open: false,
            use_24_hour_format: true,
            current_time: Time {
                hour: 99,
                minute: 99,
                second: -1,
            },
            minutes_selected: false,
            am_selected: true,
        };
        normalize_state(&mut state);
        assert_eq!(state.current_time.hour, 23);
        assert_eq!(state.current_time.minute, 59);
        assert_eq!(state.current_time.second, 0);
    }

    #[test]
    fn display_hour_maps_midnight_to_twelve_hour_clock() {
        let state = TimePickerPopupState {
            open: false,
            use_24_hour_format: false,
            current_time: Time {
                hour: 0,
                minute: 0,
                second: 0,
            },
            minutes_selected: false,
            am_selected: true,
        };
        assert_eq!(display_hour(&state), 12);
    }
}
