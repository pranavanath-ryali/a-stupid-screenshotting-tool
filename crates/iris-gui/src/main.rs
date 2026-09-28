use gtk4::{
    Application, ApplicationWindow, EventControllerKey, Label,
    cairo::ImageSurface,
    gdk::Key,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    prelude::{GtkWindowExt, WidgetExt},
};
use gtk4_layer_shell::LayerShell;

fn main() {
    let app = Application::builder()
        .application_id("com.pandioxus.iris")
        .build();

    app.connect_activate(|app| {
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

        let label = Label::new(Some("HIHI"));
        window.set_child(Some(&label));

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
