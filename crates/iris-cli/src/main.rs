use iris_core::{WaylandCapturer, compositor::{Compositor, get_active_client_bounds}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut capturer = WaylandCapturer::new()?;

    let compositor = Compositor::detect();
    let active_client_bounds = get_active_client_bounds(&compositor).unwrap();

    let image = capturer.capture_region(active_client_bounds)?;
    image.save_with_format("screenshot.png", image::ImageFormat::Png)?;

    Ok(())
}
