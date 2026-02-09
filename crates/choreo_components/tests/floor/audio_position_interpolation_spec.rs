use std::cell::RefCell;
use std::rc::Rc;

use crate::floor;

use choreo_components::audio_player::AudioPlayerPositionChangedEvent;
use choreo_components::floor::FloorAdapter;
use choreo_components::global::GlobalStateModel;
use choreo_components::preferences::InMemoryPreferences;
use choreo_components::preferences::Preferences;
use choreo_components::shell;
use choreo_models::ChoreographyModel;
use choreo_models::Colors;
use choreo_models::DancerModel;
use choreo_models::FloorModel;
use choreo_models::PositionModel;
use choreo_models::RoleModel;
use choreo_models::SceneModel;
use crossbeam_channel::unbounded;
use slint::Model;
use floor::Report;

#[test]
#[serial_test::serial]
fn audio_position_interpolation_spec() {
    let suite = rspec::describe("audio position interpolation", (), |spec| {
        spec.it(
            "updates rendered floor positions when slider audio position changes",
            |_| {
                let handle = std::thread::Builder::new()
                    .name("audio-position-interpolation-spec".to_string())
                    .stack_size(8 * 1024 * 1024)
                    .spawn(move || {
                        let context = floor::FloorTestContext::new();
                        let view = shell::create_shell_host().expect("shell host should be created");
                        let global_state = Rc::new(RefCell::new(GlobalStateModel::default()));
                        let preferences: Rc<dyn Preferences> = Rc::new(InMemoryPreferences::new());
                        let (audio_sender, audio_receiver) =
                            unbounded::<AudioPlayerPositionChangedEvent>();

                        let mut adapter = FloorAdapter::new(
                            Rc::clone(&global_state),
                            Rc::clone(&context.state_machine),
                            preferences,
                            audio_receiver,
                        );

                        let role = Rc::new(RoleModel {
                            z_index: 0,
                            name: "Lead".to_string(),
                            color: Colors::red(),
                        });
                        let dancer = Rc::new(DancerModel {
                            dancer_id: choreo_master_mobile_json::DancerId(1),
                            role,
                            name: "Alex".to_string(),
                            shortcut: "A".to_string(),
                            color: Colors::red(),
                            icon: None,
                        });

                        let first_scene_model = SceneModel {
                            scene_id: choreo_master_mobile_json::SceneId(1),
                            positions: vec![PositionModel {
                                dancer: Some(Rc::clone(&dancer)),
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
                            }],
                            name: "Scene 1".to_string(),
                            text: None,
                            fixed_positions: true,
                            timestamp: Some("0".to_string()),
                            variation_depth: 0,
                            variations: Vec::new(),
                            current_variation: Vec::new(),
                            color: Colors::transparent(),
                        };
                        let second_scene_model = SceneModel {
                            scene_id: choreo_master_mobile_json::SceneId(2),
                            positions: vec![PositionModel {
                                dancer: Some(Rc::clone(&dancer)),
                                orientation: None,
                                x: 10.0,
                                y: 0.0,
                                curve1_x: None,
                                curve1_y: None,
                                curve2_x: None,
                                curve2_y: None,
                                movement1_x: None,
                                movement1_y: None,
                                movement2_x: None,
                                movement2_y: None,
                            }],
                            name: "Scene 2".to_string(),
                            text: None,
                            fixed_positions: true,
                            timestamp: Some("10".to_string()),
                            variation_depth: 0,
                            variations: Vec::new(),
                            current_variation: Vec::new(),
                            color: Colors::transparent(),
                        };

                        let first_scene_vm = floor::map_scene_view_model(&first_scene_model);
                        let second_scene_vm = floor::map_scene_view_model(&second_scene_model);

                        {
                            let mut state = global_state.borrow_mut();
                            state.choreography = ChoreographyModel {
                                floor: FloorModel {
                                    size_front: 5,
                                    size_back: 5,
                                    size_left: 5,
                                    size_right: 5,
                                },
                                dancers: vec![dancer],
                                scenes: vec![first_scene_model, second_scene_model],
                                ..ChoreographyModel::default()
                            };
                            state.scenes = vec![first_scene_vm.clone(), second_scene_vm];
                            state.selected_scene = Some(first_scene_vm);
                        }

                        audio_sender
                            .send(AudioPlayerPositionChangedEvent {
                                position_seconds: 5.0,
                            })
                            .expect("audio position send should succeed");
                        assert!(adapter.poll_audio_position());

                        adapter.apply(&view, &mut context.view_model.borrow_mut());

                        let floor_positions = view.get_floor_positions();
                        assert_eq!(floor_positions.row_count(), 1);
                        let interpolated = floor_positions
                            .row_data(0)
                            .expect("first position should exist");
                        floor::assert_close(interpolated.x as f64, 5.0, 0.0001);
                        floor::assert_close(interpolated.y as f64, 0.0, 0.0001);
                    })
                    .expect("spec thread should start");

                match handle.join() {
                    Ok(()) => {}
                    Err(error) => {
                        if let Some(message) = error.downcast_ref::<String>() {
                            panic!("{message}");
                        }
                        if let Some(message) = error.downcast_ref::<&str>() {
                            panic!("{message}");
                        }
                        panic!("spec thread panicked");
                    }
                }
            },
        );
    });

    let report = floor::run_suite(&suite);
    assert!(report.is_success());
}
