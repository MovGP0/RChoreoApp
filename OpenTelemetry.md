# OpenTelemetry Plan (Desktop Debug Only)

## Scope

- Target only `apps/desktop` debug builds (`cfg(debug_assertions)`).
- Keep release builds and non-desktop targets unchanged.
- Enable OTLP export only when env vars are set.
- Trace user-button initiated flows through commands/events/behaviors.
- This document is planning only; no production code changes in this ticket.

## Current Repository State

- Logging is currently based on `log` + `env_logger` (for example in `crates/choreo_state_machine/src/machine.rs`).
- Behavior activation already has a centralized hook (`crates/choreo_components/src/logging/types.rs`).
- UI actions fan out through `MainPageBinding` and crossbeam channels into behavior handlers (for example `open_choreo_behavior.rs`, `open_audio_behavior.rs`, `open_audio_file_behavior.rs`).
- Workspace already separates app targets (`apps/desktop`, `apps/android`, `apps/wasm`), which supports desktop-only integration.

## Design Summary

1. Add an observability module in `apps/desktop` that initializes an OTel tracer provider only in debug.
2. Use OTLP HTTP exporter by default for desktop debug to avoid runtime constraints (`reqwest-blocking-client` path in `opentelemetry-otlp`).
3. Use the latest Aspire Dashboard Docker container as the default local OTLP endpoint and web UI.
4. Gate exporting on `OTEL_EXPORTER_OTLP_ENDPOINT` or `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`; if absent, keep existing logging only.
5. Keep Aspire Dashboard authentication enabled and use the one-time login link (`/login?t=...`) from container logs.
6. Do not run the container with `--rm`; stopped container and image must remain available in Docker.
7. Use `opentelemetry-semantic-conventions` for standardized span/resource attributes.
8. Add a debug UI action/button to start a root "user trace session".
9. Propagate trace context through existing command/event structs by adding optional trace metadata fields.
10. In behaviors, create child spans for command handling and important state transitions.
11. Shut down tracer provider on app exit to flush buffered spans.

## Proposed Crates (Desktop App Only)

In `apps/desktop/Cargo.toml`:

```toml
[dependencies]
opentelemetry = { version = "0.31", optional = true }
opentelemetry_sdk = { version = "0.31", optional = true, features = ["trace"] }
opentelemetry-otlp = { version = "0.31", optional = true, features = ["trace", "http-proto", "reqwest-blocking-client"] }
opentelemetry-semantic-conventions = { version = "0.31", optional = true }

[features]
default = []
debug-otel = [
  "dep:opentelemetry",
  "dep:opentelemetry_sdk",
  "dep:opentelemetry-otlp",
  "dep:opentelemetry-semantic-conventions",
]
```

Then compile-enable in desktop debug CI/dev command:

```sh
cargo run -p rchoreo_desktop --features debug-otel
```

And runtime-enable export via env vars (Aspire local OTLP/HTTP):

```sh
OTEL_SERVICE_NAME=rchoreo_desktop_debug
OTEL_EXPORTER_OTLP_ENDPOINT=http://127.0.0.1:4318
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
```

## Module Layout

Create new desktop-only files:

- `apps/desktop/src/observability/mod.rs`
- `apps/desktop/src/observability/otel.rs`
- `apps/desktop/src/observability/trace_context.rs`

Responsibilities:

- `otel.rs`: init/shutdown provider and tracer handle.
- `trace_context.rs`: small serializable context object for command/event propagation.
- `mod.rs`: exports a no-op API when feature/config is disabled.

## Phased Implementation Plan

### Phase 1: Bootstrap and Safety Gates

- Add `init_debug_otel()` in desktop startup (`apps/desktop/src/main.rs`).
- `init_debug_otel()` should:
  - return no-op if `!cfg!(debug_assertions)`;
  - return no-op if endpoint env vars are absent;
  - build `SdkTracerProvider` + OTLP exporter otherwise.
- Keep provider handle alive for process lifetime and call `shutdown()` before exit.

Example:

```rust
pub struct OTelGuard {
    provider: Option<opentelemetry_sdk::trace::SdkTracerProvider>,
}

impl Drop for OTelGuard {
    fn drop(&mut self) {
        if let Some(provider) = &self.provider {
            let _ = provider.shutdown();
        }
    }
}
```

### Phase 2: User-Button Root Trace

- Add a debug-only UI command such as `StartDebugTraceCommand` from the main page/nav bar.
- On click, create a root span with attributes:
  - `otel.name` (span name)
  - `otel.kind` (`Internal`)
  - `session.id`
  - `code.filepath` (when available)
- Store lightweight trace metadata in a shared debug session object (not in ViewModel text fields).

### Phase 3: Context Propagation Through Messages

