# Styling

This project follows Material Design 3 color roles and keeps styling decisions in UI/theme modules. Business behaviors must not choose visual colors.

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
