#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppShellAction {
    FrameStarted,
    SplashPresented,
    ExternalFilePathReceived { file_path: String },
}
