mod material_scheme_applier;

pub use material_scheme_applier::MaterialScheme;
pub use material_scheme_applier::MaterialSchemes;
pub use material_scheme_applier::ShellMaterialSchemeApplier;
pub use material_scheme_applier::ShellMaterialSchemeHost;

use crate::choreo_main::MainPageDependencies;
use crate::shell_host::ShellHostViewModel;

pub fn app_title() -> &'static str {
    "ChoreoApp"
}

/// Returns the canonical app icon SVG shared by egui targets.
///
/// Desktop rasterizes this asset into window-icon pixels at startup.
/// WASM keeps a checked-in favicon copy that is kept byte-equal by a parity spec.
pub fn app_icon_svg() -> &'static str {
    include_str!("../../assets/app_icon.svg")
}

pub fn create_shell_host() -> ShellHostViewModel {
    ShellHostViewModel::new(app_title())
}

pub fn create_shell_host_with_dependencies(
    main_page_dependencies: MainPageDependencies,
) -> ShellHostViewModel {
    ShellHostViewModel::new_with_dependencies(app_title(), main_page_dependencies)
}
