#![allow(dead_code)]

use std::cell::Cell;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use choreo_components::preferences::InMemoryPreferences;
use choreo_components::preferences::Preferences;
use choreo_components::settings::MaterialSchemeUpdater;
use choreo_components::settings::SettingsDependencies;
use choreo_components::settings::SettingsProvider;
use choreo_components::settings::SettingsViewModel;
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

#[derive(Clone, Default)]
pub struct TestMaterialSchemeUpdater {
    calls: Rc<Cell<usize>>,
}

impl TestMaterialSchemeUpdater {
    pub fn call_count(&self) -> usize {
        self.calls.get()
    }
}

impl MaterialSchemeUpdater for TestMaterialSchemeUpdater {
    fn update(&self, _settings: &SettingsViewModel, _preferences: &dyn Preferences) {
        self.calls.set(self.calls.get() + 1);
    }
}

pub struct SettingsTestContext {
    pub view_model: Rc<RefCell<SettingsViewModel>>,
    pub preferences: InMemoryPreferences,
    pub updater: TestMaterialSchemeUpdater,
    provider: SettingsProvider,
}

impl SettingsTestContext {
    pub fn new() -> Self {
        Self::with_preferences(|_| {})
    }

    pub fn with_preferences(seed: impl FnOnce(&InMemoryPreferences)) -> Self {
        ensure_slint_test_backend();

        let preferences = InMemoryPreferences::new();
        seed(&preferences);
        let updater = TestMaterialSchemeUpdater::default();
        let provider = SettingsProvider::new(SettingsDependencies {
            preferences: preferences.clone(),
            scheme_updater: updater.clone(),
        });
        let view_model = provider.settings_view_model();

        let context = Self {
            view_model,
            preferences,
            updater,
            provider,
        };
        context.pump_events();
        context
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

    pub fn _provider_ref(&self) -> &SettingsProvider {
        &self.provider
    }
}
