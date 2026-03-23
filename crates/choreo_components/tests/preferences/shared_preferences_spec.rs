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

    assert_eq!(shared.get_string("name", "fallback"), "shared");
    assert!(shared.get_bool("enabled", false));
}
