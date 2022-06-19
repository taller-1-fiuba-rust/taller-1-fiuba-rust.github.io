use gtk4 as gtk;
use gtk::prelude::*;
fn main() {
    let application =
    gtk::Application::new(Some("com.github.taller"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

/*
---------------------------------
→ Gobject
    → GTKwidget
        → GTKcontainer
            → GTKbin
                → GTKbutton
----------------------------------
*/

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("Little Program"));
    window.set_default_size(350, 70);

    let button = gtk::Button::with_label("Click me!");
    window.set_child(Some(&button));

    window.show();
}
