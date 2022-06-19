


use std::cell::Cell;
use std::rc::Rc;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Align, ApplicationWindow, Button, Orientation, Label};

// un poco de repaso de manejo de memoria en rust: ownership mutabilidad y lifestime
fn main() {
    let application =
    gtk::Application::new(Some("com.github.taller"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}
/* v1
fn build_ui(application: &gtk::Application) {

    let window = ApplicationWindow::builder()
    .application(application)
    .title("Little Program")
    .default_width(350)
    .default_height(70)
    .build();

    let number :i32= 0;

    let label =Label::builder()
    .margin_top(10)
    .margin_bottom(10)
    .margin_start(10)
    .margin_end(10)
    .halign(Align::Center)
    .valign(Align::Center)
    .label(&number.to_string())
    .build();

    let button = Button::builder()
    .margin_top(10)
    .margin_bottom(10)
    .margin_start(10)
    .margin_end(10)
    .halign(Align::Center)
    .valign(Align::Center)
    .label("Click me!")
    .build();

    button.connect_clicked(|button| {
        button.set_label(":)");
    });

    window.set_child(Some(&button));
    //window.set_child(Some(&label));
    window.show();
}
 */

/* 
fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::builder()
    .application(application)
    .title("Little Program")
    .default_width(350)
    .default_height(70)
    .build();

    let mut number :i32= 0;

    let label =Label::builder()
    .margin_top(10)
    .margin_bottom(10)
    .margin_start(10)
    .margin_end(10)
    .halign(Align::Center)
    .valign(Align::Center)
    .label(&number.to_string())
    .build();

    let button = Button::builder()
    .margin_top(10)
    .margin_bottom(10)
    .margin_start(10)
    .margin_end(10)
    .halign(Align::Center)
    .valign(Align::Center)
    .label("Click me!")
    .build();

    button.connect_clicked(move |_| {
        number +=1;
        //label.set_label(&number.to_string());
    });

    let gtk_box = gtk::Box::builder()
    .orientation(Orientation::Vertical)
    .build();
    gtk_box.append(&label);
    gtk_box.append(&button);
   
    window.set_child(Some(&gtk_box));
    window.show();
}
*/



fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::builder()
    .application(application)
    .title("Little Program")
    .default_width(350)
    .default_height(70)
    .build();

    let button = Button::builder()
    .margin_top(10)
    .margin_bottom(10)
    .margin_start(10)
    .margin_end(10)
    .halign(Align::Center)
    .valign(Align::Center)
    .label("Click me!")
    .build();

    // necesitamos crearlo como un contador de referencias
    let number = Rc::new(Cell::new(0));

    let label =Label::builder()
    .margin_top(10)
    .margin_bottom(10)
    .margin_start(10)
    .margin_end(10)
    .halign(Align::Center)
    .valign(Align::Center)
    .label(&number.get().to_string())
    .build();

    let number_copy = number.clone(); //clonamos la referencia al numero 
    let label_copy = label.clone(); //para el lebal y otros objetos de gtktenemos que hacer clone directamente porque ya implementan la interfaz
    button.connect_clicked( move |_| {
        number_copy.set(number_copy.get() + 1);
        label_copy.set_label(&number_copy.get().to_string());
    });
    
    let gtk_box = gtk::Box::builder().orientation(Orientation::Vertical).build();
    gtk_box.append(&label);
    gtk_box.append(&button);

    window.set_child(Some(&gtk_box));
    window.show();
}

