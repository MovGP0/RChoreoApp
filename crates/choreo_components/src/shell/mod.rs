use crate::ShellHost;

pub fn app_title() -> &'static str {
    "ChoreoApp"
}

pub fn create_shell_host() -> Result<ShellHost, slint::PlatformError> {
    ShellHost::new()
}
