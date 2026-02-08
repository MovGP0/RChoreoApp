use choreo_master_mobile_json::Color;

type NamedWebColorEntry = (&'static str, u8, u8, u8, u8);

const NAMED_WEB_COLORS: &[NamedWebColorEntry] = &[
    ("aliceblue", 255, 240, 248, 255),
    ("antiquewhite", 255, 250, 235, 215),
    ("aqua", 255, 0, 255, 255),
    ("aquamarine", 255, 127, 255, 212),
    ("azure", 255, 240, 255, 255),
    ("beige", 255, 245, 245, 220),
    ("bisque", 255, 255, 228, 196),
    ("black", 255, 0, 0, 0),
    ("blanchedalmond", 255, 255, 235, 205),
    ("blue", 255, 0, 0, 255),
    ("blueviolet", 255, 138, 43, 226),
    ("brown", 255, 165, 42, 42),
    ("burlywood", 255, 222, 184, 135),
    ("cadetblue", 255, 95, 158, 160),
    ("chartreuse", 255, 127, 255, 0),
    ("chocolate", 255, 210, 105, 30),
    ("coral", 255, 255, 127, 80),
    ("cornflowerblue", 255, 100, 149, 237),
    ("cornsilk", 255, 255, 248, 220),
    ("crimson", 255, 220, 20, 60),
    ("cyan", 255, 0, 255, 255),
    ("darkblue", 255, 0, 0, 139),
    ("darkcyan", 255, 0, 139, 139),
    ("darkgoldenrod", 255, 184, 134, 11),
    ("darkgray", 255, 169, 169, 169),
    ("darkgreen", 255, 0, 100, 0),
    ("darkgrey", 255, 169, 169, 169),
    ("darkkhaki", 255, 189, 183, 107),
    ("darkmagenta", 255, 139, 0, 139),
    ("darkolivegreen", 255, 85, 107, 47),
    ("darkorange", 255, 255, 140, 0),
    ("darkorchid", 255, 153, 50, 204),
    ("darkred", 255, 139, 0, 0),
    ("darksalmon", 255, 233, 150, 122),
    ("darkseagreen", 255, 143, 188, 143),
    ("darkslateblue", 255, 72, 61, 139),
    ("darkslategray", 255, 47, 79, 79),
    ("darkslategrey", 255, 47, 79, 79),
    ("darkturquoise", 255, 0, 206, 209),
    ("darkviolet", 255, 148, 0, 211),
    ("deeppink", 255, 255, 20, 147),
    ("deepskyblue", 255, 0, 191, 255),
    ("dimgray", 255, 105, 105, 105),
    ("dimgrey", 255, 105, 105, 105),
    ("dodgerblue", 255, 30, 144, 255),
    ("firebrick", 255, 178, 34, 34),
    ("floralwhite", 255, 255, 250, 240),
    ("forestgreen", 255, 34, 139, 34),
    ("fuchsia", 255, 255, 0, 255),
    ("gainsboro", 255, 220, 220, 220),
    ("ghostwhite", 255, 248, 248, 255),
    ("gold", 255, 255, 215, 0),
    ("goldenrod", 255, 218, 165, 32),
    ("gray", 255, 128, 128, 128),
    ("green", 255, 0, 128, 0),
    ("greenyellow", 255, 173, 255, 47),
    ("grey", 255, 128, 128, 128),
    ("honeydew", 255, 240, 255, 240),
    ("hotpink", 255, 255, 105, 180),
    ("indianred", 255, 205, 92, 92),
    ("indigo", 255, 75, 0, 130),
    ("ivory", 255, 255, 255, 240),
    ("khaki", 255, 240, 230, 140),
    ("lavender", 255, 230, 230, 250),
    ("lavenderblush", 255, 255, 240, 245),
    ("lawngreen", 255, 124, 252, 0),
    ("lemonchiffon", 255, 255, 250, 205),
    ("lightblue", 255, 173, 216, 230),
    ("lightcoral", 255, 240, 128, 128),
    ("lightcyan", 255, 224, 255, 255),
    ("lightgoldenrodyellow", 255, 250, 250, 210),
    ("lightgray", 255, 211, 211, 211),
    ("lightgreen", 255, 144, 238, 144),
    ("lightgrey", 255, 211, 211, 211),
    ("lightpink", 255, 255, 182, 193),
    ("lightsalmon", 255, 255, 160, 122),
    ("lightseagreen", 255, 32, 178, 170),
    ("lightskyblue", 255, 135, 206, 250),
    ("lightslategray", 255, 119, 136, 153),
    ("lightslategrey", 255, 119, 136, 153),
    ("lightsteelblue", 255, 176, 196, 222),
    ("lightyellow", 255, 255, 255, 224),
    ("lime", 255, 0, 255, 0),
    ("limegreen", 255, 50, 205, 50),
    ("linen", 255, 250, 240, 230),
    ("magenta", 255, 255, 0, 255),
    ("maroon", 255, 128, 0, 0),
    ("mediumaquamarine", 255, 102, 205, 170),
    ("mediumblue", 255, 0, 0, 205),
    ("mediumorchid", 255, 186, 85, 211),
    ("mediumpurple", 255, 147, 112, 219),
    ("mediumseagreen", 255, 60, 179, 113),
    ("mediumslateblue", 255, 123, 104, 238),
    ("mediumspringgreen", 255, 0, 250, 154),
    ("mediumturquoise", 255, 72, 209, 204),
    ("mediumvioletred", 255, 199, 21, 133),
    ("midnightblue", 255, 25, 25, 112),
    ("mintcream", 255, 245, 255, 250),
    ("mistyrose", 255, 255, 228, 225),
    ("moccasin", 255, 255, 228, 181),
    ("navajowhite", 255, 255, 222, 173),
    ("navy", 255, 0, 0, 128),
    ("oldlace", 255, 253, 245, 230),
    ("olive", 255, 128, 128, 0),
    ("olivedrab", 255, 107, 142, 35),
    ("orange", 255, 255, 165, 0),
    ("orangered", 255, 255, 69, 0),
    ("orchid", 255, 218, 112, 214),
    ("palegoldenrod", 255, 238, 232, 170),
    ("palegreen", 255, 152, 251, 152),
    ("paleturquoise", 255, 175, 238, 238),
    ("palevioletred", 255, 219, 112, 147),
    ("papayawhip", 255, 255, 239, 213),
    ("peachpuff", 255, 255, 218, 185),
    ("peru", 255, 205, 133, 63),
    ("pink", 255, 255, 192, 203),
    ("plum", 255, 221, 160, 221),
    ("powderblue", 255, 176, 224, 230),
    ("purple", 255, 128, 0, 128),
    ("rebeccapurple", 255, 102, 51, 153),
    ("red", 255, 255, 0, 0),
    ("rosybrown", 255, 188, 143, 143),
    ("royalblue", 255, 65, 105, 225),
    ("saddlebrown", 255, 139, 69, 19),
    ("salmon", 255, 250, 128, 114),
    ("sandybrown", 255, 244, 164, 96),
    ("seagreen", 255, 46, 139, 87),
    ("seashell", 255, 255, 245, 238),
    ("sienna", 255, 160, 82, 45),
    ("silver", 255, 192, 192, 192),
    ("skyblue", 255, 135, 206, 235),
    ("slateblue", 255, 106, 90, 205),
    ("slategray", 255, 112, 128, 144),
    ("slategrey", 255, 112, 128, 144),
    ("snow", 255, 255, 250, 250),
    ("springgreen", 255, 0, 255, 127),
    ("steelblue", 255, 70, 130, 180),
    ("tan", 255, 210, 180, 140),
    ("teal", 255, 0, 128, 128),
    ("thistle", 255, 216, 191, 216),
    ("tomato", 255, 255, 99, 71),
    ("turquoise", 255, 64, 224, 208),
    ("violet", 255, 238, 130, 238),
    ("wheat", 255, 245, 222, 179),
    ("white", 255, 255, 255, 255),
    ("whitesmoke", 255, 245, 245, 245),
    ("yellow", 255, 255, 255, 0),
    ("yellowgreen", 255, 154, 205, 50),
    ("transparent", 0, 0, 0, 0),
];

pub struct Colors;

impl Colors {
    fn color_from_entry((_, a, r, g, b): NamedWebColorEntry) -> Color {
        Color { a, r, g, b }
    }

    pub fn named_web_color(name: &str) -> Option<Color> {
        NAMED_WEB_COLORS
            .iter()
            .find(|(color_name, _, _, _, _)| color_name.eq_ignore_ascii_case(name))
            .map(|entry| Self::color_from_entry(*entry))
    }

    pub fn named_web_color_names() -> impl Iterator<Item = &'static str> {
        NAMED_WEB_COLORS.iter().map(|(name, _, _, _, _)| *name)
    }

    pub fn named_web_color_count() -> usize {
        NAMED_WEB_COLORS.len()
    }

    pub fn aliceblue() -> Color {
        Self::named_web_color("aliceblue").expect("aliceblue must be present in named web colors")
    }

    pub fn antiquewhite() -> Color {
        Self::named_web_color("antiquewhite").expect("antiquewhite must be present in named web colors")
    }

    pub fn aqua() -> Color {
        Self::named_web_color("aqua").expect("aqua must be present in named web colors")
    }

    pub fn aquamarine() -> Color {
        Self::named_web_color("aquamarine").expect("aquamarine must be present in named web colors")
    }

    pub fn azure() -> Color {
        Self::named_web_color("azure").expect("azure must be present in named web colors")
    }

    pub fn beige() -> Color {
        Self::named_web_color("beige").expect("beige must be present in named web colors")
    }

    pub fn bisque() -> Color {
        Self::named_web_color("bisque").expect("bisque must be present in named web colors")
    }

    pub fn black() -> Color {
        Self::named_web_color("black").expect("black must be present in named web colors")
    }

    pub fn blanchedalmond() -> Color {
        Self::named_web_color("blanchedalmond").expect("blanchedalmond must be present in named web colors")
    }

    pub fn blue() -> Color {
        Self::named_web_color("blue").expect("blue must be present in named web colors")
    }

    pub fn blueviolet() -> Color {
        Self::named_web_color("blueviolet").expect("blueviolet must be present in named web colors")
    }

    pub fn brown() -> Color {
        Self::named_web_color("brown").expect("brown must be present in named web colors")
    }

    pub fn burlywood() -> Color {
        Self::named_web_color("burlywood").expect("burlywood must be present in named web colors")
    }

    pub fn cadetblue() -> Color {
        Self::named_web_color("cadetblue").expect("cadetblue must be present in named web colors")
    }

    pub fn chartreuse() -> Color {
        Self::named_web_color("chartreuse").expect("chartreuse must be present in named web colors")
    }

    pub fn chocolate() -> Color {
        Self::named_web_color("chocolate").expect("chocolate must be present in named web colors")
    }

    pub fn coral() -> Color {
        Self::named_web_color("coral").expect("coral must be present in named web colors")
    }

    pub fn cornflowerblue() -> Color {
        Self::named_web_color("cornflowerblue").expect("cornflowerblue must be present in named web colors")
    }

    pub fn cornsilk() -> Color {
        Self::named_web_color("cornsilk").expect("cornsilk must be present in named web colors")
    }

    pub fn crimson() -> Color {
        Self::named_web_color("crimson").expect("crimson must be present in named web colors")
    }

    pub fn cyan() -> Color {
        Self::named_web_color("cyan").expect("cyan must be present in named web colors")
    }

    pub fn darkblue() -> Color {
        Self::named_web_color("darkblue").expect("darkblue must be present in named web colors")
    }

    pub fn darkcyan() -> Color {
        Self::named_web_color("darkcyan").expect("darkcyan must be present in named web colors")
    }

    pub fn darkgoldenrod() -> Color {
        Self::named_web_color("darkgoldenrod").expect("darkgoldenrod must be present in named web colors")
    }

    pub fn darkgray() -> Color {
        Self::named_web_color("darkgray").expect("darkgray must be present in named web colors")
    }

    pub fn darkgreen() -> Color {
        Self::named_web_color("darkgreen").expect("darkgreen must be present in named web colors")
    }

    pub fn darkgrey() -> Color {
        Self::named_web_color("darkgrey").expect("darkgrey must be present in named web colors")
    }

    pub fn darkkhaki() -> Color {
        Self::named_web_color("darkkhaki").expect("darkkhaki must be present in named web colors")
    }

    pub fn darkmagenta() -> Color {
        Self::named_web_color("darkmagenta").expect("darkmagenta must be present in named web colors")
    }

    pub fn darkolivegreen() -> Color {
        Self::named_web_color("darkolivegreen").expect("darkolivegreen must be present in named web colors")
    }

    pub fn darkorange() -> Color {
        Self::named_web_color("darkorange").expect("darkorange must be present in named web colors")
    }

    pub fn darkorchid() -> Color {
        Self::named_web_color("darkorchid").expect("darkorchid must be present in named web colors")
    }

    pub fn darkred() -> Color {
        Self::named_web_color("darkred").expect("darkred must be present in named web colors")
    }

    pub fn darksalmon() -> Color {
        Self::named_web_color("darksalmon").expect("darksalmon must be present in named web colors")
    }

    pub fn darkseagreen() -> Color {
        Self::named_web_color("darkseagreen").expect("darkseagreen must be present in named web colors")
    }

    pub fn darkslateblue() -> Color {
        Self::named_web_color("darkslateblue").expect("darkslateblue must be present in named web colors")
    }

    pub fn darkslategray() -> Color {
        Self::named_web_color("darkslategray").expect("darkslategray must be present in named web colors")
    }

    pub fn darkslategrey() -> Color {
        Self::named_web_color("darkslategrey").expect("darkslategrey must be present in named web colors")
    }

    pub fn darkturquoise() -> Color {
        Self::named_web_color("darkturquoise").expect("darkturquoise must be present in named web colors")
    }

    pub fn darkviolet() -> Color {
        Self::named_web_color("darkviolet").expect("darkviolet must be present in named web colors")
    }

    pub fn deeppink() -> Color {
        Self::named_web_color("deeppink").expect("deeppink must be present in named web colors")
    }

    pub fn deepskyblue() -> Color {
        Self::named_web_color("deepskyblue").expect("deepskyblue must be present in named web colors")
    }

    pub fn dimgray() -> Color {
        Self::named_web_color("dimgray").expect("dimgray must be present in named web colors")
    }

    pub fn dimgrey() -> Color {
        Self::named_web_color("dimgrey").expect("dimgrey must be present in named web colors")
    }

    pub fn dodgerblue() -> Color {
        Self::named_web_color("dodgerblue").expect("dodgerblue must be present in named web colors")
    }

    pub fn firebrick() -> Color {
        Self::named_web_color("firebrick").expect("firebrick must be present in named web colors")
    }

    pub fn floralwhite() -> Color {
        Self::named_web_color("floralwhite").expect("floralwhite must be present in named web colors")
    }

    pub fn forestgreen() -> Color {
        Self::named_web_color("forestgreen").expect("forestgreen must be present in named web colors")
    }

    pub fn fuchsia() -> Color {
        Self::named_web_color("fuchsia").expect("fuchsia must be present in named web colors")
    }

    pub fn gainsboro() -> Color {
        Self::named_web_color("gainsboro").expect("gainsboro must be present in named web colors")
    }

    pub fn ghostwhite() -> Color {
        Self::named_web_color("ghostwhite").expect("ghostwhite must be present in named web colors")
    }

    pub fn gold() -> Color {
        Self::named_web_color("gold").expect("gold must be present in named web colors")
    }

    pub fn goldenrod() -> Color {
        Self::named_web_color("goldenrod").expect("goldenrod must be present in named web colors")
    }

    pub fn gray() -> Color {
        Self::named_web_color("gray").expect("gray must be present in named web colors")
    }

    pub fn green() -> Color {
        Self::named_web_color("green").expect("green must be present in named web colors")
    }

    pub fn greenyellow() -> Color {
        Self::named_web_color("greenyellow").expect("greenyellow must be present in named web colors")
    }

    pub fn grey() -> Color {
        Self::named_web_color("grey").expect("grey must be present in named web colors")
    }

    pub fn honeydew() -> Color {
        Self::named_web_color("honeydew").expect("honeydew must be present in named web colors")
    }

    pub fn hotpink() -> Color {
        Self::named_web_color("hotpink").expect("hotpink must be present in named web colors")
    }

    pub fn indianred() -> Color {
        Self::named_web_color("indianred").expect("indianred must be present in named web colors")
    }

    pub fn indigo() -> Color {
        Self::named_web_color("indigo").expect("indigo must be present in named web colors")
    }

    pub fn ivory() -> Color {
        Self::named_web_color("ivory").expect("ivory must be present in named web colors")
    }

    pub fn khaki() -> Color {
        Self::named_web_color("khaki").expect("khaki must be present in named web colors")
    }

    pub fn lavender() -> Color {
        Self::named_web_color("lavender").expect("lavender must be present in named web colors")
    }

    pub fn lavenderblush() -> Color {
        Self::named_web_color("lavenderblush").expect("lavenderblush must be present in named web colors")
    }

    pub fn lawngreen() -> Color {
        Self::named_web_color("lawngreen").expect("lawngreen must be present in named web colors")
    }

    pub fn lemonchiffon() -> Color {
        Self::named_web_color("lemonchiffon").expect("lemonchiffon must be present in named web colors")
    }

    pub fn lightblue() -> Color {
        Self::named_web_color("lightblue").expect("lightblue must be present in named web colors")
    }

    pub fn lightcoral() -> Color {
        Self::named_web_color("lightcoral").expect("lightcoral must be present in named web colors")
    }

    pub fn lightcyan() -> Color {
        Self::named_web_color("lightcyan").expect("lightcyan must be present in named web colors")
    }

    pub fn lightgoldenrodyellow() -> Color {
        Self::named_web_color("lightgoldenrodyellow").expect("lightgoldenrodyellow must be present in named web colors")
    }

    pub fn lightgray() -> Color {
        Self::named_web_color("lightgray").expect("lightgray must be present in named web colors")
    }

    pub fn lightgreen() -> Color {
        Self::named_web_color("lightgreen").expect("lightgreen must be present in named web colors")
    }

    pub fn lightgrey() -> Color {
        Self::named_web_color("lightgrey").expect("lightgrey must be present in named web colors")
    }

    pub fn lightpink() -> Color {
        Self::named_web_color("lightpink").expect("lightpink must be present in named web colors")
    }

    pub fn lightsalmon() -> Color {
        Self::named_web_color("lightsalmon").expect("lightsalmon must be present in named web colors")
    }

    pub fn lightseagreen() -> Color {
        Self::named_web_color("lightseagreen").expect("lightseagreen must be present in named web colors")
    }

    pub fn lightskyblue() -> Color {
        Self::named_web_color("lightskyblue").expect("lightskyblue must be present in named web colors")
    }

    pub fn lightslategray() -> Color {
        Self::named_web_color("lightslategray").expect("lightslategray must be present in named web colors")
    }

    pub fn lightslategrey() -> Color {
        Self::named_web_color("lightslategrey").expect("lightslategrey must be present in named web colors")
    }

    pub fn lightsteelblue() -> Color {
        Self::named_web_color("lightsteelblue").expect("lightsteelblue must be present in named web colors")
    }

    pub fn lightyellow() -> Color {
        Self::named_web_color("lightyellow").expect("lightyellow must be present in named web colors")
    }

    pub fn lime() -> Color {
        Self::named_web_color("lime").expect("lime must be present in named web colors")
    }

    pub fn limegreen() -> Color {
        Self::named_web_color("limegreen").expect("limegreen must be present in named web colors")
    }

    pub fn linen() -> Color {
        Self::named_web_color("linen").expect("linen must be present in named web colors")
    }

    pub fn magenta() -> Color {
        Self::named_web_color("magenta").expect("magenta must be present in named web colors")
    }

    pub fn maroon() -> Color {
        Self::named_web_color("maroon").expect("maroon must be present in named web colors")
    }

    pub fn mediumaquamarine() -> Color {
        Self::named_web_color("mediumaquamarine").expect("mediumaquamarine must be present in named web colors")
    }

    pub fn mediumblue() -> Color {
        Self::named_web_color("mediumblue").expect("mediumblue must be present in named web colors")
    }

    pub fn mediumorchid() -> Color {
        Self::named_web_color("mediumorchid").expect("mediumorchid must be present in named web colors")
    }

    pub fn mediumpurple() -> Color {
        Self::named_web_color("mediumpurple").expect("mediumpurple must be present in named web colors")
    }

    pub fn mediumseagreen() -> Color {
        Self::named_web_color("mediumseagreen").expect("mediumseagreen must be present in named web colors")
    }

    pub fn mediumslateblue() -> Color {
        Self::named_web_color("mediumslateblue").expect("mediumslateblue must be present in named web colors")
    }

    pub fn mediumspringgreen() -> Color {
        Self::named_web_color("mediumspringgreen").expect("mediumspringgreen must be present in named web colors")
    }

    pub fn mediumturquoise() -> Color {
        Self::named_web_color("mediumturquoise").expect("mediumturquoise must be present in named web colors")
    }

    pub fn mediumvioletred() -> Color {
        Self::named_web_color("mediumvioletred").expect("mediumvioletred must be present in named web colors")
    }

    pub fn midnightblue() -> Color {
        Self::named_web_color("midnightblue").expect("midnightblue must be present in named web colors")
    }

    pub fn mintcream() -> Color {
        Self::named_web_color("mintcream").expect("mintcream must be present in named web colors")
    }

    pub fn mistyrose() -> Color {
        Self::named_web_color("mistyrose").expect("mistyrose must be present in named web colors")
    }

    pub fn moccasin() -> Color {
        Self::named_web_color("moccasin").expect("moccasin must be present in named web colors")
    }

    pub fn navajowhite() -> Color {
        Self::named_web_color("navajowhite").expect("navajowhite must be present in named web colors")
    }

    pub fn navy() -> Color {
        Self::named_web_color("navy").expect("navy must be present in named web colors")
    }

    pub fn oldlace() -> Color {
        Self::named_web_color("oldlace").expect("oldlace must be present in named web colors")
    }

    pub fn olive() -> Color {
        Self::named_web_color("olive").expect("olive must be present in named web colors")
    }

    pub fn olivedrab() -> Color {
        Self::named_web_color("olivedrab").expect("olivedrab must be present in named web colors")
    }

    pub fn orange() -> Color {
        Self::named_web_color("orange").expect("orange must be present in named web colors")
    }

    pub fn orangered() -> Color {
        Self::named_web_color("orangered").expect("orangered must be present in named web colors")
    }

    pub fn orchid() -> Color {
        Self::named_web_color("orchid").expect("orchid must be present in named web colors")
    }

    pub fn palegoldenrod() -> Color {
        Self::named_web_color("palegoldenrod").expect("palegoldenrod must be present in named web colors")
    }

    pub fn palegreen() -> Color {
        Self::named_web_color("palegreen").expect("palegreen must be present in named web colors")
    }

    pub fn paleturquoise() -> Color {
        Self::named_web_color("paleturquoise").expect("paleturquoise must be present in named web colors")
    }

    pub fn palevioletred() -> Color {
        Self::named_web_color("palevioletred").expect("palevioletred must be present in named web colors")
    }

    pub fn papayawhip() -> Color {
        Self::named_web_color("papayawhip").expect("papayawhip must be present in named web colors")
    }

    pub fn peachpuff() -> Color {
        Self::named_web_color("peachpuff").expect("peachpuff must be present in named web colors")
    }

    pub fn peru() -> Color {
        Self::named_web_color("peru").expect("peru must be present in named web colors")
    }

    pub fn pink() -> Color {
        Self::named_web_color("pink").expect("pink must be present in named web colors")
    }

    pub fn plum() -> Color {
        Self::named_web_color("plum").expect("plum must be present in named web colors")
    }

    pub fn powderblue() -> Color {
        Self::named_web_color("powderblue").expect("powderblue must be present in named web colors")
    }

    pub fn purple() -> Color {
        Self::named_web_color("purple").expect("purple must be present in named web colors")
    }

    pub fn rebeccapurple() -> Color {
        Self::named_web_color("rebeccapurple").expect("rebeccapurple must be present in named web colors")
    }

    pub fn red() -> Color {
        Self::named_web_color("red").expect("red must be present in named web colors")
    }

    pub fn rosybrown() -> Color {
        Self::named_web_color("rosybrown").expect("rosybrown must be present in named web colors")
    }

    pub fn royalblue() -> Color {
        Self::named_web_color("royalblue").expect("royalblue must be present in named web colors")
    }

    pub fn saddlebrown() -> Color {
        Self::named_web_color("saddlebrown").expect("saddlebrown must be present in named web colors")
    }

    pub fn salmon() -> Color {
        Self::named_web_color("salmon").expect("salmon must be present in named web colors")
    }

    pub fn sandybrown() -> Color {
        Self::named_web_color("sandybrown").expect("sandybrown must be present in named web colors")
    }

    pub fn seagreen() -> Color {
        Self::named_web_color("seagreen").expect("seagreen must be present in named web colors")
    }

    pub fn seashell() -> Color {
        Self::named_web_color("seashell").expect("seashell must be present in named web colors")
    }

    pub fn sienna() -> Color {
        Self::named_web_color("sienna").expect("sienna must be present in named web colors")
    }

    pub fn silver() -> Color {
        Self::named_web_color("silver").expect("silver must be present in named web colors")
    }

    pub fn skyblue() -> Color {
        Self::named_web_color("skyblue").expect("skyblue must be present in named web colors")
    }

    pub fn slateblue() -> Color {
        Self::named_web_color("slateblue").expect("slateblue must be present in named web colors")
    }

    pub fn slategray() -> Color {
        Self::named_web_color("slategray").expect("slategray must be present in named web colors")
    }

    pub fn slategrey() -> Color {
        Self::named_web_color("slategrey").expect("slategrey must be present in named web colors")
    }

    pub fn snow() -> Color {
        Self::named_web_color("snow").expect("snow must be present in named web colors")
    }

    pub fn springgreen() -> Color {
        Self::named_web_color("springgreen").expect("springgreen must be present in named web colors")
    }

    pub fn steelblue() -> Color {
        Self::named_web_color("steelblue").expect("steelblue must be present in named web colors")
    }

    pub fn tan() -> Color {
        Self::named_web_color("tan").expect("tan must be present in named web colors")
    }

    pub fn teal() -> Color {
        Self::named_web_color("teal").expect("teal must be present in named web colors")
    }

    pub fn thistle() -> Color {
        Self::named_web_color("thistle").expect("thistle must be present in named web colors")
    }

    pub fn tomato() -> Color {
        Self::named_web_color("tomato").expect("tomato must be present in named web colors")
    }

    pub fn turquoise() -> Color {
        Self::named_web_color("turquoise").expect("turquoise must be present in named web colors")
    }

    pub fn violet() -> Color {
        Self::named_web_color("violet").expect("violet must be present in named web colors")
    }

    pub fn wheat() -> Color {
        Self::named_web_color("wheat").expect("wheat must be present in named web colors")
    }

    pub fn white() -> Color {
        Self::named_web_color("white").expect("white must be present in named web colors")
    }

    pub fn whitesmoke() -> Color {
        Self::named_web_color("whitesmoke").expect("whitesmoke must be present in named web colors")
    }

    pub fn yellow() -> Color {
        Self::named_web_color("yellow").expect("yellow must be present in named web colors")
    }

    pub fn yellowgreen() -> Color {
        Self::named_web_color("yellowgreen").expect("yellowgreen must be present in named web colors")
    }

    pub fn transparent() -> Color {
        Self::named_web_color("transparent").expect("transparent must be present in named web colors")
    }
}
