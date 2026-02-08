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

    pub fn width(self) -> f32 {
        self.right - self.left
    }

    pub fn height(self) -> f32 {
        self.bottom - self.top
    }

    pub fn contains(self, point: Point) -> bool {
        point.x as f32 >= self.left
            && point.x as f32 <= self.right
            && point.y as f32 >= self.top
            && point.y as f32 <= self.bottom
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
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
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

    pub fn translation(delta_x: f32, delta_y: f32) -> Self {
        Self {
            values: [1.0, 0.0, delta_x, 0.0, 1.0, delta_y, 0.0, 0.0, 1.0],
        }
    }

    pub fn scale(scale_x: f32, scale_y: f32, origin_x: f32, origin_y: f32) -> Self {
        let translate_to_origin = Self::translation(-origin_x, -origin_y);
        let scale = Self {
            values: [scale_x, 0.0, 0.0, 0.0, scale_y, 0.0, 0.0, 0.0, 1.0],
        };
        let translate_back = Self::translation(origin_x, origin_y);
        translate_back.concat(&scale).concat(&translate_to_origin)
    }

    pub fn concat(self, other: &Self) -> Self {
        let a = self.values;
        let b = other.values;

        Self {
            values: [
                a[0] * b[0] + a[1] * b[3] + a[2] * b[6],
                a[0] * b[1] + a[1] * b[4] + a[2] * b[7],
                a[0] * b[2] + a[1] * b[5] + a[2] * b[8],
                a[3] * b[0] + a[4] * b[3] + a[5] * b[6],
                a[3] * b[1] + a[4] * b[4] + a[5] * b[7],
                a[3] * b[2] + a[4] * b[5] + a[5] * b[8],
                a[6] * b[0] + a[7] * b[3] + a[8] * b[6],
                a[6] * b[1] + a[7] * b[4] + a[8] * b[7],
                a[6] * b[2] + a[7] * b[5] + a[8] * b[8],
            ],
        }
    }

    pub fn invert(self) -> Option<Self> {
        let a = self.values[0];
        let b = self.values[1];
        let c = self.values[3];
        let d = self.values[4];
        let tx = self.values[2];
        let ty = self.values[5];

        let det = a * d - b * c;
        if det.abs() <= f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;
        let inv_a = d * inv_det;
        let inv_b = -b * inv_det;
        let inv_c = -c * inv_det;
        let inv_d = a * inv_det;
        let inv_tx = (b * ty - d * tx) * inv_det;
        let inv_ty = (c * tx - a * ty) * inv_det;

        Some(Self {
            values: [inv_a, inv_b, inv_tx, inv_c, inv_d, inv_ty, 0.0, 0.0, 1.0],
        })
    }

    pub fn map_point(self, point: Point) -> Point {
        let x = point.x as f32;
        let y = point.y as f32;
        Point {
            x: (self.values[0] * x + self.values[1] * y + self.values[2]) as f64,
            y: (self.values[3] * x + self.values[4] * y + self.values[5]) as f64,
        }
    }

    pub fn scale_x(self) -> f32 {
        self.values[0]
    }

    pub fn scale_y(self) -> f32 {
        self.values[4]
    }

    pub fn trans_x(self) -> f32 {
        self.values[2]
    }

    pub fn trans_y(self) -> f32 {
        self.values[5]
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity()
    }
}

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
