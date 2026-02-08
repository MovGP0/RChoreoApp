#![allow(dead_code)]

use std::cell::Cell;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use choreo_components::behavior::Behavior;
use choreo_components::choreo_main::MainViewModel;
use choreo_components::global::GlobalStateActor;
use choreo_components::nav_bar::NavBarSenders;
use choreo_components::nav_bar::NavBarViewModel;
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

pub struct ChoreoMainTestContext {
    pub global_state_store: Rc<GlobalStateActor>,
    pub state_machine: Rc<RefCell<choreo_state_machine::ApplicationStateMachine>>,
    pub view_model: Rc<RefCell<MainViewModel>>,
}

impl ChoreoMainTestContext {
    pub fn new(behaviors: Vec<Box<dyn Behavior<MainViewModel>>>) -> Self {
        Self::with_dependencies(
            behaviors,
            GlobalStateActor::new(),
            Rc::new(RefCell::new(
                choreo_state_machine::ApplicationStateMachine::with_default_transitions(Box::new(
                    choreo_components::global::GlobalStateModel::default(),
                )),
            )),
        )
    }

    pub fn with_dependencies(
        behaviors: Vec<Box<dyn Behavior<MainViewModel>>>,
        global_state_store: Rc<GlobalStateActor>,
        state_machine: Rc<RefCell<choreo_state_machine::ApplicationStateMachine>>,
    ) -> Self {
        ensure_slint_test_backend();

        let (open_audio_requested_sender, _open_audio_requested_receiver) = unbounded();
        let (open_image_requested_sender, _open_image_requested_receiver) = unbounded();
        let (interaction_mode_changed_sender, _interaction_mode_changed_receiver) = unbounded();
        let nav_senders = NavBarSenders {
            open_audio_requested: open_audio_requested_sender,
            open_image_requested: open_image_requested_sender,
            interaction_mode_changed: interaction_mode_changed_sender,
        };

        let nav_bar = Rc::new(RefCell::new(NavBarViewModel::new(
            global_state_store.state_handle(),
            None,
            Vec::new(),
            nav_senders,
        )));

        let view_model = Rc::new(RefCell::new(MainViewModel::new(
            global_state_store.state_handle(),
            state_machine.clone(),
            behaviors,
            nav_bar,
        )));
        view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&view_model));
        MainViewModel::activate(&view_model);

        let context = Self {
            global_state_store,
            state_machine,
            view_model,
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
