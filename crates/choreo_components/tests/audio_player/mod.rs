#![allow(dead_code)]

use std::cell::Cell;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use choreo_components::audio_player::AudioPlayer;
use choreo_components::audio_player::AudioPlayerViewModel;
use choreo_components::behavior::Behavior;
use choreo_components::global::GlobalStateActor;
use choreo_components::preferences::InMemoryPreferences;
use choreo_components::scenes::SceneMapper;
use choreo_components::scenes::SceneViewModel;
use choreo_models::Colors;
use choreo_models::SceneModel;
use crossbeam_channel::unbounded;
use rspec::ConfigurationBuilder;
use rspec::Logger;
use rspec::Runner;

pub use rspec::report::Report;

thread_local! {
    static INIT_TEST_BACKEND: Cell<bool> = const { Cell::new(false) };
}

fn ensure_slint_test_backend() {
    INIT_TEST_BACKEND.with(|initialized| {
        if initialized.get() {
            return;
        }
        i_slint_backend_testing::init_no_event_loop();
        initialized.set(true);
    });
}

pub fn run_suite<T>(suite: &rspec::block::Suite<T>) -> rspec::report::SuiteReport
where
    T: Clone + Send + Sync + std::fmt::Debug,
{
    let configuration = ConfigurationBuilder::default()
        .parallel(false)
        .exit_on_failure(false)
        .build()
        .expect("rspec configuration should build");
    let logger = Arc::new(Logger::new(io::stdout()));
    let runner = Runner::new(configuration, vec![logger]);
    runner.run(suite)
}

pub struct AudioPlayerTestContext {
    pub global_state_store: Rc<GlobalStateActor>,
    pub view_model: Rc<RefCell<AudioPlayerViewModel>>,
    pub preferences: Rc<InMemoryPreferences>,
}

impl AudioPlayerTestContext {
    pub fn new(behaviors: Vec<Box<dyn Behavior<AudioPlayerViewModel>>>) -> Self {
        Self::with_dependencies(
            behaviors,
            GlobalStateActor::new(),
            Rc::new(InMemoryPreferences::new()),
        )
    }

    pub fn with_global_state(
        behaviors: Vec<Box<dyn Behavior<AudioPlayerViewModel>>>,
        global_state_store: Rc<GlobalStateActor>,
    ) -> Self {
        Self::with_dependencies(
            behaviors,
            global_state_store,
            Rc::new(InMemoryPreferences::new()),
        )
    }

    pub fn with_dependencies(
        behaviors: Vec<Box<dyn Behavior<AudioPlayerViewModel>>>,
        global_state_store: Rc<GlobalStateActor>,
        preferences: Rc<InMemoryPreferences>,
    ) -> Self {
        ensure_slint_test_backend();

        let (link_scene_sender, _link_scene_receiver) = unbounded();
        let view_model = Rc::new(RefCell::new(AudioPlayerViewModel::new(
            None,
            link_scene_sender,
            behaviors,
        )));
        view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&view_model));
        AudioPlayerViewModel::activate(&view_model);

        let context = Self {
            global_state_store,
            view_model,
            preferences,
        };
        context.pump_events();
        context
    }

    pub fn update_global_state(&self, update: impl FnOnce(&mut choreo_components::global::GlobalStateModel)) {
        let updated = self.global_state_store.try_update(update);
        assert!(updated, "failed to update global state in test context");
    }

    pub fn read_global_state<T>(&self, read: impl FnOnce(&choreo_components::global::GlobalStateModel) -> T) -> T {
        self.global_state_store
            .try_with_state(read)
            .expect("failed to read global state in test context")
    }

    pub fn pump_events(&self) {
        for _ in 0..3 {
            i_slint_backend_testing::mock_elapsed_time(Duration::from_millis(20));
            slint::platform::update_timers_and_animations();
        }
        slint::platform::update_timers_and_animations();
    }

    pub fn wait_until(&self, timeout: Duration, mut predicate: impl FnMut() -> bool) -> bool {
        let deadline = Instant::now() + timeout;
        while Instant::now() < deadline {
            if predicate() {
                return true;
            }
            self.pump_events();
        }
        predicate()
    }
}

pub fn scene_view_model(scene_id: i32, name: &str, timestamp: Option<&str>) -> SceneViewModel {
    let scene = SceneModel {
        scene_id: choreo_master_mobile_json::SceneId(scene_id),
        positions: Vec::new(),
        name: name.to_string(),
        text: None,
        fixed_positions: false,
        timestamp: timestamp.map(str::to_string),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: Colors::transparent(),
    };

    let mapper = SceneMapper;
    let mut view_model = SceneViewModel::new(scene.scene_id, scene.name.clone(), scene.color.clone());
    mapper.map_model_to_view_model(&scene, &mut view_model);
    view_model
}

#[derive(Clone)]
pub struct TestAudioPlayerState {
    pub is_playing: bool,
    pub can_seek: bool,
    pub can_set_speed: bool,
    pub duration: f64,
    pub current_position: f64,
    pub last_seek: Option<f64>,
}

impl Default for TestAudioPlayerState {
    fn default() -> Self {
        Self {
            is_playing: false,
            can_seek: true,
            can_set_speed: true,
            duration: 0.0,
            current_position: 0.0,
            last_seek: None,
        }
    }
}

pub struct TestAudioPlayer {
    pub state: Rc<RefCell<TestAudioPlayerState>>,
}

impl AudioPlayer for TestAudioPlayer {
    fn is_playing(&self) -> bool {
        self.state.borrow().is_playing
    }

    fn can_seek(&self) -> bool {
        self.state.borrow().can_seek
    }

    fn can_set_speed(&self) -> bool {
        self.state.borrow().can_set_speed
    }

    fn duration(&self) -> f64 {
        self.state.borrow().duration
    }

    fn current_position(&self) -> f64 {
        self.state.borrow().current_position
    }

    fn play(&mut self) {
        self.state.borrow_mut().is_playing = true;
    }

    fn pause(&mut self) {
        self.state.borrow_mut().is_playing = false;
    }

    fn stop(&mut self) {
        let mut state = self.state.borrow_mut();
        state.is_playing = false;
        state.current_position = 0.0;
    }

    fn seek(&mut self, position: f64) {
        let mut state = self.state.borrow_mut();
        state.current_position = position;
        state.last_seek = Some(position);
    }

    fn set_speed(&mut self, _speed: f64) {}

    fn set_volume(&mut self, _volume: f64) {}

    fn set_balance(&mut self, _balance: f64) {}

    fn set_loop(&mut self, _loop_enabled: bool) {}
}
