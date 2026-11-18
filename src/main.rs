mod calc;

use calc::evaluate;
use gtk4::{Application, ApplicationWindow, Box, CssProvider, Entry, EventControllerKey, Label};
use gtk4::{gdk, prelude::*};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::result::Result::{Err, Ok};
use std::time::Duration;

const APP_ID: &str = "com.rvrsdev.waycalc";

const CSS: &str = r#"
    .waycalc {
        font-size: 20px;
        background-color: #000;
        border: 1px solid #faf9f6;
        padding: 3px;
    }

    .entry {
        color: #faf9f6;
        background-color: #000;
    }

    .result {
        color: #faf9f6;
    }

    .error {
        color: #fa6961;
        font-size: 17px;
        padding: 5px;
    }

"#;

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        load_css();

        let window = ApplicationWindow::new(app);
        set_window(&window);

        let result_label = Label::new(None);
        set_result_label(&result_label);

        let error_label = Label::new(None);
        set_error_label(&error_label);

        let entry = Entry::new();
        set_entry(&entry);

        let window_callback = window.clone();
        let result_label_callback = result_label.clone();
        let error_label_callback = error_label.clone();
        entry.connect_activate(move |entry| {
            let expression = entry.text().to_string();

            if expression.trim().is_empty() {
                return;
            }

            match evaluate(&expression) {
                Ok(result) => {
                    error_label_callback.set_visible(false);

                    let text = &result.to_string();
                    result_label_callback.set_text(text);
                    entry.clipboard().set_text(text);
                }
                Err(error) => {
                    error_label_callback.set_text(&error.to_string());
                    error_label_callback.set_visible(true);

                    // closing dialog after 3 seconds
                    let error_timeout_label = error_label_callback.clone();
                    let window_timeout_callback = window_callback.clone();
                    glib::timeout_add_local_once(Duration::from_secs(3), move || {
                        error_timeout_label.set_visible(false);
                        window_timeout_callback.set_default_size(200, 40);
                    });
                }
            }
        });

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

        let layout = Box::new(gtk4::Orientation::Vertical, 2);
        let main_row = Box::new(gtk4::Orientation::Horizontal, 10);

        main_row.append(&entry);
        main_row.append(&result_label);

        layout.append(&main_row);
        layout.append(&error_label);

        window.set_child(Some(&layout));
        layout.add_css_class("waycalc");

        window.present();
    });

    app.run()
}

fn set_window(window: &ApplicationWindow) {
    window.init_layer_shell();

    window.set_title(Some("waycalc"));
    window.set_default_size(200, 40);
    window.set_layer(Layer::Top);
    window.set_anchor(Edge::Top, true);
    window.set_keyboard_mode(KeyboardMode::OnDemand);
}

fn set_result_label(label: &Label) {
    label.set_size_request(60, 40);
    label.set_hexpand(true);
    label.set_halign(gtk4::Align::End);
    label.set_ellipsize(gdk::pango::EllipsizeMode::None);
    label.add_css_class("result");
}

fn set_error_label(label: &Label) {
    label.set_visible(false);
    label.set_halign(gtk4::Align::Center);
    label.add_css_class("error");
}

fn set_entry(entry: &Entry) {
    entry.set_hexpand(true);
    entry.set_halign(gtk4::Align::Fill);
    entry.add_css_class("entry");
    entry.grab_focus();
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
