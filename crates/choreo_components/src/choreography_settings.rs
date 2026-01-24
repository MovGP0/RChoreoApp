use std::cell::RefCell;
use std::rc::Rc;

use choreo_master_mobile_json::{Color, SceneId};
use crossbeam_channel::Sender;

use crate::behavior::{Behavior, CompositeDisposable};
use crate::global::GlobalStateModel;
use crate::logging::BehaviorLog;
use crate::preferences::Preferences;
use crate::scenes::SceneViewModel;
use choreo_models::{SceneModel, SettingsModel};

#[derive(Debug, Clone, PartialEq)]
pub struct GridSizeOption {
    pub value: i32,
    pub display: String,
}

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
    pub date: String,
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
    const MAX_SCENE_TIMESTAMP_SECONDS: f64 = 1440.0 * 60.0;

    pub fn new(mut behaviors: Vec<Box<dyn Behavior<ChoreographySettingsViewModel>>>) -> Self {
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
            date: String::new(),
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
        for behavior in behaviors.drain(..) {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedrawFloorCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShowTimestampsChangedEvent {
    pub is_enabled: bool,
}

pub struct LoadChoreographySettingsBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
}

impl LoadChoreographySettingsBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>) -> Self {
        Self { global_state }
    }

    pub fn load(&self, view_model: &mut ChoreographySettingsViewModel) {
        let global_state = self.global_state.borrow();
        let choreography = &global_state.choreography;
        if choreography.name.is_empty()
            && choreography.scenes.is_empty()
            && choreography.comment.is_none()
        {
            reset_view_model(view_model);
            return;
        }

        map_from_choreography(choreography, view_model);
        update_selected_scene(view_model, &global_state.selected_scene, choreography);
    }
}

impl Behavior<ChoreographySettingsViewModel> for LoadChoreographySettingsBehavior {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "LoadChoreographySettingsBehavior",
            "ChoreographySettingsViewModel",
        );
        self.load(view_model);
    }
}

pub struct LoadSettingsPreferencesBehavior<P: Preferences> {
    preferences: P,
}

impl<P: Preferences> LoadSettingsPreferencesBehavior<P> {
    pub fn new(preferences: P) -> Self {
        Self { preferences }
    }

    pub fn load(&self, settings: &mut SettingsModel) {
        settings.show_timestamps = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SHOW_TIMESTAMPS, true);
        settings.positions_at_side = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::POSITIONS_AT_SIDE, true);
        settings.snap_to_grid = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SNAP_TO_GRID, true);
    }
}

impl<P: Preferences> Behavior<SettingsModel> for LoadSettingsPreferencesBehavior<P> {
    fn activate(&self, settings: &mut SettingsModel, _disposables: &mut CompositeDisposable) {
        BehaviorLog::behavior_activated(
            "LoadSettingsPreferencesBehavior",
            "SettingsModel",
        );
        self.load(settings);
    }
}

pub struct UpdateAuthorBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateAuthorBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_author(&self, value: &str) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.author = normalize_text(value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateAuthorBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("UpdateAuthorBehavior", "ChoreographySettingsViewModel");
    }
}

pub struct UpdateCommentBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateCommentBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_comment(&self, value: &str) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.comment = normalize_text(value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateCommentBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("UpdateCommentBehavior", "ChoreographySettingsViewModel");
    }
}

pub struct UpdateDateBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateDateBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_date(&self, value: &str) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.date = normalize_text(value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateDateBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("UpdateDateBehavior", "ChoreographySettingsViewModel");
    }
}

