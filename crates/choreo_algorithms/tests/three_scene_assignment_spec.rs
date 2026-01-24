use choreo_algorithms::hungarian::compute_mid_scene_assignment;
use choreo_algorithms::min_cost_max_flow::solve_three_scene_assignment;
use choreo_algorithms::{AlgorithmError, Vector2};
use rspec::{ConfigurationBuilder, Runner};
use serde_json::Value;

fn run_suite(suite: &rspec::Suite) -> rspec::report::SuiteReport {
    let configuration = ConfigurationBuilder::default()
        .build()
        .expect("rspec configuration should build");
    let runner = Runner::new(configuration, Vec::new());
    runner.run(suite)
}

#[test]
fn three_scene_assignment_spec() {
    let suite = rspec::describe("three scene assignment", |spec| {
        spec.it("chooses non-crossing mapping when midpoints are swapped", |_| {
            let scene_a = vec![Vector2::new(0.0, 0.0), Vector2::new(2.0, 0.0)];
            let scene_b = vec![Vector2::new(2.0, 1.0), Vector2::new(0.0, 1.0)];
            let scene_c = vec![Vector2::new(0.0, 2.0), Vector2::new(2.0, 2.0)];

            let hungarian =
                compute_mid_scene_assignment(&scene_a, &scene_b, &scene_c).expect("hungarian");
            let min_cost =
                solve_three_scene_assignment(&scene_a, &scene_b, &scene_c).expect("min cost");

            assert_eq!(hungarian, vec![1, 0]);
            assert_eq!(min_cost, vec![1, 0]);
        });

        spec.it("fails when scenes differ in size", |_| {
            let scene_a = vec![Vector2::new(0.0, 0.0)];
            let scene_b = vec![Vector2::new(1.0, 0.0), Vector2::new(2.0, 0.0)];
            let scene_c = vec![Vector2::new(0.0, 1.0)];

            let hungarian = compute_mid_scene_assignment(&scene_a, &scene_b, &scene_c)
                .expect_err("expected size mismatch");
            let min_cost = solve_three_scene_assignment(&scene_a, &scene_b, &scene_c)
                .expect_err("expected size mismatch");

            assert!(matches!(hungarian, AlgorithmError::SizeMismatch(_)));
            assert!(matches!(min_cost, AlgorithmError::SizeMismatch(_)));
        });

        spec.it("keeps dancer identities on real choreography sample", |_| {
            let (scene_a, scene_b, scene_c) = load_first_three_scenes();

            let hungarian =
                compute_mid_scene_assignment(&scene_a, &scene_b, &scene_c).expect("hungarian");
            let min_cost =
                solve_three_scene_assignment(&scene_a, &scene_b, &scene_c).expect("min cost");

            for index in 0..scene_a.len() {
                assert_eq!(hungarian[index], index);
                assert_eq!(min_cost[index], index);
            }

            assert_eq!(hungarian, min_cost);
        });
    });

    let report = run_suite(&suite);
    assert!(report.is_success());
}

fn load_first_three_scenes() -> (Vec<Vector2>, Vec<Vector2>, Vec<Vector2>) {
    let file_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("test_data")
        .join("Test.choreo");

    let json = std::fs::read_to_string(&file_path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", file_path.display()));
    let doc: Value = serde_json::from_str(&json).expect("valid JSON");
    let scenes = doc
        .get("Scenes")
        .and_then(Value::as_array)
        .expect("Scenes array");

    let mut collected: Vec<Vec<(i32, Vector2)>> = vec![Vec::new(), Vec::new(), Vec::new()];

    for scene_index in 0..3 {
        let scene = &scenes[scene_index];
        let positions = scene
            .get("Positions")
            .and_then(Value::as_array)
            .expect("Positions array");

        for position in positions {
            let dancer_id = position
                .get("Dancer")
                .and_then(|value| value.get("$ref"))
                .and_then(Value::as_str)
                .expect("dancer ref")
                .parse::<i32>()
                .expect("dancer id");
            let x = position
                .get("X")
                .and_then(Value::as_f64)
                .expect("X") as f32;
            let y = position
                .get("Y")
                .and_then(Value::as_f64)
                .expect("Y") as f32;
            collected[scene_index].push((dancer_id, Vector2::new(x, y)));
        }
    }

    for entries in &mut collected {
        entries.sort_by_key(|entry| entry.0);
    }

    let scene_a = collected[0].iter().map(|entry| entry.1).collect();
    let scene_b = collected[1].iter().map(|entry| entry.1).collect();
    let scene_c = collected[2].iter().map(|entry| entry.1).collect();

    (scene_a, scene_b, scene_c)
}
