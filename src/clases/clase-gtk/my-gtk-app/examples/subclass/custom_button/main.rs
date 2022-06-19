mod custom_button;

use custom_button::CustomButton;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.github.taller")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    // Create a  cutsom button
    let button = CustomButton::with_label("Click me!");

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Little Program!")
        .default_width(350)
        .default_height(70)
        .child(&button)
        .build();

    // Present window
    window.present();
}
