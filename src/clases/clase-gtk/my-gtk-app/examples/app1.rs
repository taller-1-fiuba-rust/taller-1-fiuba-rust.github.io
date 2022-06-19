use gtk4 as gtk;
use gtk::prelude::*;


fn main() {
    let application =
        gtk::Application::new(Some("com.github.taller"), 
        Default::default());
    application.run();
}

