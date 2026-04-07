use choreo_components::choreo_info::messages::ChoreoInfoAction;
use choreo_components::choreo_info::state::ChoreoInfoState;
use choreo_components::choreo_info::ui::ChoreoInfoLabels;
use choreo_components::choreo_info::ui::choreo_date_text;
use choreo_components::choreo_info::ui::draw;
use choreo_components::choreo_info::ui::draw_transparency;
use choreo_components::choreo_info::ui::transparency_percentage_text;

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

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn choreo_info_state_defaults_match_slint_global_defaults() {
    let state = ChoreoInfoState::default();
    let mut errors = Vec::new();

    check_eq!(errors, state.choreo_name, "Lorem ipsum dolor sit amet");
    check_eq!(errors, state.choreo_subtitle, "consectetur adipiscing elit");
    check_eq!(errors, state.choreo_comment, "");
    check_eq!(errors, state.choreo_variation, "");
    check_eq!(errors, state.choreo_author, "");
    check_eq!(errors, state.choreo_description, "");
    check_eq!(errors, state.choreo_transparency, 0.0);

    assert_no_errors(errors);
}

#[test]
fn choreo_info_date_text_is_zero_padded_iso_like() {
    let text = choreo_date_text(2026, 3, 1);
    let mut errors = Vec::new();

    check_eq!(errors, text, "2026-03-01");

    assert_no_errors(errors);
}

#[test]
fn transparency_percentage_text_rounds_like_slint_math_round() {
    let mut errors = Vec::new();

    check_eq!(
        errors,
        transparency_percentage_text(0.0),
        "Transparency: 0%"
    );
    check_eq!(
        errors,
        transparency_percentage_text(0.245),
        "Transparency: 25%"
    );
    check_eq!(
        errors,
        transparency_percentage_text(0.999),
        "Transparency: 100%"
    );

    assert_no_errors(errors);
}

#[test]
fn transparency_draw_renders_shared_label_and_percentage() {
    let context = egui::Context::default();
    let mut errors = Vec::new();
    let output = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let action = draw_transparency(ui, 0.42, "Transparency");
            check_eq!(errors, action, None::<ChoreoInfoAction>);
        });
    });

    let mut found_transparency = false;
    for clipped in output.shapes {
        if format!("{:?}", clipped.shape).contains("Transparency: 42%") {
            found_transparency = true;
            break;
        }
    }

    check!(errors, found_transparency);
    assert_no_errors(errors);
}

#[test]
fn transparency_action_variant_preserves_fractional_value() {
    let action = ChoreoInfoAction::UpdateTransparency(0.42);
    let mut errors = Vec::new();

    check_eq!(
        errors,
        action,
        ChoreoInfoAction::UpdateTransparency(0.42)
    );

    assert_no_errors(errors);
}

#[test]
fn choreo_info_ui_draw_handles_all_information_inputs() {
    let state = ChoreoInfoState {
        choreo_name: "Name".to_string(),
        choreo_subtitle: "Subtitle".to_string(),
        choreo_comment: "Comment".to_string(),
        choreo_date: choreo_components::choreo_info::state::ChoreoDate {
            year: 2025,
            month: 12,
            day: 31,
        },
        choreo_variation: "Variation".to_string(),
        choreo_author: "Author".to_string(),
        choreo_description: "Description".to_string(),
        choreo_transparency: 0.42,
    };

    let labels = ChoreoInfoLabels {
        comment: "Comment".to_string(),
        name: "Name".to_string(),
        subtitle: "Subtitle".to_string(),
        date: "Date".to_string(),
        variation: "Variation".to_string(),
        author: "Author".to_string(),
        description: "Description".to_string(),
    };

    let context = egui::Context::default();
    let mut errors = Vec::new();
    let mut actions = Vec::new();

    let _ = context.run(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            actions = draw(ui, &state, &labels);
        });
    });

    check_eq!(errors, actions, Vec::<ChoreoInfoAction>::new());
    assert_no_errors(errors);
}
