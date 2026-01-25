use serde_json::{Map, Value};

use crate::errors::ChoreoJsonError;

pub(super) fn ref_map(key: &str, value: &str) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert(key.to_string(), Value::String(value.to_string()));
    map
}

pub(super) fn get_string(
    root: &Map<String, Value>,
    key: &'static str,
) -> Result<Option<String>, ChoreoJsonError> {
    Ok(root
        .get(key)
        .and_then(|value| value.as_str())
        .map(str::to_string))
}
