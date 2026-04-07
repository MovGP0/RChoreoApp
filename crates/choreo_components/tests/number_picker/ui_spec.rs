use material3::components::number_picker::NumberPickerUiState;
use material3::components::number_picker::ui::clamped_value;
use material3::components::number_picker::ui::decrease_value;
use material3::components::number_picker::ui::draw;
use material3::components::number_picker::ui::increase_value;
use material3::components::number_picker::ui::normalized_step;

fn assert_no_errors(errors: Vec<String>) {
    assert!(errors.is_empty(), "{}", errors.join("\n"));
}

#[test]
fn normalized_step_matches_slint_minimum_of_one() {
    let mut errors = Vec::new();

    macro_rules! check_eq {
        ($errors:expr, $actual:expr, $expected:expr) => {{
            let actual = $actual;
            let expected = $expected;
            if actual != expected {
                $errors.push(format!(
                    "assertion failed: left != right\n  left: `{:?}`\n right: `{:?}`",
                    actual, expected
                ));
            }
        }};
    }

    check_eq!(errors, normalized_step(-3), 1);
    check_eq!(errors, normalized_step(0), 1);
    check_eq!(errors, normalized_step(7), 7);

    assert_no_errors(errors);
}

#[test]
fn increase_and_decrease_clamp_to_bounds() {
    let mut errors = Vec::new();

    macro_rules! check_eq {
        ($errors:expr, $actual:expr, $expected:expr) => {{
            let actual = $actual;
            let expected = $expected;
            if actual != expected {
                $errors.push(format!(
                    "assertion failed: left != right\n  left: `{:?}`\n right: `{:?}`",
                    actual, expected
                ));
            }
        }};
    }

    check_eq!(errors, decrease_value(1, 1, 100, 1), None::<i32>);
    check_eq!(errors, increase_value(100, 1, 100, 1), None::<i32>);
    check_eq!(errors, decrease_value(11, 1, 100, 5), Some(6));
    check_eq!(errors, increase_value(96, 1, 100, 5), Some(100));

    assert_no_errors(errors);
}

#[test]
fn clamped_value_respects_reversed_bounds() {
    let mut errors = Vec::new();

    macro_rules! check_eq {
        ($errors:expr, $actual:expr, $expected:expr) => {{
            let actual = $actual;
            let expected = $expected;
            if actual != expected {
                $errors.push(format!(
                    "assertion failed: left != right\n  left: `{:?}`\n right: `{:?}`",
                    actual, expected
                ));
            }
        }};
    }

    check_eq!(errors, clamped_value(50, 100, 1), 50);
    check_eq!(errors, clamped_value(-5, 100, 1), 1);
    check_eq!(errors, clamped_value(200, 100, 1), 100);

    assert_no_errors(errors);
}

#[test]
fn draw_stays_within_the_available_width() {
    let context = egui::Context::default();
    let mut observed_width = 0.0_f32;
    let scoped_rect = egui::Rect::from_min_size(egui::pos2(40.0, 24.0), egui::vec2(326.0, 120.0));

    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = ui.scope_builder(egui::UiBuilder::new().max_rect(scoped_rect), |ui| {
                let _ = draw(
                    ui,
                    NumberPickerUiState {
                        label: "Floor front",
                        value: 8,
                        minimum: 1,
                        maximum: 100,
                        step: 1,
                        enabled: true,
                    },
                );
                observed_width = ui.min_rect().width();
            });
        });
    });

    assert!(observed_width <= scoped_rect.width());
}
