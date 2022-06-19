mod imp;
use gtk4 as gtk;
use glib::Object;
use gtk::{glib, Orientation};

glib::wrapper! {
    pub struct CustomBox(ObjectSubclass<imp::CustomBox>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomBox {
    pub fn new() -> Self {
        Object::new(&[("orientation",&Orientation::Vertical)]).expect("Failed to create `CustomButton`.")
    }

}

impl Default for CustomBox {
    fn default() -> Self {
        Self::new()
    }
}
