mod material_scheme_applier;

use crate::ShellHost;

pub use material_scheme_applier::ShellMaterialSchemeApplier;

pub fn app_title() -> &'static str {
    "ChoreoApp"
}

pub fn create_shell_host() -> Result<ShellHost, slint::PlatformError> {
    ShellHost::new()
}
