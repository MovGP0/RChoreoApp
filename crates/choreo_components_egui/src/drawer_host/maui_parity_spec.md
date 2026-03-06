# MAUI DrawerHost Parity Specification

## Purpose

This document captures the functional contract of the MAUI `DrawerHost` so the egui `drawer_host` can be made behaviorally equivalent.

Primary reference files:

- `D:\ChoreoApp\MaterialDesignThemes.Maui\DrawerHost.cs`
- `D:\ChoreoApp\MaterialDesignThemes.Maui\DrawerHostOpenMode.cs`
- `D:\ChoreoApp\MaterialDesignThemes.Maui\DrawerHost.LeftDrawer.cs`
- `D:\ChoreoApp\MaterialDesignThemes.Maui\DrawerHost.RightDrawer.cs`
- `D:\ChoreoApp\MaterialDesignThemes.Maui\DrawerHost.TopDrawer.cs`
- `D:\ChoreoApp\MaterialDesignThemes.Maui\DrawerHost.BottomDrawer.cs`

## Component Scope

The MAUI implementation defines a single `DrawerHost` component with:

- one `MainContent` slot
- one overlay scrim layer
- four independent drawers:
  - left
  - top
  - right
  - bottom

The host is responsive, but only the left drawer participates in responsive inline layout.

## Supported Public Surface

### Global Properties

Defined in `DrawerHost.cs`:

- `MainContent: View?`
- `OverlayBackground: Brush`
  - default: `#66000000`
- `ResponsiveBreakpoint: double`
  - default: `900`
- `OpenMode: DrawerHostOpenMode`
  - `Default`
  - `Modal`
  - `Standard`

### Per-Drawer Properties

Defined across the partial class files:

- `LeftDrawerContent: View?`
- `RightDrawerContent: View?`
- `TopDrawerContent: View?`
- `BottomDrawerContent: View?`

- `LeftDrawerWidth: double`
- `RightDrawerWidth: double`
- `TopDrawerHeight: double`
- `BottomDrawerHeight: double`
  - default size for each dimension: `320`

- `IsLeftDrawerOpen: bool`
- `IsRightDrawerOpen: bool`
- `IsTopDrawerOpen: bool`
- `IsBottomDrawerOpen: bool`
  - all are `TwoWay` bindable in MAUI

- `LeftDrawerCloseOnClickAway: bool`
- `RightDrawerCloseOnClickAway: bool`
- `TopDrawerCloseOnClickAway: bool`
- `BottomDrawerCloseOnClickAway: bool`
  - default: `true`

### Events

Defined in `DrawerHost.cs`:

- `DrawerOpened`
  - raised after a drawer transitions open
- `DrawerClosing`
  - raised before a drawer closes
  - cancelable via `DrawerClosingEventArgs`

## Internal State Contract

Important internal state from `DrawerHost.cs`:

- `_layoutMode`
  - `Overlay`
  - `InlineLeft`
  - `InlineRight` exists in the enum but is not selected by current logic
- `_suppressStateHandler`
  - prevents recursive property-changed handling when a close is canceled

Equivalent egui implementation should preserve the same state semantics even if the internal representation differs.

## Layout Contract

### Presenter Model

The MAUI host pre-creates one presenter per layer:

- `_mainPresenter`
- `_overlay`
- `_leftDrawerPresenter`
- `_topDrawerPresenter`
- `_rightDrawerPresenter`
- `_bottomDrawerPresenter`

Each drawer presenter is a `ContentView` with:

- `IsVisible = false` initially
- `InputTransparent = false`
- white background by default

### Docking Rules

Initial docking in `DrawerHost.cs`:

- left drawer:
  - `HorizontalOptions = Start`
  - `VerticalOptions = Fill`
  - width-driven
- right drawer:
  - `HorizontalOptions = End`
  - `VerticalOptions = Fill`
  - width-driven
- top drawer:
  - `HorizontalOptions = Fill`
  - `VerticalOptions = Start`
  - height-driven
- bottom drawer:
  - `HorizontalOptions = Fill`
  - `VerticalOptions = End`
  - height-driven

### Layer Ordering

MAUI z-order in `DrawerHost.cs`:

- main content: `0`
- overlay: `1`
- top drawer: `2`
- bottom drawer: `3`
- left drawer: `4`
- right drawer: `5`

Equivalent egui layering should preserve this ordering. In particular:

- overlay must sit above main content
- left/right drawers must sit above top/bottom drawers
- right drawer must be top-most among drawers

### Responsive Layout

Responsive decision is defined in `ShouldInlineLeftDrawer` and `UpdateLayoutMode` in `DrawerHost.cs`.

Rules:

- if `OpenMode == Modal`, left drawer is never inline
- otherwise, left drawer becomes inline when `host_width >= ResponsiveBreakpoint`
- current MAUI logic only selects:
  - `Overlay`
  - `InlineLeft`

There is no active inline-right mode in the current MAUI implementation.

### InlineLeft Grid Layout

When inline-left is active:

- root grid columns become:
  - column 0: `LeftDrawerWidth`
  - column 1: `*`
- left drawer occupies column 0
- main content occupies column 1
- overlay spans both columns
- top drawer spans both columns
- bottom drawer spans both columns
- right drawer stays aligned to the main-content column

Equivalent egui behavior:

