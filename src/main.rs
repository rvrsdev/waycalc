mod calc;

use calc::evaluate;
use gtk4::{Application, ApplicationWindow, Box, CssProvider, Entry, EventControllerKey, Label};
use gtk4::{gdk, prelude::*};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::result::Result::{Err, Ok};

const APP_ID: &str = "com.rvrsdev.waycalc";

const CSS: &str = r#"
    .waycalc {
        background-color: #000;
        border: 1px solid #faf9f6;
    }

    .waycalc_entry {
        padding: 5px;
    }

    .waycalc_result {
        padding: 0px 6px;
    }
"#;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        load_css();

        let window = ApplicationWindow::new(app);
        window.init_layer_shell();

        window.set_title(Some("waycalc"));
        window.set_size_request(200, 40);
        window.set_layer(Layer::Top);
        window.set_anchor(Edge::Top, true);
        window.set_keyboard_mode(KeyboardMode::OnDemand);

        let label = Label::new(None);
        label.set_size_request(60, 40);
        label.set_hexpand(false);
        label.set_halign(gtk4::Align::End);
        label.set_direction(gtk4::TextDirection::Rtl);
        label.set_ellipsize(gdk::pango::EllipsizeMode::None);

        let entry = Entry::new();
        entry.set_hexpand(true);
        entry.set_halign(gtk4::Align::Fill);

        setup_entry(&entry, &label);

        let keys_controller = EventControllerKey::new();
        {
            let window_callback = window.clone();

            keys_controller.connect_key_pressed(move |_, key, _, _| {
                if key == gdk::Key::Escape {
                    window_callback.close();
                    glib::Propagation::Stop
                } else {
                    glib::Propagation::Proceed
                }
            });
        }
        window.add_controller(keys_controller);

        let layout = Box::new(gtk4::Orientation::Horizontal, 10);

        layout.append(&entry);
        layout.append(&label);

        window.set_child(Some(&layout));

        layout.add_css_class("waycalc");
        entry.add_css_class("waycalc_entry");
        label.add_css_class("waycalc_result");

        entry.grab_focus();
        window.present();
    });

    app.run()
}

fn load_css() {
    let provider = CssProvider::new();

    provider.load_from_data(CSS);
    if let Some(display) = gdk::Display::default() {
        gtk4::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn setup_entry(entry: &Entry, label: &Label) {
    let label_clone = label.clone();

    entry.connect_activate(move |entry| {
        let expression = entry.text().to_string();

        if expression.trim().is_empty() {
            return;
        }

        match evaluate(&expression) {
            Ok(result) => {
                let text = &result.to_string();
                label_clone.set_text(text);
                entry.clipboard().set_text(text);
            }
            Err(error) => {
                label_clone.set_text(&format!("{}", error));
            }
        }
    });
}
