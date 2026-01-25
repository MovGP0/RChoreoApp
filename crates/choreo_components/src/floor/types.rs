#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix {
    pub values: [f32; 9],
}

impl Matrix {
    pub fn identity() -> Self {
        Self {
            values: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrawFloorCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PanUpdatedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PinchUpdatedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerPressedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerMovedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerReleasedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct PointerWheelChangedCommand;

#[derive(Debug, Clone, PartialEq)]
pub struct TouchCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanvasViewHandle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RgbaColor {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl RgbaColor {
    pub fn to_rgba8(self) -> [u8; 4] {
        [
            float_to_u8(self.red),
            float_to_u8(self.green),
            float_to_u8(self.blue),
            float_to_u8(self.alpha),
        ]
    }
}

fn float_to_u8(value: f32) -> u8 {
    let clamped = value.clamp(0.0, 1.0);
    (clamped * 255.0).round() as u8
}

pub trait FloorRenderGate: Send + Sync {
    fn is_rendered(&self) -> bool;
    fn mark_rendered(&self);
    fn wait_for_first_render(&self);
}

#[derive(Debug, Default)]
pub struct FloorRenderGateImpl {
    state: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
}

impl FloorRenderGateImpl {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FloorRenderGate for FloorRenderGateImpl {
    fn is_rendered(&self) -> bool {
        let (lock, _) = &*self.state;
        *lock.lock().unwrap_or_else(|guard| guard.into_inner())
    }

    fn mark_rendered(&self) {
        let (lock, condvar) = &*self.state;
        let mut rendered = lock.lock().unwrap_or_else(|guard| guard.into_inner());
        if *rendered {
            return;
        }
        *rendered = true;
        condvar.notify_all();
    }

    fn wait_for_first_render(&self) {
        let (lock, condvar) = &*self.state;
        let mut rendered = lock.lock().unwrap_or_else(|guard| guard.into_inner());
        while !*rendered {
            rendered = condvar
                .wait(rendered)
                .unwrap_or_else(|guard| guard.into_inner());
        }
    }
}
