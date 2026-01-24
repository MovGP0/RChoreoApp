# choreo_state_machine

Rust port of `ChoreoApp.StateMachine`.

## Notes / Differences

- Transitions are loaded from the CSV matrix to mirror the .NET DI registration list.
- Preconditions are supported but default transitions use none (same as .NET).
- State and trigger types are zero-sized structs; the state machine operates on `StateKind`/`TriggerKind` for matching.
