use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Floor {
    #[serde(rename = "SizeFront")]
    pub size_front: i32,
    #[serde(rename = "SizeBack")]
    pub size_back: i32,
    #[serde(rename = "SizeLeft")]
    pub size_left: i32,
    #[serde(rename = "SizeRight")]
    pub size_right: i32,
}