pub struct UpdateDescriptionBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateDescriptionBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_description(&self, value: &str) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.description = normalize_text(value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateDescriptionBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateDescriptionBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

pub struct UpdateDrawPathFromBehavior<P: Preferences> {
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl<P: Preferences> UpdateDrawPathFromBehavior<P> {
    pub fn new(preferences: P, redraw_sender: Sender<RedrawFloorCommand>) -> Self {
        Self {
            preferences,
            redraw_sender,
        }
    }

    pub fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        view_model.draw_path_from = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::DRAW_PATH_FROM, false);
    }

    pub fn update_draw_path_from(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.draw_path_from = value;
        self.preferences
            .set_bool(choreo_models::SettingsPreferenceKeys::DRAW_PATH_FROM, value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences> Behavior<ChoreographySettingsViewModel> for UpdateDrawPathFromBehavior<P> {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateDrawPathFromBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize(view_model);
    }
}

pub struct UpdateDrawPathToBehavior<P: Preferences> {
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl<P: Preferences> UpdateDrawPathToBehavior<P> {
    pub fn new(preferences: P, redraw_sender: Sender<RedrawFloorCommand>) -> Self {
        Self {
            preferences,
            redraw_sender,
        }
    }

    pub fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        view_model.draw_path_to = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::DRAW_PATH_TO, false);
    }

    pub fn update_draw_path_to(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.draw_path_to = value;
        self.preferences
            .set_bool(choreo_models::SettingsPreferenceKeys::DRAW_PATH_TO, value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences> Behavior<ChoreographySettingsViewModel> for UpdateDrawPathToBehavior<P> {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateDrawPathToBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize(view_model);
    }
}

pub struct UpdateFloorBackBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateFloorBackBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_floor_back(&self, value: i32) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.floor.size_back = value.clamp(0, 100);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateFloorBackBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateFloorBackBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

pub struct UpdateShowLegendBehavior<P: Preferences> {
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl<P: Preferences> UpdateShowLegendBehavior<P> {
    pub fn new(preferences: P, redraw_sender: Sender<RedrawFloorCommand>) -> Self {
        Self {
            preferences,
            redraw_sender,
        }
    }

    pub fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        view_model.show_legend = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SHOW_LEGEND, false);
    }

    pub fn update_show_legend(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.show_legend = value;
        self.preferences
            .set_bool(choreo_models::SettingsPreferenceKeys::SHOW_LEGEND, value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences> Behavior<ChoreographySettingsViewModel> for UpdateShowLegendBehavior<P> {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateShowLegendBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize(view_model);
    }
}

pub struct UpdateShowTimestampsBehavior<P: Preferences> {
    global_state: Rc<RefCell<GlobalStateModel>>,
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
    show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
}

impl<P: Preferences> UpdateShowTimestampsBehavior<P> {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        preferences: P,
        redraw_sender: Sender<RedrawFloorCommand>,
        show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            redraw_sender,
            show_timestamps_sender,
        }
    }

    pub fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        let value = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SHOW_TIMESTAMPS, true);
        view_model.show_timestamps = value;
        self.global_state
            .borrow_mut()
            .choreography
            .settings
            .show_timestamps = value;
    }

    pub fn update_show_timestamps(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.show_timestamps = value;
        {
            let mut global_state = self.global_state.borrow_mut();
            global_state.choreography.settings.show_timestamps = value;
        }
        let _ = self.redraw_sender.send(RedrawFloorCommand);
        let _ = self
            .show_timestamps_sender
            .send(ShowTimestampsChangedEvent { is_enabled: value });
    }
}

impl<P: Preferences> Behavior<ChoreographySettingsViewModel> for UpdateShowTimestampsBehavior<P> {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateShowTimestampsBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize(view_model);
    }
}

