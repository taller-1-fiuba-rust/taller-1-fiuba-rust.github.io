mod imp;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct ExApplication(ObjectSubclass<imp::ExApplication>) @extends gio::Application, gtk::Application, @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for ExApplication {
    fn default() -> Self {
        Self::new()
    }
}

impl ExApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &"com.github.taller"),
            ("flags", &gio::ApplicationFlags::empty()),
        ])
        .expect("Failed to create Application")
    }
}
