use choreo_master_mobile_json::Color;
use crate::date;
use time::Date;

use crate::behavior::{Behavior, CompositeDisposable};
use nject::injectable;

#[derive(Debug, Clone, PartialEq)]
pub struct GridSizeOption {
    pub value: i32,
    pub display: String,
}

#[injectable]
#[inject(|behaviors: Vec<Box<dyn Behavior<ChoreographySettingsViewModel>>>| Self::new(behaviors))]
pub struct ChoreographySettingsViewModel {
    pub floor_size_options: Vec<i32>,
    pub grid_size_options: Vec<GridSizeOption>,
    pub selected_grid_size_option: GridSizeOption,
    pub floor_front: i32,
    pub floor_back: i32,
    pub floor_left: i32,
    pub floor_right: i32,
    pub draw_path_from: bool,
    pub draw_path_to: bool,
    pub grid_lines: bool,
    pub snap_to_grid: bool,
    pub floor_color: Color,
    pub show_timestamps: bool,
    pub positions_at_side: bool,
    pub show_legend: bool,
    pub transparency: f64,
    pub comment: String,
    pub name: String,
    pub subtitle: String,
    pub date: Date,
    pub variation: String,
    pub author: String,
    pub description: String,
    pub has_selected_scene: bool,
    pub scene_name: String,
    pub scene_text: String,
    pub scene_fixed_positions: bool,
    pub scene_has_timestamp: bool,
    pub scene_timestamp_seconds: f64,
    pub scene_timestamp_minutes: i32,
    pub scene_timestamp_seconds_part: i32,
    pub scene_timestamp_millis: i32,
    pub scene_color: Color,
    is_updating_scene_timestamp: bool,
    disposables: CompositeDisposable,
}

impl ChoreographySettingsViewModel {
    pub(crate) const MAX_SCENE_TIMESTAMP_SECONDS: f64 = 1440.0 * 60.0;

    pub fn new(behaviors: Vec<Box<dyn Behavior<ChoreographySettingsViewModel>>>) -> Self {
        let floor_size_options = (0..=100).collect::<Vec<_>>();
        let grid_size_options = build_grid_size_options();
        let selected_grid_size_option = grid_size_options
            .first()
            .cloned()
            .unwrap_or(GridSizeOption {
                value: 1,
                display: "1/1 m (100 cm)".to_string(),
            });

        let mut view_model = Self {
            floor_size_options,
            grid_size_options,
            selected_grid_size_option,
            floor_front: 0,
            floor_back: 0,
            floor_left: 0,
            floor_right: 0,
            draw_path_from: true,
            draw_path_to: true,
            grid_lines: true,
            snap_to_grid: true,
            floor_color: Color::transparent(),
            show_timestamps: false,
            positions_at_side: true,
            show_legend: false,
            transparency: 0.0,
            comment: String::new(),
            name: String::new(),
            subtitle: String::new(),
            date: date::today_date(),
            variation: String::new(),
            author: String::new(),
            description: String::new(),
            has_selected_scene: false,
            scene_name: String::new(),
            scene_text: String::new(),
            scene_fixed_positions: false,
            scene_has_timestamp: false,
            scene_timestamp_seconds: 0.0,
            scene_timestamp_minutes: 0,
            scene_timestamp_seconds_part: 0,
            scene_timestamp_millis: 0,
            scene_color: Color::transparent(),
            is_updating_scene_timestamp: false,
            disposables: CompositeDisposable::new(),
        };

        let mut disposables = CompositeDisposable::new();
        for behavior in behaviors.iter() {
            behavior.activate(&mut view_model, &mut disposables);
        }
        view_model.disposables = disposables;
        view_model
    }

    pub fn grid_resolution(&self) -> i32 {
        self.selected_grid_size_option
            .value
            .max(self.grid_size_options.first().map(|opt| opt.value).unwrap_or(1))
    }

    pub fn set_grid_resolution(&mut self, value: i32) {
        if let Some(option) = self.grid_size_options.iter().find(|opt| opt.value == value) {
            self.selected_grid_size_option = option.clone();
        } else if let Some(first) = self.grid_size_options.first().cloned() {
            self.selected_grid_size_option = first;
        }
    }

    pub fn set_scene_timestamp_seconds(&mut self, seconds: f64) {
        if self.is_updating_scene_timestamp {
            return;
        }

        self.is_updating_scene_timestamp = true;
        let clamped = clamp_scene_timestamp(seconds);
        let total_millis = (clamped * 1000.0).round() as i64;
        let minutes = (total_millis / 60000) as i32;
        let seconds_part = ((total_millis / 1000) % 60) as i32;
        let millis = (total_millis % 1000) as i32;
        let millis = (millis / 10) * 10;
        self.scene_timestamp_seconds = clamped;
        self.scene_timestamp_minutes = minutes;
        self.scene_timestamp_seconds_part = seconds_part;
        self.scene_timestamp_millis = millis;
        self.is_updating_scene_timestamp = false;
    }

    pub fn set_scene_timestamp_parts(&mut self, minutes: i32, seconds: i32, millis: i32) {
        if self.is_updating_scene_timestamp {
            return;
        }

        self.is_updating_scene_timestamp = true;
        let minutes = minutes.clamp(0, 1440);
        let seconds = seconds.clamp(0, 59);
        let millis = millis.clamp(0, 999);
        let millis = (millis / 10) * 10;
        let total = (minutes as f64 * 60.0)
            + (seconds as f64)
            + (millis as f64 / 1000.0);
        let clamped = clamp_scene_timestamp(total);
        let total_millis = (clamped * 1000.0).round() as i64;
        let minutes = (total_millis / 60000) as i32;
        let seconds_part = ((total_millis / 1000) % 60) as i32;
        let millis = (total_millis % 1000) as i32;
        let millis = (millis / 10) * 10;

        self.scene_timestamp_seconds = clamped;
        self.scene_timestamp_minutes = minutes;
        self.scene_timestamp_seconds_part = seconds_part;
        self.scene_timestamp_millis = millis;
        self.is_updating_scene_timestamp = false;
    }

    pub fn dispose(&mut self) {
        self.disposables.dispose_all();
    }
}

impl Default for ChoreographySettingsViewModel {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}


pub(crate) fn clamp_scene_timestamp(seconds: f64) -> f64 {
    seconds.clamp(0.0, ChoreographySettingsViewModel::MAX_SCENE_TIMESTAMP_SECONDS)
}

fn build_grid_size_options() -> Vec<GridSizeOption> {
    let options = [100, 50, 25, 20, 10, 5, 2, 1];
    options
        .iter()
        .map(|&centimeters| {
            let meters = centimeters as f64 / 100.0;
            let display = format_decimal(meters);
            GridSizeOption {
                value: centimeters,
                display: format!("{display} m ({centimeters} cm)"),
            }
        })
        .collect()
}

fn format_decimal(value: f64) -> String {
    let rounded = round_to_two_decimals(value);
    let mut text = format!("{rounded:.2}");
    if let Some(dot) = text.find('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
        if text.len() == dot {
            text.push('0');
        }
    }
    text
}

fn round_to_two_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

