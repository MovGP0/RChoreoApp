use std::rc::Rc;

use slint::platform::software_renderer::MinimalSoftwareWindow;
use slint::platform::software_renderer::RepaintBufferType;
use slint::platform::Platform;
use slint::platform::PlatformError;
use slint::platform::WindowAdapter;
use slint::ComponentHandle;
use slint::PhysicalSize;
use slint::Rgb8Pixel;
use slint::SharedPixelBuffer;

thread_local!
{
    static WINDOW: Rc<MinimalSoftwareWindow> = MinimalSoftwareWindow::new(RepaintBufferType::ReusedBuffer);
}

struct TextRenderingTestPlatform;

impl Platform for TextRenderingTestPlatform
{
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError>
    {
        Ok(WINDOW.with(|window| window.clone()))
    }
}

slint::slint!
{
    export component TextHeightProbe inherits Window {
        in property <length> sample_font_size: 12px;
        in property <string> sample_text: "The quick brown fox jumps over the lazy dog";

        width: 1800px;
        height: 360px;
        background: white;

        Text {
            x: 24px;
            y: 24px;
            text: root.sample_text;
            color: black;
            font-size: root.sample_font_size;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TextHeightMeasurement
{
    font_size_px: f32,
    rendered_height_px: u32,
    factor_height_vs_font_size: f32,
}

#[test]
#[serial_test::serial]
fn slint_text_rendered_height_factor_table()
{
    let _ = slint::platform::set_platform(Box::new(TextRenderingTestPlatform));

    const SAMPLE_TEXT: &str = "The quick brown fox jumps over the lazy dog";
    const WINDOW_WIDTH: u32 = 1800;
    const WINDOW_HEIGHT: u32 = 360;
    const FONT_SIZES: [f32; 8] = [12.0, 16.0, 20.0, 24.0, 30.0, 36.0, 48.0, 60.0];

    let window = WINDOW.with(|window| window.clone());
    window.set_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT));

    let component = TextHeightProbe::new().expect("text probe component should be created");
    component
        .show()
        .expect("text probe component should be shown on test platform");

    let measurements: Vec<TextHeightMeasurement> = FONT_SIZES
        .iter()
        .copied()
        .map(|font_size_px|
        {
            let rendered_height_px =
                render_and_measure_text_height(&window, &component, SAMPLE_TEXT, font_size_px);
            TextHeightMeasurement {
                font_size_px,
                rendered_height_px,
                factor_height_vs_font_size: rendered_height_px as f32 / font_size_px,
            }
        })
        .collect();

    assert!(
        measurements.iter().all(|measurement| measurement.rendered_height_px > 0),
        "all measured text heights must be positive"
    );

    for pair in measurements.windows(2)
    {
        let before = pair[0];
        let after = pair[1];
        assert!(
            after.rendered_height_px >= before.rendered_height_px,
            "text height should be monotonic: {:?} -> {:?}",
            before,
            after
        );
    }

    println!("Slint rendered text height table:");
    println!("| font_size_px | rendered_text_height_px | factor(height/font-size) |");
    println!("|---:|---:|---:|");
    for measurement in &measurements
    {
        println!(
            "| {:.1} | {} | {:.4} |",
            measurement.font_size_px,
            measurement.rendered_height_px,
            measurement.factor_height_vs_font_size
        );
    }

    println!("Scale comparison table:");
    println!("| from_font_px | to_font_px | font_scale | rendered_height_scale | delta |");
    println!("|---:|---:|---:|---:|---:|");
    for pair in measurements.windows(2)
    {
        let before = pair[0];
        let after = pair[1];
        let font_scale = after.font_size_px / before.font_size_px;
        let rendered_scale = after.rendered_height_px as f32 / before.rendered_height_px as f32;
        let delta = rendered_scale - font_scale;
        println!(
            "| {:.1} | {:.1} | {:.4} | {:.4} | {:+.4} |",
            before.font_size_px,
            after.font_size_px,
            font_scale,
            rendered_scale,
            delta
        );
    }
}

fn render_and_measure_text_height(
    window: &MinimalSoftwareWindow,
    component: &TextHeightProbe,
    sample_text: &str,
    font_size_px: f32,
) -> u32
{
    component.set_sample_text(sample_text.into());
    component.set_sample_font_size(font_size_px);
    slint::platform::update_timers_and_animations();

    let mut rendered_pixels: Option<SharedPixelBuffer<Rgb8Pixel>> = None;
    let draw_happened = window.draw_if_needed(|renderer|
    {
        let mut pixels = SharedPixelBuffer::<Rgb8Pixel>::new(1800, 360);
        {
            let pixels_slice = pixels.make_mut_slice();
            pixels_slice.fill(Rgb8Pixel { r: 255, g: 255, b: 255 });
            let _ = renderer.render(pixels_slice, 1800usize);
        }
        rendered_pixels = Some(pixels);
    });

    assert!(draw_happened, "expected a redraw after text/font update");

    let rendered_pixels = rendered_pixels.expect("rendered pixel buffer should be captured");
    measure_vertical_ink_span(&rendered_pixels).unwrap_or(0)
}

fn measure_vertical_ink_span(pixels: &SharedPixelBuffer<Rgb8Pixel>) -> Option<u32>
{
    let width = pixels.width() as usize;
    let rows = pixels.as_slice().chunks_exact(width);

    let first_ink_row = rows
        .clone()
        .enumerate()
        .find(|(_, row)| row.iter().any(is_ink_pixel))
        .map(|(index, _)| index as u32)?;

    let last_ink_row = rows
        .enumerate()
        .rev()
        .find(|(_, row)| row.iter().any(is_ink_pixel))
        .map(|(index, _)| index as u32)?;

    Some(last_ink_row - first_ink_row + 1)
}

fn is_ink_pixel(pixel: &Rgb8Pixel) -> bool
{
    let luminance = (pixel.r as u32 + pixel.g as u32 + pixel.b as u32) / 3;
    luminance < 245
}
