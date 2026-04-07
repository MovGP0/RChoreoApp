use choreo_components::shell;

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
        "Assertion failures:\n{}",
        errors.join("\n")
    );
}

#[test]
fn app_title_spec() {
    let mut errors = Vec::new();

    check_eq!(errors, shell::app_title(), "ChoreoApp");
    check_eq!(errors, shell::create_shell_host().title(), "ChoreoApp");

    assert_no_errors(errors);
}
