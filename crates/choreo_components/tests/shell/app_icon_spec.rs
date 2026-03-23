use choreo_components::shell;

#[test]
fn app_icon_spec() {
    let app_icon_svg = shell::app_icon_svg();
    let wasm_favicon_svg = include_str!("../../../../apps/wasm/app_icon.svg");

    assert!(app_icon_svg.contains("id=\"background\""));
    assert!(app_icon_svg.contains("id=\"dancers\""));
    assert!(app_icon_svg.contains("fill=\"#B0BEC5\""));
    assert_eq!(app_icon_svg, wasm_favicon_svg);
}

#[test]
fn android_launcher_icons_match_expected_density_sizes() {
    let android_icons = [
        (
            include_bytes!("../../../../apps/android/res/mipmap-mdpi/ic_launcher.png")
                .as_slice(),
            48_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-mdpi/ic_launcher_round.png")
                .as_slice(),
            48_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-hdpi/ic_launcher.png")
                .as_slice(),
            72_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-hdpi/ic_launcher_round.png")
                .as_slice(),
            72_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-xhdpi/ic_launcher.png")
                .as_slice(),
            96_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-xhdpi/ic_launcher_round.png")
                .as_slice(),
            96_u32,
        ),
        (
            include_bytes!("../../../../apps/android/res/mipmap-xxhdpi/ic_launcher.png")
                .as_slice(),
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
            include_bytes!(
                "../../../../apps/android/res/mipmap-xxxhdpi/ic_launcher_round.png"
            )
            .as_slice(),
            192_u32,
        ),
    ];

    for (png_bytes, expected_size) in android_icons {
        assert!(png_bytes.starts_with(&[137, 80, 78, 71, 13, 10, 26, 10]));
        assert_eq!(&png_bytes[12..16], b"IHDR");
        assert_eq!(
            u32::from_be_bytes(png_bytes[16..20].try_into().unwrap()),
            expected_size
        );
        assert_eq!(
            u32::from_be_bytes(png_bytes[20..24].try_into().unwrap()),
            expected_size
        );
    }
}
