use std::iter::empty;

// enum that tells gtk4 library what to do with an event
use glib::Propagation;
use gtk4::{Application, ApplicationWindow, Box, Entry};
use gtk4::{gdk, prelude::*};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.rvrsdev.waycalc")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.init_layer_shell();

        window.set_title(Some("waycalc"));
        window.set_default_height(40);
        window.set_default_width(200);
        window.set_layer(Layer::Top);
        window.set_anchor(Edge::Top, true);
        window.set_keyboard_mode(KeyboardMode::OnDemand);

        let entry = Entry::new();
        {
            let window_clone = window.clone();
            entry.connect_activate(move |entry| {
                let expression = entry.text().to_string();

                if expression.is_empty() {
                    return;
                }

                let res = 55;
                println!("Result: {res}");
                window_clone.close();
            });
        }

        let layout = Box::new(gtk4::Orientation::Vertical, 10);
        layout.append(&entry);

        window.set_child(Some(&layout));

        window.present();
    });

    app.run()
}
