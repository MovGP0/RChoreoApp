use std::rc::Rc;

use choreo_components_egui::choreography_settings::actions::ChoreographySettingsAction;
use choreo_components_egui::choreography_settings::state::SelectedSceneState;
use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::DancerId;
use choreo_master_mobile_json::SceneId;
use choreo_models::ChoreographyModel;
use choreo_models::DancerModel;
use choreo_models::FloorModel;
use choreo_models::PositionModel;
use choreo_models::RoleModel;
use choreo_models::SceneModel;
use choreo_models::SettingsModel;

use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;

#[test]
fn load_choreography_projects_floor_renderer_state_from_scene_models() {
    let lead_role = Rc::new(RoleModel {
        z_index: 1,
        name: "Lead".to_string(),
        color: rgba(255, 120, 0, 0),
    });
    let follow_role = Rc::new(RoleModel {
        z_index: 2,
        name: "Follow".to_string(),
        color: rgba(255, 0, 60, 120),
    });
    let lead = Rc::new(DancerModel {
        dancer_id: DancerId(1),
        role: lead_role,
        name: "Lead".to_string(),
        shortcut: "L".to_string(),
        color: rgba(255, 220, 40, 40),
        icon: None,
    });
    let follow = Rc::new(DancerModel {
        dancer_id: DancerId(2),
        role: follow_role,
        name: "Follow".to_string(),
        shortcut: "F".to_string(),
        color: rgba(255, 40, 120, 220),
        icon: None,
    });

    let opening = SceneModel {
        scene_id: SceneId(1),
        positions: vec![
            PositionModel {
                dancer: Some(lead.clone()),
                orientation: None,
                x: -1.0,
                y: 1.0,
                curve1_x: None,
                curve1_y: None,
                curve2_x: None,
                curve2_y: None,
                movement1_x: None,
                movement1_y: None,
                movement2_x: None,
                movement2_y: None,
            },
            PositionModel {
                dancer: Some(follow.clone()),
                orientation: None,
                x: 1.0,
                y: -1.0,
                curve1_x: None,
                curve1_y: None,
                curve2_x: None,
                curve2_y: None,
                movement1_x: None,
                movement1_y: None,
                movement2_x: None,
                movement2_y: None,
            },
        ],
        name: "Opening".to_string(),
        text: None,
        fixed_positions: false,
        timestamp: Some("1.0".to_string()),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: Color::transparent(),
    };
    let travel = SceneModel {
        scene_id: SceneId(2),
        positions: vec![
            PositionModel {
                dancer: Some(lead),
                orientation: None,
                x: 1.0,
                y: 2.0,
                curve1_x: Some(0.0),
                curve1_y: Some(1.5),
                curve2_x: None,
                curve2_y: None,
                movement1_x: None,
                movement1_y: None,
                movement2_x: None,
                movement2_y: None,
            },
            PositionModel {
                dancer: Some(follow),
                orientation: None,
                x: 2.0,
                y: 0.0,
                curve1_x: None,
                curve1_y: None,
                curve2_x: None,
                curve2_y: None,
                movement1_x: None,
                movement1_y: None,
                movement2_x: None,
                movement2_y: None,
            },
        ],
        name: "Travel".to_string(),
        text: None,
        fixed_positions: false,
        timestamp: Some("3.0".to_string()),
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: Color::transparent(),
    };

    let choreography = ChoreographyModel {
        name: "Viennese Waltz".to_string(),
        floor: FloorModel {
            size_front: 6,
            size_back: 4,
            size_left: 5,
            size_right: 7,
        },
        settings: SettingsModel {
            transparency: 0.25,
            positions_at_side: true,
            grid_lines: true,
            snap_to_grid: true,
            floor_color: rgba(255, 240, 230, 200),
            dancer_size: 1.2,
            ..SettingsModel::default()
        },
        scenes: vec![opening.clone(), travel.clone()],
        ..ChoreographyModel::default()
    };

    let mut state = ChoreoMainState::default();
    reduce(
        &mut state,
        ChoreoMainAction::ChoreographySettingsAction(ChoreographySettingsAction::LoadChoreography {
            choreography: Box::new(choreography),
            selected_scene: Some(SelectedSceneState {
                scene_id: opening.scene_id,
                name: opening.name.clone(),
                text: String::new(),
                fixed_positions: false,
                timestamp: Some(1.0),
                color: Color::transparent(),
            }),
        }),
    );
    reduce(
        &mut state,
        ChoreoMainAction::ChoreographySettingsAction(ChoreographySettingsAction::UpdateShowLegend(
            true,
        )),
    );
    reduce(
        &mut state,
        ChoreoMainAction::UpdateAudioPosition { seconds: 2.0 },
    );

    assert_eq!(state.floor_state.choreography_name, "Viennese Waltz");
    assert_eq!(state.floor_state.scene_name, "Opening");
    assert!(state.floor_state.show_grid_lines);
    assert!(state.floor_state.positions_at_side);
    assert!(state.floor_state.show_legend);
    assert_eq!(state.floor_state.floor_color, [240, 230, 200, 255]);
    assert_eq!(state.floor_state.source_positions.len(), 2);
    assert_eq!(state.floor_state.source_positions[0].shortcut, "L");
    assert_eq!(state.floor_state.previous_source_positions.len(), 0);
    assert_eq!(state.floor_state.next_source_positions.len(), 2);
    assert_eq!(state.floor_state.legend_entries.len(), 2);
    assert!((state.floor_state.interpolated_positions[0].x - 0.0).abs() < 0.001);
    assert!((state.floor_state.interpolated_positions[0].y - 1.5).abs() < 0.001);
}

fn rgba(a: u8, r: u8, g: u8, b: u8) -> Color {
    Color { a, r, g, b }
}
