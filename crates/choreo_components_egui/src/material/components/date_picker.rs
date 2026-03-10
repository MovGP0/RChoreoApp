use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use egui::Align;
use egui::Align2;
use egui::Button;
use egui::CornerRadius;
use egui::Grid;
use egui::Image;
use egui::Layout;
use egui::RichText;
use egui::Stroke;
use egui::Ui;
use egui::Window;
use egui::vec2;
use time::Date;
use time::Month;
use time::OffsetDateTime;
use time::Weekday;

use crate::material::components::centered_icon_rect;
use crate::material::components::paint_icon;

const FIELD_WIDTH_PX: f32 = 192.0;
const FIELD_HEIGHT_PX: f32 = 48.0;
const POPUP_WIDTH_PX: f32 = 360.0;
const DAY_CELL_SIZE_PX: f32 = 48.0;
const YEAR_CELL_WIDTH_PX: f32 = 96.0;
const ICON_GLYPH_SIZE_PX: f32 = 24.0;
const YEAR_GRID_COLUMNS: usize = 3;
const YEAR_GRID_ROWS: usize = 5;

const WEEKDAY_LABELS: [&str; 7] = ["M", "T", "W", "T", "F", "S", "S"];
const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
const MONTH_NAMES_SHORT: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];
const WEEKDAY_NAMES_SHORT: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DatePickerValue {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl Default for DatePickerValue {
    fn default() -> Self {
        today_value()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DatePickerStrings<'a> {
    pub title: &'a str,
    pub cancel_text: &'a str,
    pub ok_text: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DatePickerPopupState {
    is_open: bool,
    display_date: DatePickerValue,
    current_date: DatePickerValue,
    year_selection: bool,
}

impl DatePickerPopupState {
    fn closed(value: DatePickerValue) -> Self {
        Self {
            is_open: false,
            display_date: display_date_for(value),
            current_date: value,
            year_selection: false,
        }
    }

    fn open(value: DatePickerValue) -> Self {
        Self {
            is_open: true,
            ..Self::closed(value)
        }
    }
}

pub fn draw(
    ui: &mut Ui,
    id_source: &str,
    value: DatePickerValue,
    strings: DatePickerStrings<'_>,
) -> Option<DatePickerValue> {
    let component_id = ui.make_persistent_id((id_source, "date_picker"));
    let popup_id = component_id.with("popup");
    let normalized_value = normalize_date_value(value);
    let mut popup_state = ui
        .data_mut(|data| data.get_temp::<DatePickerPopupState>(component_id))
        .unwrap_or_else(|| DatePickerPopupState::closed(normalized_value));
    if !popup_state.is_open {
        popup_state = DatePickerPopupState::closed(normalized_value);
    }

    if draw_trigger_field(ui, normalized_value).clicked() {
        popup_state = DatePickerPopupState::open(normalized_value);
    }

    let mut accepted = None;
    if popup_state.is_open {
        let mut open = true;
        Window::new(strings.title)
            .id(popup_id)
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, vec2(0.0, 0.0))
            .default_width(POPUP_WIDTH_PX)
            .open(&mut open)
            .show(ui.ctx(), |ui| {
                ui.set_min_width(POPUP_WIDTH_PX);
                ui.spacing_mut().item_spacing = vec2(12.0, 12.0);

                ui.label(RichText::new(current_day_text(popup_state.current_date)).strong());
                ui.separator();
                draw_month_row(ui, &mut popup_state);

                if popup_state.year_selection {
                    draw_year_grid(ui, &mut popup_state);
                } else {
                    draw_calendar_grid(ui, &mut popup_state);
                }

                ui.separator();
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(strings.ok_text).clicked() {
                        accepted = Some(popup_state.current_date);
                        popup_state.is_open = false;
                    }
                    if ui.button(strings.cancel_text).clicked() {
                        popup_state.is_open = false;
                    }
                });
            });

        if !open {
            popup_state.is_open = false;
        }
    }

    ui.data_mut(|data| {
        data.insert_temp(component_id, popup_state);
    });

    accepted
}

#[must_use]
pub fn normalize_date_value(value: DatePickerValue) -> DatePickerValue {
    if is_valid_date(value.year, value.month, value.day) {
        return value;
    }

    today_value()
}

#[must_use]
pub fn is_valid_date(year: i32, month: u8, day: u8) -> bool {
    build_date(DatePickerValue { year, month, day }).is_some()
}

#[must_use]
pub fn month_day_count(month: u8, year: i32) -> u8 {
    if !(1..=12).contains(&month) {
        return 0;
    }

    let next = if month == 12 {
        Date::from_calendar_date(year + 1, Month::January, 1)
    } else {
        Date::from_calendar_date(year, month_from_u8(month + 1), 1)
    };
    let current = Date::from_calendar_date(year, month_from_u8(month), 1);
    match (current, next) {
        (Ok(current), Ok(next)) => (next - current).whole_days() as u8,
        _ => 0,
    }
}

