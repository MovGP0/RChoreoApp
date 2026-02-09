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
