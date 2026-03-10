use std::hash::Hash;

use egui::Id;
use egui::LayerId;
use egui::Layout;
use egui::Rect;
use egui::Response;
use egui::ScrollArea;
use egui::Ui;
use egui::UiBuilder;
use egui::Vec2;
use egui::Vec2b;
use egui::emath::TSTransform;
use egui::pos2;
use egui::vec2;

const OVERSCROLL_EDGE_EPSILON: f32 = 0.5;
const OVERSCROLL_DAMPING: f32 = 0.35;
const OVERSCROLL_RELEASE_PER_SECOND: f32 = 10.0;
const OVERSCROLL_VIEWPORT_RATIO: f32 = 0.12;
const OVERSCROLL_MAX_DISTANCE: f32 = 120.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollBarPolicy {
    AlwaysOn,
    AlwaysOff,
    AsNeeded,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScrollViewState {
    pub enabled: bool,
    pub visible_width: f32,
    pub visible_height: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub viewport_x: f32,
    pub viewport_y: f32,
    pub vertical_scrollbar_policy: ScrollBarPolicy,
    pub horizontal_scrollbar_policy: ScrollBarPolicy,
    pub has_focus: bool,
    pub scroll_bar_size: f32,
    pub scroll_bar_padding: f32,
    pub min_size: Vec2,
}

impl Default for ScrollViewState {
    fn default() -> Self {
        Self {
            enabled: true,
            visible_width: 0.0,
            visible_height: 0.0,
            viewport_width: 0.0,
            viewport_height: 0.0,
            viewport_x: 0.0,
            viewport_y: 0.0,
            vertical_scrollbar_policy: ScrollBarPolicy::AsNeeded,
            horizontal_scrollbar_policy: ScrollBarPolicy::AsNeeded,
            has_focus: false,
            scroll_bar_size: 8.0,
            scroll_bar_padding: 1.0,
            min_size: vec2(50.0, 50.0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct StretchOverscrollState {
    stretch_y: f32,
}

pub struct ScrollViewResponse<R> {
    pub inner: R,
    pub response: Response,
    pub viewport_rect: Rect,
}

#[derive(Clone, Debug)]
pub struct MaterialScrollArea {
    id: Option<Id>,
    scroll_area: ScrollArea,
}

impl MaterialScrollArea {
    #[must_use]
    pub fn vertical() -> Self {
        Self::new([false, true])
    }

    #[must_use]
    pub fn both() -> Self {
        Self::new([true, true])
    }

    #[must_use]
    pub fn new(direction_enabled: impl Into<Vec2b>) -> Self {
        Self {
            id: None,
            scroll_area: ScrollArea::new(direction_enabled),
        }
    }

    #[must_use]
    pub fn auto_shrink(mut self, auto_shrink: [bool; 2]) -> Self {
        self.scroll_area = self.scroll_area.auto_shrink(auto_shrink);
        self
    }

    #[must_use]
    pub fn id_salt(mut self, id_salt: impl Hash) -> Self {
        self.id = Some(Id::new(&id_salt));
        self.scroll_area = self.scroll_area.id_salt(id_salt);
        self
    }

    #[must_use]
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.scroll_area = self.scroll_area.max_height(max_height);
        self
    }

    #[must_use]
    pub fn horizontal_scroll_offset(mut self, offset: f32) -> Self {
        self.scroll_area = self.scroll_area.horizontal_scroll_offset(offset);
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
        self.show_viewport(ui, |ui, _viewport| add_contents(ui))
    }

    pub fn show_viewport<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui, Rect) -> R,
    ) -> R {
        let wrapper_id = self
            .id
            .unwrap_or_else(|| ui.auto_id_with("material_scroll_area"));
        let stretch_state_id = wrapper_id.with("material_stretch_overscroll_state");
        let stretch_layer_id = LayerId::new(ui.layer_id().order, wrapper_id.with("stretch_layer"));
        let prior_overscroll = ui
            .ctx()
            .data(|data| data.get_temp::<StretchOverscrollState>(stretch_state_id))
            .unwrap_or_default();
        let output = self.scroll_area.show_viewport(ui, |ui, viewport| {
            let clip_rect = ui.clip_rect();
            let content_rect = ui.max_rect();
            let transform = stretch_transform_for_overscroll(clip_rect, prior_overscroll.stretch_y);

            ui.ctx().set_sublayer(ui.layer_id(), stretch_layer_id);
            ui.ctx().set_transform_layer(stretch_layer_id, transform);

            ui.scope_builder(
                UiBuilder::new().layer_id(stretch_layer_id).max_rect(content_rect),
                |ui| {
                    ui.set_clip_rect(transform.inverse() * clip_rect);
                    add_contents(ui, viewport)
                },
            )
            .inner
        });

        let stretch = apply_stretch_overscroll(
            ui,
            wrapper_id,
            output.inner_rect,
            output.content_size.y,
            output.state.offset.y,
        );
        ui.ctx().set_transform_layer(
            stretch_layer_id,
            stretch_transform_for_overscroll(output.inner_rect, stretch),
        );

        output.inner
    }
}

pub struct ScrollView<'a> {
    state: &'a mut ScrollViewState,
}

impl<'a> ScrollView<'a> {
    #[must_use]
    pub fn new(state: &'a mut ScrollViewState) -> Self {
        Self { state }
    }

    pub fn show<R>(
        mut self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> ScrollViewResponse<R> {
        let desired = vec2(
            self.state.min_size.x.max(ui.available_width().max(0.0)),
            self.state.min_size.y,
        );
        let inner = ui.allocate_ui_with_layout(desired, Layout::top_down(egui::Align::Min), |ui| {
            let mut scroll_area = MaterialScrollArea::both().auto_shrink([false, false]);
            scroll_area = match self.state.horizontal_scrollbar_policy {
                ScrollBarPolicy::AlwaysOff => scroll_area.horizontal_scroll_offset(0.0),
                ScrollBarPolicy::AlwaysOn | ScrollBarPolicy::AsNeeded => scroll_area,
            };

            if self.state.enabled {
                scroll_area.show_viewport(ui, |ui, viewport| {
                    self.update_state_from_viewport(ui, viewport);
                    let inner = add_contents(ui);
                    self.update_viewport_extent(ui);
                    (inner, viewport)
                })
            } else {
                ui.add_enabled_ui(false, |ui| {
                    scroll_area.show_viewport(ui, |ui, viewport| {
                        self.update_state_from_viewport(ui, viewport);
                        let inner = add_contents(ui);
                        self.update_viewport_extent(ui);
                        (inner, viewport)
                    })
                })
                .inner
            }
        });
        let ((inner, viewport_rect), response) = (inner.inner, inner.response);
        ScrollViewResponse {
            inner,
            response,
            viewport_rect,
        }
    }

    fn update_state_from_viewport(&mut self, ui: &Ui, viewport: Rect) {
        self.state.visible_width = viewport.width();
        self.state.visible_height = viewport.height();
        self.state.viewport_x = viewport.min.x;
        self.state.viewport_y = viewport.min.y;
        self.state.has_focus = ui.memory(|memory| memory.has_focus(ui.id()));
    }

    fn update_viewport_extent(&mut self, ui: &Ui) {
        self.state.viewport_width = self.state.viewport_width.max(ui.min_rect().width());
        self.state.viewport_height = self.state.viewport_height.max(ui.min_rect().height());
    }
}

fn apply_stretch_overscroll(
    ui: &Ui,
    scroll_area_id: Id,
    viewport_rect: Rect,
    content_height: f32,
    scroll_offset_y: f32,
) -> f32 {
    if viewport_rect.height() <= 0.0 {
        return 0.0;
    }

    let max_offset_y = (content_height - viewport_rect.height()).max(0.0);
    let state_id = scroll_area_id.with("material_stretch_overscroll_state");
    let is_hovering = ui.rect_contains_pointer(viewport_rect);
    let wheel_delta_y = if is_hovering {
        ui.input(|input| input.smooth_scroll_delta.y)
    } else {
        0.0
    };
    let is_pointer_down = ui.input(|input| input.pointer.primary_down());
    let drag_delta_y = if is_hovering && is_pointer_down {
        ui.input(|input| input.pointer.delta().y)
    } else {
        0.0
    };
    let mut overscroll = ui
        .ctx()
        .data(|data| data.get_temp::<StretchOverscrollState>(state_id))
        .unwrap_or_default();

    if wheel_delta_y != 0.0 {
        overscroll.stretch_y = apply_overscroll_delta(
            overscroll.stretch_y,
            wheel_delta_y,
            scroll_offset_y,
            max_offset_y,
            viewport_rect.height(),
        );
    }

    if drag_delta_y != 0.0 {
        overscroll.stretch_y = apply_overscroll_delta(
            overscroll.stretch_y,
            drag_delta_y,
            scroll_offset_y,
            max_offset_y,
            viewport_rect.height(),
        );
    }

    if wheel_delta_y == 0.0 && drag_delta_y == 0.0 {
        overscroll.stretch_y =
            relax_overscroll_toward_rest(overscroll.stretch_y, ui.input(|input| input.stable_dt));
    }

    if overscroll.stretch_y != 0.0 {
        ui.ctx().request_repaint();
    }

    ui.ctx()
        .data_mut(|data| data.insert_temp(state_id, overscroll));

    overscroll.stretch_y
}

#[must_use]
fn max_overscroll_distance(viewport_height: f32) -> f32 {
    (viewport_height * OVERSCROLL_VIEWPORT_RATIO).clamp(24.0, OVERSCROLL_MAX_DISTANCE)
}

#[must_use]
fn apply_overscroll_delta(
    overscroll_y: f32,
    delta_y: f32,
    scroll_offset_y: f32,
    max_offset_y: f32,
    viewport_height: f32,
) -> f32 {
    if delta_y == 0.0 || viewport_height <= 0.0 {
        return overscroll_y;
    }

    let max_stretch = max_overscroll_distance(viewport_height);
    let is_releasing_existing_stretch =
        overscroll_y != 0.0 && delta_y.signum() != overscroll_y.signum();

    if is_releasing_existing_stretch {
        return (overscroll_y + delta_y).clamp(-max_stretch, max_stretch);
    }

    let at_top = scroll_offset_y <= OVERSCROLL_EDGE_EPSILON;
    let at_bottom = scroll_offset_y >= (max_offset_y - OVERSCROLL_EDGE_EPSILON).max(0.0);
    let can_stretch =
        (delta_y > 0.0 && at_top) || (delta_y < 0.0 && at_bottom) || max_offset_y <= 0.0;

    if !can_stretch {
        return overscroll_y;
    }

    let remaining_ratio = 1.0 - (overscroll_y.abs() / max_stretch).clamp(0.0, 1.0);
    let damped_delta = delta_y * OVERSCROLL_DAMPING * remaining_ratio.max(0.15);

    (overscroll_y + damped_delta).clamp(-max_stretch, max_stretch)
}

#[must_use]
fn relax_overscroll_toward_rest(overscroll_y: f32, dt: f32) -> f32 {
    if overscroll_y.abs() <= 0.01 {
        return 0.0;
    }

    let release_factor = (1.0 - OVERSCROLL_RELEASE_PER_SECOND * dt).clamp(0.0, 1.0);
    let relaxed = overscroll_y * release_factor;

    if relaxed.abs() <= 0.1 {
        0.0
    } else {
        relaxed
    }
}

#[must_use]
fn stretch_transform_for_overscroll(viewport_rect: Rect, overscroll_y: f32) -> TSTransform {
    if overscroll_y == 0.0 || viewport_rect.height() <= 0.0 {
        return TSTransform::IDENTITY;
    }

    let anchor = if overscroll_y >= 0.0 {
        pos2(viewport_rect.center().x, viewport_rect.top())
    } else {
        pos2(viewport_rect.center().x, viewport_rect.bottom())
    };
    let scale = 1.0 + overscroll_y.abs() / viewport_rect.height();

    TSTransform::from_translation(anchor.to_vec2())
        * TSTransform::from_scaling(scale)
        * TSTransform::from_translation(-anchor.to_vec2())
}

#[cfg(test)]
mod tests {
    use egui::Context;
    use egui::Rect;
    use egui::pos2;

    use super::ScrollBarPolicy;
    use super::ScrollView;
    use super::ScrollViewState;
    use super::apply_overscroll_delta;
    use super::max_overscroll_distance;
    use super::relax_overscroll_toward_rest;
    use super::stretch_transform_for_overscroll;

    #[test]
    fn scroll_view_defaults_match_slint_shell() {
        let state = ScrollViewState::default();
        assert_eq!(state.vertical_scrollbar_policy, ScrollBarPolicy::AsNeeded);
        assert_eq!(state.horizontal_scrollbar_policy, ScrollBarPolicy::AsNeeded);
        assert_eq!(state.scroll_bar_size, 8.0);
        assert_eq!(state.scroll_bar_padding, 1.0);
        assert_eq!(state.min_size.x, 50.0);
        assert_eq!(state.min_size.y, 50.0);
    }

    #[test]
    fn scroll_view_reports_visible_viewport_size() {
        let context = Context::default();
        let mut state = ScrollViewState::default();
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let _ = ScrollView::new(&mut state).show(ui, |ui| {
                    ui.add_sized([200.0, 120.0], egui::Label::new("content"));
                });
            });
        });
        assert!(state.visible_width > 0.0);
        assert!(state.visible_height > 0.0);
    }

    #[test]
    fn overscroll_delta_accumulates_only_at_active_edges() {
        assert!(apply_overscroll_delta(0.0, 24.0, 0.0, 180.0, 240.0) > 0.0);
        assert!(apply_overscroll_delta(0.0, -24.0, 180.0, 180.0, 240.0) < 0.0);
        assert_eq!(apply_overscroll_delta(0.0, 24.0, 72.0, 180.0, 240.0), 0.0);
    }

    #[test]
    fn overscroll_delta_is_clamped_to_viewport_limit() {
        let overscroll = apply_overscroll_delta(80.0, 200.0, 0.0, 180.0, 240.0);
        assert!(overscroll <= max_overscroll_distance(240.0));
    }

    #[test]
    fn overscroll_relaxes_toward_rest_after_release() {
        let relaxed = relax_overscroll_toward_rest(48.0, 1.0 / 60.0);
        assert!(relaxed < 48.0);
        assert!(relaxed > 0.0);
    }

    #[test]
    fn overscroll_transform_keeps_the_active_edge_fixed() {
        let viewport = Rect::from_min_max(pos2(24.0, 32.0), pos2(224.0, 332.0));
        let top_transform = stretch_transform_for_overscroll(viewport, 36.0);
        let bottom_transform = stretch_transform_for_overscroll(viewport, -36.0);
        let top_anchor = pos2(viewport.center().x, viewport.top());
        let bottom_anchor = pos2(viewport.center().x, viewport.bottom());

        assert_eq!(top_transform * top_anchor, top_anchor);
        assert_eq!(bottom_transform * bottom_anchor, bottom_anchor);
    }
}
