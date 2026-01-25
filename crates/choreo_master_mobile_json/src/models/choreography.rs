use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{Dancer, Floor, Role, Scene, Settings};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Choreography {
    #[serde(rename = "_Comment")]
    pub comment: Option<String>,
    #[serde(rename = "Settings")]
    pub settings: Settings,
    #[serde(rename = "Floor")]
    pub floor: Floor,
    #[serde(rename = "Roles")]
    pub roles: Vec<Role>,
    #[serde(rename = "Dancers")]
    pub dancers: Vec<Dancer>,
    #[serde(rename = "Scenes")]
    pub scenes: Vec<Scene>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Subtitle")]
    pub subtitle: Option<String>,
    #[serde(rename = "Date")]
    pub date: Option<String>,
    #[serde(rename = "Variation")]
    pub variation: Option<String>,
    #[serde(rename = "Author")]
    pub author: Option<String>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "LastSaveDate", with = "time::serde::rfc3339")]
    pub last_save_date: OffsetDateTime,
}

impl Default for Choreography {
    fn default() -> Self {
        Self {
            comment: None,
            settings: Settings::default(),
            floor: Floor::default(),
            roles: Vec::new(),
            dancers: Vec::new(),
            scenes: Vec::new(),
            name: String::new(),
            subtitle: None,
            date: None,
            variation: None,
            author: None,
            description: None,
            last_save_date: OffsetDateTime::now_utc(),
        }
    }
}
