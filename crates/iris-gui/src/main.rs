mod gui;

use gtk4::{
    Application, ApplicationWindow, CssProvider, EventControllerMotion,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
    gdk::Display,
    gio::prelude::{ApplicationExt, ApplicationExtManual},
    prelude::{BoxExt, GtkWindowExt, WidgetExt},
};
use gtk4_layer_shell::LayerShell;
use iris_core::WaylandCapturer;
use std::{cell::Cell, rc::Rc, time::Duration};

use crate::gui::background::build_picture_from_screenshot;

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(
        "
        window.preview-window {
            background: transparent;
            border: none;
            box-shadow: none;
            padding: 0;
        }

        .preview-frame {
            background: rgba(255, 255, 255, 0.02);
            border: 3px solid rgba(255, 255, 255, 0.28);
            border-radius: 14px;
            padding: 0;
        }

        .preview-frame > * {
            border-radius: inherit;
        }

        .action-button {
            min-width: 32px;
            min-height: 32px;
            width: 32px;
            height: 32px;
            padding: 0;
            border: 1px solid rgba(255, 255, 255, 0.25);
            border-radius: 8px;
            background: rgba(0, 0, 0, 0.02);
        }

        .floating-actions {
            background: transparent;
            border: none;
            box-shadow: none;
            padding: 0;
        }
        ",
    );

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("No default display"),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    let app = Application::builder()
        .application_id("com.pandioxus.iris")
        .build();

    let mut capturer = WaylandCapturer::new().expect("Couldn't create new capturer");
    let screen_image = capturer
        .capture_screen()
        .expect("Couldn't take a screenshot of screen");

    app.connect_activate(move |app| {
        load_css();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Iris Preview")
            .build();

        // Fade out on close: animate opacity from 1 to 0 over 500ms, then destroy
        window.connect_close_request(move |win| {
            let start_time = std::time::Instant::now();
            let win_clone = win.clone();
            gtk4::glib::timeout_add_local(Duration::from_millis(2), move || {
                let elapsed = start_time.elapsed().as_millis() as f64;
                let opacity = (1.0 - (elapsed / 500.0)).max(0.0);
                win_clone.set_opacity(opacity);
                if opacity <= 0.0 {
                    win_clone.destroy();
                    gtk4::glib::ControlFlow::Break
                } else {
                    gtk4::glib::ControlFlow::Continue
                }
            });
            gtk4::glib::Propagation::Stop // don't close yet
        });

        // Fade in: animate opacity from 0 to 1 over 500ms
        window.set_opacity(0.0);
        let window_clone = window.clone();
        let start_time = std::time::Instant::now();
        gtk4::glib::timeout_add_local(Duration::from_millis(2), move || {
            let elapsed = start_time.elapsed().as_millis() as f64;
            let opacity = (elapsed / 500.0).min(1.0);
            window_clone.set_opacity(opacity);
            if opacity >= 1.0 {
                gtk4::glib::ControlFlow::Break
            } else {
                gtk4::glib::ControlFlow::Continue
            }
        });

        window.init_layer_shell();
        window.set_layer(gtk4_layer_shell::Layer::Overlay);
        window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::None);

        window.set_decorated(false);
        window.set_exclusive_zone(-1);

        window.set_anchor(gtk4_layer_shell::Edge::Bottom, true);
        window.set_anchor(gtk4_layer_shell::Edge::Left, true);

        window.set_margin(gtk4_layer_shell::Edge::Left, 12);
        window.set_margin(gtk4_layer_shell::Edge::Bottom, 12);

        let picture = build_picture_from_screenshot(&screen_image);
        picture.set_can_shrink(true);
        picture.set_content_fit(gtk4::ContentFit::Cover);
        picture.set_size_request(280, 160);

        let preview = gtk4::AspectFrame::new(0.5, 0.5, 1.78, true);
        preview.set_child(Some(&picture));
        preview.set_overflow(gtk4::Overflow::Hidden);
        preview.set_size_request(280, 160);
        preview.add_css_class("preview-frame");

        let hover_active = Rc::new(Cell::new(false));
        let hover_active_enter = hover_active.clone();
        let hover_active_leave = hover_active.clone();
        let motion_controller = EventControllerMotion::new();
        motion_controller.connect_enter(move |_, _, _| {
            hover_active_enter.set(true);
        });
        motion_controller.connect_leave(move |_| {
            hover_active_leave.set(false);
        });
        window.add_controller(motion_controller);

        let actions = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
        actions.set_valign(gtk4::Align::Center);
        actions.set_halign(gtk4::Align::Center);
        actions.set_size_request(48, 160);
        actions.add_css_class("floating-actions");
        actions.set_margin_start(2);
        actions.set_margin_end(2);

        let edit_button = gtk4::Button::from_icon_name("document-edit-symbolic");
        edit_button.add_css_class("action-button");
        edit_button.set_tooltip_text(Some("Edit"));
        edit_button.set_hexpand(false);
        edit_button.set_size_request(48, 48);
        actions.append(&edit_button);

        let copy_only_button = gtk4::Button::from_icon_name("edit-copy-symbolic");
        copy_only_button.add_css_class("action-button");
        copy_only_button.set_tooltip_text(Some("Copy Only"));
        copy_only_button.set_hexpand(false);
        copy_only_button.set_size_request(48, 48);
        actions.append(&copy_only_button);

        let copy_save_button = gtk4::Button::from_icon_name("document-save-symbolic");
        copy_save_button.add_css_class("action-button");
        copy_save_button.set_tooltip_text(Some("Copy and Save"));
        copy_save_button.set_hexpand(false);
        copy_save_button.set_size_request(48, 48);
        actions.append(&copy_save_button);

        let content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        content.set_margin_start(10);
        content.set_margin_end(10);
        content.set_margin_top(10);
        content.set_margin_bottom(10);
        content.append(&preview);
        content.append(&actions);

        let remaining_seconds = Rc::new(Cell::new(5));
        let remaining_seconds_clone = remaining_seconds.clone();
        let hover_state_timer = hover_active.clone();
        let window_for_timer = window.clone();
        gtk4::glib::timeout_add_local(Duration::from_secs(1), move || {
            if hover_state_timer.get() {
                return gtk4::glib::ControlFlow::Continue;
            }

            let value = remaining_seconds_clone.get() - 1;
            remaining_seconds_clone.set(value);

            if value <= 0 {
                window_for_timer.close();
                return gtk4::glib::ControlFlow::Break;
            }

            gtk4::glib::ControlFlow::Continue
        });

        window.set_default_size(320, 180);
        window.add_css_class("preview-window");
        window.set_child(Some(&content));
        window.present();
    });

    app.run();
}
