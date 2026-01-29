use std::rc::Rc;

use super::Preferences;

#[derive(Clone)]
pub struct SharedPreferences
{
    inner: Rc<dyn Preferences>,
}

impl SharedPreferences
{
    pub fn new(inner: Rc<dyn Preferences>) -> Self
    {
        Self { inner }
    }
}

impl Preferences for SharedPreferences
{
    fn get_string(&self, key: &str, default_value: &str) -> String
    {
        self.inner.get_string(key, default_value)
    }

    fn set_string(&self, key: &str, value: String)
    {
        self.inner.set_string(key, value)
    }

    fn remove(&self, key: &str)
    {
        self.inner.remove(key)
    }

    fn get_bool(&self, key: &str, default_value: bool) -> bool
    {
        self.inner.get_bool(key, default_value)
    }

    fn set_bool(&self, key: &str, value: bool)
    {
        self.inner.set_bool(key, value)
    }
}
