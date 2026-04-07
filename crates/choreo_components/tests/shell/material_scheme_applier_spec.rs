use std::cell::RefCell;
use std::rc::Rc;

use choreo_components::shell::MaterialScheme;
use choreo_components::shell::MaterialSchemes;
use choreo_components::shell::ShellMaterialSchemeApplier;
use choreo_components::shell::ShellMaterialSchemeHost;

macro_rules! check_eq {
    ($errors:expr, $left:expr, $right:expr) => {
        if $left != $right {
            $errors.push(format!(
                "{} != {} (left = {:?}, right = {:?})",
                stringify!($left),
                stringify!($right),
                $left,
                $right
            ));
        }
    };
}

fn assert_no_errors(errors: Vec<String>) {
    assert!(
        errors.is_empty(),
        "{}",
        errors.join("\n")
    );
}

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
    let mut errors = Vec::new();
    check_eq!(errors, applied.light.background_hex, "#FFEEEEEE");
    check_eq!(errors, applied.dark.background_hex, "#FF222222");
    assert_no_errors(errors);
}
