#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreferencesAction {
    Initialize {
        app_name: String,
    },
    LoadString {
        key: String,
        value: String,
    },
    LoadBool {
        key: String,
        value: bool,
    },
    SetString {
        key: String,
        value: String,
    },
    SetBool {
        key: String,
        value: bool,
    },
    ToggleBool {
        key: String,
        default: bool,
    },
    Remove {
        key: String,
    },
    ClearPendingWrites,
    ClearAll,
}