pub struct UpdateSnapToGridBehavior<P: Preferences> {
    global_state: Rc<RefCell<GlobalStateModel>>,
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl<P: Preferences> UpdateSnapToGridBehavior<P> {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        preferences: P,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            redraw_sender,
        }
    }

    pub fn initialize(&self, view_model: &mut ChoreographySettingsViewModel) {
        let snap_to_grid = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::SNAP_TO_GRID, true);
        view_model.snap_to_grid = snap_to_grid;
        self.global_state
            .borrow_mut()
            .choreography
            .settings
            .snap_to_grid = snap_to_grid;
    }

    pub fn update_snap_to_grid(&self, view_model: &mut ChoreographySettingsViewModel, value: bool) {
        view_model.snap_to_grid = value;
        {
            let mut global_state = self.global_state.borrow_mut();
            global_state.choreography.settings.snap_to_grid = value;
        }
        self.preferences
            .set_bool(choreo_models::SettingsPreferenceKeys::SNAP_TO_GRID, value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences> Behavior<ChoreographySettingsViewModel> for UpdateSnapToGridBehavior<P> {
    fn activate(
        &self,
        view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateSnapToGridBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize(view_model);
    }
}

pub struct UpdateSubtitleBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateSubtitleBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_subtitle(&self, value: &str) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.subtitle = normalize_text(value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateSubtitleBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateSubtitleBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

pub struct UpdateTransparencyBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateTransparencyBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_transparency(&self, value: f64) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.settings.transparency = value.clamp(0.0, 1.0);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateTransparencyBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateTransparencyBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

pub struct UpdateVariationBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateVariationBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_variation(&self, value: &str) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.variation = normalize_text(value);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateVariationBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateVariationBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

pub struct UpdateFloorColorBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateFloorColorBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_floor_color(&self, color: Color) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.settings.floor_color = color;
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateFloorColorBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateFloorColorBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

pub struct UpdateFloorFrontBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateFloorFrontBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_floor_front(&self, value: i32) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.floor.size_front = value.clamp(0, 100);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateFloorFrontBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateFloorFrontBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

pub struct UpdateFloorLeftBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateFloorLeftBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_floor_left(&self, value: i32) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.floor.size_left = value.clamp(0, 100);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateFloorLeftBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateFloorLeftBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

pub struct UpdateFloorRightBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateFloorRightBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_floor_right(&self, value: i32) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.floor.size_right = value.clamp(0, 100);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateFloorRightBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateFloorRightBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

pub struct UpdateGridLinesBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateGridLinesBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_grid_lines(&self, value: bool) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.settings.grid_lines = value;
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateGridLinesBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateGridLinesBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

pub struct UpdateGridResolutionBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateGridResolutionBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_grid_resolution(&self, value: i32) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.settings.resolution = value.clamp(1, 16);
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateGridResolutionBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateGridResolutionBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

pub struct UpdateNameBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl UpdateNameBehavior {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            redraw_sender,
        }
    }

    pub fn update_name(&self, value: &str) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.name = value.trim().to_string();
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateNameBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated("UpdateNameBehavior", "ChoreographySettingsViewModel");
    }
}

pub struct UpdatePositionsAtSideBehavior<P: Preferences> {
    global_state: Rc<RefCell<GlobalStateModel>>,
    preferences: P,
    redraw_sender: Sender<RedrawFloorCommand>,
}

impl<P: Preferences> UpdatePositionsAtSideBehavior<P> {
    pub fn new(
        global_state: Rc<RefCell<GlobalStateModel>>,
        preferences: P,
        redraw_sender: Sender<RedrawFloorCommand>,
    ) -> Self {
        Self {
            global_state,
            preferences,
            redraw_sender,
        }
    }

    pub fn initialize(&self) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.settings.positions_at_side = self
            .preferences
            .get_bool(choreo_models::SettingsPreferenceKeys::POSITIONS_AT_SIDE, true);
    }

    pub fn update_positions_at_side(&self, value: bool) {
        let mut global_state = self.global_state.borrow_mut();
        global_state.choreography.settings.positions_at_side = value;
        let _ = self.redraw_sender.send(RedrawFloorCommand);
    }
}

impl<P: Preferences> Behavior<ChoreographySettingsViewModel> for UpdatePositionsAtSideBehavior<P> {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdatePositionsAtSideBehavior",
            "ChoreographySettingsViewModel",
        );
        self.initialize();
    }
}

pub struct UpdateSelectedSceneBehavior {
    global_state: Rc<RefCell<GlobalStateModel>>,
    is_updating: bool,
}

impl UpdateSelectedSceneBehavior {
    pub fn new(global_state: Rc<RefCell<GlobalStateModel>>) -> Self {
        Self {
            global_state,
            is_updating: false,
        }
    }

    pub fn sync_from_selected(&mut self, view_model: &mut ChoreographySettingsViewModel) {
        let global_state = self.global_state.borrow();
        self.is_updating = true;
        update_selected_scene(
            view_model,
            &global_state.selected_scene,
            &global_state.choreography,
        );
        self.is_updating = false;
    }

