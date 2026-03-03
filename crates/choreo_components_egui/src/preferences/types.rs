pub trait Preferences {
    fn get_string(&self, key: &str, default_value: &str) -> String;
    fn set_string(&self, key: &str, value: String);
    fn remove(&self, key: &str);
    fn get_bool(&self, key: &str, default_value: bool) -> bool;
    fn set_bool(&self, key: &str, value: bool);
}
