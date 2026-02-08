#![allow(dead_code)]

use std::cell::Cell;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use choreo_master_mobile_json::SceneId;
use choreo_models::ChoreographyModel;
use choreo_models::Colors;
use choreo_models::DancerModel;
use choreo_models::PositionModel;
use choreo_models::RoleModel;
use choreo_models::SceneModel;
use choreo_components::behavior::Behavior;
use choreo_components::global::GlobalStateActor;
use choreo_components::preferences::InMemoryPreferences;
use choreo_components::preferences::Preferences;
use choreo_components::scenes::SceneMapper;
use choreo_components::scenes::SceneViewModel;
use choreo_components::scenes::ScenesPaneViewModel;
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

pub struct ScenesTestContext {
    pub global_state_store: Rc<GlobalStateActor>,
    pub view_model: Rc<RefCell<ScenesPaneViewModel>>,
    pub preferences: Rc<InMemoryPreferences>,
}

impl ScenesTestContext {
    pub fn new() -> Self {
        ensure_slint_test_backend();

        let global_state_store = GlobalStateActor::new();
        let global_state = global_state_store.state_handle();
        let preferences = Rc::new(InMemoryPreferences::new());

        let (show_dialog_sender, _show_dialog_receiver) = unbounded();
        let (close_dialog_sender, _close_dialog_receiver) = unbounded();

        let preferences_dyn: Rc<dyn Preferences> = preferences.clone();
        let view_model = Rc::new(RefCell::new(ScenesPaneViewModel::new(
            global_state,
            preferences_dyn,
            show_dialog_sender,
            close_dialog_sender,
            None,
        )));
        view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&view_model));

        let context = Self {
            global_state_store,
            view_model,
            preferences,
        };
        context.pump_events();
        context
    }

    pub fn activate_behaviors(&self, behaviors: Vec<Box<dyn Behavior<ScenesPaneViewModel>>>) {
        ScenesPaneViewModel::activate(&self.view_model, behaviors);
        self.pump_events();
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

pub fn build_scene_model(
    scene_id: i32,
    name: &str,
    timestamp: Option<&str>,
    positions: Vec<PositionModel>,
) -> SceneModel {
    SceneModel {
        scene_id: SceneId(scene_id),
        positions,
        name: name.to_string(),
        text: None,
        fixed_positions: false,
        timestamp: timestamp.map(str::to_string),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: Colors::transparent(),
    }
}

pub fn build_position(x: f64, y: f64) -> PositionModel {
    PositionModel {
        dancer: None,
        orientation: None,
        x,
        y,
        curve1_x: None,
        curve1_y: None,
        curve2_x: None,
        curve2_y: None,
        movement1_x: None,
        movement1_y: None,
        movement2_x: None,
        movement2_y: None,
    }
}

pub fn build_dancer(dancer_id: i32, name: &str) -> Rc<DancerModel> {
    let role = Rc::new(RoleModel {
        z_index: 0,
        name: "role".to_string(),
        color: Colors::transparent(),
    });

    Rc::new(DancerModel {
        dancer_id: choreo_master_mobile_json::DancerId(dancer_id),
        role,
        name: name.to_string(),
        shortcut: name.to_string(),
        color: Colors::transparent(),
        icon: None,
    })
}

pub fn map_scene_view_model(scene: &SceneModel) -> SceneViewModel {
    let mapper = SceneMapper;
    let mut view_model = SceneViewModel::new(scene.scene_id, scene.name.clone(), scene.color.clone());
    mapper.map_model_to_view_model(scene, &mut view_model);
    view_model
}

pub fn set_choreography(
    context: &ScenesTestContext,
    mut choreography: ChoreographyModel,
    selected_scene_id: Option<SceneId>,
) {
    let scenes = choreography
        .scenes
        .iter()
        .map(map_scene_view_model)
        .collect::<Vec<_>>();

    let selected_scene = selected_scene_id
        .and_then(|id| scenes.iter().find(|scene| scene.scene_id == id).cloned())
        .or_else(|| scenes.first().cloned());

    context.update_global_state(|state| {
        choreography.scenes = choreography.scenes.clone();
        state.choreography = choreography;
        state.scenes = scenes;
        state.selected_scene = selected_scene;
    });

    context.view_model.borrow_mut().refresh_scenes();
    context.view_model
        .borrow_mut()
        .set_selected_scene(context.read_global_state(|state| state.selected_scene.clone()));
}
