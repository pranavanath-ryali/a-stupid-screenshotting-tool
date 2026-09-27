use iris_core::capture_screen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    capture_screen()?;

    Ok(())
}
