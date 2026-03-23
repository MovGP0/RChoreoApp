use std::cell::RefCell;
use std::rc::Rc;

use choreo_components::shell::MaterialScheme;
use choreo_components::shell::MaterialSchemes;
use choreo_components::shell::ShellMaterialSchemeApplier;
use choreo_components::shell::ShellMaterialSchemeHost;

#[derive(Clone)]
struct StubShellHost {
    schemes: Rc<RefCell<Option<MaterialSchemes>>>,
}

impl ShellMaterialSchemeHost for StubShellHost {
    fn apply_material_schemes(&self, schemes: &MaterialSchemes) {
        self.schemes.replace(Some(schemes.clone()));
    }
}

#[test]
fn material_scheme_applier_spec() {
    let sink = Rc::new(RefCell::new(None));
    let host = StubShellHost {
        schemes: Rc::clone(&sink),
    };
    let applier = ShellMaterialSchemeApplier::new(host);
    applier.apply(MaterialSchemes {
        light: MaterialScheme {
            background_hex: "#FFEEEEEE".to_string(),
        },
        dark: MaterialScheme {
            background_hex: "#FF222222".to_string(),
        },
    });

    let applied = sink.borrow().clone().expect("schemes should be applied");
    assert_eq!(applied.light.background_hex, "#FFEEEEEE");
    assert_eq!(applied.dark.background_hex, "#FF222222");
}
