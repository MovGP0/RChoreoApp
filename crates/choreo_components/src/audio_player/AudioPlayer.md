# Audio Player State and Interaction Plan

## Goal
Define a single, deterministic playback and slider interaction model so that:
- `Play` does not immediately flip back to pause/play.
- The slider moves with playback when the user is not interacting.
- User seek/drag always wins while interacting.
- Only the latest event is applied, by arrival order, using absolute position values.

## Core Principles
1. Single source of truth by phase:
- Idle phase (no user seek interaction): actor/runtime is the source of truth for `position` and `is_playing`.
- User interaction phase (click/drag/keyboard seek): UI/view-model is the temporary source of truth for slider position.

2. Absolute positions only:
- Seek commands carry absolute seconds (`f64`), never deltas.
- Runtime publishes absolute current position.

3. Latest event wins by order, not value:
- Coalesce by event arrival sequence.
- Never use "max timestamp" as conflict resolution.

4. No feedback loop:
- Programmatic UI updates must not emit user seek callbacks.
- User callbacks must be clearly separated from actor sync callbacks.

## State Model
Use two orthogonal state groups.

### A) Playback State
- `NoMedia`
- `ReadyPaused`
- `ReadyPlaying`
- `Ended`
- `Error`

### B) Seek Interaction State
- `NotInteracting`
- `SeekingPreview` (user clicked/dragging/keyboard adjusting position)
- `SeekCommitPending` (user released, commit queued to actor)

Represent this with a compact struct in view-model state:
```text
playback_state
interaction_state
was_playing_before_seek
pending_seek_position
```

## Event Ownership Matrix
| Event | Allowed Source | Allowed When | Effect |
|---|---|---|---|
| `ActorPositionUpdate(abs_pos, is_playing)` | Actor timer | `NotInteracting` | update slider/time/play glyph |
| `UserSeekPreview(abs_pos)` | UI slider user event | any ready state | update slider/time only |
| `UserSeekCommit(abs_pos)` | UI slider release/click commit | any ready state | send seek command to actor |
| `UserTogglePlayPause` | Play button | any ready state | send play/pause intent |
| `OpenMedia` | file open | `NoMedia`/`Error` | initialize runtime and UI |
| `CloseMedia` | user/system | any | reset to `NoMedia` |

## Command Bus Contract
Use a dedicated audio control queue (actor inbox) with these commands:
- `Play`
- `Pause`
- `Stop`
- `Seek(abs_pos)`
- `SeekAndPlay(abs_pos)`
- `SetSpeed(abs_speed)`
- `SetVolume(abs_volume)`
- `SetLoop(bool)`
- `Shutdown`

Rules:
- Coalesce latest per command family in one drain cycle.
- Preserve last-arrival ordering across coalesced families.
- `SeekAndPlay` is atomic and preferred for drag-release resume.

## UI Interaction Patterns

### 1) Play/Pause Button
- On click, emit only one intent command based on current view-model playback state.
- Do not directly flip state twice in UI.
- UI optimistic update is optional, but actor update is authoritative within next tick.

Expected:
- `ReadyPaused -> ReadyPlaying` or `ReadyPlaying -> ReadyPaused` exactly once per click.

### 2) Slider Drag
On drag start:
- `interaction_state = SeekingPreview`
- store `was_playing_before_seek`
- if currently playing, send `Pause` once (or rely on `SeekAndPlay` on release, but do not send repeated pause)

During drag:
- apply `UserSeekPreview(abs_pos)` to label + slider only
- block actor-driven slider overwrite while `SeekingPreview`

On drag complete:
- `interaction_state = SeekCommitPending`
- if `was_playing_before_seek`: send `SeekAndPlay(abs_pos)`
- else: send `Seek(abs_pos)`
- clear pending and set `interaction_state = NotInteracting`

### 3) Slider Click/Step (non-drag)
- Treat as quick seek commit:
- preview abs pos
- send `Seek(abs_pos)` immediately (or `SeekAndPlay` if currently playing and pause-on-seek policy applies)

### 4) Speed Slider
- user event emits absolute speed
- actor applies latest speed
- UI reflects actor-confirmed speed label

## Synchronization Rules
1. Timer pull from actor runs continuously.
2. Actor updates are ignored for slider position only when `interaction_state != NotInteracting`.
3. Actor updates for `is_playing` may still update button glyph during interaction if desired, but must not re-position slider in `SeekingPreview`.
4. Immediately after seek commit, actor update resumes ownership.

## Loop Prevention Checklist
- Slider component emits `position_changed` only for user-generated events.
- Programmatic `set_audio_position(...)` does not trigger seek callbacks.
- No duplicate command dispatch from both binding layer and behavior for same user action.
- Avoid sending `Play` after already sending `SeekAndPlay`.

## Suggested Transition Table (Condensed)
- `ReadyPlaying + UserDragStart -> ReadyPaused + SeekingPreview`
- `ReadyPaused + UserDragStart -> ReadyPaused + SeekingPreview`
- `SeekingPreview + UserDragMove -> SeekingPreview` (preview only)
- `SeekingPreview + UserDragEnd + was_playing=true -> ReadyPlaying + NotInteracting` via `SeekAndPlay`
- `SeekingPreview + UserDragEnd + was_playing=false -> ReadyPaused + NotInteracting` via `Seek`
- `NotInteracting + ActorPositionUpdate -> keep playback state, update slider/time`
- `ReadyPlaying + EndOfTrack (loop=false) -> Ended + NotInteracting`

## Diagnostics to Keep
Add trace logs for:
- interaction state transitions
- commands enqueued (`Play`, `Pause`, `Seek`, `SeekAndPlay`)
- actor command drain order
- actor publish values (`position`, `is_playing`)

This should make pause/play bounce and slider ownership bugs immediately visible.

## Acceptance Criteria
1. Pressing Play from paused starts playback and remains playing.
2. While playing and not touching slider, slider/time advance continuously.
3. During drag, slider follows user only; actor updates do not snap it back.
4. Releasing drag while previously playing resumes playback immediately at released position.
5. Releasing drag while paused seeks and remains paused.
6. Rapid seek gestures process only latest seek intent by arrival order.
7. No visible play/pause flicker from a single button click.
