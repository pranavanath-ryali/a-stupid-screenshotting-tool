use gtk4::{Picture, gdk::MemoryTexture, glib::Bytes};
use image::DynamicImage;

pub(crate) fn build_picture_from_screenshot(image: &DynamicImage) -> Picture {
    let image = image.to_rgb8();
    let width = image.width() as i32;
    let height = image.height() as i32;
    let stride = width as usize * 3;

    let image_bytes = Bytes::from(&image.into_raw());
    let texture = MemoryTexture::new(
        width,
        height,
        gtk4::gdk::MemoryFormat::R8g8b8,
        &image_bytes,
        stride,
    );

    Picture::builder()
        .paintable(&texture)
        .content_fit(gtk4::ContentFit::Contain)
        .build()
}
