use material3::components::number_picker::ui::clamped_value;
use material3::components::number_picker::ui::draw;
use material3::components::number_picker::ui::decrease_value;
use material3::components::number_picker::ui::increase_value;
use material3::components::number_picker::ui::normalized_step;
use material3::components::number_picker::NumberPickerUiState;

#[test]
fn normalized_step_matches_slint_minimum_of_one() {
    assert_eq!(normalized_step(-3), 1);
    assert_eq!(normalized_step(0), 1);
    assert_eq!(normalized_step(7), 7);
}

#[test]
fn increase_and_decrease_clamp_to_bounds() {
    assert_eq!(decrease_value(1, 1, 100, 1), None);
    assert_eq!(increase_value(100, 1, 100, 1), None);
    assert_eq!(decrease_value(11, 1, 100, 5), Some(6));
    assert_eq!(increase_value(96, 1, 100, 5), Some(100));
}

#[test]
fn clamped_value_respects_reversed_bounds() {
    assert_eq!(clamped_value(50, 100, 1), 50);
    assert_eq!(clamped_value(-5, 100, 1), 1);
    assert_eq!(clamped_value(200, 100, 1), 100);
}

#[test]
fn draw_stays_within_the_available_width() {
    let context = egui::Context::default();
    let mut observed_width = 0.0_f32;
    let scoped_rect =
        egui::Rect::from_min_size(egui::pos2(40.0, 24.0), egui::vec2(326.0, 120.0));

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
