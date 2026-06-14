# Styling

This project follows Material Design 3 color roles and keeps styling decisions in UI/theme modules. Business behaviors must not choose visual colors.

## Terminology

- In Material 3 naming, a palette is a list of colors that are constant in hue and chroma, but vary in tone.
- A swatch is one color's tone ladder: tone `0` is pure black, tone `100` is pure white, and tone `50` is the baseline tone for the color. Common Material usage includes tone `90` for light-theme containers, tones `40` or `50` for main elements or on-color content, and tone `10` for deep dark-theme surfaces.
- Material 3 typically generates five essential swatches from the core colors: primary for key components and prominent actions, secondary for less prominent components, tertiary for contrasting accents, neutral for backgrounds and surfaces, and neutral variant for unaccented borders and dividers.
- A scheme is a named set of resolved Material color roles, for example `Primary`, `OnSurface`, or `SurfaceContainerHigh`.
- A theme combines the active scheme with non-scheme UI tokens such as state-layer opacities, modal overlays, and elevation shadows.

## Scheme Generation

Theme generation and role lookup live in:

- `crates/material3/src/styling/material_schemes.rs`
- `crates/material3/src/styling/material_palette.rs`
- UI components under `crates/material3/src/components`
- App-specific widgets under `crates/choreo_components/src`

Generate Material colors with:

- Default seed: `#FF1976D2`
- Variant: `SchemeContent`
- Spec: `Spec2025`
- Platform: `Phone`
- Contrast: `0.5`

The user should be able to modify primary, secondary, and tertiary colors, as well as the variant in settings.

- Secondary is ignored unless primary custom color is enabled.
- Tertiary is ignored unless secondary custom color is enabled.
- If primary is disabled, use the default seed and ignore all stored custom colors.

Keep this gating in settings state/reducer code before calling `MaterialSchemes::from_seed_colors`.

## Role Usage

Use semantic Material roles, not literal colors. If a widget needs a color, choose the role by surface/function:

| UI element | Material role |
| --- | --- |
| App root background | `Background` |
| Window/content surface | `Surface` |
| Standard text on surfaces | `OnSurface` |
| Secondary/supporting text | `OnSurfaceVariant` |
| Primary action fill | `Primary` |
| Text on primary action fill | `OnPrimary` |
| Selected list/card background | `PrimaryContainer` |
| Text on selected list/card | `OnPrimaryContainer` |
| Selected list/card border or accent | `Primary` |
| Navigation selected indicator | `SecondaryContainer` |
| Navigation selected content | `OnSecondaryContainer` |
| Navigation unselected content | `OnSurface` or `OnSurfaceVariant` per original control |
| Outlines and separators | `OutlineVariant` |
| Strong outlined component border | `Outline` |
| Error badge/fill | `Error` |
| Text on error fill | `OnError` |
| Tooltip background | `InverseSurface` |
| Tooltip text | `InverseOnSurface` |
| Modal scrim | local `background_modal` derived from black alpha |
| Disabled filled-button background | `SurfaceVariant` |
| Disabled filled-button text | `OnSurfaceVariant` |
| Filled text field background | `SurfaceContainerLow` |

## Component-Specific Rules

### State Layers and Ripple Effects

- Hover, focus, pressed, dragged, and disabled feedback should use Material state-layer opacity tokens from the active Material theme, not ad hoc alpha values.
- State-layer base colors must come from semantic roles such as `OnSurface`, `OnSurfaceVariant`, `Primary`, or the selected content role for the component state.
- Light and dark themes must resolve state-layer colors through the active palette. A light theme should generally use a darker state color on light surfaces; a dark theme should use a lighter state color on dark surfaces.
- Hover feedback may be a static shape that matches the control shape.
- Press feedback may be a ripple only when it visibly animates. Do not draw a static full-size fill and call it a ripple.
- Ripple geometry must match the component shape. Circular icon buttons need circular hover and pressed feedback; do not clip an expanding ripple with a square rect unless the component itself is rectangular.
- Pressed and hover feedback must be visually distinct, typically by using the `press` state-layer opacity for pressed/ripple feedback and the `hover` state-layer opacity for hover feedback.

### Floor Canvas

- Canvas clear/background: `Surface`
- Floor border and center border: `Primary`
- Grid lines: `Secondary`
- Selection stroke: `Secondary`
- Titles: `OnSurface`
- Subtitles, axis labels, and supporting labels: `OnSurfaceVariant`
- Transparent/missing dancer fallback fill: `SurfaceVariant`
- Transparent/missing dancer fallback border: `OutlineVariant`
- Position text: choose black or white from contrast against the dancer fill

Keep geometry and state in `floor_adapter.rs` / reducer code; keep paint colors in `floor/ui.rs`.

### Scenes

- Pane background: `Surface`
- Unselected item background: `Surface`
- Unselected item stroke: `OutlineVariant`
- Unselected title: `OnSurface`
- Unselected timestamp: `OnSurfaceVariant`
- Selected item background: `PrimaryContainer`
- Selected item stroke/accent: `Primary`
- Selected title/timestamp: `OnPrimaryContainer`
- Scene swatch: bound scene color, not a Material role

### Navigation and Hamburger

- Hamburger normal bar: `OnSurface`
- Hamburger checked bar: `Primary`
- Hamburger disabled bar: `OnSurfaceVariant`
- Hamburger checked state-layer/background: `SurfaceVariant`
- Navigation selected indicator: `SecondaryContainer`
- Navigation selected content: `OnSecondaryContainer`
- Navigation drawer unselected content: `OnSurfaceVariant`

### Buttons

Symbolic Buttons:

- Enabled background: `Primary`
- Enabled text/icon: `OnPrimary`
- Disabled background: `SurfaceVariant`
- Disabled text/icon: `OnSurfaceVariant`

Text buttons:

- Enabled text/icon: `Primary`
- Explicit inverse text button: `InversePrimary`
- Disabled text/icon: `OnSurfaceVariant`

Do not silently use `OnSurface` for disabled action content.

### Fields

Filled fields and filled pickers use:

- Container: `SurfaceContainerLow`
- Active indicator/focus: `Primary`
- Error indicator/supporting text: `Error`
- Inactive indicator/label: `OnSurfaceVariant`
- Disabled container: `SurfaceContainerLow` with disabled opacity

### Audio Player

- Container background: `SurfaceContainer`
- Container stroke: `OutlineVariant`
- Time labels: `OnSurfaceVariant`
- Tick marks may use `Secondary` when they are timeline markers.

### Dancers

Model colors remain data colors:

- Dancer swatches/icons use dancer or role color from the model.
- Text on panels uses `OnSurface` / `OnSurfaceVariant`.
- Dialog surfaces use `Surface` unless the original style specifies a container role.
