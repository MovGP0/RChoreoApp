use egui::Rect;
use egui::Response;
use egui::ScrollArea;
use egui::Ui;
use egui::Vec2;
use egui::vec2;

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

pub struct ScrollViewResponse<R> {
    pub inner: R,
    pub response: Response,
    pub viewport_rect: Rect,
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
        let inner = ui.allocate_ui_with_layout(
            desired,
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                let mut scroll_area = ScrollArea::both().auto_shrink([false, false]);
                scroll_area = match self.state.horizontal_scrollbar_policy {
                    ScrollBarPolicy::AlwaysOff => scroll_area.horizontal_scroll_offset(0.0),
                    ScrollBarPolicy::AlwaysOn | ScrollBarPolicy::AsNeeded => scroll_area,
                };
                if self.state.enabled {
                    scroll_area
                        .show_viewport(ui, |ui, viewport| {
                            self.update_state_from_viewport(ui, viewport);
                            let inner = add_contents(ui);
                            self.update_viewport_extent(ui);
                            (inner, viewport)
                        })
                        .inner
                } else {
                    ui.add_enabled_ui(false, |ui| {
                        scroll_area
                            .show_viewport(ui, |ui, viewport| {
                                self.update_state_from_viewport(ui, viewport);
                                let inner = add_contents(ui);
                                self.update_viewport_extent(ui);
                                (inner, viewport)
                            })
                            .inner
                    })
                    .inner
                }
            },
        );
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

#[cfg(test)]
mod tests {
    use egui::Context;

    use super::ScrollBarPolicy;
    use super::ScrollView;
    use super::ScrollViewState;

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
}
