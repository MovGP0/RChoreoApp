use crate::choreo_main::Report;
use crate::choreo_main::actions::ChoreoMainAction;
use crate::choreo_main::reducer::reduce;
use crate::choreo_main::state::ChoreoMainState;
use crate::choreo_main::state::MainContent;
use choreo_components::choreography_settings::actions::ChoreographySettingsAction;
use choreo_components::choreography_settings::state::SelectedSceneState;
use choreo_components::dancers::actions::DancersAction;
use choreo_master_mobile_json::Color;
use choreo_master_mobile_json::DancerId;
use choreo_master_mobile_json::SceneId;
use choreo_models::ChoreographyModel;
use choreo_models::DancerModel;
use choreo_models::PositionModel;
use choreo_models::RoleModel;
use choreo_models::SceneModel;
use std::rc::Rc;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn navigate_main_to_dancers_spec() {
    let suite = rspec::describe("navigate from main page to dancers page", (), |spec| {
        spec.it(
            "shows dancers when navigation to dancer settings is requested",
            |_| {
                let mut state = ChoreoMainState::default();
                let mut errors = Vec::new();
                let initial_content = state.content.clone();

                check_eq!(errors, initial_content, MainContent::Main);
                reduce(&mut state, ChoreoMainAction::NavigateToDancers);
                check_eq!(errors, state.content, MainContent::Dancers);
                assert_no_errors(errors);
            },
        );
        spec.it(
            "loads dancers from the selected choreography when dancer settings opens",
            |_| {
                let mut state = state_with_loaded_choreography();
                let mut errors = Vec::new();

                reduce(&mut state, ChoreoMainAction::NavigateToDancers);

                check_eq!(errors, state.content, MainContent::Dancers);
                check_eq!(errors, state.dancers_state.dancers.len(), 2);
                check_eq!(errors, state.dancers_state.dancers[0].name, "Alice");
                check_eq!(errors, state.dancers_state.dancers[1].name, "Bob");
                check_eq!(errors, state.dancers_state.global.roles.len(), 2);
                check_eq!(errors, state.dancers_state.global.scenes.len(), 1);
                check_eq!(errors, state.dancers_state.global.scenes[0].positions.len(), 2);

                assert_no_errors(errors);
            },
        );
        spec.it(
            "saves edited dancers back into the loaded choreography",
            |_| {
                let mut state = state_with_loaded_choreography();
                let mut errors = Vec::new();

                reduce(&mut state, ChoreoMainAction::NavigateToDancers);
                reduce(
                    &mut state,
                    ChoreoMainAction::DancersAction(DancersAction::DeleteSelectedDancer),
                );
                reduce(
                    &mut state,
                    ChoreoMainAction::DancersAction(DancersAction::SaveToGlobal),
                );

                check_eq!(
                    errors,
                    state.choreography_settings_state.choreography.dancers.len(),
                    1
                );
                check_eq!(
                    errors,
                    state.choreography_settings_state.choreography.dancers[0].name,
                    "Bob"
                );
                check_eq!(
                    errors,
                    state.choreography_settings_state.choreography.scenes[0]
                        .positions
                        .len(),
                    1
                );

                assert_no_errors(errors);
            },
        );
    });

    let report = crate::choreo_main::run_suite(&suite);
    assert!(report.is_success());
}

fn state_with_loaded_choreography() -> ChoreoMainState {
    let lead_role = Rc::new(RoleModel {
        z_index: 1,
        name: "Lead".to_string(),
        color: Color::transparent(),
    });
    let follow_role = Rc::new(RoleModel {
        z_index: 2,
        name: "Follow".to_string(),
        color: Color::transparent(),
    });
    let lead = Rc::new(DancerModel {
        dancer_id: DancerId(1),
        role: lead_role.clone(),
        name: "Alice".to_string(),
        shortcut: "A".to_string(),
        color: Color::transparent(),
        icon: None,
    });
    let follow = Rc::new(DancerModel {
        dancer_id: DancerId(2),
        role: follow_role.clone(),
        name: "Bob".to_string(),
        shortcut: "B".to_string(),
        color: Color::transparent(),
        icon: None,
    });
    let opening = SceneModel {
        scene_id: SceneId(10),
        positions: vec![
            PositionModel {
                dancer: Some(lead.clone()),
                orientation: None,
                x: 0.0,
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
            PositionModel {
                dancer: Some(follow.clone()),
                orientation: None,
                x: 1.0,
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
        name: "Opening".to_string(),
        text: None,
        fixed_positions: false,
        timestamp: None,
        variation_depth: 0,
        variations: Vec::new(),
        current_variation: Vec::new(),
        color: Color::transparent(),
    };
    let choreography = ChoreographyModel {
        roles: vec![lead_role, follow_role],
        dancers: vec![lead, follow],
        scenes: vec![opening.clone()],
        ..ChoreographyModel::default()
    };
    let mut state = ChoreoMainState::default();

    reduce(
        &mut state,
        ChoreoMainAction::ChoreographySettingsAction(
            ChoreographySettingsAction::LoadChoreography {
                choreography: Box::new(choreography),
                selected_scene: Some(SelectedSceneState {
                    scene_id: opening.scene_id,
                    name: opening.name.clone(),
                    text: String::new(),
                    fixed_positions: false,
                    timestamp: None,
                    color: Color::transparent(),
                }),
            },
        ),
    );

    state
}
