#![allow(dead_code)]

use std::cell::Cell;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use choreo_components::dancers::CloseDancerDialogCommand;
use choreo_components::dancers::DancerSettingsViewModel;
use choreo_components::dancers::DancersProvider;
use choreo_components::dancers::DancersProviderDependencies;
use choreo_components::dancers::ShowDancerDialogCommand;
use choreo_components::global::GlobalStateActor;
use choreo_master_mobile_json::DancerId;
use choreo_master_mobile_json::SceneId;
use choreo_models::Colors;
use choreo_models::DancerModel;
use choreo_models::PositionModel;
use choreo_models::RoleModel;
use choreo_models::SceneModel;
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

pub struct DancersTestContext {
    pub global_state_store: Rc<GlobalStateActor>,
    pub view_model: Rc<RefCell<DancerSettingsViewModel>>,
    provider: DancersProvider,
}

impl DancersTestContext {
    pub fn new() -> Self {
        Self::with_global_state(|_| {})
    }

    pub fn with_global_state(
        update: impl FnOnce(&mut choreo_components::global::GlobalStateModel),
    ) -> Self {
        ensure_slint_test_backend();

        let global_state_store = GlobalStateActor::new();
        let updated = global_state_store.try_update(update);
        assert!(updated, "failed to seed global state in test context");

        let provider = DancersProvider::new(DancersProviderDependencies {
            global_state: global_state_store.clone(),
            haptic_feedback: None,
        });
        let view_model = provider.dancer_settings_view_model();

        let context = Self {
            global_state_store,
            view_model,
            provider,
        };
        context.pump_events();
        context
    }

    pub fn show_dialog_sender(&self) -> crossbeam_channel::Sender<ShowDancerDialogCommand> {
        self.provider.show_dialog_sender()
    }

    pub fn close_dialog_sender(&self) -> crossbeam_channel::Sender<CloseDancerDialogCommand> {
        self.provider.close_dialog_sender()
    }

    pub fn update_global_state(
        &self,
        update: impl FnOnce(&mut choreo_components::global::GlobalStateModel),
    ) {
        let updated = self.global_state_store.try_update(update);
        assert!(updated, "failed to update global state in test context");
    }

    pub fn read_global_state<T>(
        &self,
        read: impl FnOnce(&choreo_components::global::GlobalStateModel) -> T,
    ) -> T {
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

pub fn build_role(name: &str) -> Rc<RoleModel> {
    Rc::new(RoleModel {
        z_index: 0,
        name: name.to_string(),
        color: Colors::transparent(),
    })
}

pub fn build_dancer(
    dancer_id: i32,
    role: Rc<RoleModel>,
    name: &str,
    shortcut: &str,
    icon: Option<&str>,
) -> Rc<DancerModel> {
    Rc::new(DancerModel {
        dancer_id: DancerId(dancer_id),
        role,
        name: name.to_string(),
        shortcut: shortcut.to_string(),
        color: Colors::transparent(),
        icon: icon.map(str::to_string),
    })
}

pub fn build_position(dancer: Option<Rc<DancerModel>>) -> PositionModel {
    PositionModel {
        dancer,
        orientation: None,
        x: 0.0,
        y: 0.0,
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

pub fn build_scene(scene_id: i32, positions: Vec<PositionModel>) -> SceneModel {
    SceneModel {
        scene_id: SceneId(scene_id),
        positions,
        name: format!("Scene {scene_id}"),
        text: None,
        fixed_positions: false,
        timestamp: None,
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: Colors::transparent(),
    }
}
