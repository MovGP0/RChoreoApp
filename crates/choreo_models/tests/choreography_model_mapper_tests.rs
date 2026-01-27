use choreo_master_mobile_json::{
    Choreography, Dancer, DancerId, Floor, FrontPosition, Position, Role, Scene, SceneId, Settings,
};
use choreo_models::{
    ChoreographyModel, ChoreographyModelMapper, Colors, DancerModel, FloorModel, PositionModel,
    RoleModel, SceneModel, SettingsModel,
};
use std::rc::Rc;
use time::{Date, Month, PrimitiveDateTime, Time};

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

macro_rules! check {
    ($errors:expr, $condition:expr, $message:expr) => {
        if !$condition {
            $errors.push($message.to_string());
        }
    };
}

#[test]
fn should_map_json_choreography_to_model_when_invoked() {
    // arrange
    let subject = ChoreographyModelMapper;
    let source = build_json_choreography();

    // act
    let result = subject.map_to_model(&source);

    // assert
    let mut errors = Vec::new();
    check_eq!(errors, result.comment.as_deref(), Some("comment"));
    check_eq!(errors, result.name, "Choreo");
    check_eq!(errors, result.subtitle.as_deref(), Some("Subtitle"));
    check_eq!(
        errors,
        result.date,
        Some(Date::from_calendar_date(2026, Month::January, 10).expect("valid date"))
    );
    check_eq!(errors, result.variation.as_deref(), Some("Variation"));
    check_eq!(errors, result.author.as_deref(), Some("Author"));
    check_eq!(errors, result.description.as_deref(), Some("Description"));
    check_eq!(errors, result.last_save_date, fixed_last_save_date());
    check_eq!(errors, result.settings.animation_milliseconds, 250);
    check_eq!(errors, result.settings.front_position, FrontPosition::Left);
    check_eq!(errors, result.settings.dancer_position, FrontPosition::Right);
    check_eq!(errors, result.settings.resolution, 12);
    check_eq!(errors, result.settings.transparency, 0.75);
    check!(errors, result.settings.positions_at_side, "positions_at_side mismatch");
    check!(errors, result.settings.grid_lines, "grid_lines mismatch");
    check!(errors, result.settings.snap_to_grid, "snap_to_grid mismatch");
    check_eq!(errors, result.settings.floor_color, Colors::blue());
    check_eq!(errors, result.settings.dancer_size, 0.9);
    check!(
        errors,
        !result.settings.show_timestamps,
        "show_timestamps mismatch"
    );
    check_eq!(
        errors,
        result.settings.music_path_absolute.as_deref(),
        Some("C:\\music\\track.mp3")
    );
    check_eq!(
        errors,
        result.settings.music_path_relative.as_deref(),
        Some("track.mp3")
    );
    check_eq!(errors, result.floor.size_front, 10);
    check_eq!(errors, result.floor.size_back, 11);
    check_eq!(errors, result.floor.size_left, 12);
    check_eq!(errors, result.floor.size_right, 13);
    check_eq!(errors, result.roles.len(), 2);
    check_eq!(errors, result.dancers.len(), 2);
    check_eq!(errors, result.scenes.len(), 2);
    check!(
        errors,
        Rc::ptr_eq(&result.dancers[0].role, &result.roles[0]),
        "dancer 0 role should reference role 0"
    );
    check!(
        errors,
        Rc::ptr_eq(&result.dancers[1].role, &result.roles[1]),
        "dancer 1 role should reference role 1"
    );
    check_eq!(errors, result.scenes[0].name, "Scene 1");
    check_eq!(errors, result.scenes[0].text.as_deref(), Some("Text"));
    check!(
        errors,
        result.scenes[0].fixed_positions,
        "scene 0 fixed_positions mismatch"
    );
    check_eq!(errors, result.scenes[0].timestamp.as_deref(), Some("00:00:12"));
    check_eq!(errors, result.scenes[0].variation_depth, 1);
    check_eq!(errors, result.scenes[0].color, Colors::green());
    check_eq!(errors, result.scenes[0].positions.len(), 2);
    check!(
        errors,
        Rc::ptr_eq(
            result.scenes[0].positions[0].dancer.as_ref().expect("dancer"),
            &result.dancers[0]
        ),
        "scene 0 position 0 dancer reference mismatch"
    );
    check_eq!(errors, result.scenes[0].positions[0].orientation, Some(90.0));
    check_eq!(errors, result.scenes[0].positions[0].x, 1.25);
    check_eq!(errors, result.scenes[0].positions[0].y, 2.5);
    check_eq!(errors, result.scenes[0].positions[0].curve1_x, Some(0.1));
    check_eq!(errors, result.scenes[0].positions[0].curve1_y, Some(0.2));
    check_eq!(errors, result.scenes[0].positions[0].curve2_x, Some(0.3));
    check_eq!(errors, result.scenes[0].positions[0].curve2_y, Some(0.4));
    check_eq!(errors, result.scenes[0].positions[0].movement1_x, Some(0.5));
    check_eq!(errors, result.scenes[0].positions[0].movement1_y, Some(0.6));
    check_eq!(errors, result.scenes[0].positions[0].movement2_x, Some(0.7));
    check_eq!(errors, result.scenes[0].positions[0].movement2_y, Some(0.8));
    check_eq!(errors, result.scenes[0].variations.len(), 1);
    check_eq!(errors, result.scenes[0].variations[0].len(), 1);
    check_eq!(errors, result.scenes[0].variations[0][0].name, "Variation Scene");
    check_eq!(errors, result.scenes[0].current_variation.len(), 1);
    check_eq!(
        errors,
        result.scenes[0].current_variation[0].name,
        "Current Variation"
    );
    check!(
        errors,
        result.scenes[1].positions.is_empty(),
        "scene 1 positions should be empty"
    );

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn should_map_model_choreography_to_json_when_invoked() {
    // arrange
    let subject = ChoreographyModelMapper;
    let source = build_model_choreography();

    // act
    let result = subject.map_to_json(&source);

    // assert
    let mut errors = Vec::new();
    check_eq!(errors, result.comment.as_deref(), Some("comment"));
    check_eq!(errors, result.name, "Choreo");
    check_eq!(errors, result.subtitle.as_deref(), Some("Subtitle"));
    check_eq!(errors, result.date.as_deref(), Some("2026-01-10"));
    check_eq!(errors, result.variation.as_deref(), Some("Variation"));
    check_eq!(errors, result.author.as_deref(), Some("Author"));
    check_eq!(errors, result.description.as_deref(), Some("Description"));
    check_eq!(errors, result.last_save_date, fixed_last_save_date());
    check_eq!(errors, result.settings.animation_milliseconds, 250);
    check_eq!(errors, result.settings.front_position, FrontPosition::Left);
    check_eq!(errors, result.settings.dancer_position, FrontPosition::Right);
    check_eq!(errors, result.settings.resolution, 12);
    check_eq!(errors, result.settings.transparency, 0.75);
    check!(errors, result.settings.positions_at_side, "positions_at_side mismatch");
    check!(errors, result.settings.grid_lines, "grid_lines mismatch");
    check!(errors, result.settings.snap_to_grid, "snap_to_grid mismatch");
    check_eq!(errors, result.settings.floor_color, Colors::blue());
    check_eq!(errors, result.settings.dancer_size, 0.9);
    check!(
        errors,
        !result.settings.show_timestamps,
        "show_timestamps mismatch"
    );
    check_eq!(
        errors,
        result.settings.music_path_absolute.as_deref(),
        Some("C:\\music\\track.mp3")
    );
    check_eq!(
        errors,
        result.settings.music_path_relative.as_deref(),
        Some("track.mp3")
    );
    check_eq!(errors, result.floor.size_front, 10);
    check_eq!(errors, result.floor.size_back, 11);
    check_eq!(errors, result.floor.size_left, 12);
    check_eq!(errors, result.floor.size_right, 13);
    check_eq!(errors, result.roles.len(), 2);
    check_eq!(errors, result.dancers.len(), 2);
    check_eq!(errors, result.scenes.len(), 2);
    check_eq!(errors, result.scenes[0].name, "Scene 1");
    check_eq!(errors, result.scenes[0].text.as_deref(), Some("Text"));
    check!(
        errors,
        result.scenes[0].fixed_positions,
        "scene 0 fixed_positions mismatch"
    );
    check_eq!(errors, result.scenes[0].timestamp.as_deref(), Some("00:00:12"));
    check_eq!(errors, result.scenes[0].variation_depth, 1);
    check_eq!(errors, result.scenes[0].color, Colors::green());
    check_eq!(
        errors,
        result.scenes[0].positions.as_ref().expect("positions").len(),
        2
    );
    check_eq!(
        errors,
        result.scenes[0].positions.as_ref().expect("positions")[0].orientation,
        Some(90.0)
    );
    check_eq!(
        errors,
        result.scenes[0].positions.as_ref().expect("positions")[0].x,
        1.25
    );
    check_eq!(
        errors,
        result.scenes[0].positions.as_ref().expect("positions")[0].y,
        2.5
    );
    check_eq!(
        errors,
        result.scenes[0].positions.as_ref().expect("positions")[0].curve1_x,
        Some(0.1)
    );
    check_eq!(
        errors,
        result.scenes[0].positions.as_ref().expect("positions")[0].curve1_y,
        Some(0.2)
    );
    check_eq!(
        errors,
        result.scenes[0].positions.as_ref().expect("positions")[0].curve2_x,
        Some(0.3)
    );
    check_eq!(
        errors,
        result.scenes[0].positions.as_ref().expect("positions")[0].curve2_y,
        Some(0.4)
    );
    check_eq!(
        errors,
        result.scenes[0].positions.as_ref().expect("positions")[0].movement1_x,
        Some(0.5)
    );
    check_eq!(
        errors,
        result.scenes[0].positions.as_ref().expect("positions")[0].movement1_y,
        Some(0.6)
    );
    check_eq!(
        errors,
        result.scenes[0].positions.as_ref().expect("positions")[0].movement2_x,
        Some(0.7)
    );
    check_eq!(
        errors,
        result.scenes[0].positions.as_ref().expect("positions")[0].movement2_y,
        Some(0.8)
    );
    check_eq!(
        errors,
        result.scenes[0]
            .variations
            .as_ref()
            .expect("variations")
            .len(),
        1
    );
    check_eq!(
        errors,
        result.scenes[0]
            .variations
            .as_ref()
            .expect("variations")[0]
            .len(),
        1
    );
    check_eq!(
        errors,
        result.scenes[0]
            .variations
            .as_ref()
            .expect("variations")[0][0]
            .name,
        "Variation Scene"
    );
    check_eq!(
        errors,
        result.scenes[0]
            .current_variation
            .as_ref()
            .expect("current_variation")
            .len(),
        1
    );
    check_eq!(
        errors,
        result.scenes[0]
            .current_variation
            .as_ref()
            .expect("current_variation")[0]
            .name,
        "Current Variation"
    );
    check!(
        errors,
        result.scenes[1].positions.is_none(),
        "scene 1 positions should be none"
    );

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

fn fixed_last_save_date() -> time::OffsetDateTime {
    let date = Date::from_calendar_date(2026, Month::January, 2)
        .expect("valid date");
    let time = Time::from_hms(12, 0, 0).expect("valid time");
    PrimitiveDateTime::new(date, time).assume_utc()
}

fn build_json_choreography() -> Choreography {
    let role_lead = Role {
        z_index: 1,
        name: "Lead".to_string(),
        color: Colors::red(),
    };
    let role_follow = Role {
        z_index: 2,
        name: "Follow".to_string(),
        color: Colors::purple(),
    };

    let dancer_a = Dancer {
        dancer_id: DancerId(1),
        role: role_lead.clone(),
        name: "Alice".to_string(),
        shortcut: "A".to_string(),
        color: Colors::orange(),
        icon: Some("icon-a".to_string()),
    };
    let dancer_b = Dancer {
        dancer_id: DancerId(2),
        role: role_follow.clone(),
        name: "Bob".to_string(),
        shortcut: "B".to_string(),
        color: Colors::teal(),
        icon: Some("icon-b".to_string()),
    };

    let mut scene1 = build_json_scene(&dancer_a, &dancer_b);
    scene1.scene_id = SceneId(10);
    let mut scene2 = build_json_scene(&dancer_a, &dancer_b);
    scene2.scene_id = SceneId(11);
    scene2.name = "Scene 2".to_string();
    scene2.text = None;
    scene2.fixed_positions = false;
    scene2.timestamp = None;
    scene2.variation_depth = 0;
    scene2.color = Colors::transparent();
    scene2.positions = None;
    scene2.variations = None;
    scene2.current_variation = None;

    Choreography {
        comment: Some("comment".to_string()),
        name: "Choreo".to_string(),
        subtitle: Some("Subtitle".to_string()),
        date: Some("2026-01-10".to_string()),
        variation: Some("Variation".to_string()),
        author: Some("Author".to_string()),
        description: Some("Description".to_string()),
        last_save_date: fixed_last_save_date(),
        settings: Settings {
            animation_milliseconds: 250,
            front_position: FrontPosition::Left,
            dancer_position: FrontPosition::Right,
            resolution: 12,
            transparency: 0.75,
            positions_at_side: true,
            grid_lines: true,
            snap_to_grid: true,
            floor_color: Colors::blue(),
            dancer_size: 0.9,
            show_timestamps: false,
            music_path_absolute: Some("C:\\music\\track.mp3".to_string()),
            music_path_relative: Some("track.mp3".to_string()),
        },
        floor: Floor {
            size_front: 10,
            size_back: 11,
            size_left: 12,
            size_right: 13,
        },
        roles: vec![role_lead, role_follow],
        dancers: vec![dancer_a, dancer_b],
        scenes: vec![scene1, scene2],
    }
}

fn build_json_scene(dancer_a: &Dancer, dancer_b: &Dancer) -> Scene {
    Scene {
        scene_id: SceneId(10),
        name: "Scene 1".to_string(),
        text: Some("Text".to_string()),
        fixed_positions: true,
        timestamp: Some("00:00:12".to_string()),
        variation_depth: 1,
        color: Colors::green(),
        positions: Some(vec![
            Position {
                dancer: Some(dancer_a.clone()),
                orientation: Some(90.0),
                x: 1.25,
                y: 2.5,
                curve1_x: Some(0.1),
                curve1_y: Some(0.2),
                curve2_x: Some(0.3),
                curve2_y: Some(0.4),
                movement1_x: Some(0.5),
                movement1_y: Some(0.6),
                movement2_x: Some(0.7),
                movement2_y: Some(0.8),
            },
            Position {
                dancer: Some(dancer_b.clone()),
                orientation: None,
                x: 4.0,
                y: 5.0,
                curve1_x: None,
                curve1_y: None,
                curve2_x: None,
                curve2_y: None,
                movement1_x: None,
                movement1_y: None,
                movement2_x: None,
                movement2_y: None,
            },
        ]),
        variations: Some(vec![vec![Scene {
            scene_id: SceneId(20),
            name: "Variation Scene".to_string(),
            text: None,
            fixed_positions: false,
            timestamp: None,
            variation_depth: 2,
            color: Colors::gold(),
            positions: None,
            variations: None,
            current_variation: None,
        }]]),
        current_variation: Some(vec![Scene {
            scene_id: SceneId(30),
            name: "Current Variation".to_string(),
            text: None,
            fixed_positions: false,
            timestamp: None,
            variation_depth: 3,
            color: Colors::cyan(),
            positions: None,
            variations: None,
            current_variation: None,
        }]),
    }
}

fn build_model_choreography() -> ChoreographyModel {
    let role_lead = Rc::new(RoleModel {
        z_index: 1,
        name: "Lead".to_string(),
        color: Colors::red(),
    });
    let role_follow = Rc::new(RoleModel {
        z_index: 2,
        name: "Follow".to_string(),
        color: Colors::purple(),
    });

    let dancer_a = Rc::new(DancerModel {
        dancer_id: DancerId(1),
        role: role_lead.clone(),
        name: "Alice".to_string(),
        shortcut: "A".to_string(),
        color: Colors::orange(),
        icon: Some("icon-a".to_string()),
    });
    let dancer_b = Rc::new(DancerModel {
        dancer_id: DancerId(2),
        role: role_follow.clone(),
        name: "Bob".to_string(),
        shortcut: "B".to_string(),
        color: Colors::teal(),
        icon: Some("icon-b".to_string()),
    });

    let mut scene1 = build_model_scene(dancer_a.clone(), dancer_b.clone());
    scene1.scene_id = SceneId(10);
    let mut scene2 = build_model_scene(dancer_a.clone(), dancer_b.clone());
    scene2.scene_id = SceneId(11);
    scene2.name = "Scene 2".to_string();
    scene2.text = None;
    scene2.fixed_positions = false;
    scene2.timestamp = None;
    scene2.variation_depth = 0;
    scene2.color = Colors::transparent();
    scene2.positions.clear();
    scene2.variations.clear();
    scene2.current_variation.clear();

    ChoreographyModel {
        comment: Some("comment".to_string()),
        name: "Choreo".to_string(),
        subtitle: Some("Subtitle".to_string()),
        date: Some(Date::from_calendar_date(2026, Month::January, 10).expect("valid date")),
        variation: Some("Variation".to_string()),
        author: Some("Author".to_string()),
        description: Some("Description".to_string()),
        last_save_date: fixed_last_save_date(),
        settings: SettingsModel {
            animation_milliseconds: 250,
            front_position: FrontPosition::Left,
            dancer_position: FrontPosition::Right,
            resolution: 12,
            transparency: 0.75,
            positions_at_side: true,
            grid_lines: true,
            snap_to_grid: true,
            floor_color: Colors::blue(),
            dancer_size: 0.9,
            show_timestamps: false,
            music_path_absolute: Some("C:\\music\\track.mp3".to_string()),
            music_path_relative: Some("track.mp3".to_string()),
        },
        floor: FloorModel {
            size_front: 10,
            size_back: 11,
            size_left: 12,
            size_right: 13,
        },
        roles: vec![role_lead, role_follow],
        dancers: vec![dancer_a, dancer_b],
        scenes: vec![scene1, scene2],
    }
}

fn build_model_scene(dancer_a: Rc<DancerModel>, dancer_b: Rc<DancerModel>) -> SceneModel {
    let mut scene = SceneModel {
        scene_id: SceneId(10),
        name: "Scene 1".to_string(),
        text: Some("Text".to_string()),
        fixed_positions: true,
        timestamp: Some("00:00:12".to_string()),
        variation_depth: 1,
        color: Colors::green(),
        positions: Vec::new(),
        variations: Vec::new(),
        current_variation: Vec::new(),
    };

    scene.positions.push(PositionModel {
        dancer: Some(dancer_a),
        orientation: Some(90.0),
        x: 1.25,
        y: 2.5,
        curve1_x: Some(0.1),
        curve1_y: Some(0.2),
        curve2_x: Some(0.3),
        curve2_y: Some(0.4),
        movement1_x: Some(0.5),
        movement1_y: Some(0.6),
        movement2_x: Some(0.7),
        movement2_y: Some(0.8),
    });

    scene.positions.push(PositionModel {
        dancer: Some(dancer_b),
        orientation: None,
        x: 4.0,
        y: 5.0,
        curve1_x: None,
        curve1_y: None,
        curve2_x: None,
        curve2_y: None,
        movement1_x: None,
        movement1_y: None,
        movement2_x: None,
        movement2_y: None,
    });

    scene.variations.push(vec![SceneModel {
        scene_id: SceneId(20),
        name: "Variation Scene".to_string(),
        text: None,
        fixed_positions: false,
        timestamp: None,
        variation_depth: 2,
        color: Colors::gold(),
        positions: Vec::new(),
        variations: Vec::new(),
        current_variation: Vec::new(),
    }]);

    scene.current_variation.push(SceneModel {
        scene_id: SceneId(30),
        name: "Current Variation".to_string(),
        text: None,
        fixed_positions: false,
        timestamp: None,
        variation_depth: 3,
        color: Colors::cyan(),
        positions: Vec::new(),
        variations: Vec::new(),
        current_variation: Vec::new(),
    });

    scene
}
