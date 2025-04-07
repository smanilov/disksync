use gtk4 as gtk;

use gtk::glib::ExitCode;
use gtk::prelude::{ApplicationExt, ApplicationExtManual, ButtonExt, GtkWindowExt};
use gtk::{Application, ApplicationWindow, Button};

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
        .default_width(400)
        .default_height(300)
        .build();

    // Create a button with an initial label.
    let button = Button::with_label("Click me!");

    // When the button is clicked, change its label.
    button.connect_clicked(|btn| {
        btn.set_label("Hello, World!");
    });

    // Set the window's child to be the button.
    window.set_child(Some(&button));

    // Present (display) the window.
    window.present();
}
