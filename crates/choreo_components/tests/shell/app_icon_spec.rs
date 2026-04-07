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

macro_rules! check {
    ($errors:expr, $condition:expr) => {
        if !$condition {
            $errors.push(format!("condition failed: {}", stringify!($condition)));
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
fn app_icon_spec() {
    let app_icon_svg = shell::app_icon_svg();
    let wasm_favicon_svg = include_str!("../../../../apps/wasm/app_icon.svg");

    let mut errors = Vec::new();

    check!(errors, app_icon_svg.contains("id=\"background\""));
    check!(errors, app_icon_svg.contains("id=\"dancers\""));
    check!(errors, app_icon_svg.contains("fill=\"#B0BEC5\""));
    check_eq!(errors, app_icon_svg, wasm_favicon_svg);

    assert_no_errors(errors);
}

#[test]
fn android_launcher_icons_match_expected_density_sizes() {
    let android_icons = [
        (
            include_bytes!("../../../../apps/android/res/mipmap-mdpi/ic_launcher.png").as_slice(),
            48_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-mdpi/ic_launcher_round.png")
                .as_slice(),
            48_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-hdpi/ic_launcher.png").as_slice(),
            72_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-hdpi/ic_launcher_round.png")
                .as_slice(),
            72_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-xhdpi/ic_launcher.png").as_slice(),
            96_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-xhdpi/ic_launcher_round.png")
                .as_slice(),
            96_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-xxhdpi/ic_launcher.png").as_slice(),
            144_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-xxhdpi/ic_launcher_round.png")
                .as_slice(),
            144_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-xxxhdpi/ic_launcher.png")
                .as_slice(),
            192_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-xxxhdpi/ic_launcher_round.png")
                .as_slice(),
            192_u32,
        ),
    ];

    let mut errors = Vec::new();

    for (png_bytes, expected_size) in android_icons {
        check!(errors, png_bytes.starts_with(&[137, 80, 78, 71, 13, 10, 26, 10]));
        check_eq!(errors, &png_bytes[12..16], b"IHDR");
        check_eq!(
            errors,
            u32::from_be_bytes(png_bytes[16..20].try_into().unwrap()),
            expected_size
        );
        check_eq!(
            errors,
            u32::from_be_bytes(png_bytes[20..24].try_into().unwrap()),
            expected_size
        );
    }

    assert_no_errors(errors);
}
