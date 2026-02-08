use std::collections::HashSet;

use choreo_models::Colors;

#[test]
fn named_web_color_count_matches_reference() {
    assert_eq!(149, Colors::named_web_color_count());
}

#[test]
fn named_web_color_names_are_unique() {
    let names: Vec<&str> = Colors::named_web_color_names().collect();
    let unique_names: HashSet<&str> = names.iter().copied().collect();

    assert_eq!(names.len(), unique_names.len());
}

#[test]
fn known_aliases_and_css4_color_are_present() {
    let aqua = Colors::named_web_color("aqua").expect("aqua must exist");
    let cyan = Colors::named_web_color("cyan").expect("cyan must exist");
    assert_eq!(aqua, cyan);

    let gray = Colors::named_web_color("gray").expect("gray must exist");
    let grey = Colors::named_web_color("grey").expect("grey must exist");
    assert_eq!(gray, grey);

    let rebecca_purple =
        Colors::named_web_color("rebeccapurple").expect("rebeccapurple must exist");
    assert_eq!(rebecca_purple.r, 102);
    assert_eq!(rebecca_purple.g, 51);
    assert_eq!(rebecca_purple.b, 153);
    assert_eq!(rebecca_purple.a, 255);
}

#[test]
fn named_web_color_lookup_is_case_insensitive_and_includes_transparent() {
    let tomato = Colors::named_web_color("ToMaTo").expect("tomato must exist");
    assert_eq!(tomato.r, 255);
    assert_eq!(tomato.g, 99);
    assert_eq!(tomato.b, 71);
    assert_eq!(tomato.a, 255);

    let transparent = Colors::named_web_color("transparent").expect("transparent must exist");
    assert_eq!(transparent.r, 0);
    assert_eq!(transparent.g, 0);
    assert_eq!(transparent.b, 0);
    assert_eq!(transparent.a, 0);
}
