mod material_scheme_applier;

pub use material_scheme_applier::MaterialScheme;
pub use material_scheme_applier::MaterialSchemes;
pub use material_scheme_applier::ShellMaterialSchemeApplier;
pub use material_scheme_applier::ShellMaterialSchemeHost;

use crate::shell_host::ShellHostViewModel;

pub fn app_title() -> &'static str {
    "ChoreoApp"
}

pub fn app_icon_svg() -> &'static str {
    include_str!("../../../choreo_components/ui/app_icon.svg")
}

pub fn create_shell_host() -> ShellHostViewModel {
    ShellHostViewModel::new(app_title())
}
