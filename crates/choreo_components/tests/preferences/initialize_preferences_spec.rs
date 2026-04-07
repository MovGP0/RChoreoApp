use super::actions::PreferencesAction;
use super::create_state;
use super::reducer::reduce;

#[test]
fn initialize_sets_app_name_and_scoped_keys() {
    let mut state = create_state();

    reduce(
        &mut state,
        PreferencesAction::Initialize {
            app_name: "rchoreo".to_string(),
        },
    );

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

    let mut errors = Vec::new();

    check_eq!(errors, state.app_name, "rchoreo");
    check_eq!(
        errors,
        state.scoped_key("show_timestamps"),
        "rchoreo.show_timestamps"
    );

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}
