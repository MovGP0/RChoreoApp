use std::error::Error;

pub struct AppLogger;

impl AppLogger {
    pub fn log_unhandled_exception(exception: &dyn Error, is_terminating: bool) {
        log::error!(
            "AppDomain unhandled exception. IsTerminating: {}. Error: {}",
            is_terminating,
            exception
        );
    }

    pub fn log_unhandled_exception_without_error(is_terminating: bool) {
        log::error!(
            "AppDomain unhandled exception. IsTerminating: {}",
            is_terminating
        );
    }

    pub fn log_unobserved_task_exception(exception: &dyn Error) {
        log::error!("Unobserved task exception. Error: {}", exception);
    }

    pub fn log_app_initialization_error(exception: &dyn Error) {
        log::error!("Failed to initialize App resources. Error: {}", exception);
    }
}

pub struct BehaviorLog;

impl BehaviorLog {
    pub fn behavior_activated(behavior_name: &str, view_model_name: &str) {
        log::debug!(
            "Behavior activated: {} for {}.",
            behavior_name,
            view_model_name
        );
    }
}

pub fn configure_logging() {
    if cfg!(debug_assertions) {
        let _ = env_logger::builder().is_test(false).try_init();
    }
}
