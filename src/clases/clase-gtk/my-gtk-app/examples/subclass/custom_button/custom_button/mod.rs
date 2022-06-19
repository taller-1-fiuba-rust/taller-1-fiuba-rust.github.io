mod imp;
use gtk4 as gtk;
use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl CustomButton {
    pub fn new() -> Self {
        println!("mod CustomButton new()");
        Object::new(&[]).expect("Failed to create `CustomButton`.")
    }

    pub fn with_label(label: &str) -> Self {
        println!("mod CustomButton with_label() init");
        Object::new(&[("label", &label)]).expect("Failed to create `CustomButton`.")
    }
}

impl Default for CustomButton {
    fn default() -> Self {
        println!("mod Default default()");
        Self::new()
    }
}
