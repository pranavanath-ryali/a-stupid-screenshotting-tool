use libwayshot::WayshotConnection;

pub fn capture_screen() -> Result<(), Box<dyn std::error::Error>> {
    let wayshot = WayshotConnection::new()?;

    let image_buffer = wayshot.screenshot_all(false)?;
    image_buffer.save_with_format("screenshot.png", image::ImageFormat::Png)?;

    Ok(())
}
