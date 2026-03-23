# Timestamp Synchronization Scenarios

## Purpose
This document defines all synchronization scenarios for the current/selected timestamp represented by the `SliderWithTicks` control.

## Core Concepts
- `Current timestamp`: the active timestamp value shown by the audio slider (`audio_position` / `AudioPlayerViewModel.position`).
- `Selected scene`: the scene selected in the scenes list (`GlobalStateModel.selected_scene`).
- `Floor scene`: the scene context used by floor rendering (current selected scene + adjacent scenes).
- `Interpolated positions`: dancer positions rendered between current and next timestamped scenes based on current timestamp.

## Main Data Flows
- Audio actor -> Audio player view model -> Slider UI.
- Slider UI -> Audio player view model -> Scene selection behavior (timestamp window) -> Floor redraw.
- Scenes list selection -> Selected scene -> Slider timestamp update -> Floor redraw.
- Floor rendering -> selected scene + current timestamp -> interpolated positions + paths.

## Source-of-Truth Rules
- Slider value is the UI representation of current timestamp.
- Audio actor position is authoritative while playing, unless temporarily gated by pending seek/drag logic.
- Selected scene is authoritative for floor scene context when user explicitly selects a scene.
- Interpolation uses current timestamp against valid `(current_scene.timestamp, next_scene.timestamp)` range where `next > current`.

## Scenario Matrix

### 1. Audio playback updates timestamp (no user drag)
- Trigger: audio actor position changes while playing.
- Expected:
- Slider moves to actor timestamp.
- `SelectSceneFromAudioPositionBehavior` may switch selected scene when timestamp enters a valid scene window.
- Floor redraw occurs and positions interpolate for the active timestamp.

### 2. User drags slider thumb
- Trigger: pointer down + move + up on slider thumb.
- Expected during drag:
- Slider visual value follows pointer continuously.
- `position_changed` events are emitted continuously.
- Floor updates continuously to interpolated positions for drag timestamp.
- Scene selection updates continuously to the scene preceding/in-range for drag timestamp.
- Expected on drag complete:
- Seek is applied.
- Pending seek guard prevents immediate snap-back from stale actor position.
- If playback was active before drag, playback resumes.

### 3. User drags slider track (not thumb)
- Trigger: pointer down/move/up on slider track.
- Expected:
- Same as thumb drag behavior (continuous updates + final seek on release).
- No jump to `0` or previous position.

### 4. User clicks slider once (no drag)
- Trigger: pointer down/up at target timestamp.
- Expected:
- Slider moves to clicked timestamp.
- Seek is applied once.
- Scene and floor update to clicked timestamp.

### 5. User selects a scene with timestamp from scene list
- Trigger: `scenes_select_scene(index)`.
- Expected:
- Selected scene text/details update.
- Slider is set to selected scene timestamp.
- Audio player seek is applied when seek is available.
- Floor scene switches to selected scene context.
- Floor positions render for that timestamp (interpolated if next scene is timestamped and valid).

### 6. User selects a scene without timestamp
- Trigger: scene selection where `timestamp == None`.
- Expected:
- Selected scene text/details update.
- Slider remains unchanged.
- Floor scene updates to selected scene (non-interpolated positions unless current timestamp still produces valid interpolation in selected context).

### 7. Timestamp before first timestamped scene
- Trigger: slider/audio timestamp less than first valid timestamp.
- Expected:
- No automatic scene switch unless explicitly defined by behavior rules.
- Floor should still render selected scene context; interpolation disabled when no valid range.

### 8. Timestamp between two valid timestamped scenes
- Trigger: `current_timestamp <= t <= next_timestamp` with `next > current`.
- Expected:
- Selected scene should be the current scene of that range (scene before/floor of the range).
- Floor positions interpolate between current and next scenes with percent:
- `percent = (t - current) / (next - current)`.

### 9. Timestamp exactly on boundary
- Trigger: `t == current_timestamp` or `t == next_timestamp`.
- Expected:
- At `current`: interpolation percent `0%` and current scene selected.
- At `next`: selection behavior must be deterministic (current window end-inclusive currently), and floor must not flicker.

### 10. Invalid timestamp pair in scene sequence
- Trigger: next scene missing timestamp or `next <= current`.
- Expected:
- Scene auto-selection for that pair is skipped.
- Interpolation is disabled for that pair.
- Floor renders non-interpolated selected-scene positions for affected dancers.

### 11. Seek not available
- Trigger: slider change when `audio_player.can_seek == false`.
- Expected:
- Slider value can still represent desired timestamp in UI state.
- No player seek call.
- Floor/scene updates may still follow slider timestamp events if emitted.

### 12. Playback + manual scene selection race
- Trigger: user selects scene while audio is playing and actor updates continue.
- Expected:
- Scene selection updates slider timestamp and applies seek.
- Pending seek guard prevents immediate overwrite by stale actor sample.
- Final steady state converges: actor position, slider value, selected scene, floor positions.

## Floor Update Requirements
- Floor redraw must happen on:
- Scene selection change.
- Timestamp change relevant to interpolation.
- Layout/bounds changes.
- Floor render input must include:
- Selected scene + adjacent scenes.
- Current timestamp.
- Validity of current/next timestamps.
- Calculated interpolation percent and interpolated dancer positions.

## Tracing Requirements (OTEL)
- Scene event/behavior spans should include:
- Command enqueue/handle for scene select.
- Position-driven scene-select decisions.
- Reload/load/open/save behavior handling.
- Core attributes:
- scene id/index.
- timestamp seconds (when present).
- success/failure.
- selection changed flag.
- count attributes (scenes, candidates) where relevant.

## Consistency Invariants
- Slider, selected scene, and floor must never diverge for more than one UI tick after a handled user action.
- A user-driven slider timestamp change must always be visible in floor interpolation and scene selection outcome.
- A user-driven scene selection with timestamp must always be visible in slider position and floor scene/positions.
