use egui::Color32;
use egui::Context;
use egui::Id;

use super::tokens::checked_animation_spec;
use super::tokens::unchecked_animation_easing_spec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StateLayerVisualState {
    pub(crate) interactive: bool,
    pub(crate) hovered: bool,
    pub(crate) pressed: bool,
    pub(crate) focused: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct ToggleAnimationState {
    progress: f32,
    from_progress: f32,
    started_at: f64,
    target_checked: bool,
}

impl Default for ToggleAnimationState {
    fn default() -> Self {
        Self {
            progress: 0.0,
            from_progress: 0.0,
            started_at: 0.0,
            target_checked: false,
        }
    }
}

#[must_use]
pub fn next_checked_state(
    checked: bool,
    enabled: bool,
    toggle_on_click: bool,
    clicked: bool,
) -> bool {
    if enabled && clicked && toggle_on_click {
        !checked
    } else {
        checked
    }
}

#[must_use]
pub fn toggled_state_after_click(checked: bool, toggle_on_click: bool, clicked: bool) -> bool {
    next_checked_state(checked, true, toggle_on_click, clicked)
}

pub(crate) fn checked_animation_progress(ctx: &Context, id: Id, checked: bool) -> f32 {
    let now = ctx.input(|input| input.time);
    let mut state = ctx
        .data(|data| data.get_temp::<ToggleAnimationState>(id))
        .unwrap_or(ToggleAnimationState {
            progress: if checked { 1.0 } else { 0.0 },
            from_progress: if checked { 1.0 } else { 0.0 },
            started_at: now,
            target_checked: checked,
        });

    if state.target_checked != checked {
        state.from_progress = state.progress;
        state.started_at = now;
        state.target_checked = checked;
    }

    let spec = if state.target_checked {
        checked_animation_spec()
    } else {
        unchecked_animation_easing_spec()
    };
    let target_progress = if state.target_checked { 1.0 } else { 0.0 };
    let duration = spec.duration.as_secs_f64();

    if duration <= 0.0 {
        state.progress = target_progress;
    } else {
        let elapsed_fraction = ((now - state.started_at) / duration).clamp(0.0, 1.0) as f32;
        let eased = spec.sample(elapsed_fraction);
        state.progress = egui::lerp(state.from_progress..=target_progress, eased);
        if elapsed_fraction < 1.0 {
            ctx.request_repaint();
        }
    }

    ctx.data_mut(|data| data.insert_temp(id, state));
    state.progress
}

#[must_use]
pub(crate) fn with_opacity(color: Color32, alpha_factor: f32) -> Color32 {
    let [r, g, b, a] = color.to_array();
    let next_alpha = (f32::from(a) * alpha_factor).round().clamp(0.0, 255.0) as u8;
    Color32::from_rgba_unmultiplied(r, g, b, next_alpha)
}