#[must_use]
pub fn month_offset(month: u8, year: i32) -> usize {
    let Some(date) = build_date(DatePickerValue {
        year,
        month,
        day: 1,
    }) else {
        return 0;
    };

    weekday_index(date.weekday())
}

fn draw_trigger_field(ui: &mut Ui, value: DatePickerValue) -> egui::Response {
    let icon = calendar_icon()
        .fit_to_exact_size(vec2(ICON_GLYPH_SIZE_PX, ICON_GLYPH_SIZE_PX))
        .tint(ui.visuals().widgets.inactive.fg_stroke.color);
    let button = Button::image_and_text(icon, format_date_text(value))
        .min_size(vec2(FIELD_WIDTH_PX, FIELD_HEIGHT_PX))
        .wrap_mode(egui::TextWrapMode::Truncate)
        .fill(ui.visuals().extreme_bg_color)
        .stroke(ui.visuals().widgets.inactive.bg_stroke)
        .corner_radius(CornerRadius::same(8));
    ui.add(button)
}

fn draw_month_row(ui: &mut Ui, popup_state: &mut DatePickerPopupState) {
    ui.horizontal(|ui| {
        let month_toggle = Button::new(month_year_text(
            popup_state.display_date.month,
            popup_state.display_date.year,
        ))
        .right_text(RichText::new("v"))
        .selected(popup_state.year_selection)
        .min_size(vec2(180.0, FIELD_HEIGHT_PX));
        if ui.add(month_toggle).clicked() {
            popup_state.year_selection = !popup_state.year_selection;
        }

        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            if add_month_nav_button(ui, next_month_icon()).clicked() {
                show_next_month(popup_state);
            }
            if add_month_nav_button(ui, previous_month_icon()).clicked() {
                show_previous_month(popup_state);
            }
        });
    });
}

fn draw_calendar_grid(ui: &mut Ui, popup_state: &mut DatePickerPopupState) {
    let today = today_value();
    Grid::new(ui.id().with("calendar"))
        .spacing(vec2(0.0, 0.0))
        .show(ui, |ui| {
            for label in WEEKDAY_LABELS {
                ui.add_sized(
                    vec2(DAY_CELL_SIZE_PX, DAY_CELL_SIZE_PX),
                    egui::Label::new(RichText::new(label).strong()),
                );
            }
            ui.end_row();

            let days_in_month = month_day_count(
                popup_state.display_date.month,
                popup_state.display_date.year,
            );
            let start_column = month_offset(
                popup_state.display_date.month,
                popup_state.display_date.year,
            );

            for cell_index in 0_i32..42 {
                let day_number = cell_index - start_column as i32 + 1;
                if day_number <= 0 || day_number > i32::from(days_in_month) {
                    ui.allocate_exact_size(
                        vec2(DAY_CELL_SIZE_PX, DAY_CELL_SIZE_PX),
                        egui::Sense::hover(),
                    );
                } else {
                    let value = DatePickerValue {
                        year: popup_state.display_date.year,
                        month: popup_state.display_date.month,
                        day: day_number as u8,
                    };
                    let response = ui.add_sized(
                        vec2(DAY_CELL_SIZE_PX, DAY_CELL_SIZE_PX),
                        day_button(ui, value, popup_state.current_date == value, today == value),
                    );
                    if response.clicked() {
                        popup_state.current_date = value;
                    }
                }

                if (cell_index + 1) % 7 == 0 {
                    ui.end_row();
                }
            }
        });
}

fn draw_year_grid(ui: &mut Ui, popup_state: &mut DatePickerPopupState) {
    let start_year =
        popup_state.display_date.year - ((YEAR_GRID_COLUMNS * YEAR_GRID_ROWS) as i32 / 2);
    Grid::new(ui.id().with("years"))
        .spacing(vec2(12.0, 12.0))
        .show(ui, |ui| {
            for index in 0..(YEAR_GRID_COLUMNS * YEAR_GRID_ROWS) {
                let year = start_year + index as i32;
                let is_selected = popup_state.display_date.year == year;
                let response = ui.add_sized(
                    vec2(YEAR_CELL_WIDTH_PX, DAY_CELL_SIZE_PX),
                    Button::new(year.to_string()).selected(is_selected),
                );
                if response.clicked() {
                    select_year(popup_state, year);
                }

                if (index + 1) % YEAR_GRID_COLUMNS == 0 {
                    ui.end_row();
                }
            }
        });
}

