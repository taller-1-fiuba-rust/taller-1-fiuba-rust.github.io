use gtk4 as gtk;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;

// Object holding the state
#[derive(Default)]
pub struct CustomBox {
    number: Cell<i32>,
    label: gtk::Label,
    button: gtk::Button,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomBox {
    const NAME: &'static str = "MyGtkAppCustomBox";
    type Type = super::CustomBox;
    type ParentType = gtk::Box;
}

// Trait shared by all GObjects
impl ObjectImpl for CustomBox {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
        let _ = &self.label.set_label(&self.number.get().to_string());
        let number_copy = self.number.clone(); 
        let label_copy= self.label.clone(); 
        let _ = &self.button.set_label("click me");
        let _ = &self.button.connect_clicked( move |_| {
            number_copy.set(number_copy.get() + 1);
            label_copy.set_label(&number_copy.get().to_string());
        });
        obj.append(&self.label);
        obj.append(&self.button);
        obj.set_margin_top(12);
        obj.set_margin_bottom(12);
        obj.set_margin_start(12);
        obj.set_margin_end(12);
    }

  
}

// Trait shared by all widgets
impl WidgetImpl for CustomBox{}

// Trait shared by all buttons
impl BoxImpl for CustomBox {}