- Extend relevant command/event structs with optional metadata, for example:
  - `OpenChoreoRequested`
  - `OpenAudioRequested`
  - `OpenAudioFileCommand`
- In producers (`MainPageBinding` and action handlers), attach current trace metadata.
- In consumers (behaviors), create child spans from the incoming metadata.

Example shape:

```rust
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TraceContext {
    pub trace_id_hex: Option<String>,
    pub span_id_hex: Option<String>,
}
```

### Phase 4: Behavior-Level Instrumentation

- Add spans around major behavior work blocks, especially:
  - file open/load (`OpenChoreoBehavior`, `OpenAudioFileBehavior`);
  - state machine transitions (`ApplicationStateMachine::try_apply`);
  - floor redraw/gesture loops where expensive operations happen.
- Add semantic attributes via `opentelemetry-semantic-conventions` instead of ad-hoc keys where possible.
- Keep custom app attributes in a small `choreo.*` namespace only when no semantic key exists.
- Add attributes for command type, file extension, success/failure, and elapsed timing.
- Keep cardinality bounded (no raw large payload attributes).

### Phase 5: Failure and Shutdown Behavior

- On exporter init failure, log warning and continue with no-op tracing.
- On shutdown failure, log warning and continue app exit.
- Keep telemetry non-blocking from business logic perspective.

## Verification Strategy

## 1. Build/Static

- `cargo build -p rchoreo_desktop --features debug-otel`
- `cargo clippy -p rchoreo_desktop --all-targets --all-features -- -D warnings`
- `cargo test -p rchoreo_desktop`

## 2. Local End-to-End

Pull and run Aspire Dashboard (latest) as local collector + web UI:

```sh
docker pull mcr.microsoft.com/dotnet/aspire-dashboard:latest
docker run -d --name aspire-dashboard `
  -p 18888:18888 `
  -p 4317:18889 `
  -p 4318:18890 `
  mcr.microsoft.com/dotnet/aspire-dashboard:latest
```

Get the secure login link from container logs (PowerShell):

```sh
docker logs aspire-dashboard 2>&1 | Select-String "login\?t="
```

Open the returned `http://localhost:18888/login?t=...` link in browser.

Run app with env vars and debug feature:

```sh
OTEL_SERVICE_NAME=rchoreo_desktop_debug
OTEL_EXPORTER_OTLP_ENDPOINT=http://127.0.0.1:4318
OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf
cargo run -p rchoreo_desktop --features debug-otel
```

Container lifecycle (keep container/image after stop):

```sh
docker stop aspire-dashboard
docker start aspire-dashboard
```

Assertions:

- clicking trace button emits a root span;
- open choreo/audio flows create child spans in sequence;
- when env vars are missing, no export attempts are made;
- release build has no debug tracing behavior.

## 3. Regression Protection

- Add unit tests for env gating decisions (enabled vs disabled).
- Add behavior tests that verify trace metadata propagation in command handling paths.

## Risks and Mitigations

- Runtime/exporter mismatch risk: stay on HTTP + `reqwest-blocking-client` for desktop debug first.
- Span volume risk in timer loops: sample user-initiated flows first, add throttling where needed.
- Context propagation complexity: introduce one shared `TraceContext` type and only attach to selected message types initially.

## Recommended First Follow-Up Ticket Breakdown

1. `observability: add desktop debug OTel bootstrap module with env gating`
2. `observability: add debug trace button and root span`
3. `observability: propagate TraceContext through open choreo/audio commands`
4. `observability: instrument state machine transitions and core behaviors`
5. `observability: add tests for gating and trace propagation`

## References

- OpenTelemetry SDK environment variables spec: https://opentelemetry.io/docs/specs/otel/configuration/sdk-environment-variables/
- OpenTelemetry general SDK config (`OTEL_SERVICE_NAME`, etc.): https://opentelemetry.io/docs/languages/sdk-configuration/general/
- `opentelemetry-otlp` crate docs and feature flags/constants (current latest shown in docs.rs): https://docs.rs/opentelemetry-otlp/latest/opentelemetry_otlp/
- `opentelemetry-semantic-conventions` crate: https://crates.io/crates/opentelemetry-semantic-conventions
- `opentelemetry_sdk::trace::BatchSpanProcessor` behavior and exporter/runtime notes: https://docs.rs/opentelemetry_sdk/latest/opentelemetry_sdk/trace/struct.BatchSpanProcessor.html
- `opentelemetry_sdk::trace::SdkTracerProvider` shutdown/flush semantics: https://docs.rs/opentelemetry_sdk/latest/opentelemetry_sdk/trace/struct.SdkTracerProvider.html
- Aspire Dashboard overview: https://aspire.dev/dashboard/overview/
- Aspire Dashboard container image (Docker Hub): https://hub.docker.com/r/microsoft/dotnet-aspire-dashboard
