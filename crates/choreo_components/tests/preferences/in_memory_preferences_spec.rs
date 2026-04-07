use crate::preferences::preferences::InMemoryPreferences;
use crate::preferences::preferences::Preferences;

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
fn in_memory_preferences_store_strings_and_bools() {
    let prefs = InMemoryPreferences::new();
    prefs.set_string("name", "demo".to_string());
    prefs.set_bool("flag", true);

    let mut errors = Vec::new();

    check_eq!(errors, prefs.get_string("name", "fallback"), "demo");
    check!(errors, prefs.get_bool("flag", false));

    assert_no_errors(errors);
}

#[test]
fn in_memory_preferences_remove_clears_values() {
    let prefs = InMemoryPreferences::new();
    prefs.set_string("k", "v".to_string());
    prefs.set_bool("k", true);
    prefs.remove("k");

    let mut errors = Vec::new();

    check_eq!(errors, prefs.get_string("k", "default"), "default");
    check!(errors, !prefs.get_bool("k", false));

    assert_no_errors(errors);
}
