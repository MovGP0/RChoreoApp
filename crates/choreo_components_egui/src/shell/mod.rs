mod material_scheme_applier;

pub use material_scheme_applier::MaterialScheme;
pub use material_scheme_applier::MaterialSchemes;
pub use material_scheme_applier::ShellMaterialSchemeApplier;
pub use material_scheme_applier::ShellMaterialSchemeHost;

use crate::AppShellViewModel;

pub fn app_title() -> &'static str {
    "ChoreoApp"
}

pub fn create_shell_host() -> AppShellViewModel {
    AppShellViewModel::new(app_title())
}
