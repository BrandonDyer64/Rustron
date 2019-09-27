extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let package_name = "com.github.gtk-rs.examples.basic";
    let application = Application::new(Some(package_name), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });
        window.add(&button);

        window.show_all();
    });

    application.run(&[]);
}
