mod trace_context;

pub(crate) use trace_context::TraceContext;

#[cfg(all(debug_assertions, feature = "debug-otel"))]
mod otel;

#[cfg(all(debug_assertions, feature = "debug-otel"))]
pub(crate) use otel::{capture_trace_context, init_debug_otel};

#[cfg(not(all(debug_assertions, feature = "debug-otel")))]
pub(crate) struct OTelGuard;

#[cfg(not(all(debug_assertions, feature = "debug-otel")))]
pub(crate) fn init_debug_otel() -> OTelGuard {
    OTelGuard
}

#[cfg(not(all(debug_assertions, feature = "debug-otel")))]
pub(crate) fn capture_trace_context(_action_name: &str) -> Option<TraceContext> {
    None
}

#[cfg(all(test, not(all(debug_assertions, feature = "debug-otel"))))]
mod tests {
    use super::capture_trace_context;
    use super::init_debug_otel;

    #[test]
    fn debug_otel_init_is_a_no_op_without_feature_support() {
        let _guard = init_debug_otel();

        assert!(capture_trace_context("ui.start").is_none());
    }
}
