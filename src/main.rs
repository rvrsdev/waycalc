mod calc;

use calc::evaluate;
use gtk4::{
    Align, Application, ApplicationWindow, Box, CssProvider, Entry, EventControllerKey, Label,
};
use gtk4::{gdk, prelude::*};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::result::Result::{Err, Ok};

const APP_ID: &str = "com.rvrsdev.waycalc";

const CSS: &str = r#"
    .waycalc {
        background-color: #000;
    }

    .waycalc-entry {
        border-bottom: 2px solid #222;
    }
"#;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        build_ui(app);
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

fn build_ui(app: &Application) {
    load_css();

    let window = ApplicationWindow::new(app);
    window.init_layer_shell();

    window.set_title(Some("waycalc"));
    window.set_default_height(40);
    window.set_default_width(200);
    window.set_layer(Layer::Top);
    window.set_anchor(Edge::Top, true);
    window.set_keyboard_mode(KeyboardMode::OnDemand);

    let label = Label::new(None);
    label.set_halign(Align::Start);

    let entry = Entry::new();
    {
        let label_clone = label.clone();

        entry.connect_activate(move |entry| {
            let expression = entry.text().to_string();

            if expression.trim().is_empty() {
                return;
            }

            match evaluate(&expression) {
                Ok(result) => {
                    label_clone.set_text(&format!("res: {}", result));
                }
                Err(error) => {
                    label_clone.set_text(&format!("err: {}", error));
                }
            }
        });
    }

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

    let layout = Box::new(gtk4::Orientation::Vertical, 10);

    layout.append(&entry);
    layout.append(&label);

    window.set_child(Some(&layout));
    entry.grab_focus();

    layout.add_css_class("waycalc");
    window.add_css_class("waycalc");
    entry.add_css_class("waycalc-entry");

    window.present();
}