fn month_nav_button(image: Image<'static>) -> Button<'static> {
    let _ = image;
    Button::new("")
        .min_size(vec2(FIELD_HEIGHT_PX, FIELD_HEIGHT_PX))
        .corner_radius(CornerRadius::same(24))
}

fn add_month_nav_button(ui: &mut Ui, image: Image<'static>) -> egui::Response {
    let response = ui.add(month_nav_button(image.clone()));
    let tint = ui.style().interact(&response).fg_stroke.color;
    paint_icon(
        ui,
        &image,
        centered_icon_rect(
            response.rect,
            vec2(ICON_GLYPH_SIZE_PX, ICON_GLYPH_SIZE_PX),
        ),
        tint,
    );
    response
}

fn day_button(ui: &Ui, value: DatePickerValue, selected: bool, today: bool) -> Button<'static> {
    let mut button = Button::new(value.day.to_string()).corner_radius(CornerRadius::same(24));
    if selected {
        button = button
            .fill(ui.visuals().selection.bg_fill)
            .stroke(Stroke::new(1.0, ui.visuals().selection.bg_fill));
    } else if today {
        button = button.stroke(Stroke::new(1.0, ui.visuals().selection.stroke.color));
    }

    button.min_size(vec2(DAY_CELL_SIZE_PX, DAY_CELL_SIZE_PX))
}

fn show_next_month(popup_state: &mut DatePickerPopupState) {
    if popup_state.display_date.month >= 12 {
        popup_state.display_date.month = 1;
        popup_state.display_date.year += 1;
    } else {
        popup_state.display_date.month += 1;
    }
}

fn show_previous_month(popup_state: &mut DatePickerPopupState) {
    if popup_state.display_date.month <= 1 {
        popup_state.display_date.month = 12;
        popup_state.display_date.year -= 1;
    } else {
        popup_state.display_date.month -= 1;
    }
}

fn select_year(popup_state: &mut DatePickerPopupState, year: i32) {
    let value = DatePickerValue {
        year,
        month: 1,
        day: 1,
    };
    popup_state.display_date = value;
    popup_state.current_date = value;
    popup_state.year_selection = false;
}

fn build_date(value: DatePickerValue) -> Option<Date> {
    if value.year <= 0 {
        return None;
    }

    Date::from_calendar_date(value.year, month_from_u8(value.month), value.day).ok()
}

fn display_date_for(value: DatePickerValue) -> DatePickerValue {
    DatePickerValue { day: 1, ..value }
}

fn today_value() -> DatePickerValue {
    let now_nanos = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos() as i128,
        Err(_) => 0,
    };

    let date = OffsetDateTime::from_unix_timestamp_nanos(now_nanos)
        .map(|timestamp| timestamp.date())
        .unwrap_or_else(|_| {
            Date::from_calendar_date(1970, Month::January, 1).expect("static valid date")
        });

    DatePickerValue {
        year: date.year(),
        month: date.month() as u8,
        day: date.day(),
    }
}

fn current_day_text(value: DatePickerValue) -> String {
    let Some(date) = build_date(value) else {
        return format_date_text(value);
    };

    format!(
        "{}, {} {:02}",
        WEEKDAY_NAMES_SHORT[weekday_index(date.weekday())],
        MONTH_NAMES_SHORT[usize::from(value.month - 1)],
        value.day,
    )
}

fn format_date_text(value: DatePickerValue) -> String {
    format!("{:04}-{:02}-{:02}", value.year, value.month, value.day)
}

fn month_year_text(month: u8, year: i32) -> String {
    format!(
        "{} {}",
        MONTH_NAMES[usize::from(month.saturating_sub(1))],
        year
    )
}

fn calendar_icon() -> Image<'static> {
    Image::new(egui::include_image!(
        "../../../assets/icons/CalendarToday.svg"
    ))
}

fn previous_month_icon() -> Image<'static> {
    Image::new(egui::include_image!(
        "../../../assets/icons/ChevronLeft.svg"
    ))
}

fn next_month_icon() -> Image<'static> {
    Image::new(egui::include_image!(
        "../../../assets/icons/ChevronRight.svg"
    ))
}

const fn month_from_u8(month: u8) -> Month {
    match month {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        _ => Month::December,
    }
}

const fn weekday_index(weekday: Weekday) -> usize {
    match weekday {
        Weekday::Monday => 0,
        Weekday::Tuesday => 1,
        Weekday::Wednesday => 2,
        Weekday::Thursday => 3,
        Weekday::Friday => 4,
        Weekday::Saturday => 5,
        Weekday::Sunday => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::DatePickerValue;
    use super::display_date_for;
    use super::month_day_count;
    use super::month_offset;
    use super::normalize_date_value;

    #[test]
    fn normalize_date_value_falls_back_for_invalid_dates() {
        let normalized = normalize_date_value(DatePickerValue {
            year: 2026,
            month: 2,
            day: 31,
        });

        assert_ne!(
            normalized,
            DatePickerValue {
                year: 2026,
                month: 2,
                day: 31,
            }
        );
    }

    #[test]
    fn display_date_for_keeps_month_and_year_but_resets_day() {
        assert_eq!(
            display_date_for(DatePickerValue {
                year: 2026,
                month: 4,
                day: 18,
            }),
            DatePickerValue {
                year: 2026,
                month: 4,
                day: 1,
            }
        );
    }

    #[test]
    fn february_2028_has_twenty_nine_days() {
        assert_eq!(month_day_count(2, 2028), 29);
    }

    #[test]
    fn march_2026_starts_on_sunday_column() {
        assert_eq!(month_offset(3, 2026), 6);
    }
}
