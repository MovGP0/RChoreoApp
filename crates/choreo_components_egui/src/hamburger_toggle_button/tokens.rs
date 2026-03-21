use crate::material::styling::material_animations::MaterialAnimation;
use crate::material::styling::material_animations::MaterialAnimationSpec;
use crate::material::styling::material_animations::MaterialAnimations;
use crate::material::styling::material_style_metrics::material_style_metrics;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateOpacityTokens {
    pub disabled: f32,
    pub hover: f32,
    pub pressed: f32,
}

#[must_use]
pub const fn minimum_button_size_token() -> f32 {
    material_style_metrics().sizes.size_40
}

#[must_use]
pub const fn content_padding_token() -> f32 {
    material_style_metrics().paddings.padding_10
}

#[must_use]
pub const fn state_opacity_tokens() -> StateOpacityTokens {
    let metrics = material_style_metrics();
    StateOpacityTokens {
        disabled: metrics.state_opacities.content_disabled,
        hover: metrics.state_opacities.hover,
        pressed: metrics.state_opacities.pressed,
    }
}

#[must_use]
pub const fn checked_animation_spec() -> MaterialAnimationSpec {
    MaterialAnimations::spec(MaterialAnimation::Emphasized)
}

#[must_use]
pub const fn unchecked_animation_spec() -> MaterialAnimationSpec {
    MaterialAnimations::spec(MaterialAnimation::StandardFast)
}

#[must_use]
pub const fn unchecked_animation_easing_spec() -> MaterialAnimationSpec {
    MaterialAnimations::spec(MaterialAnimation::StandardDecelerate)
}

#[must_use]
pub const fn state_layer_animation_spec() -> MaterialAnimationSpec {
    MaterialAnimations::spec(MaterialAnimation::Opacity)
}
