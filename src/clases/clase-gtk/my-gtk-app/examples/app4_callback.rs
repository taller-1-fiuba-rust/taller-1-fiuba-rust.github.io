use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Align, ApplicationWindow, Button};

fn main() {
    let application =
    gtk::Application::new(Some("com.github.taller"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {

    let window = ApplicationWindow::builder()
    .application(application)
    .title("Little Program")
    .default_width(350)
    .default_height(70)
    .build();

    let button = Button::builder()
    .margin_top(10)
    .margin_bottom(10)
    .margin_start(10)
    .margin_end(10)
    .halign(Align::Center)
    .valign(Align::Center)
    .label("Click me!")
    .build();

    button.connect_clicked(|button| {
        button.set_label(":)");
    });

    window.set_child(Some(&button));

    window.show();

}
