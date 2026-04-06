#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppShellEffect {
    ApplyTypography,
    InitializeMainPage,
    RequestRepaint,
    RouteExternalFilePath { file_path: String },
}
