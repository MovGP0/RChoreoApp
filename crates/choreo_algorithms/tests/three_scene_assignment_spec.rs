use choreo_algorithms::hungarian::compute_mid_scene_assignment;
use choreo_algorithms::min_cost_max_flow::solve_three_scene_assignment;
use choreo_algorithms::{AlgorithmError, Vector2};
use rspec::report::Report;
use rspec::{ConfigurationBuilder, Logger, Runner};
use serde_json::Value;
use std::io;
use std::sync::Arc;

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
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
        }
    };
}

fn run_suite<T>(suite: &rspec::block::Suite<T>) -> rspec::report::SuiteReport
where
    T: Clone + Send + Sync + std::fmt::Debug,
{
    let configuration = ConfigurationBuilder::default()
        .exit_on_failure(false)
        .build()
        .expect("rspec configuration should build");
    let logger = Arc::new(Logger::new(io::stdout()));
    let runner = Runner::new(configuration, vec![logger]);
    runner.run(suite)
}

#[test]
fn three_scene_assignment_spec() {
    let suite = rspec::describe("three scene assignment", (), |spec| {
        spec.it(
            "chooses non-crossing mapping when midpoints are swapped",
            |_| {
                let scene_a = vec![Vector2::new(0.0, 0.0), Vector2::new(2.0, 0.0)];
                let scene_b = vec![Vector2::new(2.0, 1.0), Vector2::new(0.0, 1.0)];
                let scene_c = vec![Vector2::new(0.0, 2.0), Vector2::new(2.0, 2.0)];

                let hungarian =
                    compute_mid_scene_assignment(&scene_a, &scene_b, &scene_c).expect("hungarian");
                let min_cost =
                    solve_three_scene_assignment(&scene_a, &scene_b, &scene_c).expect("min cost");

                let mut errors = Vec::new();

                check_eq!(errors, hungarian, vec![1, 0]);
                check_eq!(errors, min_cost, vec![1, 0]);

                assert!(
                    errors.is_empty(),
                    "Assertion failures:\n{}",
                    errors.join("\n")
                );
            },
        );

        spec.it("fails when scenes differ in size", |_| {
            let scene_a = vec![Vector2::new(0.0, 0.0)];
            let scene_b = vec![Vector2::new(1.0, 0.0), Vector2::new(2.0, 0.0)];
            let scene_c = vec![Vector2::new(0.0, 1.0)];

            let hungarian = compute_mid_scene_assignment(&scene_a, &scene_b, &scene_c)
                .expect_err("expected size mismatch");
            let min_cost = solve_three_scene_assignment(&scene_a, &scene_b, &scene_c)
                .expect_err("expected size mismatch");

            let mut errors = Vec::new();

            check_size_mismatch(&mut errors, &hungarian);
            check_size_mismatch(&mut errors, &min_cost);

            assert!(
                errors.is_empty(),
                "Assertion failures:\n{}",
                errors.join("\n")
            );
        });

        spec.it(
            "returns valid assignments on real choreography sample",
            |_| {
                let (scene_a, scene_b, scene_c) = load_first_three_scenes();

                let hungarian =
                    compute_mid_scene_assignment(&scene_a, &scene_b, &scene_c).expect("hungarian");
                let min_cost =
                    solve_three_scene_assignment(&scene_a, &scene_b, &scene_c).expect("min cost");

                let mut errors = Vec::new();

                check_is_permutation(&mut errors, &hungarian, scene_a.len());
                check_is_permutation(&mut errors, &min_cost, scene_a.len());

                assert!(
                    errors.is_empty(),
                    "Assertion failures:\n{}",
                    errors.join("\n")
                );
            },
        );
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
            let x = position.get("X").and_then(Value::as_f64).expect("X") as f32;
            let y = position.get("Y").and_then(Value::as_f64).expect("Y") as f32;
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

fn check_is_permutation(errors: &mut Vec<String>, values: &[usize], expected_len: usize) {
    check_eq!(errors, values.len(), expected_len);

    let mut seen = vec![false; expected_len];
    for &value in values {
        check!(errors, value < expected_len);
        if value < expected_len {
            check!(errors, !seen[value]);
        }
        seen[value] = true;
    }
}

fn check_size_mismatch(errors: &mut Vec<String>, value: &AlgorithmError) {
    check!(errors, matches!(value, AlgorithmError::SizeMismatch(_)));
}