    pub fn update_scene_name(&mut self, view_model: &mut ChoreographySettingsViewModel) {
        if self.is_updating {
            return;
        }
        let mut global_state = self.global_state.borrow_mut();
        let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
        if let Some(scene) = global_state.selected_scene.as_mut() {
            scene.name = view_model.scene_name.clone();
        }
        if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
            model_scene.name = view_model.scene_name.clone();
        }
    }

    pub fn update_scene_text(&mut self, view_model: &mut ChoreographySettingsViewModel) {
        if self.is_updating {
            return;
        }
        let mut global_state = self.global_state.borrow_mut();
        let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
        if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
            model_scene.text = normalize_text(&view_model.scene_text);
        }
    }

    pub fn update_scene_fixed_positions(&mut self, view_model: &mut ChoreographySettingsViewModel) {
        if self.is_updating {
            return;
        }
        let mut global_state = self.global_state.borrow_mut();
        let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
        if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
            model_scene.fixed_positions = view_model.scene_fixed_positions;
        }
    }

    pub fn update_scene_color(&mut self, view_model: &mut ChoreographySettingsViewModel) {
        if self.is_updating {
            return;
        }
        let mut global_state = self.global_state.borrow_mut();
        let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
        if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
            model_scene.color = view_model.scene_color.clone();
        }
    }

    pub fn update_scene_timestamp(&mut self, view_model: &mut ChoreographySettingsViewModel) {
        if self.is_updating {
            return;
        }
        let mut global_state = self.global_state.borrow_mut();
        let scene_id = global_state.selected_scene.as_ref().map(|scene| scene.scene_id);
        let timestamp = if view_model.scene_has_timestamp {
            Some(format_seconds(view_model.scene_timestamp_seconds))
        } else {
            None
        };
        if let Some(scene) = global_state.selected_scene.as_mut() {
            scene.timestamp = if view_model.scene_has_timestamp {
                Some(view_model.scene_timestamp_seconds)
            } else {
                None
            };
        }
        if let Some(model_scene) = find_scene_mut(&mut global_state.choreography, scene_id) {
            model_scene.timestamp = timestamp;
        }
    }
}

impl Behavior<ChoreographySettingsViewModel> for UpdateSelectedSceneBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "UpdateSelectedSceneBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

fn clamp_scene_timestamp(seconds: f64) -> f64 {
    if seconds < 0.0 {
        return 0.0;
    }

    if seconds > ChoreographySettingsViewModel::MAX_SCENE_TIMESTAMP_SECONDS {
        return ChoreographySettingsViewModel::MAX_SCENE_TIMESTAMP_SECONDS;
    }

    seconds
}

fn build_grid_size_options() -> Vec<GridSizeOption> {
    let mut options = Vec::new();
    for denominator in 1..=16 {
        let centimeters = 100.0 / denominator as f64;
        let centimeters_text = format_decimal(centimeters);
        let display = format!("1/{denominator} m ({centimeters_text} cm)");
        options.push(GridSizeOption {
            value: denominator,
            display,
        });
    }
    options
}

fn format_decimal(value: f64) -> String {
    let mut text = format!("{value:.2}");
    while text.ends_with('0') {
        text.pop();
    }
    if text.ends_with('.') {
        text.pop();
    }
    text
}

