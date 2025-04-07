use gtk4::prelude::BoxExt;
use gtk4::{self as gtk, Orientation};

use gtk::glib::ExitCode;
use gtk::prelude::{ApplicationExt, ApplicationExtManual, ButtonExt, GtkWindowExt};
use gtk::{Application, ApplicationWindow, Box, Button};

fn main() -> ExitCode {
    // Create a new application with a unique application ID.
    let app = Application::builder()
        .application_id("com.example.disksync")
        .build();

    // Connect to the "activate" signal, which is emitted when the application starts.
    app.connect_activate(build_ui);

    // Run the application.
    app.run()
}

fn build_ui(app: &Application) {
    // Create a new application window, associating it with the application.
    let window = ApplicationWindow::builder()
        .application(app)
        .title("disksync")
        .default_width(200)
        .build();

    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(16)
        .margin_top(16)
        .margin_start(16)
        .margin_bottom(16)
        .margin_end(16)
        .build();

    // Create a button with an initial label.
    let button1 = Button::with_label("sync");
    let button2 = Button::with_label("umount");
    let button3 = Button::with_label("power-off");

    vbox.append(&button1);
    vbox.append(&button2);
    vbox.append(&button3);

    // When the button is clicked, change its label.
    button1.connect_clicked(|btn| {
        btn.set_label("Hello, World!");
    });

    // Set the window's child to be the button.
    window.set_child(Some(&vbox));

    // Present (display) the window.
    window.present();
}
