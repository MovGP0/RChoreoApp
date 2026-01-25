use std::collections::HashMap;
use std::rc::Rc;

use time::OffsetDateTime;

use crate::clone_mode::CloneMode;
use crate::models::dancer::DancerModel;
use crate::models::floor::FloorModel;
use crate::models::role::RoleModel;
use crate::models::scene::SceneModel;
use crate::models::settings::SettingsModel;

#[derive(Debug, Clone, PartialEq)]
pub struct ChoreographyModel {
    pub comment: Option<String>,
    pub settings: SettingsModel,
    pub floor: FloorModel,
    pub roles: Vec<Rc<RoleModel>>,
    pub dancers: Vec<Rc<DancerModel>>,
    pub scenes: Vec<SceneModel>,
    pub name: String,
    pub subtitle: Option<String>,
    pub date: Option<String>,
    pub variation: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub last_save_date: OffsetDateTime,
}

impl ChoreographyModel {
    pub fn clone_with(&self, mode: CloneMode) -> Self {
        if matches!(mode, CloneMode::Shallow) {
            return self.clone();
        }

        let mut role_map = HashMap::new();
        let mut dancer_map = HashMap::new();

        let roles = self
            .roles
            .iter()
            .map(|role| crate::mapping::clone_role(role, &mut role_map))
            .collect::<Vec<_>>();
        let dancers = self
            .dancers
            .iter()
            .map(|dancer| {
                crate::mapping::clone_dancer(Some(dancer), &mut dancer_map, &mut role_map)
                    .unwrap_or_else(|| dancer.clone())
            })
            .collect::<Vec<_>>();
        let scenes = self
            .scenes
            .iter()
            .map(|scene| scene.clone_internal(&mut dancer_map, &mut role_map))
            .collect();

        Self {
            comment: self.comment.clone(),
            settings: self.settings.clone(),
            floor: self.floor.clone(),
            roles,
            dancers,
            scenes,
            name: self.name.clone(),
            subtitle: self.subtitle.clone(),
            date: self.date.clone(),
            variation: self.variation.clone(),
            author: self.author.clone(),
            description: self.description.clone(),
            last_save_date: self.last_save_date,
        }
    }
}

impl Default for ChoreographyModel {
    fn default() -> Self {
        Self {
            comment: None,
            settings: SettingsModel::default(),
            floor: FloorModel::default(),
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
