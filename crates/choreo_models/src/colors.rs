use choreo_master_mobile_json::Color;

pub struct Colors;

impl Colors {
    pub fn transparent() -> Color {
        Color::transparent()
    }

    pub fn blue() -> Color {
        Color {
            a: 255,
            r: 0,
            g: 0,
            b: 255,
        }
    }

    pub fn red() -> Color {
        Color {
            a: 255,
            r: 255,
            g: 0,
            b: 0,
        }
    }

    pub fn purple() -> Color {
        Color {
            a: 255,
            r: 128,
            g: 0,
            b: 128,
        }
    }

    pub fn orange() -> Color {
        Color {
            a: 255,
            r: 255,
            g: 165,
            b: 0,
        }
    }

    pub fn teal() -> Color {
        Color {
            a: 255,
            r: 0,
            g: 128,
            b: 128,
        }
    }

    pub fn green() -> Color {
        Color {
            a: 255,
            r: 0,
            g: 128,
            b: 0,
        }
    }

    pub fn gold() -> Color {
        Color {
            a: 255,
            r: 255,
            g: 215,
            b: 0,
        }
    }

    pub fn cyan() -> Color {
        Color {
            a: 255,
            r: 0,
            g: 255,
            b: 255,
        }
    }
}
