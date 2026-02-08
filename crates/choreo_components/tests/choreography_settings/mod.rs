#![allow(dead_code)]

use std::cell::Cell;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use choreo_components::behavior::Behavior;
use choreo_components::behavior::CompositeDisposable;
use choreo_components::choreography_settings::ChoreographySettingsViewModel;
use choreo_components::global::GlobalStateActor;
use choreo_components::preferences::InMemoryPreferences;
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

pub struct ChoreographySettingsTestContext {
    pub global_state_store: Rc<GlobalStateActor>,
    pub view_model: Rc<RefCell<ChoreographySettingsViewModel>>,
    pub preferences: InMemoryPreferences,
    pub redraw_receiver: crossbeam_channel::Receiver<choreo_components::choreography_settings::RedrawFloorCommand>,
    disposables: RefCell<CompositeDisposable>,
}

impl ChoreographySettingsTestContext {
    pub fn new() -> Self {
        ensure_slint_test_backend();

        let global_state_store = GlobalStateActor::new();
        let view_model = Rc::new(RefCell::new(ChoreographySettingsViewModel::default()));
        view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&view_model));

        let preferences = InMemoryPreferences::new();
        let (_redraw_sender, redraw_receiver) = unbounded();

        let context = Self {
            global_state_store,
            view_model,
            preferences,
            redraw_receiver,
            disposables: RefCell::new(CompositeDisposable::new()),
        };
        context.pump_events();
        context
    }

    pub fn with_redraw_receiver(
        redraw_receiver: crossbeam_channel::Receiver<choreo_components::choreography_settings::RedrawFloorCommand>,
    ) -> Self {
        ensure_slint_test_backend();

        let global_state_store = GlobalStateActor::new();
        let view_model = Rc::new(RefCell::new(ChoreographySettingsViewModel::default()));
        view_model
            .borrow_mut()
            .set_self_handle(Rc::downgrade(&view_model));

        let context = Self {
            global_state_store,
            view_model,
            preferences: InMemoryPreferences::new(),
            redraw_receiver,
            disposables: RefCell::new(CompositeDisposable::new()),
        };
        context.pump_events();
        context
    }

    pub fn activate_behaviors(&self, behaviors: Vec<Box<dyn Behavior<ChoreographySettingsViewModel>>>) {
        for behavior in behaviors {
            behavior.activate(
                &mut self.view_model.borrow_mut(),
                &mut self.disposables.borrow_mut(),
            );
        }
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
