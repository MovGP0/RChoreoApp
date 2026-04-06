use std::collections::HashMap;
use std::str;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;

use egui::Color32;
use egui::Context;
use egui::Image;
use egui::ImageSource;
use egui::Rect;
use egui::Response;
use egui::Sense;
use egui::Ui;
use egui::Vec2;
use egui::epaint::Mesh;
use egui::epaint::Shape;
use egui::epaint::WHITE_UV;
use egui::vec2;
use lyon_extra::parser::ParserOptions;
use lyon_extra::parser::PathParser;
use lyon_extra::parser::Source;
use lyon_path::Path;
use lyon_tessellation::BuffersBuilder;
use lyon_tessellation::FillOptions;
use lyon_tessellation::FillTessellator;
use lyon_tessellation::FillVertex;
use lyon_tessellation::VertexBuffers;

use crate::styling::material_palette::material_palette_for_visuals;
use crate::styling::material_style_metrics::material_style_metrics;

const SVG_FLATTENING_TOLERANCE_PX: f32 = 0.05;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialIconStyle {
    pub size: Vec2,
    pub tint: Color32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct SvgViewBox {
    min_x: f32,
    min_y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone)]
struct ParsedSvgIcon {
    view_box: SvgViewBox,
    vertices: Vec<lyon_path::math::Point>,
    indices: Vec<u32>,
}

type ParsedSvgIconCache = HashMap<String, Option<Arc<ParsedSvgIcon>>>;

impl MaterialIconStyle {
    #[must_use]
    pub fn for_ui(ui: &Ui) -> Self {
        let metrics = material_style_metrics();
        let palette = material_palette_for_visuals(ui.visuals());
        Self {
            size: vec2(
                metrics.icon_sizes.icon_size_18,
                metrics.icon_sizes.icon_size_18,
            ),
            tint: palette.on_background,
        }
    }
}

pub type MaterialIcon<'a> = Image<'a>;

pub fn icon<'a>(ui: &Ui, image: Image<'a>) -> MaterialIcon<'a> {
    let style = MaterialIconStyle::for_ui(ui);
    icon_with_style(image, style)
}

pub fn icon_with_style<'a>(image: Image<'a>, style: MaterialIconStyle) -> MaterialIcon<'a> {
    image.fit_to_exact_size(style.size).tint(style.tint)
}

pub fn show_icon<'a>(ui: &mut Ui, image: Image<'a>) -> Response {
    let style = MaterialIconStyle::for_ui(ui);
    show_icon_with_style(ui, &image, style)
}

pub fn show_icon_with_style(ui: &mut Ui, image: &Image<'_>, style: MaterialIconStyle) -> Response {
    let (rect, response) = ui.allocate_exact_size(style.size, Sense::hover());
    paint_icon(ui, image, rect, style.tint);
    response
}

#[must_use]
pub fn centered_icon_rect(container_rect: Rect, icon_size: Vec2) -> Rect {
    Rect::from_center_size(container_rect.center(), icon_size)
}

pub fn paint_icon(ui: &Ui, image: &Image<'_>, rect: Rect, tint: Color32) {
    if let Some(parsed_icon) = cached_svg_icon_from_image(ui.ctx(), image) {
        ui.painter()
            .add(Shape::mesh(svg_icon_mesh(parsed_icon.as_ref(), rect, tint)));
        return;
    }

    // Fall back to egui's image pipeline for textures or non-project image assets.
    image
        .clone()
        .fit_to_exact_size(rect.size())
        .tint(tint)
        .paint_at(ui, rect);
}

fn cached_svg_icon_from_image(ctx: &Context, image: &Image<'_>) -> Option<Arc<ParsedSvgIcon>> {
    match image.source(ctx) {
        ImageSource::Bytes { uri, bytes } => cached_svg_icon(uri.as_ref(), bytes.as_ref()),
        ImageSource::Uri(_) | ImageSource::Texture(_) => None,
    }
}

fn cached_svg_icon(uri: &str, bytes: &[u8]) -> Option<Arc<ParsedSvgIcon>> {
    let mut cache = svg_icon_cache()
        .lock()
        .unwrap_or_else(|poison| poison.into_inner());
    if let Some(parsed_icon) = cache.get(uri) {
        return parsed_icon.clone();
    }

    let parsed_icon = parse_svg_icon(bytes).map(Arc::new);
    let result = parsed_icon.clone();
    let _ = cache.insert(uri.to_owned(), parsed_icon);
    result
}

