use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontPosition {
    Top,
    Right,
    Bottom,
    Left,
}

impl FrontPosition {
    fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Top),
            1 => Some(Self::Right),
            2 => Some(Self::Bottom),
            3 => Some(Self::Left),
            _ => None,
        }
    }

    fn to_i32(self) -> i32 {
        match self {
            Self::Top => 0,
            Self::Right => 1,
            Self::Bottom => 2,
            Self::Left => 3,
        }
    }
}

impl Serialize for FrontPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.to_i32())
    }
}

impl<'de> Deserialize<'de> for FrontPosition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FrontPositionVisitor;

        impl<'de> serde::de::Visitor<'de> for FrontPositionVisitor {
            type Value = FrontPosition;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("an integer 0..=3 or a string front position")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let value = i32::try_from(value)
                    .map_err(|_| E::custom("front position out of range"))?;
                FrontPosition::from_i32(value)
                    .ok_or_else(|| E::custom("front position out of range"))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let value = i32::try_from(value)
                    .map_err(|_| E::custom("front position out of range"))?;
                FrontPosition::from_i32(value)
                    .ok_or_else(|| E::custom("front position out of range"))
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value.to_ascii_lowercase().as_str() {
                    "top" => Ok(FrontPosition::Top),
                    "right" => Ok(FrontPosition::Right),
                    "bottom" => Ok(FrontPosition::Bottom),
                    "left" => Ok(FrontPosition::Left),
                    _ => Err(E::custom("unknown front position string")),
                }
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_any(FrontPositionVisitor)
    }
}
