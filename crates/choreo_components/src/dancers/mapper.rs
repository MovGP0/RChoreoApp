use std::collections::HashMap;
use std::rc::Rc;

use choreo_master_mobile_json::DancerId;
use choreo_models::{Colors, DancerModel, RoleModel, SceneModel};

use super::dancer_settings_view_model::IconOption;

pub(crate) fn default_role(name: &str) -> RoleModel {
    RoleModel {
        z_index: 0,
        name: name.to_string(),
        color: Colors::transparent(),
    }
}

pub(crate) fn ensure_default_roles(roles: &mut Vec<Rc<RoleModel>>) {
    if !roles.is_empty() {
        return;
    }

    roles.push(Rc::new(default_role("Dame")));
    roles.push(Rc::new(default_role("Herr")));
}

pub(crate) fn next_dancer_id(dancers: &[Rc<DancerModel>]) -> DancerId {
    let mut max_id = 0;
    for dancer in dancers {
        let value = dancer.dancer_id.0;
        if value > max_id {
            max_id = value;
        }
    }
    DancerId(max_id + 1)
}

pub(crate) fn update_scene_dancers(
    scene: &mut SceneModel,
    dancer_map: &HashMap<DancerId, Rc<DancerModel>>,
) {
    for index in (0..scene.positions.len()).rev() {
        let dancer_id = scene.positions[index]
            .dancer
            .as_ref()
            .map(|dancer| dancer.dancer_id);
        let Some(dancer_id) = dancer_id else {
            continue;
        };

        if let Some(new_dancer) = dancer_map.get(&dancer_id) {
            scene.positions[index].dancer = Some(new_dancer.clone());
        } else {
            scene.positions.remove(index);
        }
    }

    for variation in &mut scene.variations {
        for variation_scene in variation {
            update_scene_dancers(variation_scene, dancer_map);
        }
    }

    for variation_scene in &mut scene.current_variation {
        update_scene_dancers(variation_scene, dancer_map);
    }
}

pub(crate) fn is_icon_match(option: &IconOption, icon_value: Option<&str>) -> bool {
    let Some(icon_value) = icon_value else {
        return false;
    };

    if option.key.eq_ignore_ascii_case(icon_value) {
        return true;
    }

    let normalized = icon_value.replace('\\', "/");
    let file_name = normalized.split('/').next_back().unwrap_or(icon_value);
    let name = file_name.split('.').next().unwrap_or(file_name);

    option.key.eq_ignore_ascii_case(name) || option.icon_name.eq_ignore_ascii_case(name)
}

pub(crate) fn normalize_icon_name(icon_name: &str) -> String {
    let normalized = icon_name.replace('\\', "/");
    let file_name = normalized.split('/').next_back().unwrap_or(&normalized);
    let name = file_name.split('.').next().unwrap_or(file_name);
    if name.trim().is_empty() {
        icon_name.to_string()
    } else {
        name.to_string()
    }
}
