use gtk4 as gtk;

use gtk::gio::prelude::ListModelExtManual; // for observe_children().iter()
use gtk::glib::object::Cast; // for child.unwrap().downcast
use gtk::glib::{ExitCode, Object};
use gtk::prelude::{
    ApplicationExt,       // for app.connect_activate()
    ApplicationExtManual, // for app.run()
    BoxExt,               // for vbox.append()
    ButtonExt,            // for button.connect_clicked()
    EditableExt,          // for entry.text()
    GtkWindowExt,         // for window.present()
    WidgetExt,            // for button.set_sensitive()
};
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Entry, Orientation};

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

    // Create a vertical box and add margins.
    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(16)
        .margin_top(16)
        .margin_start(16)
        .margin_bottom(16)
        .margin_end(16)
        .build();

    // Create main widgets: one entry box and three buttons.
    let entry_device = Entry::builder().placeholder_text("/dev/sdX3").build();
    entry_device.set_alignment(0.5f32); // sadly, not in builder
    let button_sync = Button::with_label("sync");
    let button_umount = Button::with_label("umount");
    let button_power_off = Button::with_label("power-off");

    // Append components to vertical box.
    vbox.append(&entry_device);
    vbox.append(&button_sync);
    vbox.append(&button_umount);
    vbox.append(&button_power_off);

    // Connect handlers.
    button_sync.connect_clicked(|button| handle_sync_click(button));
    button_umount.connect_clicked(|button| handle_umount_click(button));
    button_power_off.connect_clicked(|button| handle_power_off_click(button));

    // Set the window's child to be the vertical box.
    window.set_child(Some(&vbox));

    // Present (display) the window.
    window.present();
}

/// Assume there is an Entry that is a sibling to the given button and return its text.
fn get_device_name(button: &Button) -> Option<String> {
    let mut device_name: Option<String> = None;

    for child in button.parent().unwrap().observe_children().iter::<Object>() {
        if let Ok(entry) = child.unwrap().downcast::<Entry>() {
            device_name = Some(entry.text().to_string());
            break;
        }
    }

    device_name
}

use std::process::Command;
use std::thread;

fn handle_sync_click(button: &Button) {
    button.set_sensitive(false);

    thread::spawn(move || {
        match Command::new("sync").status() {
            Ok(status) => {
                if status.success() {
                    // TODO: display in the UI too
                    println!("sync completed successfully");
                } else {
                    eprintln!("sync exited with status: {:?}", status.code());
                }
            }
            Err(e) => {
                eprintln!("Failed to run sync: {}", e);
            }
        };

        // TODO: Re-enable button.
        // idle_add(move || {
        //     button.set_sensitive(true);
        //     ControlFlow::Break
        // });
    });
}

fn handle_umount_click(button: &Button) {
    let device_name = if let Some(device_name) = get_device_name(button) {
        device_name
    } else {
        // TODO: use tracing::info and tracing::error
        eprintln!("Failed to find Entry widget");
        return;
    };

    button.set_sensitive(false);

    thread::spawn(move || {
        match Command::new("umount").arg(device_name).status() {
            Ok(status) => {
                if status.success() {
                    println!("umount completed successfully");
                } else {
                    eprintln!("umount exited with status: {:?}", status.code());
                }
            }
            Err(e) => {
                eprintln!("Failed to run umount: {}", e);
            }
        };

        // TODO: Re-enable button.
        // idle_add(move || {
        //     button.set_sensitive(true);
        //     ControlFlow::Break
        // });
    });
}

fn handle_power_off_click(button: &Button) {
    let device_name = if let Some(device_name) = get_device_name(button) {
        device_name
    } else {
        // TODO: use tracing::info and tracing::error
        eprintln!("Failed to find Entry widget");
        return;
    };

    button.set_sensitive(false);

    thread::spawn(move || {
        match Command::new("udisksctl")
            .args(["power-off", "-b"])
            .arg(device_name)
            .status()
        {
            Ok(status) => {
                if status.success() {
                    println!("udisksctl power-off completed successfully");
                } else {
                    eprintln!(
                        "udisksctl power-off exited with status: {:?}",
                        status.code()
                    );
                }
            }
            Err(e) => {
                eprintln!("Failed to run udisksctl power-off: {}", e);
            }
        };

        // TODO: Re-enable button.
        // idle_add(move || {
        //     button.set_sensitive(true);
        //     ControlFlow::Break
        // });
    });
}
