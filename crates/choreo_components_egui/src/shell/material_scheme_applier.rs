#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialScheme {
    pub background_hex: String,
}

impl Default for MaterialScheme {
    fn default() -> Self {
        Self {
            background_hex: "#FFFFFBFF".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaterialSchemes {
    pub light: MaterialScheme,
    pub dark: MaterialScheme,
}

impl Default for MaterialSchemes {
    fn default() -> Self {
        Self {
            light: MaterialScheme::default(),
            dark: MaterialScheme {
                background_hex: "#FF131318".to_string(),
            },
        }
    }
}

pub trait ShellMaterialSchemeHost {
    fn apply_material_schemes(&self, schemes: &MaterialSchemes);
}

#[derive(Clone)]
pub struct ShellMaterialSchemeApplier<T: ShellMaterialSchemeHost + Clone> {
    host: T,
}

impl<T: ShellMaterialSchemeHost + Clone> ShellMaterialSchemeApplier<T> {
    pub fn new(host: T) -> Self {
        Self { host }
    }

    pub fn apply(&self, schemes: MaterialSchemes) {
        self.host.apply_material_schemes(&schemes);
    }
}
