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

    assert_eq!(state.app_name, "rchoreo");
    assert_eq!(state.scoped_key("show_timestamps"), "rchoreo.show_timestamps");
}
