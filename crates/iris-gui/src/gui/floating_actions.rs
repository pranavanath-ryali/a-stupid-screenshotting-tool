use gtk4::{Box, Button, prelude::BoxExt};

pub fn build_floating_actions_bar() -> Box {
    let floating_action_box = Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .spacing(12)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::End)
        .margin_bottom(24)
        .build();

    let btn_screen = Button::from_icon_name("view-fullscreen-symbolic");
    let btn_region = Button::from_icon_name("applets-screenshooter-symbolic");

    floating_action_box.append(&btn_screen);
    floating_action_box.append(&btn_region);

    floating_action_box
}
