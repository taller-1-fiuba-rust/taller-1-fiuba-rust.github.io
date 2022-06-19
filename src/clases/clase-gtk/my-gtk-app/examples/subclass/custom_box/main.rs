mod custom_box;

use custom_box::CustomBox;
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
    // Create a button
    let custom_box = CustomBox::new();
   

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Little Program!")
        .default_width(350)
        .default_height(70)
        .child(&custom_box)
        .build();

    // Present window
    window.present();
}
