use egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RectSpec {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PanelSpec {
    pub visible: bool,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MainPageDrawerHostState {
    pub left_drawer_width: f32,
    pub right_drawer_width: f32,
    pub top_inset: f32,
    pub inline_left: bool,
    pub is_left_open: bool,
    pub is_right_open: bool,
    pub left_close_on_click_away: bool,
    pub right_close_on_click_away: bool,
    pub overlay_color: Color32,
    pub drawer_background: Color32,
    pub viewport_width: f32,
    pub viewport_height: f32,
}

impl MainPageDrawerHostState {
    #[must_use]
    pub fn inline_left_width(&self) -> f32 {
        if self.inline_left && self.is_left_open {
            self.left_drawer_width
        } else {
            0.0
        }
    }

    #[must_use]
    pub fn overlay_visible(&self) -> bool {
        (self.is_left_open && !self.inline_left && self.left_close_on_click_away)
            || (self.is_right_open && self.right_close_on_click_away)
    }

    #[must_use]
    pub fn content_rect(&self) -> RectSpec {
        let inline_left_width = self.inline_left_width();
        RectSpec {
            x: inline_left_width,
            y: self.top_inset,
            width: (self.viewport_width - inline_left_width).max(0.0),
            height: (self.viewport_height - self.top_inset).max(0.0),
        }
    }

    #[must_use]
    pub fn panel_area_rect(&self) -> RectSpec {
        RectSpec {
            x: 0.0,
            y: self.top_inset,
            width: self.viewport_width,
            height: (self.viewport_height - self.top_inset).max(0.0),
        }
    }

    #[must_use]
    pub fn left_panel(&self) -> PanelSpec {
        let panel_area = self.panel_area_rect();
        let x = if self.inline_left || self.is_left_open {
            0.0
        } else {
            -self.left_drawer_width
        };

        PanelSpec {
            visible: self.inline_left || self.is_left_open,
            x,
            y: 0.0,
            width: self.left_drawer_width,
            height: panel_area.height,
        }
    }

    #[must_use]
    pub fn right_panel(&self) -> PanelSpec {
        let panel_area = self.panel_area_rect();
        let x = if self.is_right_open {
            panel_area.width - self.right_drawer_width
        } else {
            panel_area.width
        };

        PanelSpec {
            visible: self.is_right_open,
            x,
            y: 0.0,
            width: self.right_drawer_width,
            height: panel_area.height,
        }
    }
}

impl Default for MainPageDrawerHostState {
    fn default() -> Self {
        Self {
            left_drawer_width: 320.0,
            right_drawer_width: 480.0,
            top_inset: 0.0,
            inline_left: false,
            is_left_open: false,
            is_right_open: false,
            left_close_on_click_away: true,
            right_close_on_click_away: true,
            overlay_color: Color32::from_rgba_unmultiplied(0, 0, 0, 128),
            drawer_background: Color32::from_rgb(238, 238, 238),
            viewport_width: 1280.0,
            viewport_height: 720.0,
        }
    }
}
