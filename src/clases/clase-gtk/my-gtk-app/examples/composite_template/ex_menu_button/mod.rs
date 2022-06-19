mod imp;
use gtk4 as gtk;
use gtk::glib;

glib::wrapper! {
    pub struct ExMenuButton(ObjectSubclass<imp::ExMenuButton>) @extends gtk::Widget;
}