fn parse_svg_icon(bytes: &[u8]) -> Option<ParsedSvgIcon> {
    let svg = str::from_utf8(bytes).ok()?;
    let view_box = parse_svg_view_box(svg)?;
    let path_data = parse_path_data(svg);
    if path_data.is_empty() {
        return None;
    }

    let mut builder = Path::builder().flattened(SVG_FLATTENING_TOLERANCE_PX);
    let mut parser = PathParser::new();
    for path_data in path_data {
        let mut source = Source::new(path_data.chars());
        parser
            .parse(&ParserOptions::DEFAULT, &mut source, &mut builder)
            .ok()?;
    }

    let path = builder.build();
    let mut tessellator = FillTessellator::new();
    let mut geometry: VertexBuffers<lyon_path::math::Point, u16> = VertexBuffers::new();
    tessellator
        .tessellate_path(
            path.as_slice(),
            &FillOptions::default(),
            &mut BuffersBuilder::new(&mut geometry, |vertex: FillVertex<'_>| vertex.position()),
        )
        .ok()?;

    if geometry.vertices.is_empty() || geometry.indices.is_empty() {
        return None;
    }

    Some(ParsedSvgIcon {
        view_box,
        vertices: geometry.vertices,
        indices: geometry.indices.into_iter().map(u32::from).collect(),
    })
}

fn svg_icon_mesh(icon: &ParsedSvgIcon, rect: Rect, tint: Color32) -> Mesh {
    let mut mesh = Mesh::default();
    mesh.indices.reserve(icon.indices.len());
    mesh.vertices.reserve(icon.vertices.len());

    for vertex in &icon.vertices {
        mesh.vertices.push(egui::epaint::Vertex {
            pos: transform_svg_point(icon.view_box, rect, vertex.x, vertex.y),
            uv: WHITE_UV,
            color: tint,
        });
    }
    mesh.indices.extend(icon.indices.iter().copied());

    mesh
}

#[must_use]
fn transform_svg_point(view_box: SvgViewBox, rect: Rect, x: f32, y: f32) -> egui::Pos2 {
    let scale = (rect.width() / view_box.width).min(rect.height() / view_box.height);
    let scaled_width = view_box.width * scale;
    let scaled_height = view_box.height * scale;
    let offset_x = rect.center().x - (scaled_width * 0.5) - (view_box.min_x * scale);
    let offset_y = rect.center().y - (scaled_height * 0.5) - (view_box.min_y * scale);

    egui::pos2(offset_x + (x * scale), offset_y + (y * scale))
}

fn parse_svg_view_box(svg: &str) -> Option<SvgViewBox> {
    let svg_tag = extract_element_tag(svg, "svg")?;
    let view_box = extract_attribute(svg_tag, "viewBox")?;
    let values = split_numbers(view_box);
    if values.len() != 4 {
        return None;
    }

    Some(SvgViewBox {
        min_x: values[0],
        min_y: values[1],
        width: values[2],
        height: values[3],
    })
}

fn parse_path_data(svg: &str) -> Vec<&str> {
    let mut remaining = svg;
    let mut paths = Vec::new();

    while let Some(tag) = extract_next_element_tag(remaining, "path") {
        if let Some(path_data) = extract_attribute(tag.0, "d") {
            paths.push(path_data);
        }
        remaining = tag.1;
    }

    paths
}

fn extract_element_tag<'a>(svg: &'a str, name: &str) -> Option<&'a str> {
    let start_index = svg.find(&format!("<{name}"))?;
    let tag_start = &svg[start_index..];
    let tag_end = tag_start.find('>')?;
    Some(&tag_start[..=tag_end])
}

fn extract_next_element_tag<'a>(svg: &'a str, name: &str) -> Option<(&'a str, &'a str)> {
    let start_index = svg.find(&format!("<{name}"))?;
    let tag_start = &svg[start_index..];
    let tag_end = tag_start.find('>')?;
    Some((&tag_start[..=tag_end], &tag_start[tag_end + 1..]))
}

fn extract_attribute<'a>(tag: &'a str, attribute_name: &str) -> Option<&'a str> {
    let attribute = format!("{attribute_name}=\"");
    let value_start = tag.find(&attribute)? + attribute.len();
    let remainder = &tag[value_start..];
    let value_end = remainder.find('"')?;
    Some(&remainder[..value_end])
}

fn split_numbers(source: &str) -> Vec<f32> {
    source
        .split(|character: char| character.is_ascii_whitespace() || character == ',')
        .filter(|part| !part.is_empty())
        .filter_map(|part| part.parse::<f32>().ok())
        .collect()
}

fn svg_icon_cache() -> &'static Mutex<ParsedSvgIconCache> {
    static SVG_ICON_CACHE: OnceLock<Mutex<ParsedSvgIconCache>> = OnceLock::new();
    SVG_ICON_CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use egui::Context;

    use super::MaterialIconStyle;
    use super::SvgViewBox;
    use super::cached_svg_icon;
    use super::centered_icon_rect;
    use super::parse_path_data;
    use super::parse_svg_view_box;
    use crate::icons;
    use crate::icons::UiIconKey;

    #[test]
    fn default_style_matches_slint_defaults() {
        let context = Context::default();
        let mut style = None;
        let mut expected_tint = None;
        let _ = context.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                style = Some(MaterialIconStyle::for_ui(ui));
                expected_tint = Some(
                    crate::styling::material_palette::material_palette_for_visuals(ui.visuals())
                        .on_background,
                );
            });
        });
        let style = style.expect("icon style");
        assert_eq!(style.size, egui::vec2(18.0, 18.0));
        assert_eq!(style.tint, expected_tint.expect("expected tint"));
    }

    #[test]
    fn svg_parser_extracts_project_icon_view_box_and_path_data() {
        let svg = icons::icon(UiIconKey::SettingsNavigateBack).svg;

        assert_eq!(
            parse_svg_view_box(svg),
            Some(SvgViewBox {
                min_x: 0.0,
                min_y: 0.0,
                width: 24.0,
                height: 24.0,
            })
        );
        assert_eq!(parse_path_data(svg).len(), 1);
    }

    #[test]
    fn svg_icon_cache_reuses_parsed_geometry_for_same_uri() {
        let svg = icons::icon(UiIconKey::SettingsNavigateBack).svg;

        let first =
            cached_svg_icon("bytes://tests/arrow_left.svg", svg.as_bytes()).expect("first parse");
        let second =
            cached_svg_icon("bytes://tests/arrow_left.svg", svg.as_bytes()).expect("cached parse");

        assert!(Arc::ptr_eq(&first, &second));
    }

    #[test]
    fn centered_icon_rect_keeps_glyph_centered_on_both_axes() {
        let outer = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(40.0, 40.0));
        let inner = centered_icon_rect(outer, egui::vec2(24.0, 24.0));

        assert_eq!(inner.center(), outer.center());
    }
}