- the left drawer consumes horizontal content space
- the main content is shifted right by left drawer width
- overlay does not obscure the inline left drawer itself
- right drawer still anchors to the content area on the right side

### Overlay Layout

When not inline-left:

- the root grid has a single `*` column
- all layers overlap in the same host space
- left and right drawers anchor to the host edges
- top and bottom drawers anchor to the host top and bottom edges

Equivalent egui behavior:

- the drawer host rectangle defines the coordinate space for all drawers
- overlay and drawers are positioned in host coordinates, not page-local widget coordinates

## Overlay Behavior

Defined in `UpdateOverlayVisibility` and `OnOverlayTapped` in `DrawerHost.cs`.

Overlay visibility is enabled only when at least one open drawer also qualifies for click-away closing:

- left drawer contributes only if:
  - `IsLeftDrawerOpen == true`
  - layout mode is `Overlay`
  - `LeftDrawerCloseOnClickAway == true`
- top drawer contributes if:
  - `IsTopDrawerOpen == true`
  - `TopDrawerCloseOnClickAway == true`
- right drawer contributes if:
  - `IsRightDrawerOpen == true`
  - `RightDrawerCloseOnClickAway == true`
- bottom drawer contributes if:
  - `IsBottomDrawerOpen == true`
  - `BottomDrawerCloseOnClickAway == true`

The overlay:

- becomes visible when any qualifying drawer is open
- becomes input-transparent when hidden
- uses `OverlayBackground`
- handles tap-to-close

Equivalent egui behavior:

- overlay visibility must be recomputed from open-state and click-away flags
- inline-left must be excluded from overlay gating
- clicking the scrim must attempt to close every qualifying drawer in one interaction

## Open and Close Semantics

### Opening

Per-drawer property changed callbacks in the partial files dispatch to `HandleDrawerStateChanged`.

On open:

- drawer presenter becomes visible
- overlay visibility is recalculated
- `DrawerOpened` is raised

### Closing

On close:

- `DrawerClosing(dock)` is raised first
- handlers may cancel closure
- if canceled:
  - the open flag is restored to `true`
  - `_suppressStateHandler` prevents recursive re-entry
- if not canceled:
  - drawer presenter visibility is updated
  - overlay visibility is recalculated

Equivalent egui behavior:

- closing must support a cancelable pre-close hook/effect
- state restoration on cancellation must not recurse infinitely
- post-open and pre-close events/effects must remain dock-specific

## Multi-Drawer Semantics

The MAUI implementation does not enforce mutual exclusion between drawers.

Observed contract:

- multiple drawers may be open at the same time
- overlay close tap may close more than one drawer in a single interaction
- drawer visibility is independently managed per side

Equivalent egui behavior should not introduce hidden exclusivity rules unless intentionally documented as a divergence.

## Sizing Rules

From the partial files:

- default drawer width: `320`
- default drawer height: `320`

Size property changes apply immediately:

- left width updates presenter width
- right width updates presenter width
- top height updates presenter height
- bottom height updates presenter height

Special case:

- if left drawer width changes while layout mode is `InlineLeft`, the inline grid column width is updated immediately
- MAUI also contains an `InlineRight` update branch in `DrawerHost.RightDrawer.cs`, but current layout selection logic never enters `InlineRight`

Equivalent egui behavior:

- per-side size properties must update layout immediately
- inline-left width changes must also update content offset/content bounds

## Non-Features in the Current MAUI Version

The referenced MAUI implementation does not currently provide:

- open/close animation
- drag gestures for drawers
- drawer edge swipe gestures
- min/max clamping rules
- mutual exclusion between drawers
- sophisticated presenter lifecycle beyond visibility and content assignment

Equivalent egui work should not invent these behaviors as parity defaults.

## Integration Guidance for egui

### Shared `drawer_host`

The egui `drawer_host` should be the shared primitive used by:

- `main_page`
- `dancer_settings_page`

It should not require a page-specific fork for runtime behavior parity.

### `main_page` Integration

Expected usage:

- host rect fills the shell content area
- nav bar lives above the host
- audio player, when open, reserves space below the host
- left/right drawers use shared host coordinates
- responsive inline-left behavior should map to host width vs breakpoint

### `dancer_settings_page` Integration

Expected usage:

- page should depend on shared `drawer_host`, not `main_page_drawer_host`
- left drawer behavior remains page-local through page-specific state mapping
- overlay close should emit page-specific close action

## Required egui Parity Checklist

For the egui drawer host to be MAUI-equivalent, it must support:

- four drawers in one host
- independent content per drawer
- independent open flags per drawer
- independent click-away flags per drawer
- global overlay background
- `OpenMode` equivalent with:
  - `Modal`
  - `Standard`
- responsive breakpoint with default `900`
- left-only inline responsive behavior
- per-drawer sizing with default `320`
- simultaneous open drawers
- overlay gating that excludes inline-left
- cancelable close pipeline
- post-open notification/effect
- layer ordering equivalent to MAUI z-order
- host-relative coordinate layout, not local widget-relative drift

## Known MAUI Implementation Quirks to Preserve or Document

- `DrawerHostOpenMode.Default` is aliased to modal behavior
- `InlineRight` exists in internal enum/width update logic but is not selected by `UpdateLayoutMode`
- drawers are shown/hidden by visibility toggles rather than animated transitions

If egui intentionally diverges from any of these points, the divergence should be documented explicitly beside the implementation.
