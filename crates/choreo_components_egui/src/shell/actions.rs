#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellAction {
    Initialize,
    SetThemeMode {
        is_dark: bool,
    },
    ApplyMaterialSchemes {
        light_background_hex: String,
        dark_background_hex: String,
    },
}