fn round_to_two_decimals(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub struct DecimalDoubleConverter;

impl DecimalDoubleConverter {
    pub fn convert(value: f64) -> f64 {
        value
    }

    pub fn convert_back(value: f64) -> f64 {
        round_to_two_decimals(value.clamp(0.0, 1.0))
    }
}

pub struct ChoreographySettingsBehavior;

impl Behavior<ChoreographySettingsViewModel> for ChoreographySettingsBehavior {
    fn activate(
        &self,
        _view_model: &mut ChoreographySettingsViewModel,
        _disposables: &mut CompositeDisposable,
    ) {
        BehaviorLog::behavior_activated(
            "ChoreographySettingsBehavior",
            "ChoreographySettingsViewModel",
        );
    }
}

fn reset_view_model(view_model: &mut ChoreographySettingsViewModel) {
    view_model.comment.clear();
    view_model.name.clear();
    view_model.subtitle.clear();
    view_model.variation.clear();
    view_model.author.clear();
    view_model.description.clear();
    view_model.date.clear();
    view_model.floor_front = 0;
    view_model.floor_back = 0;
    view_model.floor_left = 0;
    view_model.floor_right = 0;
    view_model.set_grid_resolution(1);
    view_model.transparency = 0.0;
    view_model.positions_at_side = false;
    view_model.grid_lines = false;
    view_model.snap_to_grid = true;
    view_model.floor_color = Color::transparent();
    view_model.show_timestamps = false;
    view_model.has_selected_scene = false;
    view_model.scene_name.clear();
    view_model.scene_text.clear();
    view_model.scene_fixed_positions = false;
    view_model.scene_has_timestamp = false;
    view_model.scene_timestamp_seconds = 0.0;
    view_model.scene_timestamp_minutes = 0;
    view_model.scene_timestamp_seconds_part = 0;
    view_model.scene_timestamp_millis = 0;
    view_model.scene_color = Color::transparent();
}

fn map_from_choreography(
    choreography: &choreo_models::ChoreographyModel,
    view_model: &mut ChoreographySettingsViewModel,
) {
    view_model.comment = choreography.comment.clone().unwrap_or_default();
    view_model.name = choreography.name.clone();
    view_model.subtitle = choreography.subtitle.clone().unwrap_or_default();
    view_model.variation = choreography.variation.clone().unwrap_or_default();
    view_model.author = choreography.author.clone().unwrap_or_default();
    view_model.description = choreography.description.clone().unwrap_or_default();
    view_model.date = choreography.date.clone().unwrap_or_default();

    view_model.floor_front = choreography.floor.size_front;
    view_model.floor_back = choreography.floor.size_back;
    view_model.floor_left = choreography.floor.size_left;
    view_model.floor_right = choreography.floor.size_right;
    view_model.set_grid_resolution(choreography.settings.resolution);
    view_model.transparency = choreography.settings.transparency;
    view_model.positions_at_side = choreography.settings.positions_at_side;
    view_model.grid_lines = choreography.settings.grid_lines;
    view_model.snap_to_grid = choreography.settings.snap_to_grid;
    view_model.floor_color = choreography.settings.floor_color.clone();
    view_model.show_timestamps = choreography.settings.show_timestamps;
}

pub struct ChoreographySettingsMapper;

impl ChoreographySettingsMapper {
    pub fn map_to_view_model(
        &self,
        source: &choreo_models::ChoreographyModel,
        target: &mut ChoreographySettingsViewModel,
    ) {
        map_from_choreography(source, target);
    }

    pub fn map_to_model(
        &self,
        source: &ChoreographySettingsViewModel,
        target: &mut choreo_models::ChoreographyModel,
    ) {
        target.comment = normalize_text(&source.comment);
        target.name = source.name.clone();
        target.subtitle = normalize_text(&source.subtitle);
        target.variation = normalize_text(&source.variation);
        target.author = normalize_text(&source.author);
        target.description = normalize_text(&source.description);
        target.date = normalize_text(&source.date);

        target.settings.resolution = clamp_grid_resolution(source.grid_resolution());
        target.settings.transparency = clamp_transparency(source.transparency);
        target.settings.positions_at_side = source.positions_at_side;
        target.settings.grid_lines = source.grid_lines;
        target.settings.snap_to_grid = source.snap_to_grid;
        target.settings.floor_color = source.floor_color.clone();
        target.settings.show_timestamps = source.show_timestamps;
        target.floor.size_front = clamp_floor_size(source.floor_front);
        target.floor.size_back = clamp_floor_size(source.floor_back);
        target.floor.size_left = clamp_floor_size(source.floor_left);
        target.floor.size_right = clamp_floor_size(source.floor_right);
    }
}

pub struct ChoreographySettingsDependencies<P: Preferences> {
    pub global_state: Rc<RefCell<GlobalStateModel>>,
    pub preferences: P,
    pub redraw_sender: Sender<RedrawFloorCommand>,
    pub show_timestamps_sender: Sender<ShowTimestampsChangedEvent>,
}

pub fn build_choreography_settings_behaviors<P: Preferences + Clone + 'static>(
    deps: ChoreographySettingsDependencies<P>,
) -> Vec<Box<dyn Behavior<ChoreographySettingsViewModel>>> {
    let global_state = deps.global_state;
    let preferences = deps.preferences;
    let redraw_sender = deps.redraw_sender;
    let show_timestamps_sender = deps.show_timestamps_sender;

    vec![
        Box::new(LoadChoreographySettingsBehavior::new(
            global_state.clone(),
        )),
        Box::new(UpdateSelectedSceneBehavior::new(
            global_state.clone(),
        )),
        Box::new(UpdateCommentBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateNameBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateSubtitleBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateDateBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateVariationBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateAuthorBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateDescriptionBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateFloorFrontBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateFloorBackBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateFloorLeftBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateFloorRightBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateGridResolutionBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateDrawPathFromBehavior::new(
            preferences.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateDrawPathToBehavior::new(
            preferences.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateGridLinesBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateSnapToGridBehavior::new(
            global_state.clone(),
            preferences.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateFloorColorBehavior::new(
            global_state.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdateShowTimestampsBehavior::new(
            global_state.clone(),
            preferences.clone(),
            redraw_sender.clone(),
            show_timestamps_sender.clone(),
        )),
        Box::new(UpdateShowLegendBehavior::new(
            preferences.clone(),
            redraw_sender.clone(),
        )),
        Box::new(UpdatePositionsAtSideBehavior::new(
            global_state.clone(),
            preferences,
            redraw_sender.clone(),
        )),
        Box::new(UpdateTransparencyBehavior::new(
            global_state,
            redraw_sender.clone(),
        )),
    ]
}

pub fn build_settings_model_behaviors<P: Preferences + Clone + 'static>(
    preferences: P,
) -> Vec<Box<dyn Behavior<SettingsModel>>> {
    vec![Box::new(LoadSettingsPreferencesBehavior::new(
        preferences,
    ))]
}

fn update_selected_scene(
    view_model: &mut ChoreographySettingsViewModel,
    selected_scene: &Option<SceneViewModel>,
    choreography: &choreo_models::ChoreographyModel,
) {
    let Some(selected_scene) = selected_scene else {
        view_model.has_selected_scene = false;
        view_model.scene_name.clear();
        view_model.scene_text.clear();
        view_model.scene_fixed_positions = false;
        view_model.scene_has_timestamp = false;
        view_model.scene_timestamp_seconds = 0.0;
        view_model.scene_timestamp_minutes = 0;
        view_model.scene_timestamp_seconds_part = 0;
        view_model.scene_timestamp_millis = 0;
        view_model.scene_color = Color::transparent();
        return;
    };

    let scene_model = choreography
        .scenes
        .iter()
        .find(|scene| scene.scene_id == selected_scene.scene_id);

    view_model.has_selected_scene = scene_model.is_some();
    if let Some(scene_model) = scene_model {
        view_model.scene_name = scene_model.name.clone();
        view_model.scene_text = scene_model.text.clone().unwrap_or_default();
        view_model.scene_fixed_positions = scene_model.fixed_positions;
        view_model.scene_has_timestamp = scene_model.timestamp.is_some();
        let seconds = scene_model
            .timestamp
            .as_deref()
            .and_then(parse_timestamp_seconds)
            .unwrap_or(0.0);
        view_model.set_scene_timestamp_seconds(seconds);
        view_model.scene_color = scene_model.color.clone();
    }
}

fn parse_timestamp_seconds(value: &str) -> Option<f64> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }

    let mut parts = value.split(':').collect::<Vec<_>>();
    if parts.len() > 3 {
        return None;
    }

    let seconds_part = parts.pop()?;
    let minutes_part = parts.pop().unwrap_or("0");
    let hours_part = parts.pop().unwrap_or("0");

    let seconds = seconds_part.parse::<f64>().ok()?;
    let minutes = minutes_part.parse::<f64>().ok()?;
    let hours = hours_part.parse::<f64>().ok()?;

    Some(hours * 3600.0 + minutes * 60.0 + seconds)
}

fn normalize_text(value: &str) -> Option<String> {
    if value.trim().is_empty() {
        None
    } else {
        Some(value.trim().to_string())
    }
}

fn clamp_floor_size(value: i32) -> i32 {
    value.clamp(0, 100)
}

fn clamp_grid_resolution(value: i32) -> i32 {
    value.clamp(1, 16)
}

fn clamp_transparency(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

fn find_scene_mut(
    choreography: &mut choreo_models::ChoreographyModel,
    scene_id: Option<SceneId>,
) -> Option<&mut SceneModel> {
    let scene_id = scene_id?;
    choreography
        .scenes
        .iter_mut()
        .find(|scene| scene.scene_id == scene_id)
}

fn format_seconds(value: f64) -> String {
    let mut text = format!("{value:.3}");
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
