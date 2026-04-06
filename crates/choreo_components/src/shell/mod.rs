mod material_scheme_applier;

pub use material_scheme_applier::MaterialScheme;
pub use material_scheme_applier::MaterialSchemes;
pub use material_scheme_applier::ShellMaterialSchemeApplier;
pub use material_scheme_applier::ShellMaterialSchemeHost;

use crate::choreo_main::MainPageDependencies;
use crate::choreo_main::ChoreoMainBehaviorDependencies;
use crate::preferences::PlatformPreferences;
use crate::preferences::Preferences;
use crate::shell_host::ShellHostViewModel;
use std::rc::Rc;

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
    create_shell_host_with_dependencies(default_main_page_dependencies())
}

pub fn create_shell_host_with_dependencies(
    main_page_dependencies: MainPageDependencies,
) -> ShellHostViewModel {
    ShellHostViewModel::new_with_dependencies(app_title(), main_page_dependencies)
}

pub fn default_main_page_dependencies() -> MainPageDependencies {
    let preferences: Rc<dyn Preferences> = Rc::new(PlatformPreferences::new(app_title()));
    MainPageDependencies {
        behavior_dependencies: ChoreoMainBehaviorDependencies {
            preferences: Some(preferences),
            ..ChoreoMainBehaviorDependencies::default()
        },
        ..MainPageDependencies::default()
    }
}
