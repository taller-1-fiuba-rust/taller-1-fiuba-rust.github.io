use gtk4 as gtk;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;

// Object holding the state
#[derive(Default)]
pub struct CustomButton {
    number: Cell<i32>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for CustomButton {
    fn constructed(&self, obj: &Self::Type) {
        println!("imp  ObjectImpl  constructed()");
        self.parent_constructed(obj);
        obj.set_label(&self.number.get().to_string());
        obj.set_margin_start(60);
        obj.set_margin_end(60);
        obj.set_margin_top(60);
        obj.set_margin_bottom(60);
    }
}

// Trait shared by all widgets
impl WidgetImpl for CustomButton {}

// Trait shared by all buttons
impl ButtonImpl for CustomButton {
    fn clicked(&self, button: &Self::Type) {
        self.number.set(self.number.get() + 1);
        button.set_label(&self.number.get().to_string())
    }
}
