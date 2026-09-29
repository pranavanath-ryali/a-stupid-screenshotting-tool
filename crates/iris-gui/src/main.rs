use gtk4::{Picture, prelude::*};
use gtk4::{
    Application, ApplicationWindow, CssProvider, EventControllerKey, Image,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
    gdk::{Display, Key, MemoryTexture},
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    glib::Bytes,
    prelude::{GtkWindowExt, WidgetExt},
};
use gtk4_layer_shell::LayerShell;
use iris_core::WaylandCapturer;

fn main() {
    let app = Application::builder()
        .application_id("com.pandioxus.iris")
        .build();

    let mut capturer = WaylandCapturer::new().expect("Couldn't create new capturer");
    let image = capturer.capture_screen().unwrap().to_rgb8();
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

    app.connect_activate(move |app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Iris Screenshot")
            .build();

        window.init_layer_shell();
        window.set_layer(gtk4_layer_shell::Layer::Overlay);
        window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);

        window.set_exclusive_zone(-1);

        window.set_anchor(gtk4_layer_shell::Edge::Top, true);
        window.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
        window.set_anchor(gtk4_layer_shell::Edge::Left, true);
        window.set_anchor(gtk4_layer_shell::Edge::Right, true);

        let screen = Picture::for_paintable(&texture);
        screen.set_content_fit(gtk4::ContentFit::Contain);

        // let label = Label::new(Some("HIHI"));
        window.set_child(Some(&screen));
        // window.set_child(Some(&label));

        let key_controller = EventControllerKey::new();
        let window_clone = window.clone();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == Key::Escape {
                window_clone.close();
                return gtk4::glib::Propagation::Stop;
            }

            gtk4::glib::Propagation::Proceed
        });

        window.add_controller(key_controller);
        window.present();
    });
    app.run();
}

fn load_css() {
    let provider = CssProvider::new();

    provider.load_from_data(
        "
        window {
            background-color: transparent;
            background-image: none;
            box-shadow: none;
            border: none;
        }
        ",
    );

    if let Some(display) = Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
