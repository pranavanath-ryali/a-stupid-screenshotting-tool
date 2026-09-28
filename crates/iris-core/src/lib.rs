pub mod compositor;

use image::DynamicImage;
use libwayshot::{
    LogicalRegion, Size, WayshotConnection,
    region::{Position, Region},
};

pub struct WaylandCapturer {
    wayshot: WayshotConnection,
}

impl WaylandCapturer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let wayshot = WayshotConnection::new()?;
        Ok(Self { wayshot })
    }

    pub fn capture_screen(&mut self) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        let image_buffer = self.wayshot.screenshot_all(false)?;
        Ok(image_buffer)
    }

    pub fn capture_region(
        &mut self,
        bounds: (i32, i32, u32, u32),
    ) -> Result<DynamicImage, Box<dyn std::error::Error>> {
        let image_buffer = self.wayshot.screenshot(
            LogicalRegion {
                inner: Region {
                    position: Position {
                        x: bounds.0,
                        y: bounds.1,
                    },
                    size: Size {
                        width: bounds.2,
                        height: bounds.3,
                    },
                },
            },
            false,
        )?;

        Ok(image_buffer)
    }
}
