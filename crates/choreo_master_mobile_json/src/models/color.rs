use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Color {
    #[serde(skip)]
    pub a: u8,
    #[serde(skip)]
    pub r: u8,
    #[serde(skip)]
    pub g: u8,
    #[serde(skip)]
    pub b: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self::transparent()
    }
}

impl Color {
    pub fn transparent() -> Self {
        Self {
            a: 0,
            r: 0,
            g: 0,
            b: 0,
        }
    }

    pub fn from_hex(value: &str) -> Option<Self> {
        let value = value.trim();
        let hex = value.strip_prefix('#').unwrap_or(value);
        if hex.len() != 8 {
            return None;
        }

        let a = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let r = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let g = u8::from_str_radix(&hex[4..6], 16).ok()?;
        let b = u8::from_str_radix(&hex[6..8], 16).ok()?;
        Some(Self { a, r, g, b })
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}{:02X}", self.a, self.r, self.g, self.b)
    }
}
