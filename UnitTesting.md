# Unit Testing

## Assertions
When multiple asserts in a unit tests are required, the errors need to be collected and reported together.
This ensures that all failures are visible in a single test run, rather than stopping at the first failure.
Provide macros like `check_eq!` when needed.

**BAD Example**
```rust
assert_eq!(result.floor.size_front, 10);
assert_eq!(result.floor.size_back, 11);
assert_eq!(result.floor.size_left, 12);
assert_eq!(result.floor.size_right, 13);
```

**GOOD Example**
```rust
macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}
```
```rust
let mut errors = Vec::new();

check_eq!(errors, result.floor.size_front, 10);
check_eq!(errors, result.floor.size_back, 11);
check_eq!(errors, result.floor.size_left, 12);
check_eq!(errors, result.floor.size_right, 13);

assert!(
    errors.is_empty(),
    "Assertion failures:\n{}",
    errors.join("\n")
);
```

Note that this is not required when there is only a single assertion in the test.
BDD style integration testing using `rspec` also do not require this.

## Unit Test Structure
- Unit tests should be structured using the Arrange-Act-Assert (AAA) pattern for clarity and maintainability.
- The sections should be clearly marked by `// arrange` `// act` and `//assert` comments.
- The object under test should be named `subject` for consistency across tests.
- The primary result object of the action should be named `result` (if applicable).

## Integration Test Structure
- Integration tests shall be done using BDD style testing using `rspec` and follow Behavior-Driven Development (BDD) principles, using descriptive names and scenarios to enhance readability and understanding of the test cases.
- When the user requires a specific behavior from the system, it should be implemented as a scenario in `rspec` to ensure that the system meets user requirements.

## Floor BDD Test Structure Notes
- Existing floor integration tests use one spec file per behavior in `crates/choreo_components/tests/floor/*_spec.rs`.
- Every spec wraps scenarios in a single `rspec::describe("<behavior>", (), |spec| { ... })` suite and executes it with `run_suite(&suite)` from `crates/choreo_components/tests/floor/mod.rs`.
- The outer Rust test function name matches the file purpose (example: `move_positions_behavior_spec`) and is marked with `#[serial_test::serial]` to avoid shared Slint backend timing interference.
- Shared setup belongs in local helpers (for example `setup_context`, `select_rectangle`, `drag_from_to`) so each `spec.it(...)` contains only scenario-specific intent.
- Scenario assertions use a two-stage pattern for async/event-driven updates:
  1. trigger events through `FloorTestContext` helpers,
  2. poll with `context.wait_until(Duration::from_secs(1), || ...)`,
  3. assert the wait result and then validate exact state values.
- Floating-point checks should use tolerance helpers (`assert_close`) instead of exact equality.
- Read and update app state only through test context helpers (`update_global_state`, `read_global_state`, `update_state_machine`) rather than direct field mutation from tests.
- Keep each `spec.it(...)` focused on one behavior outcome (primary path plus explicit edge-case scenarios as separate `it` blocks).
