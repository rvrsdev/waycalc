use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Entry};
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
        entry.set_placeholder_text(Some("type here…"));
        entry.connect_changed(|entry| {
            println!("text: {}", entry.text());
        });

        let layout = Box::new(gtk4::Orientation::Vertical, 0);
        layout.append(&entry);

        window.set_child(Some(&layout));

        window.present();
    });

    app.run()
}
