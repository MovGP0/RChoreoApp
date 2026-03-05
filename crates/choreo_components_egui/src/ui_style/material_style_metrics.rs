#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SizeTokens {
    pub size_1: f32,
    pub size_2: f32,
    pub size_3: f32,
    pub size_4: f32,
    pub size_6: f32,
    pub size_8: f32,
    pub size_14: f32,
    pub size_16: f32,
    pub size_18: f32,
    pub size_20: f32,
    pub size_24: f32,
    pub size_30: f32,
    pub size_32: f32,
    pub size_36: f32,
    pub size_38: f32,
    pub size_40: f32,
    pub size_48: f32,
    pub size_49: f32,
    pub size_52: f32,
    pub size_56: f32,
    pub size_64: f32,
    pub size_72: f32,
    pub size_80: f32,
    pub size_90: f32,
    pub size_200: f32,
    pub size_256: f32,
    pub size_344: f32,
    pub size_360: f32,
    pub size_572: f32,
    pub size_640: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IconSizeTokens {
    pub icon_size_18: f32,
    pub icon_size_24: f32,
    pub icon_size_36: f32,
    pub icon_size_90: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PaddingTokens {
    pub padding_4: f32,
    pub padding_6: f32,
    pub padding_8: f32,
    pub padding_10: f32,
    pub padding_12: f32,
    pub padding_14: f32,
    pub padding_16: f32,
    pub padding_20: f32,
    pub padding_24: f32,
    pub padding_28: f32,
    pub padding_30: f32,
    pub padding_44: f32,
    pub padding_56: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpacingTokens {
    pub spacing_2: f32,
    pub spacing_4: f32,
    pub spacing_6: f32,
    pub spacing_8: f32,
    pub spacing_12: f32,
    pub spacing_16: f32,
    pub spacing_40: f32,
    pub spacing_52: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CornerRadiusTokens {
    pub border_radius_2: f32,
    pub border_radius_4: f32,
    pub border_radius_8: f32,
    pub border_radius_12: f32,
    pub border_radius_16: f32,
    pub border_radius_28: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrokeTokens {
    pub outline: f32,
    pub focus: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElevationShadow {
    pub offset_y: f32,
    pub blur: f32,
    pub opacity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElevationLevel {
    pub outer: ElevationShadow,
    pub inner: ElevationShadow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElevationTokens {
    pub level_1: ElevationLevel,
    pub level_2: ElevationLevel,
    pub level_3: ElevationLevel,
    pub level_4: ElevationLevel,
    pub level_5: ElevationLevel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateOpacityTokens {
    pub hover: f32,
    pub focus: f32,
    pub pressed: f32,
    pub disabled: f32,
    pub drag: f32,
    pub content_disabled: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialStyleMetrics {
    pub sizes: SizeTokens,
    pub icon_sizes: IconSizeTokens,
    pub paddings: PaddingTokens,
    pub spacings: SpacingTokens,
    pub corner_radii: CornerRadiusTokens,
    pub strokes: StrokeTokens,
    pub elevations: ElevationTokens,
    pub state_opacities: StateOpacityTokens,
}

pub const fn material_style_metrics() -> MaterialStyleMetrics {
    MaterialStyleMetrics {
        sizes: SizeTokens {
            size_1: 1.0,
            size_2: 2.0,
            size_3: 3.0,
            size_4: 4.0,
            size_6: 6.0,
            size_8: 8.0,
            size_14: 14.0,
            size_16: 16.0,
            size_18: 18.0,
            size_20: 20.0,
            size_24: 24.0,
            size_30: 30.0,
            size_32: 32.0,
            size_36: 36.0,
            size_38: 38.0,
            size_40: 40.0,
            size_48: 48.0,
            size_49: 49.0,
            size_52: 52.0,
            size_56: 56.0,
            size_64: 64.0,
            size_72: 72.0,
            size_80: 80.0,
            size_90: 96.0,
            size_200: 200.0,
            size_256: 256.0,
            size_344: 344.0,
            size_360: 360.0,
            size_572: 572.0,
            size_640: 640.0,
        },
        icon_sizes: IconSizeTokens {
            icon_size_18: 18.0,
            icon_size_24: 24.0,
            icon_size_36: 36.0,
            icon_size_90: 27.0,
        },
        paddings: PaddingTokens {
            padding_4: 4.0,
            padding_6: 6.0,
            padding_8: 8.0,
            padding_10: 10.0,
            padding_12: 12.0,
            padding_14: 14.0,
            padding_16: 16.0,
            padding_20: 20.0,
            padding_24: 24.0,
            padding_28: 28.0,
            padding_30: 30.0,
            padding_44: 44.0,
            padding_56: 56.0,
        },
        spacings: SpacingTokens {
            spacing_2: 2.0,
            spacing_4: 4.0,
            spacing_6: 6.0,
            spacing_8: 8.0,
            spacing_12: 12.0,
            spacing_16: 16.0,
            spacing_40: 40.0,
            spacing_52: 52.0,
        },
        corner_radii: CornerRadiusTokens {
            border_radius_2: 2.0,
            border_radius_4: 4.0,
            border_radius_8: 8.0,
            border_radius_12: 12.0,
            border_radius_16: 16.0,
            border_radius_28: 28.0,
        },
        strokes: StrokeTokens {
            outline: 1.0,
            focus: 2.0,
        },
        elevations: ElevationTokens {
            level_1: ElevationLevel {
                outer: ElevationShadow {
                    offset_y: 1.0,
                    blur: 2.0,
                    opacity: 0.30,
                },
                inner: ElevationShadow {
                    offset_y: 1.0,
                    blur: 2.0,
                    opacity: 0.15,
                },
            },
            level_2: ElevationLevel {
                outer: ElevationShadow {
                    offset_y: 1.0,
                    blur: 2.0,
                    opacity: 0.30,
                },
                inner: ElevationShadow {
                    offset_y: 2.0,
                    blur: 6.0,
                    opacity: 0.15,
                },
            },
            level_3: ElevationLevel {
                outer: ElevationShadow {
                    offset_y: 4.0,
                    blur: 8.0,
                    opacity: 0.15,
                },
                inner: ElevationShadow {
                    offset_y: 1.0,
                    blur: 3.0,
                    opacity: 0.30,
                },
            },
            level_4: ElevationLevel {
                outer: ElevationShadow {
                    offset_y: 6.0,
                    blur: 10.0,
                    opacity: 0.15,
                },
                inner: ElevationShadow {
                    offset_y: 2.0,
                    blur: 3.0,
                    opacity: 0.30,
                },
            },
            level_5: ElevationLevel {
                outer: ElevationShadow {
                    offset_y: 8.0,
                    blur: 12.0,
                    opacity: 0.15,
                },
                inner: ElevationShadow {
                    offset_y: 4.0,
                    blur: 4.0,
                    opacity: 0.30,
                },
            },
        },
        state_opacities: StateOpacityTokens {
            hover: 0.08,
            focus: 0.10,
            pressed: 0.10,
            disabled: 0.12,
            drag: 0.16,
            content_disabled: 0.38,
        },
    }
}
