use crate::preferences::preferences::InMemoryPreferences;
use crate::preferences::preferences::Preferences;
use crate::preferences::preferences::SharedPreferences;
use std::rc::Rc;

#[test]
fn shared_preferences_delegate_to_inner_preferences() {
    let inner: Rc<dyn Preferences> = Rc::new(InMemoryPreferences::new());
    let shared = SharedPreferences::new(Rc::clone(&inner));
    shared.set_string("name", "shared".to_string());
    shared.set_bool("enabled", true);

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

    check_eq!(errors, shared.get_string("name", "fallback"), "shared");
    check_eq!(errors, shared.get_bool("enabled", false), true);

    assert!(
        errors.is_empty(),
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}
